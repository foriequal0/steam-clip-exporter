use crate::proto::Video;
use eyre::{Context, Result, eyre};
use glob::{Pattern, glob};
use regex::Regex;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Ancestors, Path, PathBuf};

pub fn get_default_steam_root_dir() -> PathBuf {
    if let Ok(steam_root) = std::env::var("STEAM_ROOT") {
        return PathBuf::from(&steam_root);
    }

    get_default_steam_root_dir_impl()
}

fn get_default_steam_root_dir_impl() -> PathBuf {
    // TODO: cfg_select!
    #[cfg(target_os = "windows")]
    {
        return PathBuf::from("C:/Program Files (x86)/Steam");
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".local/share/Steam");
        }

        panic!("environment variable $HOME is not set");
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    compile_error!("Unsupported platform: {}", std::env::consts::OS);
}

pub struct SteamRoot {
    root: PathBuf,
}

impl Default for SteamRoot {
    fn default() -> Self {
        Self::new()
    }
}

impl SteamRoot {
    pub fn new() -> Self {
        let root = get_default_steam_root_dir();
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn join(&self, path: impl AsRef<Path>) -> PathBuf {
        self.root.join(path.as_ref())
    }

    pub fn userdata_dir_path(&self) -> PathBuf {
        self.root.join("userdata")
    }

    pub fn localconfig_paths(&self) -> Result<Vec<PathBuf>> {
        let Some(root) = self.root.to_str() else {
            return Err(eyre!("Steam root contains non-utf8 characters"));
        };

        let mut result = Vec::new();
        let root = Pattern::escape(root);
        let pattern = format!("{}/userdata/*/config/localconfig.vdf", root);
        for path in glob(&pattern).context("Failed to glob")?.flatten() {
            result.push(path);
        }

        Ok(result)
    }
}

pub struct GameRecordingsRoots {
    roots: Vec<GameRecordingsRoot>,
}

impl GameRecordingsRoots {
    pub fn read_from_localconfig_paths(
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self> {
        let mut roots = Vec::new();
        for path in paths {
            if let Ok(root) = GameRecordingsRoot::read_from_localconfig(path.as_ref()) {
                roots.push(root);
            }
        }
        Ok(Self { roots })
    }

    pub fn clips_dir_paths(&self) -> Result<Vec<PathBuf>> {
        let mut result = Vec::new();
        for root in &self.roots {
            result.push(root.clips_dir_path());
        }
        Ok(result)
    }

    pub fn clip_paths(&self) -> Result<Vec<ClipPath>> {
        let mut result = Vec::new();
        for root in &self.roots {
            let paths = root.clip_paths()?;
            result.extend(paths);
        }

        Ok(result)
    }
}

pub struct GameRecordingsRoot {
    root: PathBuf,
}

impl GameRecordingsRoot {
    fn read_from_localconfig(localconfig: &Path) -> Result<Self> {
        let root = if let Some(manual_path) = parse_localconfig(localconfig)? {
            PathBuf::from(manual_path)
        } else {
            let per_user = localconfig
                .parent()
                .expect("config dir")
                .parent()
                .expect("per_user");
            per_user.join("gamerecordings")
        };

        Ok(Self { root })
    }
}

fn parse_localconfig(path: &Path) -> Result<Option<String>> {
    let regex =
        Regex::new(r#"^\s*"BackgroundRecordPath"\s*"(?<path>.*?)"\s*$"#).expect("Invalid regex");

    let file = BufReader::new(File::open(path).context("localconfig.vdf")?);

    for line in file.lines() {
        let line = line?;
        if let Some(captures) = regex.captures(&line) {
            let path = captures.name("path").expect("captured").as_str();
            let unescaped = unescape(path);
            if unescaped.is_empty() {
                break;
            }

            return Ok(Some(unescaped));
        }
    }

    Ok(None)
}

fn unescape(s: &str) -> String {
    let mut result = String::new();
    let mut it = s.chars();
    while let Some(ch) = it.next() {
        if ch == '\\' {
            let sequence = it.next().expect("escape sequence");
            let unescaped = sequence;
            // TODO: handle control chars?
            result.push(unescaped)
        } else {
            result.push(ch)
        }
    }

    result
}

impl GameRecordingsRoot {
    pub fn clips_dir_path(&self) -> PathBuf {
        self.root.join("clips")
    }

    pub fn clip_paths(&self) -> Result<Vec<ClipPath>> {
        let mut result = Vec::new();
        let Some(gamerecordings) = self.root.to_str() else {
            return Err(eyre!("gamerecordings root contains non-utf8 characters"));
        };

        let gamerecordings = Pattern::escape(gamerecordings);
        let pattern = format!("{}/clips/*/clip.pb", gamerecordings);
        for path in glob(&pattern).context("Failed to glob")?.flatten() {
            let Ok(file) = ClipPath::try_from(path.as_path()) else {
                continue;
            };

            result.push(file);
        }

        Ok(result)
    }
}

/// Parsed form of a path that contains `clip.pb` file.
/// ```ignore
/// use std::path::PathBuf;
///
/// let path = Path::from(
///     "/home/user/.local/share/Steam/userdata/123456789/gamerecordings/clips/clip_1234567_20260102_030405/clip.pb",
/// );
/// let clip_path = ClipPath::try_from(path).unwrap();
/// assert!(clip_path, ClipPath {
///    root: PathBuf::from_str("/home/user/.local/share/Steam/userdata/123456789/gamerecordings/clips/clip_1234567_20260102_030405"),
///    user_id: "123456789".to_owned(),
///    clip_id: "clip_1234567_20260102_030405".to_owned(),
/// });
/// ```
#[derive(Debug, Clone)]
pub struct ClipPath {
    /// root directory that contains `clip.pb`
    root: PathBuf,

    /// equivalent to `basename(self.root)`
    clip_id: String,
}

impl TryFrom<&Path> for ClipPath {
    type Error = eyre::Report;

    fn try_from(path: &Path) -> Result<Self> {
        fn next_segment<'a>(ancestors: &mut Ancestors<'a>) -> Option<&'a OsStr> {
            let ancestor = ancestors.next()?;
            ancestor.file_name()
        }

        fn check_segment<T: AsRef<OsStr>>(ancestors: &mut Ancestors, expected: &T) -> Result<()> {
            let expected = expected.as_ref();
            let segment = next_segment(ancestors);
            if segment != Some(expected) {
                return Err(eyre!(
                    "Assertion: expected '{}', but got None",
                    expected.display()
                ));
            }
            Ok(())
        }

        fn parse(path: &Path) -> Result<ClipPath> {
            let mut ancestors = path.ancestors();
            check_segment(&mut ancestors, &"clip.pb")?;

            let root = ancestors
                .next()
                .ok_or_else(|| eyre!("no CLIP_ID segment"))?;

            let clip_id = root
                .file_name()
                .ok_or_else(|| eyre!("CLIP_ID segment should have a filename part"))?
                .to_str()
                .ok_or_else(|| eyre!("CLIP_ID segment contains non-utf8 characters"))?;

            check_segment(&mut ancestors, &"clips")?;

            Ok(ClipPath {
                root: root.to_owned(),
                clip_id: clip_id.to_owned(),
            })
        }

        parse(path).with_context(|| format!("Failed to parse clip file path: {}", path.display()))
    }
}
impl ClipPath {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn clip_id(&self) -> &str {
        &self.clip_id
    }

    pub fn clip_pb(&self) -> PathBuf {
        self.root.join("clip.pb")
    }

    pub fn thumbnail_jpg(&self) -> PathBuf {
        self.root.join("thumbnail.jpg")
    }

    pub fn session_mpd(&self, video: &Video) -> PathBuf {
        let mut path = self.root.join("video");
        path.push(&video.name);
        path.push("session.mpd");
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_file() {
        let path = PathBuf::from(
            "/home/user/.local/share/Steam/userdata/123456789/gamerecordings/clips/clip_1234567_20260102_030405/clip.pb",
        );
        let clip_file = ClipPath::try_from(path.as_path()).expect("Should be Ok");
        assert_eq!(
            &clip_file.root,
            Path::new(
                "/home/user/.local/share/Steam/userdata/123456789/gamerecordings/clips/clip_1234567_20260102_030405",
            )
        );
        assert_eq!(clip_file.clip_id, "clip_1234567_20260102_030405");
    }
}
