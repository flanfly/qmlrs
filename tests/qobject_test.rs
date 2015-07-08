/*
Q_OBJECT! {
    Factorial:

    properties {
        u64 name READ get_name WRITE set_name,
        property age: String
    }

    signals {
        test
    }

    slots {
        calculate(i64)
    }
}
*/
#[macro_use]
extern crate qmlrs;

extern crate libc;

use qmlrs::variant::ToQVariant;
use std::sync::{Once,ONCE_INIT};

struct Person {
    name: String,
    age: usize,
    __qmlrs_object: qmlrs::Object,
}

impl Person {
    pub fn new(n: &str, a: usize) -> Person {
        static mut METAOBJ: Option<qmlrs::MetaObject> = None;
        static ALLOC_METAOBJ: Once = ONCE_INIT;
        ALLOC_METAOBJ.call_once(|| {
            unsafe {
                let metaobj = qmlrs::MetaObject::new();

                metaobj.signal("nameChanged",0);
                metaobj.signal("ageChanged",0);

                METAOBJ = Some(metaobj);
            }
        });

        let obj = unsafe {
            let obj = METAOBJ.as_ref().unwrap().instantiate();
            let var = qmlrs::ffi::qmlrs_variant_create();

            n.to_string().to_qvariant(var);
            qmlrs::ffi::qmlrs_object_set_property(obj.p,"name".as_ptr() as *const libc::c_char,4 as libc::c_uint,var);

            qmlrs::ffi::qmlrs_variant_destroy(var);

            obj
        };

        Person{
            name: n.to_string(),
            age: a,
            __qmlrs_object: obj,
        }
    }

    pub fn object(&self) -> qmlrs::Object {
        self.__qmlrs_object.clone()
    }
}

#[test]
pub fn main() {
    let mut engine = qmlrs::Engine::new();
    let p1 = Person::new("Kai",29);

    engine.load_local_file("tests/qobject_test.qml");
    engine.set_property("person_one", &p1.object());
    engine.exec();
}

/*
 *
 * struct Factorial {
 *     name: u64,
 *     age: String,
 *     __qmlrs_object: Object,
 * }
 *
 * impl Factorial {
 *     pub fn new(name: u64, age: String) -> Factorial {
 *         ONCE...
 *         Factorial {
 *             name: name,
 *             age: age,
 *             __qmlrs_object: FACTORIAL_METAOBJ.unwrap().instance(),
 *         }
 *     }
 *
 *     pub fn test(&self) {
 *         self.__qmlrs_object.emit(1);
 *     }
 *
 *     pub fn name_changed(&self) {
 *         self.__qmlrs_object.emit(0);
 *     }
 *
 *     pub fn set_name(&mut self,val: String) {
 *         self.name = val;
 *         self.name_changed();
 *     }
 *
 *     pub fn get_name(&self) -> String {
 *         self.name.clone()
 *     }
 *
 * }
 */

