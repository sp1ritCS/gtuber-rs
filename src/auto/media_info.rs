// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ 040ee85266c6)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::AdaptiveStream;
use crate::Stream;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtuberMediaInfo")]
    pub struct MediaInfo(Object<ffi::GtuberMediaInfo, ffi::GtuberMediaInfoClass>);

    match fn {
        type_ => || ffi::gtuber_media_info_get_type(),
    }
}

impl MediaInfo {
    #[doc(alias = "gtuber_media_info_get_adaptive_streams")]
    #[doc(alias = "get_adaptive_streams")]
    pub fn adaptive_streams(&self) -> Vec<AdaptiveStream> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtuber_media_info_get_adaptive_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_media_info_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtuber_media_info_get_description(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_media_info_get_duration")]
    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> u32 {
        unsafe {
            ffi::gtuber_media_info_get_duration(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gtuber_media_info_get_has_adaptive_streams")]
    #[doc(alias = "get_has_adaptive_streams")]
    pub fn has_adaptive_streams(&self) -> bool {
        unsafe {
            from_glib(ffi::gtuber_media_info_get_has_adaptive_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_media_info_get_has_streams")]
    #[doc(alias = "get_has_streams")]
    pub fn has_streams(&self) -> bool {
        unsafe {
            from_glib(ffi::gtuber_media_info_get_has_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_media_info_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtuber_media_info_get_id(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_media_info_get_streams")]
    #[doc(alias = "get_streams")]
    pub fn streams(&self) -> Vec<Stream> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtuber_media_info_get_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_media_info_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtuber_media_info_get_title(self.to_glib_none().0))
        }
    }

    #[doc(alias = "description")]
    pub fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<F: Fn(&MediaInfo) + 'static>(this: *mut ffi::GtuberMediaInfo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_description_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "duration")]
    pub fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<F: Fn(&MediaInfo) + 'static>(this: *mut ffi::GtuberMediaInfo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_duration_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "has-adaptive-streams")]
    pub fn connect_has_adaptive_streams_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_adaptive_streams_trampoline<F: Fn(&MediaInfo) + 'static>(this: *mut ffi::GtuberMediaInfo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-adaptive-streams\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_has_adaptive_streams_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "has-streams")]
    pub fn connect_has_streams_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_streams_trampoline<F: Fn(&MediaInfo) + 'static>(this: *mut ffi::GtuberMediaInfo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-streams\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_has_streams_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "id")]
    pub fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<F: Fn(&MediaInfo) + 'static>(this: *mut ffi::GtuberMediaInfo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_id_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&MediaInfo) + 'static>(this: *mut ffi::GtuberMediaInfo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_title_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MediaInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MediaInfo")
    }
}
