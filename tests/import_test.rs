extern crate qmlrs;

#[test]
fn test_import() {
    let mut engine = qmlrs::Engine::new_headless();

    engine.load_local_file("tests/import_test.qml");

    engine.exec();
}
