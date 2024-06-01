// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from ../gir-files-gstreamer
// from ../gir-files-gtk
// DO NOT EDIT

use crate::{Stream};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "ClapperSubtitleStream")]
    pub struct SubtitleStream(Object<ffi::ClapperSubtitleStream, ffi::ClapperSubtitleStreamClass>) @extends Stream, gst::Object;

    match fn {
        type_ => || ffi::clapper_subtitle_stream_get_type(),
    }
}

impl SubtitleStream {
    #[doc(alias = "clapper_subtitle_stream_get_lang_code")]
    #[doc(alias = "get_lang_code")]
    pub fn lang_code(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::clapper_subtitle_stream_get_lang_code(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_subtitle_stream_get_lang_name")]
    #[doc(alias = "get_lang_name")]
    pub fn lang_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::clapper_subtitle_stream_get_lang_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "lang-code")]
    pub fn connect_lang_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lang_code_trampoline<F: Fn(&SubtitleStream) + 'static>(this: *mut ffi::ClapperSubtitleStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::lang-code\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_lang_code_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "lang-name")]
    pub fn connect_lang_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lang_name_trampoline<F: Fn(&SubtitleStream) + 'static>(this: *mut ffi::ClapperSubtitleStream, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::lang-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_lang_name_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}