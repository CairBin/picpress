use image::imageops::FilterType;
use image::{DynamicImage, ImageError, ImageFormat, RgbImage};
use ravif::RGBA8;
use std::ffi::{c_char, CStr};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum PicPressError{
    #[error("Invalid format `{0}`")]
    InvalidFormat(String),

    #[error("Cannot infer format")]
    InferFormatError,

    #[error("Invalid method `{0}`")]
    InvalidMethod(String),

    // image lib internal error
    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Compression error: picture format = `{0}`. Details: {1}")]
    CompressError(String, String),

    #[error("Command parameter error: {0}")]
    ParameterError(String)
}

pub type Result<T> = std::result::Result<T, PicPressError>;


pub fn get_fmt_from_str(s: &str) -> Result<ImageFormat>{
    match s.to_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
        "png" => Ok(ImageFormat::Png),
        "webp" => Ok(ImageFormat::WebP),
        "avif" => Ok(ImageFormat::Avif),
        _ => Err(PicPressError::InvalidFormat(s.to_string()))
    }
}

pub fn determine_output_format(output_path: &str, format: Option<String>) -> Result<ImageFormat>{
    let fmt = if format.is_none(){
        let path = Path::new(output_path);
        let ext = path.extension().and_then(|e| e.to_str()).ok_or_else(|| PicPressError::InferFormatError)?;
        ext.to_string()
    }else{
        format.unwrap()
    };

    return get_fmt_from_str(&fmt);
}


fn convert_to_rgb8(img: &DynamicImage) ->RgbImage{
    match img {
        DynamicImage::ImageRgb8(rgb) => rgb.clone(),
        DynamicImage::ImageRgba8(rgba) => {
            // 处理透明通道，将 RGBA 转换为 RGB
            let mut rgb = RgbImage::new(rgba.width(), rgba.height());
            for (x, y, pixel) in rgb.enumerate_pixels_mut() {
                let rgba_pixel = rgba.get_pixel(x, y);
                *pixel = image::Rgb([rgba_pixel.0[0], rgba_pixel.0[1], rgba_pixel.0[2]]);
            }
            rgb
        }
        _ => {
            // 对于其他格式，统一转换为 RGB8
            img.to_rgb8()
        }
    }
}

fn resize_image(img: &DynamicImage, dimensions:Option<(u32, u32)>, method:Option<&str>)->Result<DynamicImage>{
    if dimensions.is_some(){
        let (width, height) = dimensions.unwrap();

        match method{
            Some(m)=>{
                if m == "fill"{
                    return Ok(img.resize_to_fill(width, height, FilterType::Lanczos3));
                }else if m == "exact"{
                    return Ok(img.resize_exact(width, height, FilterType::Lanczos3));
                }else if m == "fit"{
                    return Ok(img.resize(width, height, FilterType::Lanczos3));
                }else{
                    return Err(PicPressError::InvalidMethod(m.to_string()));
                }
            }

            _=>{
                return Ok(img.resize(width, height, FilterType::Lanczos3));
            }
        }

    }
    Ok(img.clone())
}

pub fn compress_img(input:&str, output:&str, format:Option<String>,  quality:u8, resize:Option<(u32,u32)>, method:Option<&str>, speed:u8)->Result<()>{
    let img = image::open(input)?;

    let output_format = determine_output_format(output, format)?;

    let output_path = Path::new(output);
    let mut output_file = File::create(output_path)?;

    let img = resize_image(&img, resize, method)?;
    match output_format{
        ImageFormat::Jpeg => {
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output_file, quality);
            let rgb_img = convert_to_rgb8(&img);
            
            encoder.encode(
                rgb_img.as_raw(),
                rgb_img.width(),
                rgb_img.height(),
                image::ColorType::Rgb8
            ).map_err(|e| PicPressError::CompressError("jpeg".to_string(), e.to_string()))?;
        },
        ImageFormat::Png=>{
            img.write_to(&mut output_file, ImageFormat::Png)
                .map_err(|e| PicPressError::CompressError("png".to_string(), e.to_string()))?;
        },
        ImageFormat::WebP => {
            let encoder = webp::Encoder::new(img.as_bytes(), webp::PixelLayout::Rgb, img.width(), img.height());
            let buf = encoder.encode(quality as f32);
            output_file.write_all(&buf)
                .map_err(|e| PicPressError::CompressError("webp".to_string(), e.to_string()))?;
        },
        ImageFormat::Avif => {
            let rgba_img = img.to_rgba8();
            let (width, height) = rgba_img.dimensions();
            let rgba_slice: &[RGBA8] = {
                unsafe{
                    std::slice::from_raw_parts(
                        rgba_img.as_ptr() as *const RGBA8,
                        rgba_img.len() / 4 
                    )
                }
            };

            let img_data = ravif::Img::new(rgba_slice, width as usize, height as usize);
            let res = ravif::Encoder::new()
                .with_quality(quality as f32)
                .with_speed(speed)
                .with_internal_color_model(ravif::ColorModel::RGB)
                .encode_rgba(img_data)
                .map_err(|e| PicPressError::CompressError("avif".to_string(), e.to_string()))?;
            output_file.write_all(&res.avif_file)?;
        }

        _ => {
            return Err(PicPressError::InvalidFormat("Unknow".to_string()));
        }
    }

    Ok(())
}


#[unsafe(no_mangle)]
pub extern "C" fn compress_img_c(
    input: *const c_char,
    output: *const c_char,
    format: *const c_char,
    quality: u8,
    width: u32,
    height: u32,
    method: u8,
    speed: u8
)->i32{
    if input.is_null() || output.is_null(){
        return -1;
    }

    let resize = if width <= 0 || height <=0{
        None
    }else{
        Some((width, height))
    };
    let quality = if quality <= 0 || quality >100{
        100
    }else {
        quality
    };

    let speed = if speed > 10 || speed < 1{
        4
    }else{
        speed
    };

    let method = match method{
        1 => {
            Some("fill")
        },
        3 => {
            Some("exact")
        },
        _ => {
            Some("fit")
        }
    };

    let input = unsafe{CStr::from_ptr(input)}.to_str();
    if input.is_err(){
        return -1;
    }
    let output = unsafe{CStr::from_ptr(output)}.to_str();
    if output.is_err(){
        return -1;
    }
    let fmt: Option<String>;
    if format.is_null(){
        fmt = None;
    }else{
        let fmt_res = unsafe{CStr::from_ptr(format)}.to_str();
        if fmt_res.is_err(){
            fmt = None;
        }else{
            fmt = Some(fmt_res.unwrap().to_string());
        }
    }
    let input = input.unwrap();
    let output = output.unwrap();


    let res = compress_img(input, output, fmt, quality, resize, method, speed);
    if res.is_ok() {
        return 0;
    }

    /*
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
 */

    match res.unwrap_err(){
        PicPressError::InvalidFormat(_) => -2,
        PicPressError::InferFormatError => -3,
        PicPressError::InvalidMethod(_) => -4,
        PicPressError::ImageError(_) => -5,
        PicPressError::IoError(_) => -6,
        PicPressError::CompressError(_, _) => -7,
        _ => -1
    }
}