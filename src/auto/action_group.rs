// This file was generated by gir (746446b) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ActionGroup(Object<ffi::GActionGroup, ffi::GActionGroupInterface>);

    match fn {
        get_type => || ffi::g_action_group_get_type(),
    }
}

pub trait ActionGroupExt {
    fn action_added(&self, action_name: &str);

    fn action_enabled_changed(&self, action_name: &str, enabled: bool);

    fn action_removed(&self, action_name: &str);

    fn action_state_changed(&self, action_name: &str, state: &glib::Variant);

    fn activate_action<'a, P: Into<Option<&'a glib::Variant>>>(&self, action_name: &str, parameter: P);

    fn change_action_state(&self, action_name: &str, value: &glib::Variant);

    fn get_action_enabled(&self, action_name: &str) -> bool;

    fn get_action_parameter_type(&self, action_name: &str) -> Option<glib::VariantType>;

    fn get_action_state(&self, action_name: &str) -> Option<glib::Variant>;

    fn get_action_state_hint(&self, action_name: &str) -> Option<glib::Variant>;

    fn get_action_state_type(&self, action_name: &str) -> Option<glib::VariantType>;

    fn has_action(&self, action_name: &str) -> bool;

    fn list_actions(&self) -> Vec<String>;

    fn connect_action_added<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_action_enabled_changed<F: Fn(&Self, &str, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_action_removed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_action_state_changed<F: Fn(&Self, &str, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ActionGroup> + IsA<glib::object::Object>> ActionGroupExt for O {
    fn action_added(&self, action_name: &str) {
        unsafe {
            ffi::g_action_group_action_added(self.to_glib_none().0, action_name.to_glib_none().0);
        }
    }

    fn action_enabled_changed(&self, action_name: &str, enabled: bool) {
        unsafe {
            ffi::g_action_group_action_enabled_changed(self.to_glib_none().0, action_name.to_glib_none().0, enabled.to_glib());
        }
    }

    fn action_removed(&self, action_name: &str) {
        unsafe {
            ffi::g_action_group_action_removed(self.to_glib_none().0, action_name.to_glib_none().0);
        }
    }

    fn action_state_changed(&self, action_name: &str, state: &glib::Variant) {
        unsafe {
            ffi::g_action_group_action_state_changed(self.to_glib_none().0, action_name.to_glib_none().0, state.to_glib_none().0);
        }
    }

    fn activate_action<'a, P: Into<Option<&'a glib::Variant>>>(&self, action_name: &str, parameter: P) {
        let parameter = parameter.into();
        let parameter = parameter.to_glib_none();
        unsafe {
            ffi::g_action_group_activate_action(self.to_glib_none().0, action_name.to_glib_none().0, parameter.0);
        }
    }

    fn change_action_state(&self, action_name: &str, value: &glib::Variant) {
        unsafe {
            ffi::g_action_group_change_action_state(self.to_glib_none().0, action_name.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_action_enabled(&self, action_name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_action_group_get_action_enabled(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn get_action_parameter_type(&self, action_name: &str) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_group_get_action_parameter_type(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn get_action_state(&self, action_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_group_get_action_state(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn get_action_state_hint(&self, action_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_group_get_action_state_hint(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn get_action_state_type(&self, action_name: &str) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_group_get_action_state_type(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn has_action(&self, action_name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_action_group_has_action(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn list_actions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_action_group_list_actions(self.to_glib_none().0))
        }
    }

    fn connect_action_added<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-added",
                transmute(action_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_action_enabled_changed<F: Fn(&Self, &str, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-enabled-changed",
                transmute(action_enabled_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_action_removed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-removed",
                transmute(action_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_action_state_changed<F: Fn(&Self, &str, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, &glib::Variant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-state-changed",
                transmute(action_state_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn action_added_trampoline<P>(this: *mut ffi::GActionGroup, action_name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<ActionGroup> {
    callback_guard!();
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&ActionGroup::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(action_name))
}

unsafe extern "C" fn action_enabled_changed_trampoline<P>(this: *mut ffi::GActionGroup, action_name: *mut libc::c_char, enabled: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<ActionGroup> {
    callback_guard!();
    let f: &&(Fn(&P, &str, bool) + 'static) = transmute(f);
    f(&ActionGroup::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(action_name), from_glib(enabled))
}

unsafe extern "C" fn action_removed_trampoline<P>(this: *mut ffi::GActionGroup, action_name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<ActionGroup> {
    callback_guard!();
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&ActionGroup::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(action_name))
}

unsafe extern "C" fn action_state_changed_trampoline<P>(this: *mut ffi::GActionGroup, action_name: *mut libc::c_char, value: *mut glib_ffi::GVariant, f: glib_ffi::gpointer)
where P: IsA<ActionGroup> {
    callback_guard!();
    let f: &&(Fn(&P, &str, &glib::Variant) + 'static) = transmute(f);
    f(&ActionGroup::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(action_name), &from_glib_borrow(value))
}
