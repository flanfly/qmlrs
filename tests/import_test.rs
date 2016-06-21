extern crate qmlrs;

use std::env;

#[test]
fn test_import() {
    let mut engine = qmlrs::Engine::new_headless();
    let path = env::current_exe().ok().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("tests").join("import_test.qml");

    engine.load_local_file(&format!("{}",path.display()));

    engine.exec();
}
