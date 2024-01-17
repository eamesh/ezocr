use std::{env, fs, path::PathBuf};
use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "";

    let pkg_config_path = env::current_dir().unwrap().join("libs");
    env::set_var("PKG_CONFIG_PATH", pkg_config_path);

    let packages_path = env::current_dir().unwrap().join("packages");
    let rapid_path = packages_path.join("Project_RapidOcrOnnx-1.2.2");

    let onnxruntime_path = rapid_path.join("onnxruntime-static/linux");
    env::set_var("ONNXRUNTIME_PATH", onnxruntime_path);

    let opencv4_path = rapid_path.join("opencv-static/linux");
    env::set_var("OPENCV4_PATH", opencv4_path);

    let opencv = pkg_config::probe_library("opencv4").unwrap();
    let onnxruntime = pkg_config::probe_library("onnxruntime").unwrap();

    let rapid_cpps: Vec<PathBuf> = fs::read_dir(rapid_path.join("src"))
        .unwrap()
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .collect();

    cxx_build::bridge("src/main.rs")
        .include(rapid_path.join("include"))
        .files(&rapid_cpps)
        .file("src/ocr.cpp")
        .flag_if_supported("-std=c++11")
        .includes(opencv.include_paths)
        .includes(onnxruntime.include_paths)
        .compile("ezocr");

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
}