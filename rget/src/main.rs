use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

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

fn create_progress_bar(
    msg: &'static str,
    length: Option<u64>,
    quite_mode: Option<bool>,
) -> ProgressBar {
    let bar = match quite_mode {
        Some(true) => ProgressBar::hidden(),
        Some(false) | None => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(msg);
    let style = match length.is_some() {
        true => ProgressStyle::default_bar()
                .progress_chars("=> ")
                .template("{msg} {spinner:green} [{elapsed_precise}] [{wide_bar:cyan/blue}] {bytes}/{total_bytes} eta:{eta}")
                .unwrap(),
        false => ProgressStyle::default_spinner(),
    };
    bar.set_style(style);

    bar
}
