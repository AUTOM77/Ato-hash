use clap::Parser;
use libato::mesh::Data;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'd', long, name = "DATA_PATH")]
    data: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();

    let _args = Args::parse();
    let start = std::time::Instant::now();
    let dir = &_args.data;

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}
