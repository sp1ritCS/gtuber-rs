// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ 0ac89bf4906a)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::StreamMimeType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtuberStream")]
    pub struct Stream(Object<ffi::GtuberStream, ffi::GtuberStreamClass>);

    match fn {
        type_ => || ffi::gtuber_stream_get_type(),
    }
}

pub const NONE_STREAM: Option<&Stream> = None;

pub trait StreamExt: 'static {
    #[doc(alias = "gtuber_stream_get_audio_codec")]
    #[doc(alias = "get_audio_codec")]
    fn audio_codec(&self) -> Option<glib::GString>;

    #[doc(alias = "gtuber_stream_get_bitrate")]
    #[doc(alias = "get_bitrate")]
    fn bitrate(&self) -> u32;

    //#[doc(alias = "gtuber_stream_get_codec_flags")]
    //#[doc(alias = "get_codec_flags")]
    //fn codec_flags(&self) -> /*Ignored*/CodecFlags;

    #[doc(alias = "gtuber_stream_get_fps")]
    #[doc(alias = "get_fps")]
    fn fps(&self) -> u32;

    #[doc(alias = "gtuber_stream_get_height")]
    #[doc(alias = "get_height")]
    fn height(&self) -> u32;

    #[doc(alias = "gtuber_stream_get_itag")]
    #[doc(alias = "get_itag")]
    fn itag(&self) -> u32;

    #[doc(alias = "gtuber_stream_get_mime_type")]
    #[doc(alias = "get_mime_type")]
    fn mime_type(&self) -> StreamMimeType;

    #[doc(alias = "gtuber_stream_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> glib::GString;

    #[doc(alias = "gtuber_stream_get_video_codec")]
    #[doc(alias = "get_video_codec")]
    fn video_codec(&self) -> Option<glib::GString>;

    #[doc(alias = "gtuber_stream_get_width")]
    #[doc(alias = "get_width")]
    fn width(&self) -> u32;

    #[doc(alias = "gtuber_stream_obtain_codecs_string")]
    fn obtain_codecs_string(&self) -> Option<glib::GString>;

    #[doc(alias = "audio-codec")]
    fn connect_audio_codec_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "bitrate")]
    fn connect_bitrate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "codec-flags")]
    fn connect_codec_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "fps")]
    fn connect_fps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "itag")]
    fn connect_itag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mime-type")]
    fn connect_mime_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "uri")]
    fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "video-codec")]
    fn connect_video_codec_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Stream>> StreamExt for O {
    fn audio_codec(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtuber_stream_get_audio_codec(self.as_ref().to_glib_none().0))
        }
    }

    fn bitrate(&self) -> u32 {
        unsafe {
            ffi::gtuber_stream_get_bitrate(self.as_ref().to_glib_none().0)
        }
    }

    //fn codec_flags(&self) -> /*Ignored*/CodecFlags {
    //    unsafe { TODO: call ffi:gtuber_stream_get_codec_flags() }
    //}

    fn fps(&self) -> u32 {
        unsafe {
            ffi::gtuber_stream_get_fps(self.as_ref().to_glib_none().0)
        }
    }

    fn height(&self) -> u32 {
        unsafe {
            ffi::gtuber_stream_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn itag(&self) -> u32 {
        unsafe {
            ffi::gtuber_stream_get_itag(self.as_ref().to_glib_none().0)
        }
    }

    fn mime_type(&self) -> StreamMimeType {
        unsafe {
            from_glib(ffi::gtuber_stream_get_mime_type(self.as_ref().to_glib_none().0))
        }
    }

    fn uri(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gtuber_stream_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn video_codec(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtuber_stream_get_video_codec(self.as_ref().to_glib_none().0))
        }
    }

    fn width(&self) -> u32 {
        unsafe {
            ffi::gtuber_stream_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn obtain_codecs_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtuber_stream_obtain_codecs_string(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_audio_codec_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_codec_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::audio-codec\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_audio_codec_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_bitrate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bitrate_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::bitrate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_bitrate_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_codec_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_codec_flags_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::codec-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_codec_flags_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_fps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fps_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::fps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_fps_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_height_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_itag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_itag_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::itag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_itag_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_mime_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mime_type_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mime-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_mime_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_uri_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_video_codec_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_codec_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::video-codec\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_video_codec_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P: IsA<Stream>, F: Fn(&P) + 'static>(this: *mut ffi::GtuberStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_width_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Stream")
    }
}
