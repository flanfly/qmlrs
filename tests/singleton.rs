extern crate qmlrs;

extern crate libc;

use qmlrs::*;

#[test]
pub fn singleton_test() {
    let mut engine = qmlrs::Engine::new();

    extern "C" fn slot(p: *mut ffi::QObject, id: libc::c_int, _: *const ffi::QVariantList, _: *mut ffi::QVariant) {
        println!("id: {}",id);
        if id == 2 {
            Object::from_ptr(p).emit(1,&[]);
        }
    }
    extern "C" fn singleton(_: *mut ffi::QQmlEngine, _: *mut ffi::QJSEngine) -> *mut ffi::QObject {
        let mut metaobj = MetaObject::new("Person",slot);

        assert_eq!(metaobj.add_signal("nameChanged()"),0);
        assert_eq!(metaobj.add_signal("testSignal()"),1);
        metaobj.add_property("name","QString",Some("nameChanged()"));
        assert_eq!(metaobj.add_method("test()","void"),2);

        let mut obj = metaobj.instantiate();
        obj.set_property("name",Variant::String("Kai".to_string()));
        obj.get_property("name");
        obj.as_ptr()
    }

    register_singleton_type(&"Test",1,2,&"Person",singleton);

    engine.load_local_file("tests/singleton.qml");
    engine.exec();
}
