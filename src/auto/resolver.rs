// This file was generated by gir (746446b) from gir-files (469db10)
// DO NOT EDIT

use Cancellable;
use Error;
use InetAddress;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use ResolverRecordType;
use SrvTarget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Resolver(Object<ffi::GResolver, ffi::GResolverClass>);

    match fn {
        get_type => || ffi::g_resolver_get_type(),
    }
}

impl Resolver {
    //pub fn free_addresses(addresses: /*Unimplemented*/&[&Fundamental: Pointer]) {
    //    unsafe { TODO: call ffi::g_resolver_free_addresses() }
    //}

    //pub fn free_targets(targets: /*Unimplemented*/&[&Fundamental: Pointer]) {
    //    unsafe { TODO: call ffi::g_resolver_free_targets() }
    //}

    pub fn get_default() -> Option<Resolver> {
        unsafe {
            from_glib_full(ffi::g_resolver_get_default())
        }
    }
}

pub trait ResolverExt {
    fn lookup_by_address<'a, P: Into<Option<&'a Cancellable>>>(&self, address: &InetAddress, cancellable: P) -> Result<String, Error>;

    fn lookup_by_name<'a, P: Into<Option<&'a Cancellable>>>(&self, hostname: &str, cancellable: P) -> Result<Vec<InetAddress>, Error>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records<'a, P: Into<Option<&'a Cancellable>>>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: P) -> Result<Vec<glib::Variant>, Error>;

    fn lookup_service<'a, P: Into<Option<&'a Cancellable>>>(&self, service: &str, protocol: &str, domain: &str, cancellable: P) -> Result<Vec<SrvTarget>, Error>;

    fn set_default(&self);

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Resolver> + IsA<glib::object::Object>> ResolverExt for O {
    fn lookup_by_address<'a, P: Into<Option<&'a Cancellable>>>(&self, address: &InetAddress, cancellable: P) -> Result<String, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_address(self.to_glib_none().0, address.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_by_name<'a, P: Into<Option<&'a Cancellable>>>(&self, hostname: &str, cancellable: P) -> Result<Vec<InetAddress>, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name(self.to_glib_none().0, hostname.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records<'a, P: Into<Option<&'a Cancellable>>>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: P) -> Result<Vec<glib::Variant>, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_records(self.to_glib_none().0, rrname.to_glib_none().0, record_type.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_service<'a, P: Into<Option<&'a Cancellable>>>(&self, service: &str, protocol: &str, domain: &str, cancellable: P) -> Result<Vec<SrvTarget>, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_service(self.to_glib_none().0, service.to_glib_none().0, protocol.to_glib_none().0, domain.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_default(&self) {
        unsafe {
            ffi::g_resolver_set_default(self.to_glib_none().0);
        }
    }

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "reload",
                transmute(reload_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn reload_trampoline<P>(this: *mut ffi::GResolver, f: glib_ffi::gpointer)
where P: IsA<Resolver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Resolver::from_glib_borrow(this).downcast_unchecked())
}
