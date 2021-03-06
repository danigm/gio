// This file was generated by gir (746446b) from gir-files (469db10)
// DO NOT EDIT

use ffi;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SettingsSchemaKey(Shared<ffi::GSettingsSchemaKey>);

    match fn {
        ref => |ptr| ffi::g_settings_schema_key_ref(ptr),
        unref => |ptr| ffi::g_settings_schema_key_unref(ptr),
        get_type => || ffi::g_settings_schema_key_get_type(),
    }
}

impl SettingsSchemaKey {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_default_value(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_schema_key_get_default_value(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_description(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_name(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_range(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_schema_key_get_range(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_summary(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_summary(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_value_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_value_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn range_check(&self, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::g_settings_schema_key_range_check(self.to_glib_none().0, value.to_glib_none().0))
        }
    }
}
