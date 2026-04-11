use std::path::PathBuf;

use eyre::{Context, Result, eyre};
use prost::Message;

use crate::proto::Clip;
use crate::steam_root::ClipPath;

#[derive(Debug)]
pub struct ClipInfo {
    pub clip_path: ClipPath,
    pub session_mpd: PathBuf,
    pub title: String,
    pub timestamp: i32,
    pub appid: String,
    pub length_ms: Option<i32>,
    pub resolution: ClipResolution,
}

#[derive(Debug)]
pub struct ClipResolution {
    pub width: i32,
    pub height: i32,
}

impl ClipInfo {
    pub fn load(clip_path: &ClipPath) -> Result<ClipInfo> {
        fn load(clip_path: &ClipPath) -> Result<ClipInfo> {
            let pb = std::fs::read(clip_path.clip_pb())
                .with_context(|| eyre!("Failed to read clip file",))?;

            let proto =
                Clip::decode(pb.as_slice()).with_context(|| eyre!("Failed to parse clip file"))?;

            let title = if is_whitespace(&proto.clip_title) {
                clip_path.clip_id().to_owned()
            } else {
                proto.clip_title.clone()
            };

            let length_ms = proto.timeline.as_ref().map(|x| x.length_ms);

            let session_mpd = {
                let video = proto
                    .timeline
                    .as_ref()
                    .ok_or_else(|| eyre!("Clip timeline is missing"))?
                    .video
                    .as_ref()
                    .ok_or_else(|| eyre!("Clip timeline video is missing"))?;

                clip_path.session_mpd(&video)
            };

            Ok(ClipInfo {
                clip_path: clip_path.clone(),
                title,
                session_mpd,
                timestamp: proto.timestamp,
                appid: proto.appid.to_string(),
                length_ms,
                resolution: ClipResolution {
                    width: proto.width,
                    height: proto.height,
                },
            })
        }

        load(clip_path)
            .with_context(|| eyre!("Failed to load clip info in {}", clip_path.clip_id()))
    }
}

fn is_whitespace(str: &str) -> bool {
    str.chars().all(char::is_whitespace)
}
