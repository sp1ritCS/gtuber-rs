// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ f6573a461c85)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::AdaptiveStreamManifest;
use crate::Stream;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtuberAdaptiveStream")]
    pub struct AdaptiveStream(Object<ffi::GtuberAdaptiveStream, ffi::GtuberAdaptiveStreamClass>) @extends Stream;

    match fn {
        type_ => || ffi::gtuber_adaptive_stream_get_type(),
    }
}

impl AdaptiveStream {
    #[doc(alias = "gtuber_adaptive_stream_get_index_range")]
    #[doc(alias = "get_index_range")]
    pub fn index_range(&self) -> Option<(u64, u64)> {
        unsafe {
            let mut start = mem::MaybeUninit::uninit();
            let mut end = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtuber_adaptive_stream_get_index_range(self.to_glib_none().0, start.as_mut_ptr(), end.as_mut_ptr()));
            let start = start.assume_init();
            let end = end.assume_init();
            if ret { Some((start, end)) } else { None }
        }
    }

    #[doc(alias = "gtuber_adaptive_stream_get_init_range")]
    #[doc(alias = "get_init_range")]
    pub fn init_range(&self) -> Option<(u64, u64)> {
        unsafe {
            let mut start = mem::MaybeUninit::uninit();
            let mut end = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtuber_adaptive_stream_get_init_range(self.to_glib_none().0, start.as_mut_ptr(), end.as_mut_ptr()));
            let start = start.assume_init();
            let end = end.assume_init();
            if ret { Some((start, end)) } else { None }
        }
    }

    #[doc(alias = "gtuber_adaptive_stream_get_manifest_type")]
    #[doc(alias = "get_manifest_type")]
    pub fn manifest_type(&self) -> AdaptiveStreamManifest {
        unsafe {
            from_glib(ffi::gtuber_adaptive_stream_get_manifest_type(self.to_glib_none().0))
        }
    }

    #[doc(alias = "manifest-type")]
    pub fn connect_manifest_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_manifest_type_trampoline<F: Fn(&AdaptiveStream) + 'static>(this: *mut ffi::GtuberAdaptiveStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::manifest-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_manifest_type_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for AdaptiveStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AdaptiveStream")
    }
}
