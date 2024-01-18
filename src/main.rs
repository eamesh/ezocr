// use std::env;

use std::path::Path;

use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("src/ocr.cpp");

        fn detect(
            detPath: &CxxString,
            clsPath: &CxxString,
            recPath: &CxxString,
            keysPath: &CxxString
        ) -> i32;
    }
}

fn main() {
    let_cxx_string!(det_path = Path::new("models/ch_PP-OCRv3_det_infer.onnx").to_str().unwrap());
    let_cxx_string!(cls_path = Path::new("models/ch_ppocr_mobile_v2.0_cls_infer.onnx").to_str().unwrap());
    let_cxx_string!(rec_path = Path::new("models/ch_PP-OCRv3_rec_infer.onnx").to_str().unwrap());
    let_cxx_string!(keys_path = Path::new("models/ppocr_keys_v1.txt").to_str().unwrap());

    let result = ffi::detect(&det_path, &cls_path, &rec_path, &keys_path);
    println!("{}", result);
    // let pkgconfig = env::current_dir().unwrap().join("pkgconfig");
    // env::set_var("PKG_CONFIG_PATH", &pkgconfig);

    // let onnxruntime = pkg_config::probe_library("onnxruntime").unwrap();
    // let opencv = pkg_config::probe_library("opencv4").unwrap();

    // println!("{:?}", onnxruntime.include_paths);
    // println!("{:?}", onnxruntime.libs);
    // println!("{:?}", opencv.include_paths);
    // println!("{:?}", opencv.libs);

}
