use std::{env, fs, path::PathBuf};

use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "";
    let pkgconfig = env::current_dir().unwrap().join("pkgconfig");
    env::set_var("PKG_CONFIG_PATH", &pkgconfig);

    let packages_path = env::current_dir().unwrap().join("packages");
    let rapid_path = packages_path.join("RapidOcrOnnx");

    let onnxruntime = pkg_config::probe_library("onnxruntime").unwrap();
    let opencv = pkg_config::probe_library("opencv4").unwrap();

    let rapid_cpps: Vec<PathBuf> = fs::read_dir(rapid_path.join("src"))
        .unwrap()
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .collect();

    cxx_build::bridge("src/main.rs")
        .include(rapid_path.join("include"))
        .files(&rapid_cpps)
        .flag_if_supported("-std=c++14")
        .includes(opencv.include_paths)
        .includes(onnxruntime.include_paths)
        .compile("demo");

    for link_path in opencv.link_paths {
        println!("cargo:rustc-link-search={}", link_path.to_str().unwrap());
    }

    for lib in opencv.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    for link_path in onnxruntime.link_paths {
        println!("cargo:rustc-link-search={}", link_path.to_str().unwrap());
    }

    for lib in onnxruntime.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    #[cfg(target_os = "macos")]
    macos_link_lib();

}

#[cfg(target_os = "macos")]
fn macos_link_lib() {
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=Foundation");
}
