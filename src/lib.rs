extern crate libc;

use libc::{c_char, c_uint, c_int};
use std::sync::Arc;
use ffi::{QrsEngine, QObject};
use std::path::Path;
use std::convert::AsRef;

/* Re-exports */

pub use variant::{Variant, FromQVariant, ToQVariant};
pub use ffi::QVariant as OpaqueQVariant;

/* Submodules */

#[allow(dead_code)]
pub mod ffi;
mod macros;
pub mod variant;

struct EngineInternal {
    p: *mut QrsEngine,
}

/* Hack to get invoke working. Need to figure out better way for invokes anyway.. */
unsafe impl Send for EngineInternal { }
unsafe impl Sync for EngineInternal { }

impl Drop for EngineInternal {
    fn drop(&mut self) {
        unsafe { ffi::qmlrs_destroy_engine(self.p); }
    }
}

pub struct Engine {
    i: Arc<EngineInternal>,
}

impl Engine {
    pub fn new() -> Engine {
        let p = unsafe { ffi::qmlrs_create_engine() };
        assert!(!p.is_null());

        let i = Arc::new(EngineInternal {
            p: p,
        });

        Engine {
            i: i
        }
    }

    pub fn new_headless() -> Engine {
        let p = unsafe { ffi::qmlrs_create_engine_headless() };
        assert!(!p.is_null());

        let i = Arc::new(EngineInternal {
            p: p,
        });

        Engine {
            i: i
        }
    }

    pub fn load_url(&mut self, path: &str) {
        unsafe {
            ffi::qmlrs_engine_load_url(self.i.p, path.as_ptr() as *const c_char,
                                       path.len() as c_uint);
        }
    }

    pub fn load_data(&mut self, data: &str) {
        unsafe {
            ffi::qmlrs_engine_load_from_data(self.i.p, data.as_ptr() as *const c_char,
                                             data.len() as c_uint);
        }
    }



    pub fn load_local_file<P: AsRef<Path>>(&mut self, name: P) {
        let path_raw = std::env::current_dir().unwrap().join(name);
        let path
            = if cfg!(windows) {
                format!("file:///{}",path_raw.display())
            } else {
                format!("file://{}",path_raw.display())
            } ;
        self.load_url(&path);
    }

    pub fn exec(self) {
        unsafe { ffi::qmlrs_app_exec(); }
    }

    pub fn set_property(&mut self, name: &str, obj: &Object) {
        unsafe {
            ffi::qmlrs_engine_set_property(self.i.p, name.as_ptr() as *const c_char,
                                           name.len() as c_uint, obj.p);
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MetaObject {
    p: *mut ffi::QrsMetaObject
}

impl MetaObject {
    pub fn new(name: &str, fun: ffi::SlotFunction) -> MetaObject {
        let p = unsafe { ffi::qmlrs_metaobject_create(name.as_ptr() as *const c_char,name.len() as c_uint,fun) };
        assert!(!p.is_null());

        MetaObject { p: p }
    }

    pub fn add_signal(&mut self, sig: &str) -> isize {
        unsafe { ffi::qmlrs_metaobject_add_signal(self.p,sig.as_ptr() as *const c_char,sig.len() as c_uint) as isize }
    }

    pub fn add_property(&mut self, name: &str, ty: &str, signal: Option<&str>) {
        unsafe {
            if let Some(sig) = signal {
                ffi::qmlrs_metaobject_add_property(self.p,name.as_ptr() as *const c_char,name.len() as c_uint,
                                                   ty.as_ptr() as *const c_char, ty.len() as c_uint,
                                                   sig.as_ptr() as *const c_char, sig.len() as c_uint);
            } else {
                let s = "";
                ffi::qmlrs_metaobject_add_property(self.p,name.as_ptr() as *const c_char,name.len() as c_uint,
                                                   ty.as_ptr() as *const c_char,ty.len() as c_uint,
                                                   s.as_ptr() as *const c_char, 0);
            }
        }
    }

    pub fn add_slot(&mut self, sig: &str) -> isize {
        unsafe { ffi::qmlrs_metaobject_add_slot(self.p,sig.as_ptr() as *const c_char,sig.len() as c_uint) as isize }
    }

    pub fn add_method(&mut self, sig: &str) -> isize {
        unsafe { ffi::qmlrs_metaobject_add_method(self.p,sig.as_ptr() as *const c_char,sig.len() as c_uint) as isize }
    }

    pub fn instantiate(&mut self) -> Object {
        Object{ p: unsafe {
            ffi::qmlrs_metaobject_instantiate(self.p)
        } }
    }
}

pub struct Object {
    p: *mut ffi::QObject
}

impl Object {
    pub fn set_property(&mut self, name: &str, value: Variant) {
        unsafe {
            let var = ffi::qmlrs_variant_create();
            value.to_qvariant(var);

            ffi::qmlrs_object_set_property(self.p,name.as_ptr() as *const c_char,name.len() as c_uint,var);
            ffi::qmlrs_variant_destroy(var);
        }
    }

    pub fn get_property(&self, name: &str) -> Variant {
        unsafe {
            let var = ffi::qmlrs_variant_create();

            ffi::qmlrs_object_get_property(self.p,name.as_ptr() as *const c_char,name.len() as c_uint,var);
            let ret = Variant::from_qvariant(var);

            ffi::qmlrs_variant_destroy(var);
            ret.unwrap()
        }
    }

    pub fn call(&self, id: isize, args: &[Variant]) -> Option<Variant> {
        unsafe {
            let vl = ffi::qmlrs_varlist_create();
            let ret = ffi::qmlrs_variant_create();

            for v in args {
                let var = ffi::qmlrs_varlist_push(vl);
                v.to_qvariant(var);
            }

            ffi::qmlrs_object_call(self.p, id as c_int, vl, ret);
            ffi::qmlrs_varlist_destroy(vl);

            let r = Variant::from_qvariant(ret);
            ffi::qmlrs_variant_destroy(ret);

            r
        }
    }

    pub fn emit(&self, id: isize, args: &[Variant]) {
        unsafe {
            let vl = ffi::qmlrs_varlist_create();

            for v in args {
                let var = ffi::qmlrs_varlist_push(vl);
                v.to_qvariant(var);
            }

            ffi::qmlrs_object_emit(self.p, id as c_int, vl);
            ffi::qmlrs_varlist_destroy(vl);
        }
    }

    pub fn delete_later(&mut self) {
        unsafe { ffi::qmlrs_object_delete_later(self.p) };
    }

    pub fn as_ptr(&mut self) -> *mut QObject {
        self.p
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_engine() {
        Engine::new_headless();
    }
}
