mod cipher;
mod ui;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    code: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();

    let host = cipher::get_host();
    let drive = "http://127.0.0.1:4444";
    let _ = ui::interface(drive, &host, &cli.code);

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}