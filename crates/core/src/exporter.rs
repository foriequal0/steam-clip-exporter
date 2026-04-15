use std::path::Path;
use std::process::Command;

pub fn ffmpeg(session: &Path, dst: &Path) -> Result<(), std::io::Error> {
    let mut child = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(session)
        .args(["-codec", "copy"])
        .arg(dst)
        .spawn()?;
    child.wait()?;
    Ok(())
}
