use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true, help = "url to download")]
    url: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.url)
}
