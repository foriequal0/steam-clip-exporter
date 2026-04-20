use path_slash::PathBufExt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn ffmpeg(session: &Path, dst: &Path) -> Result<(), std::io::Error> {
    let result = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(to_forward_slash_for_windows(session))
        .args(["-codec", "copy"])
        .arg(to_forward_slash_for_windows(dst))
        .spawn();
    match result {
        Ok(mut child) => {
            child.wait()?;
            Ok(())
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "'ffmpeg' not found in PATH, or failed to execute",
                ))
            } else {
                Err(err)
            }
        }
    }
}

fn to_forward_slash_for_windows(path: &Path) -> PathBuf {
    if !cfg!(target_os = "windows") {
        return path.to_owned();
    }

    PathBuf::from_slash_lossy(path)
}
