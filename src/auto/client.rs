// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ 040ee85266c6)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::MediaInfo;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GtuberClient")]
    pub struct Client(Object<ffi::GtuberClient, ffi::GtuberClientClass>);

    match fn {
        type_ => || ffi::gtuber_client_get_type(),
    }
}

impl Client {
    #[doc(alias = "gtuber_client_new")]
    pub fn new() -> Client {
        unsafe {
            from_glib_full(ffi::gtuber_client_new())
        }
    }

    #[doc(alias = "gtuber_client_fetch_media_info")]
    pub fn fetch_media_info(&self, uri: &str, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<MediaInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtuber_client_fetch_media_info(self.to_glib_none().0, uri.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "gtuber_client_fetch_media_info_async")]
    pub fn fetch_media_info_async<P: FnOnce(Result<MediaInfo, glib::Error>) + Send + 'static>(&self, uri: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn fetch_media_info_async_trampoline<P: FnOnce(Result<MediaInfo, glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::gtuber_client_fetch_media_info_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = fetch_media_info_async_trampoline::<P>;
        unsafe {
            ffi::gtuber_client_fetch_media_info_async(self.to_glib_none().0, uri.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    pub fn fetch_media_info_async_future(&self, uri: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<MediaInfo, glib::Error>> + 'static>> {

        let uri = String::from(uri);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.fetch_media_info_async(
                &uri,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }
}

impl Default for Client {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Client")
    }
}
