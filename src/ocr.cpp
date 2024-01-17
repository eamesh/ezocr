#include <iostream>
#include <stdio.h>
#include "OcrLite.h"

OcrLite *OCRLiteP = NULL;

void detect(
    const std::string &detPath, const std::string &clsPath,
    const std::string &recPath, const std::string &keysPath)
{
    printf("detPath: %s\n", detPath.c_str());
    printf("clsPath: %s\n", clsPath.c_str());
    printf("recPath: %s\n", recPath.c_str());
    printf("keysPath: %s\n", keysPath.c_str());

    OcrLite ocrLite;
    ocrLite.initLogger( // 设置日志输出
        true,//isOutputConsole
        false,//isOutputPartImg
        true);//isOutputResultImg
    ocrLite.setNumThread(3); // 设置线程
    ocrLite.setGpuIndex(0); // 设置启用GPU
    ocrLite.initModels(detPath, clsPath, recPath, keysPath); // 加载模型

    OCRLiteP = &ocrLite;

    // std::cout << "OCR" << std::endl; // 版本提示
    // std::cout << "OCR init completed." << std::endl; // 完成提示
}
