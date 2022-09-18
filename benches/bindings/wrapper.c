#define QOI_IMPLEMENTATION
#include "qoi.h"
#include <stdint.h>
#include <stdlib.h>

void ffi_encode(uint8_t* data, uint32_t width, uint32_t height, uint8_t channels, uint8_t colorspace) {
    qoi_desc desc = {
        .width = width,
        .height = height,
        .channels = channels,
        .colorspace = colorspace
    };
    int _len;
    free(qoi_encode(data, &desc, &_len));
}

void ffi_decode(uint8_t* data, int32_t len, uint8_t channels) {
    qoi_desc desc;
    free(qoi_decode((void*)data, len, &desc, channels));
}
