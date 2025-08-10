use anyhow::{Context, Result};
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, RgbImage};
use ravif::RGBA8;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn determine_output_format(output_path:&str, format:Option<&str>)->Result<ImageFormat>{
    if let Some(fmt) = format{
        match fmt.to_lowercase().as_str(){
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "png" => Ok(ImageFormat::Png),
            "webp" => Ok(ImageFormat::WebP),
            "avif" => Ok(ImageFormat::Avif),
            _ => Err(anyhow::anyhow!("Unsupported format: {}", fmt)),
        }
    }else {
        let path = Path::new(output_path);
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow::anyhow!("Cannot infer the format."))?;

        match ext.to_lowercase().as_str(){
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "png" => Ok(ImageFormat::Png),
            "webp" => Ok(ImageFormat::WebP),
            "avif" => Ok(ImageFormat::Avif),
            _ => Err(anyhow::anyhow!("Unsupported format: {}", ext)),
        }
    }
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
                    return Err(anyhow::anyhow!("Unsupported method"));
                }
            }

            _=>{
                return Ok(img.resize(width, height, FilterType::Lanczos3));
            }
        }

    }
    Ok(img.clone())
}

pub fn compress_img(input:&str, output:&str, format:Option<&str>,  quality:u8, resize:Option<(u32,u32)>, method:Option<&str>, speed:u8)->Result<()>{
    let img = image::open(input)
        .with_context(|| format!("Cannot open the picture {}", input))?;

    let output_format = determine_output_format(output, format)
        .with_context(|| "Cannot ensure ouput format.")?;

    let output_path = Path::new(output);
    let mut output_file = File::create(output_path)
        .with_context(|| format!("Cannot create output file {}", output))?;

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
            ).with_context(|| "Failed to compress the picture. format = jpeg")?;
        },
        ImageFormat::Png=>{
            img.write_to(&mut output_file, ImageFormat::Png)
                .with_context(|| "Failed to compress the picture. format = png")?;
        },
        ImageFormat::WebP => {
            let encoder = webp::Encoder::new(img.as_bytes(), webp::PixelLayout::Rgb, img.width(), img.height());
            let buf = encoder.encode(quality as f32);
            output_file.write_all(&buf)
                .with_context(|| "Failed to compress the picture. format = webp")?;
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
                .encode_rgba(img_data)?;
            output_file.write_all(&res.avif_file)?;
        }

        _ => {
            return Err(anyhow::anyhow!("Unsupported format"));
        }
    }

    Ok(())
}