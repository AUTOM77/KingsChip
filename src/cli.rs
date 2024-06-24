mod cipher;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    code: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();

    let host = cipher::get_host();
    println!("{:#?}", host);
    println!("{:#?}", cli.code);

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}