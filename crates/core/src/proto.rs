use prost::Message;

// missing: framerate, encoding, quality, bitrate, background recording

/// Schema of files located in
/// `$STEAM/userdata/$USER_ID/gamerecordings/clips/clip_${APPID}_${DATE_CLIP}_${TIME_CLIP}/clip.pb`
#[derive(Message, Clone, Eq, PartialEq)]
pub struct Clip {
    #[prost(message, optional, tag = "1")]
    pub timeline: Option<Timeline>,

    #[prost(int32, tag = "2")]
    pub unknown2: i32,

    /// Timestamp of `${DATE_CLIP}_${TIME_CLIP}`.
    /// Similar to [`Timeline::timeline`], but a little bit earlier than that.
    #[prost(int32, tag = "3")]
    pub timestamp: i32,

    #[prost(int32, tag = "4")]
    pub appid: i32,

    /// video size?
    #[prost(int32, tag = "6")]
    pub unknown6: i32,

    #[prost(string, tag = "7")]
    pub clip_title: String,

    /// always 0
    #[prost(int32, tag = "8")]
    pub unknown8: i32,

    /// it's not always correct
    #[prost(int32, tag = "12")]
    pub width: i32,

    /// it's not always correct
    #[prost(int32, tag = "13")]
    pub height: i32,
}

#[derive(Message, Clone, Eq, PartialEq)]
pub struct Timeline {
    /// Format: `"timeline_${APPID}${DATE_META}_${TIME_META}"` (no `_` after `${APPID}`)
    #[prost(string, tag = "1")]
    pub timeline: String,

    #[prost(int32, tag = "2")]
    pub appid: i32,

    /// Timestamp of `${DATE_META}_${TIME_META}`
    #[prost(int32, tag = "3")]
    pub timestamp: i32,

    /// always similar but >= [Video::length_ms]
    #[prost(int32, tag = "4")]
    pub length_ms: i32,

    #[prost(message, optional, tag = "5")]
    pub video: Option<Video>,
}

#[derive(Message, Clone, Eq, PartialEq)]
pub struct Video {
    /// Format: `"fg_$APPID_${DATE_VIDEO}_${TIME_VIDEO}"`
    #[prost(string, tag = "1")]
    pub name: String,

    /// always fit in 8bit
    #[prost(int32, tag = "2")]
    pub unknown2: i32,

    #[prost(int32, tag = "3")]
    pub length_ms: i32,

    /// always 4
    #[prost(int32, tag = "4")]
    pub unknown4: i32,

    /// always similar to Clip.unknown2
    #[prost(int32, tag = "10")]
    pub unknown10: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const SAMPLE_CLIP: &[u8] = &[
        0x0a, 0x58, 0x0a, 0x1e, 0x74, 0x69, 0x6d, 0x65, 0x6c, 0x69, 0x6e, 0x65,
        0x5f, 0x36, 0x39, 0x30, 0x37, 0x39, 0x30, 0x32, 0x30, 0x32, 0x35, 0x30,
        0x33, 0x31, 0x30, 0x5f, 0x30, 0x30, 0x34, 0x33, 0x35, 0x32, 0x10, 0xe6,
        0x94, 0x2a, 0x18, 0xc8, 0xeb, 0xb8, 0xbe, 0x06, 0x20, 0xe8, 0xd5, 0x15,
        0x2a, 0x28, 0x0a, 0x19, 0x66, 0x67, 0x5f, 0x36, 0x39, 0x30, 0x37, 0x39,
        0x30, 0x5f, 0x32, 0x30, 0x32, 0x35, 0x30, 0x33, 0x31, 0x30, 0x5f, 0x30,
        0x30, 0x34, 0x35, 0x32, 0x35, 0x10, 0xd3, 0x01, 0x18, 0x95, 0xd4, 0x15,
        0x20, 0x04, 0x50, 0xce, 0xd1, 0x05, 0x10, 0xfb, 0xcf, 0x05, 0x18, 0xa4,
        0xec, 0xb8, 0xbe, 0x06, 0x20, 0xe6, 0x94, 0x2a, 0x30, 0xca, 0xb2, 0x9a,
        0x82, 0x02, 0x40, 0x00, 0x60, 0x80, 0x0f, 0x68, 0xb8, 0x08,
    ];

    #[test]
    fn smoke_test_proto() {
        let clip = Clip::decode(SAMPLE_CLIP).expect("Failed to decode clip");
        assert_eq!(
            clip,
            Clip {
                timeline: Some(Timeline {
                    timeline: String::from("timeline_69079020250310_004352"),
                    appid: 690790,
                    timestamp: 1741567432,
                    length_ms: 355048,
                    video: Some(Video {
                        name: String::from("fg_690790_20250310_004525"),
                        unknown2: 211,
                        length_ms: 354837,
                        unknown4: 4,
                        unknown10: 92366
                    })
                }),
                unknown2: 92155,
                timestamp: 1741567524,
                appid: 690790,
                unknown6: 541497674,
                clip_title: String::new(),
                unknown8: 0,
                width: 1920,
                height: 1080
            }
        )
    }
}
