use clap::Parser;
use image::ImageFormat;
use picpress::{compress_img, determine_output_format, Result, PicPressError};

fn parse_resize(s: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return Err(PicPressError::ParameterError("Dimensions must be in WIDTHxHEIGHT format".to_string()));
    }
    
    let width = parts[0].parse::<u32>().map_err(|e| PicPressError::ParameterError(e.to_string()))?;
    let height = parts[1].parse::<u32>().map_err(|e| PicPressError::ParameterError(e.to_string()))?;
    
    Ok((width, height))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short, long, help="Source path")]
    input: String,

    #[arg(short,  long, help="Output path")]
    output: String,

    #[arg(short, long, default_value_t = 100, help="Quality(percentage) of output picture, supported webp/jpeg/avif")]
    quality: u8,

    #[arg(short, long, help="Output file format")]
    format: Option<String>,

    #[arg(short, long, value_parser = parse_resize, value_name="WIDTHxHEIGHT", help="Resize dimensions in WIDTHxHEIGHT format (e.g., 800x600)")]
    resize: Option<(u32, u32)>,

    #[arg(short, long, help="Resize style")]
    method: Option<String>,

    #[arg(short, long, default_value_t = 4, help="AVIF speed")]
    speed: u8
}

fn main() -> Result<()>{
    let args = Args::parse();

    if args.quality < 1 || args.quality > 100{
        return Err(PicPressError::ParameterError("The quality must be between 1-100.".to_string()));
    }

    let fmt = determine_output_format(&args.output, args.format.clone())?;

    println!("Input file: {}", args.input);
    println!("Output file: {}", args.output);
    match fmt{
        ImageFormat::Avif | ImageFormat::Jpeg | ImageFormat::WebP => {
            println!("Quality: {}", args.quality);
            if fmt == ImageFormat::Avif{
                println!("Speed: {}", args.speed);
            }
        }
        _ => {
            println!("Quality: Output format not support");
        }
    }
    if args.resize.is_some(){
        let temp = args.resize.clone();
        let temp = temp.unwrap();
        println!("Resize: {}x{}", temp.0, temp.1);

        if args.method.is_none(){
            println!("Resize Style: fit")
        }else{
            let temp2 = args.method.clone();
            println!("Resize Style: {}", temp2.unwrap());
        }
    }

    compress_img(&args.input, &args.output, args.format, args.quality, args.resize, args.method.as_deref(), args.speed)?;
    Ok(())
}