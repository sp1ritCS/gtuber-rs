// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ f6573a461c85)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use glib_sys as glib;
use gio_sys as gio;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GtuberAdaptiveStreamManifest = c_int;
pub const GTUBER_ADAPTIVE_STREAM_MANIFEST_UNKNOWN: GtuberAdaptiveStreamManifest = 0;
pub const GTUBER_ADAPTIVE_STREAM_MANIFEST_DASH: GtuberAdaptiveStreamManifest = 1;
pub const GTUBER_ADAPTIVE_STREAM_MANIFEST_HLS: GtuberAdaptiveStreamManifest = 2;

pub type GtuberClientError = c_int;
pub const GTUBER_CLIENT_ERROR_NO_PLUGIN: GtuberClientError = 0;
pub const GTUBER_CLIENT_ERROR_MISSING_INFO: GtuberClientError = 1;

pub type GtuberFlow = c_int;
pub const GTUBER_FLOW_OK: GtuberFlow = 0;
pub const GTUBER_FLOW_ERROR: GtuberFlow = 1;
pub const GTUBER_FLOW_RESTART: GtuberFlow = 2;

pub type GtuberManifestGeneratorError = c_int;
pub const GTUBER_MANIFEST_GENERATOR_ERROR_NO_DATA: GtuberManifestGeneratorError = 0;

pub type GtuberStreamMimeType = c_int;
pub const GTUBER_STREAM_MIME_TYPE_UNKNOWN: GtuberStreamMimeType = 0;
pub const GTUBER_STREAM_MIME_TYPE_VIDEO_MP4: GtuberStreamMimeType = 1;
pub const GTUBER_STREAM_MIME_TYPE_AUDIO_MP4: GtuberStreamMimeType = 2;
pub const GTUBER_STREAM_MIME_TYPE_VIDEO_WEBM: GtuberStreamMimeType = 3;
pub const GTUBER_STREAM_MIME_TYPE_AUDIO_WEBM: GtuberStreamMimeType = 4;

pub type GtuberWebsiteError = c_int;
pub const GTUBER_WEBSITE_ERROR_REQUEST_CREATE_FAILED: GtuberWebsiteError = 0;
pub const GTUBER_WEBSITE_ERROR_PARSE_FAILED: GtuberWebsiteError = 1;
pub const GTUBER_WEBSITE_ERROR_OTHER: GtuberWebsiteError = 2;

// Constants
pub const GTUBER_MAJOR_VERSION: c_int = 0;
pub const GTUBER_MICRO_VERSION: c_int = 1;
pub const GTUBER_MINOR_VERSION: c_int = 0;
pub const GTUBER_VERSION_S: *const c_char = b"0.0.1\0" as *const u8 as *const c_char;

// Callbacks
pub type GtuberAdaptiveStreamFilter = Option<unsafe extern "C" fn(*mut GtuberAdaptiveStream, gpointer) -> gboolean>;
pub type GtuberFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer)>;

// Records
#[repr(C)]
pub struct _GtuberAdaptiveStreamClass(c_void);

pub type GtuberAdaptiveStreamClass = *mut _GtuberAdaptiveStreamClass;

#[repr(C)]
pub struct _GtuberClientClass(c_void);

pub type GtuberClientClass = *mut _GtuberClientClass;

#[repr(C)]
pub struct _GtuberManifestGeneratorClass(c_void);

pub type GtuberManifestGeneratorClass = *mut _GtuberManifestGeneratorClass;

#[repr(C)]
pub struct _GtuberMediaInfoClass(c_void);

pub type GtuberMediaInfoClass = *mut _GtuberMediaInfoClass;

#[repr(C)]
pub struct _GtuberStreamClass(c_void);

pub type GtuberStreamClass = *mut _GtuberStreamClass;

// Classes
#[repr(C)]
pub struct GtuberAdaptiveStream(c_void);

impl ::std::fmt::Debug for GtuberAdaptiveStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GtuberAdaptiveStream @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct GtuberClient(c_void);

impl ::std::fmt::Debug for GtuberClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GtuberClient @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct GtuberManifestGenerator(c_void);

impl ::std::fmt::Debug for GtuberManifestGenerator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GtuberManifestGenerator @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct GtuberMediaInfo(c_void);

impl ::std::fmt::Debug for GtuberMediaInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GtuberMediaInfo @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct GtuberStream(c_void);

impl ::std::fmt::Debug for GtuberStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GtuberStream @ {:p}", self))
         .finish()
    }
}

