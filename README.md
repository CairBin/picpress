# picpress

## Description

A command tool for image compression and conversion.

## Supproted Formats

* png
* jpeg/jpg
* webp
* avif

## Usage

Convert the format

```sh
picpress -i a.png -o b.jpeg
```

Compress the image quality(percentage)
```sh
picpress -i a.png -o b.jpeg -q 80
```

Resize the picture(default style is fit)
```sh
picpress -i a.png -o b.jpeg -r 800x600
```

Specify the style for picture resize, supported fill/fit/exact
```sh
picpress -i a.png -o b.jpeg -r 800*600 -m fill
```

For more details:
```sh
picpress --help
```

## Build & Install

Build and install the program.
```sh
make
make install
```

## C Interface

```c
int compress_img_c(const char* input, const char* ouput, const char* format, uint8_t quality, uint32_t width, uint32_t height, int method, uint8_t speed);
```

* input: input picture file path
* output: output path
* format: output format(can be `null`, defaulty infered from output parameter)
* quality: compression quality (webp/jpeg/avif)
* width/height: output picture size
* method: `fill/fit/exact`(please refer to `enum ResizeStyle` in `include/picpress.h`)
* speed: avif compression speed (Default value is 4. The range is 1 to 10 and cannot work for other pic type)

The return value of function indicates success or error; the error type as follows:

```c
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
```


Compiled as static lib in linux with gcc:
```sh
gcc test.c -L./target/release -static -lpicpress -lm -o a
```