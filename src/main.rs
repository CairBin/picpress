use anyhow::Result;
use clap::Parser;
use picpress::compress_img;

fn parse_resize(s: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Dimensions must be in WIDTHxHEIGHT format"));
    }
    
    let width = parts[0].parse::<u32>()?;
    let height = parts[1].parse::<u32>()?;
    
    Ok((width, height))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short, long, help="Source file")]
    input: String,

    #[arg(short,  long, help="Output file")]
    output: String,

    #[arg(short, long, default_value_t = 100, help="Quality(percentage) of output picture, supported webp/jpeg/avif")]
    quality: u8,

    #[arg(short, long, help="Output file format")]
    format: Option<String>,

    #[arg(short, long, value_parser = parse_resize, value_name="WIDTHxHEIGHT", help="Resize dimensions in WIDTHxHEIGHT format (e.g., 800x600)")]
    resize: Option<(u32, u32)>,

    #[arg(short, long, help="Resize style")]
    method: Option<String>
}

fn main() -> Result<()>{
    let args = Args::parse();

    if args.quality < 1 || args.quality > 100{
        return Err(anyhow::anyhow!("The quality must be between 1-100."));
    }

    println!("Input file: {}", args.input);
    println!("Output file: {}", args.output);
    println!("Quality: {}", args.quality);
    if args.resize.is_some(){
        let temp = args.resize.clone();
        let temp = temp.unwrap();
        println!("Resize: {}x{}", temp.0, temp.1);

        if args.method.is_none(){
            println!("Resize Method: fit")
        }else{
            let temp2 = args.method.clone();
            println!("Resize Method: {}", temp2.unwrap());
        }
    }

    compress_img(&args.input, &args.output, args.format.as_deref(), args.quality, args.resize, args.method.as_deref())?;
    Ok(())
}