#[link(name = "gtuber-0.0")]
extern "C" {

    //=========================================================================
    // GtuberAdaptiveStreamManifest
    //=========================================================================
    pub fn gtuber_adaptive_stream_manifest_get_type() -> GType;

    //=========================================================================
    // GtuberClientError
    //=========================================================================
    pub fn gtuber_client_error_get_type() -> GType;
    pub fn gtuber_client_error_quark() -> glib::GQuark;

    //=========================================================================
    // GtuberFlow
    //=========================================================================
    pub fn gtuber_flow_get_type() -> GType;

    //=========================================================================
    // GtuberManifestGeneratorError
    //=========================================================================
    pub fn gtuber_manifest_generator_error_get_type() -> GType;
    pub fn gtuber_manifest_generator_error_quark() -> glib::GQuark;

    //=========================================================================
    // GtuberStreamMimeType
    //=========================================================================
    pub fn gtuber_stream_mime_type_get_type() -> GType;

    //=========================================================================
    // GtuberWebsiteError
    //=========================================================================
    pub fn gtuber_website_error_get_type() -> GType;

    //=========================================================================
    // GtuberAdaptiveStream
    //=========================================================================
    pub fn gtuber_adaptive_stream_get_type() -> GType;
    pub fn gtuber_adaptive_stream_get_index_range(stream: *mut GtuberAdaptiveStream, start: *mut u64, end: *mut u64) -> gboolean;
    pub fn gtuber_adaptive_stream_get_init_range(stream: *mut GtuberAdaptiveStream, start: *mut u64, end: *mut u64) -> gboolean;
    pub fn gtuber_adaptive_stream_get_manifest_type(stream: *mut GtuberAdaptiveStream) -> GtuberAdaptiveStreamManifest;

    //=========================================================================
    // GtuberClient
    //=========================================================================
    pub fn gtuber_client_get_type() -> GType;
    pub fn gtuber_client_new() -> *mut GtuberClient;
    pub fn gtuber_client_fetch_media_info(client: *mut GtuberClient, uri: *const c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut GtuberMediaInfo;
    pub fn gtuber_client_fetch_media_info_async(client: *mut GtuberClient, uri: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn gtuber_client_fetch_media_info_finish(client: *mut GtuberClient, res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> *mut GtuberMediaInfo;

    //=========================================================================
    // GtuberManifestGenerator
    //=========================================================================
    pub fn gtuber_manifest_generator_get_type() -> GType;
    pub fn gtuber_manifest_generator_new() -> *mut GtuberManifestGenerator;
    pub fn gtuber_manifest_generator_get_indent(gen: *mut GtuberManifestGenerator) -> c_uint;
    pub fn gtuber_manifest_generator_get_manifest_type(gen: *mut GtuberManifestGenerator) -> GtuberAdaptiveStreamManifest;
    pub fn gtuber_manifest_generator_get_pretty(gen: *mut GtuberManifestGenerator) -> gboolean;
    pub fn gtuber_manifest_generator_set_filter_func(gen: *mut GtuberManifestGenerator, filter: GtuberAdaptiveStreamFilter, user_data: gpointer, destroy: glib::GDestroyNotify);
    pub fn gtuber_manifest_generator_set_indent(gen: *mut GtuberManifestGenerator, indent: c_uint);
    pub fn gtuber_manifest_generator_set_manifest_type(gen: *mut GtuberManifestGenerator, type_: GtuberAdaptiveStreamManifest);
    pub fn gtuber_manifest_generator_set_media_info(gen: *mut GtuberManifestGenerator, info: *mut GtuberMediaInfo);
    pub fn gtuber_manifest_generator_set_pretty(gen: *mut GtuberManifestGenerator, pretty: gboolean);
    pub fn gtuber_manifest_generator_to_data(gen: *mut GtuberManifestGenerator) -> *mut c_char;
    pub fn gtuber_manifest_generator_to_file(gen: *mut GtuberManifestGenerator, filename: *const c_char, error: *mut *mut glib::GError) -> gboolean;

    //=========================================================================
    // GtuberMediaInfo
    //=========================================================================
    pub fn gtuber_media_info_get_type() -> GType;
    pub fn gtuber_media_info_get_adaptive_streams(info: *mut GtuberMediaInfo) -> *const glib::GPtrArray;
    pub fn gtuber_media_info_get_description(info: *mut GtuberMediaInfo) -> *const c_char;
    pub fn gtuber_media_info_get_duration(info: *mut GtuberMediaInfo) -> c_uint;
    pub fn gtuber_media_info_get_has_adaptive_streams(info: *mut GtuberMediaInfo) -> gboolean;
    pub fn gtuber_media_info_get_has_streams(info: *mut GtuberMediaInfo) -> gboolean;
    pub fn gtuber_media_info_get_id(info: *mut GtuberMediaInfo) -> *const c_char;
    pub fn gtuber_media_info_get_streams(info: *mut GtuberMediaInfo) -> *const glib::GPtrArray;
    pub fn gtuber_media_info_get_title(info: *mut GtuberMediaInfo) -> *const c_char;

    //=========================================================================
    // GtuberStream
    //=========================================================================
    pub fn gtuber_stream_get_type() -> GType;
    pub fn gtuber_stream_get_audio_codec(stream: *mut GtuberStream) -> *const c_char;
    pub fn gtuber_stream_get_bitrate(stream: *mut GtuberStream) -> c_uint;
    pub fn gtuber_stream_get_codecs(stream: *mut GtuberStream, vcodec: *mut *const c_char, acodec: *mut *const c_char) -> gboolean;
    pub fn gtuber_stream_get_fps(stream: *mut GtuberStream) -> c_uint;
    pub fn gtuber_stream_get_height(stream: *mut GtuberStream) -> c_uint;
    pub fn gtuber_stream_get_itag(stream: *mut GtuberStream) -> c_uint;
    pub fn gtuber_stream_get_mime_type(stream: *mut GtuberStream) -> GtuberStreamMimeType;
    pub fn gtuber_stream_get_uri(stream: *mut GtuberStream) -> *const c_char;
    pub fn gtuber_stream_get_video_codec(stream: *mut GtuberStream) -> *const c_char;
    pub fn gtuber_stream_get_width(stream: *mut GtuberStream) -> c_uint;
    pub fn gtuber_stream_obtain_codecs_string(stream: *mut GtuberStream) -> *mut c_char;

}
