// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ 0ac89bf4906a)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

mod adaptive_stream;
pub use self::adaptive_stream::{AdaptiveStream};

mod client;
pub use self::client::{Client};

mod manifest_generator;
pub use self::manifest_generator::{ManifestGenerator};

mod media_info;
pub use self::media_info::{MediaInfo};

mod stream;
pub use self::stream::{Stream, NONE_STREAM};

mod enums;
pub use self::enums::AdaptiveStreamManifest;
pub use self::enums::StreamMimeType;

#[doc(hidden)]
pub mod traits {
    pub use super::stream::StreamExt;
}
