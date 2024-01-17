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
            keysPath: &CxxString,
        );
    }
}

fn main() {
    let_cxx_string!(det_path = Path::new("models/ch_PP-OCRv3_det_infer.onnx").to_str().unwrap());
    let_cxx_string!(cls_path = Path::new("models/ch_ppocr_mobile_v2.0_cls_infer.onnx").to_str().unwrap());
    let_cxx_string!(rec_path = Path::new("models/ch_PP-OCRv3_rec_infer.onnx").to_str().unwrap());
    let_cxx_string!(keys_path = Path::new("models/ppocr_keys_v1.txt").to_str().unwrap());


    ffi::detect(&det_path, &cls_path, &rec_path, &keys_path);
    println!("Hello, world!");
}
