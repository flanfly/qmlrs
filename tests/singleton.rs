extern crate qmlrs;

extern crate libc;

use qmlrs::*;

#[test]
pub fn singleton_test() {
    let mut engine = qmlrs::Engine::new();

    extern "C" fn slot(_: *mut ffi::QObject, id: libc::c_int, _: *const ffi::QVariantList, _: *mut ffi::QVariant) {}
    extern "C" fn singleton(_: *mut ffi::QQmlEngine, _: *mut ffi::QJSEngine) -> *mut ffi::QObject {
        let mut metaobj = MetaObject::new("Person",slot);

        assert_eq!(metaobj.add_signal("nameChanged()"),0);
        metaobj.add_property("name","QString",Some("nameChanged()"));

        let mut obj = metaobj.instantiate();
        obj.set_property("name",Variant::String("Kai".to_string()));
        obj.as_ptr()
    }

    unsafe {
        ffi::qmlrs_register_singleton_type("Test".as_ptr() as *const libc::c_char,4,1,2,"Person".as_ptr() as *const libc::c_char,6,singleton);
    }

    engine.load_local_file("tests/singleton.qml");
    engine.exec();
}
