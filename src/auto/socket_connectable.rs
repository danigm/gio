// This file was generated by gir (746446b) from gir-files (469db10)
// DO NOT EDIT

use SocketAddressEnumerator;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SocketConnectable(Object<ffi::GSocketConnectable, ffi::GSocketConnectableIface>);

    match fn {
        get_type => || ffi::g_socket_connectable_get_type(),
    }
}

pub trait SocketConnectableExt {
    fn enumerate(&self) -> Option<SocketAddressEnumerator>;

    fn proxy_enumerate(&self) -> Option<SocketAddressEnumerator>;

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    fn to_string(&self) -> Option<String>;
}

impl<O: IsA<SocketConnectable>> SocketConnectableExt for O {
    fn enumerate(&self) -> Option<SocketAddressEnumerator> {
        unsafe {
            from_glib_full(ffi::g_socket_connectable_enumerate(self.to_glib_none().0))
        }
    }

    fn proxy_enumerate(&self) -> Option<SocketAddressEnumerator> {
        unsafe {
            from_glib_full(ffi::g_socket_connectable_proxy_enumerate(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    fn to_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_socket_connectable_to_string(self.to_glib_none().0))
        }
    }
}
