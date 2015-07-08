#[macro_export]
macro_rules! Q_OBJECT(
    (
        $t:ty [ $c:ident ]
        slots:
            $(
                fn $name:ident ( $($at:ty),* );
            )*
        signals:
            $(
                fn $sname:ident ( );
            )*

    ) => (

        impl $t {
            #[allow(dead_code, unused_mut, unused_variables, unused_assignments)]
            fn __qmlrs_signal_id(&self, name: &str) -> u32 {
                let mut id = 0;
                $(
                    if stringify!($sname) == name {
                        return id;
                    }
                    id += 1;
                )*
                panic!("__qmlrs_signal_id called with invalid signal name!");
            }

            $(
                #[allow(dead_code)]
                fn $sname (&self) {
                    println!("call {}",stringify!($sname));
                    qmlrs::__qobject_emit(self.qt_metaobject().ins, self.__qmlrs_signal_id(stringify!($sname)));
                }
            )*
        }


        static mut $c: Option<qmlrs::MetaObject> = None;

        impl qmlrs::Object for $t {
            #[allow(unused_mut, unused_variables)]
            fn qt_metaobject(&self) -> &'static qmlrs::MetaObject {
                unsafe {
                    if $c.is_none() {
                        let x = qmlrs::MetaObject::new();
                        $(
                            let mut argc = 0;
                            $(
                                let _: $at;
                                argc += 1;
                            )*
                            let x = x.slot(stringify!($name), argc);
                        )*

                        $(
                            let x = x.signal(stringify!($sname), 0);
                        )*

                        $c = Some(x)
                    }
                    return $c.as_ref().unwrap();
                }
            }

            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn qt_metacall(&mut self, slot: i32, args: *const *const qmlrs::OpaqueQVariant) {
                use qmlrs::ToQVariant;
                let mut i = 0;

                $(
                    if i == slot {
                        let mut argi = 1u8; /* 0 is for return value */
                        let ret = self.$name(
                            $(
                                {
                                    let _: $at;
                                    match qmlrs::FromQVariant::from_qvariant(unsafe { *args.offset(argi as isize) }) {
                                        Some(arg) => { argi += 1; arg }
                                        None      => {
                                            println!("qmlrs: Invalid argument {} type for slot {}", argi, stringify!($name));
                                            return;
                                        }
                                    }
                                }
                            )*
                        );
                        ret.to_qvariant(unsafe { *args.offset(0) as *mut qmlrs::OpaqueQVariant });
                        return
                    }
                    i += 1;
                )*

                $(
                    let _ = stringify!($sname);
                    i += 1;
                )*

            }

        }
    )
);
