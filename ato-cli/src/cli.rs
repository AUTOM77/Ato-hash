use clap::Parser;
use libato::process;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'd', long, name = "DATA_PATH")]
    dataset: String,
    #[clap(short = 'e', long, name = "EXTERNAL")]
    external: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();
    let start = std::time::Instant::now();
    let _ = process(&_args.dataset, &_args.external);
    let duration = start.elapsed();
    println!("Time elapsed in process() is: {:?}", duration);
    Ok(())
}
