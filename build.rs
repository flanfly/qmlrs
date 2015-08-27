extern crate pkg_config;

use std::process::Command;
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    let wcd = env::current_dir().unwrap();
    let build = PathBuf::from(&wcd.join("ext/libqmlrswrapper/build"));
    let _ = fs::create_dir_all(&build);

    if cfg!(windows) {
        Command::new("cmake")
            .args(&vec!["-GVisual Studio 12 2013 Win64",".."])
            .current_dir(&build)
            .status().and_then(|x| Ok(x.success()) ).unwrap_or_else(|e| {
                panic!("Failed to run cmake: {}", e);
            });
    } else {
        Command::new("cmake")
            .args(&vec![".."])
            .current_dir(&build)
            .status().and_then(|x| Ok(x.success()) ).unwrap_or_else(|e| {
                panic!("Failed to run cmake: {}", e);
            });
    }

    Command::new("cmake")
        .args(&vec!["--build","."])
        .current_dir(&build)
        .status().and_then(|x| Ok(x.success()) ).unwrap_or_else(|e| {
            panic!("Failed to run build: {}", e);
        });

    println!("cargo:rustc-link-lib=static=qmlrswrapper");

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}\\system32",env::var("WINDIR").unwrap());
        println!("cargo:rustc-link-search=native={}\\Debug",build.display());
        println!("cargo:rustc-link-search=native={}\\lib",env::var("QTDIR").unwrap());

        println!("cargo:rustc-link-lib=dylib=Qt5Core");
        println!("cargo:rustc-link-lib=dylib=Qt5Gui");
        println!("cargo:rustc-link-lib=dylib=Qt5Qml");
        println!("cargo:rustc-link-lib=dylib=Qt5Quick");
    } else {
        println!("cargo:rustc-link-search=native={}{}",build.display(),if cfg!(windows) { "\\Debug" } else { "" });
        println!("cargo:rustc-link-lib=dylib=stdc++");
        pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick").unwrap();
    }
}
