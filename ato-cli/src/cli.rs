use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'd', long, name = "DATA_PATH")]
    data: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let args = Args::parse();
    let _ = libato::two_stage(&args.data);

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}
