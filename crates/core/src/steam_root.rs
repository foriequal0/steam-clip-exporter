use std::ffi::OsStr;
use std::path::{Ancestors, Path, PathBuf};

use crate::proto::Video;
use eyre::{Context, Result, eyre};
use glob::{Pattern, glob};
use xdg::BaseDirectories;

pub fn get_default_steam_root_dir() -> PathBuf {
    let basedirs = BaseDirectories::new();
    let data_home = basedirs
        .get_data_home()
        .expect("environment variable $HOME is not set");

    data_home.join("Steam")
}

pub struct SteamRoot {
    root: PathBuf,
}

impl SteamRoot {
    pub fn new() -> Self {
        let root = get_default_steam_root_dir();
        Self { root }
    }

    pub fn clip_paths(&self) -> Result<Vec<ClipPath>> {
        let Some(root) = self.root.to_str() else {
            return Err(eyre!("Steam root contains non-utf8 characters"));
        };

        let root = Pattern::escape(root);
        let pattern = format!("{}/userdata/*/gamerecordings/clips/*/clip.pb", root);

        let mut result = Vec::new();

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
            check_segment(&mut ancestors, &"gamerecordings")?;

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
