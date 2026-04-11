use std::path::Path;
use std::process::Command;

pub fn vlc(session: &Path, dst: &Path) -> Result<(), std::io::Error> {
    let mut child = Command::new("cvlc")
        .arg(session)
        .arg("--play-and-exit")
        .arg("--sout")
        .arg("#std{access=file,mux=mp4}")
        .arg("--sout-standard-dst")
        .arg(dst)
        .spawn()?;
    child.wait()?;
    Ok(())
}
