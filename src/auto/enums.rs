// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ a7c320739d65)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtuberAdaptiveStreamManifest")]
pub enum AdaptiveStreamManifest {
    #[doc(alias = "GTUBER_ADAPTIVE_STREAM_MANIFEST_UNKNOWN")]
    Unknown,
    #[doc(alias = "GTUBER_ADAPTIVE_STREAM_MANIFEST_DASH")]
    Dash,
    #[doc(alias = "GTUBER_ADAPTIVE_STREAM_MANIFEST_HLS")]
    Hls,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for AdaptiveStreamManifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AdaptiveStreamManifest::{}", match *self {
            Self::Unknown => "Unknown",
            Self::Dash => "Dash",
            Self::Hls => "Hls",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for AdaptiveStreamManifest {
    type GlibType = ffi::GtuberAdaptiveStreamManifest;

    fn into_glib(self) -> ffi::GtuberAdaptiveStreamManifest {
        match self {
            Self::Unknown => ffi::GTUBER_ADAPTIVE_STREAM_MANIFEST_UNKNOWN,
            Self::Dash => ffi::GTUBER_ADAPTIVE_STREAM_MANIFEST_DASH,
            Self::Hls => ffi::GTUBER_ADAPTIVE_STREAM_MANIFEST_HLS,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtuberAdaptiveStreamManifest> for AdaptiveStreamManifest {
    unsafe fn from_glib(value: ffi::GtuberAdaptiveStreamManifest) -> Self {
        match value {
            ffi::GTUBER_ADAPTIVE_STREAM_MANIFEST_UNKNOWN => Self::Unknown,
            ffi::GTUBER_ADAPTIVE_STREAM_MANIFEST_DASH => Self::Dash,
            ffi::GTUBER_ADAPTIVE_STREAM_MANIFEST_HLS => Self::Hls,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for AdaptiveStreamManifest {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtuber_adaptive_stream_manifest_get_type()) }
    }
}

impl glib::value::ValueType for AdaptiveStreamManifest {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AdaptiveStreamManifest {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AdaptiveStreamManifest {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtuberStreamMimeType")]
pub enum StreamMimeType {
    #[doc(alias = "GTUBER_STREAM_MIME_TYPE_UNKNOWN")]
    Unknown,
    #[doc(alias = "GTUBER_STREAM_MIME_TYPE_VIDEO_MP4")]
    VideoMp4,
    #[doc(alias = "GTUBER_STREAM_MIME_TYPE_AUDIO_MP4")]
    AudioMp4,
    #[doc(alias = "GTUBER_STREAM_MIME_TYPE_VIDEO_WEBM")]
    VideoWebm,
    #[doc(alias = "GTUBER_STREAM_MIME_TYPE_AUDIO_WEBM")]
    AudioWebm,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for StreamMimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StreamMimeType::{}", match *self {
            Self::Unknown => "Unknown",
            Self::VideoMp4 => "VideoMp4",
            Self::AudioMp4 => "AudioMp4",
            Self::VideoWebm => "VideoWebm",
            Self::AudioWebm => "AudioWebm",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for StreamMimeType {
    type GlibType = ffi::GtuberStreamMimeType;

    fn into_glib(self) -> ffi::GtuberStreamMimeType {
        match self {
            Self::Unknown => ffi::GTUBER_STREAM_MIME_TYPE_UNKNOWN,
            Self::VideoMp4 => ffi::GTUBER_STREAM_MIME_TYPE_VIDEO_MP4,
            Self::AudioMp4 => ffi::GTUBER_STREAM_MIME_TYPE_AUDIO_MP4,
            Self::VideoWebm => ffi::GTUBER_STREAM_MIME_TYPE_VIDEO_WEBM,
            Self::AudioWebm => ffi::GTUBER_STREAM_MIME_TYPE_AUDIO_WEBM,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtuberStreamMimeType> for StreamMimeType {
    unsafe fn from_glib(value: ffi::GtuberStreamMimeType) -> Self {
        match value {
            ffi::GTUBER_STREAM_MIME_TYPE_UNKNOWN => Self::Unknown,
            ffi::GTUBER_STREAM_MIME_TYPE_VIDEO_MP4 => Self::VideoMp4,
            ffi::GTUBER_STREAM_MIME_TYPE_AUDIO_MP4 => Self::AudioMp4,
            ffi::GTUBER_STREAM_MIME_TYPE_VIDEO_WEBM => Self::VideoWebm,
            ffi::GTUBER_STREAM_MIME_TYPE_AUDIO_WEBM => Self::AudioWebm,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for StreamMimeType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtuber_stream_mime_type_get_type()) }
    }
}

impl glib::value::ValueType for StreamMimeType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for StreamMimeType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for StreamMimeType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

