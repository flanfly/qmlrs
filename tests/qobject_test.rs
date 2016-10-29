extern crate qmlrs;

extern crate libc;

use qmlrs::*;

#[test]
pub fn qobject_test() {
    let mut engine = qmlrs::Engine::new("test");

    extern "C" fn test_slot(_: *mut ffi::QObject, id: libc::c_int, _: *const ffi::QVariantList, ret: *mut ffi::QVariant) {
        println!("slot: {} called",id);

        if 1 == id {
            println!("test_slot called");
        } else if 2 == id {
            println!("func(int) called");
            unsafe {
                ffi::qmlrs_variant_set_int64(ret,42);
            }
        } else {
            panic!("unknown id {}",id);
        }
    }


    let mut metaobj = MetaObject::new("Person",test_slot);

    assert_eq!(metaobj.add_signal("nameChanged()"),0);
    metaobj.add_property("name","QString",Some("nameChanged()"));
    assert_eq!(metaobj.add_slot("test_slot()"),1);
    assert_eq!(metaobj.add_method("func(QVariant)","int"),2);

        let mut obj = metaobj.instantiate();

    obj.set_property("name",Variant::String("Kai".to_string()));
    obj.emit(0,&[]);
    //obj.call(1,&[]);
    //obj.call(2,&[Variant::I64(42)]);

    engine.set_property("person_one", &obj);
    engine.load_local_file("tests/qobject_test.qml");
    engine.exec();
}
