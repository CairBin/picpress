#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C"{
#endif

enum ResizeStyle{
    RS_DEFAULT = 0,
    RS_FILL = 1,
    RS_FIT = 2,
    RS_EXACT = 3
};


enum PicPressError{
    PP_INVALID_FORMAT = -2,

    PP_INFER_FORMAT_ERROR = -3,

    PP_INVALID_METHOD = -4,

    // image lib internal error
    PP_IMAGE_ERROR = -5,

    PP_IO_ERROR = -6,

    PP_COMPRESS_ERROR = -7,

    PP_OTHER_ERROR = -1
};


int compress_img_c(const char* input, const char* ouput, const char* format, uint8_t quality, uint32_t width, uint32_t height, int method, uint8_t speed);


#ifdef __cplusplus
}
#endif