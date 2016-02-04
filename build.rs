extern crate pkg_config;

use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;
use std::path::PathBuf;

fn main() {
    let wcd = env::current_dir().unwrap();
    let build = PathBuf::from(&wcd.join("ext/libqmlrswrapper/build"));
    let _ = fs::create_dir_all(&build);

    /*
     * Prameters for supporting QT on OS X installed via homebres
     *
     * Because QT5 conflicts with QT4 the homebrew package manager won't link
     * the QT5 package into the default search paths for libraries, to deal
     * with this we need to give pkg-config and cmake a nudge in the right
     * direction.
     */
    if cfg!(target_os = "macos") {
        // Point at homebrew's QT5 install location, this will likely fail if
        // another package manager was used.
        let qt5_lib_path = Path::new("/usr/local/opt/qt5/lib");

        if Path::exists(qt5_lib_path) {
            // First nudge cmake in the direction of the .cmake files added by
            // homebrew. This clobbers the existing value if present, it's
            // unlikely to be present though.
            env::set_var("CMAKE_PREFIX_PATH", qt5_lib_path.join("cmake"));

            // Nudge pkg-config in the direction of the brewed QT to ensure the
            // correct compiler flags get found for the project.
            env::set_var("PKG_CONFIG_PATH", qt5_lib_path.join("pkgconfig"));
        } else {
            panic!("The QT5 was not found at the expected location ({}) please install it via homebrew.", qt5_lib_path.display());
        }
    }

    if cfg!(windows) {
        let is_msys = env::var("MSYSTEM").is_ok();
        let mut myargs = vec![];

        if is_msys {
            myargs.push("-GMSYS Makefiles");
        } else {
            myargs.push("-GVisual Studio 12 2013 Win64");
        }

        myargs.push("..");

        Command::new("cmake")
            .args(&myargs)
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
