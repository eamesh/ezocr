#include <iostream>
#include <stdio.h>
#include "OcrLite.h"

OcrLite *OCRLiteP = NULL;

int detect(
    const std::string &detPath, const std::string &clsPath,
    const std::string &recPath, const std::string &keysPath)
{
    printf("detPath: %s\n", detPath.c_str());
    printf("clsPath: %s\n", clsPath.c_str());
    printf("recPath: %s\n", recPath.c_str());
    printf("keysPath: %s\n", keysPath.c_str());

    OcrLite ocrLite;
    ocrLite.initLogger( // 设置日志输出
        false,
        false,
        false);
    ocrLite.setNumThread(3);
    ocrLite.setGpuIndex(0);
    ocrLite.initModels(detPath, clsPath, recPath, keysPath);

    OCRLiteP = &ocrLite;

    std::cout << "OCR" << std::endl;
    std::cout << "OCR init completed." << std::endl;

    return 0;
}