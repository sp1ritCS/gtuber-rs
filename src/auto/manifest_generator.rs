// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ f6573a461c85)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::AdaptiveStream;
use crate::AdaptiveStreamManifest;
use crate::MediaInfo;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GtuberManifestGenerator")]
    pub struct ManifestGenerator(Object<ffi::GtuberManifestGenerator, ffi::GtuberManifestGeneratorClass>);

    match fn {
        type_ => || ffi::gtuber_manifest_generator_get_type(),
    }
}

impl ManifestGenerator {
    #[doc(alias = "gtuber_manifest_generator_new")]
    pub fn new() -> ManifestGenerator {
        unsafe {
            from_glib_full(ffi::gtuber_manifest_generator_new())
        }
    }

    #[doc(alias = "gtuber_manifest_generator_get_indent")]
    #[doc(alias = "get_indent")]
    pub fn indent(&self) -> u32 {
        unsafe {
            ffi::gtuber_manifest_generator_get_indent(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gtuber_manifest_generator_get_manifest_type")]
    #[doc(alias = "get_manifest_type")]
    pub fn manifest_type(&self) -> AdaptiveStreamManifest {
        unsafe {
            from_glib(ffi::gtuber_manifest_generator_get_manifest_type(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_manifest_generator_get_pretty")]
    #[doc(alias = "get_pretty")]
    pub fn is_pretty(&self) -> bool {
        unsafe {
            from_glib(ffi::gtuber_manifest_generator_get_pretty(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_manifest_generator_set_filter_func")]
    pub fn set_filter_func(&self, filter: Option<Box_<dyn Fn(&AdaptiveStream) -> bool + 'static>>) {
        let filter_data: Box_<Option<Box_<dyn Fn(&AdaptiveStream) -> bool + 'static>>> = Box_::new(filter);
        unsafe extern "C" fn filter_func(stream: *mut ffi::GtuberAdaptiveStream, user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let stream = from_glib_borrow(stream);
            let callback: &Option<Box_<dyn Fn(&AdaptiveStream) -> bool + 'static>> = &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&stream)
            } else {
                panic!("cannot get closure...")
            };
            res.into_glib()
        }
        let filter = if filter_data.is_some() { Some(filter_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&AdaptiveStream) -> bool + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&AdaptiveStream) -> bool + 'static>>> = filter_data;
        unsafe {
            ffi::gtuber_manifest_generator_set_filter_func(self.to_glib_none().0, filter, Box_::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    #[doc(alias = "gtuber_manifest_generator_set_indent")]
    pub fn set_indent(&self, indent: u32) {
        unsafe {
            ffi::gtuber_manifest_generator_set_indent(self.to_glib_none().0, indent);
        }
    }

    #[doc(alias = "gtuber_manifest_generator_set_manifest_type")]
    pub fn set_manifest_type(&self, type_: AdaptiveStreamManifest) {
        unsafe {
            ffi::gtuber_manifest_generator_set_manifest_type(self.to_glib_none().0, type_.into_glib());
        }
    }

    #[doc(alias = "gtuber_manifest_generator_set_media_info")]
    pub fn set_media_info(&self, info: &MediaInfo) {
        unsafe {
            ffi::gtuber_manifest_generator_set_media_info(self.to_glib_none().0, info.to_glib_none().0);
        }
    }

    #[doc(alias = "gtuber_manifest_generator_set_pretty")]
    pub fn set_pretty(&self, pretty: bool) {
        unsafe {
            ffi::gtuber_manifest_generator_set_pretty(self.to_glib_none().0, pretty.into_glib());
        }
    }

    #[doc(alias = "gtuber_manifest_generator_to_data")]
    pub fn to_data(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtuber_manifest_generator_to_data(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtuber_manifest_generator_to_file")]
    pub fn to_file(&self, filename: impl AsRef<std::path::Path>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtuber_manifest_generator_to_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "indent")]
    pub fn connect_indent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indent_trampoline<F: Fn(&ManifestGenerator) + 'static>(this: *mut ffi::GtuberManifestGenerator, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::indent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_indent_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "manifest-type")]
    pub fn connect_manifest_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_manifest_type_trampoline<F: Fn(&ManifestGenerator) + 'static>(this: *mut ffi::GtuberManifestGenerator, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::manifest-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_manifest_type_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "pretty")]
    pub fn connect_pretty_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pretty_trampoline<F: Fn(&ManifestGenerator) + 'static>(this: *mut ffi::GtuberManifestGenerator, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pretty\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_pretty_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for ManifestGenerator {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

impl fmt::Display for ManifestGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ManifestGenerator")
    }
}