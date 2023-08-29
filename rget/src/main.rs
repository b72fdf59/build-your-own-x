use clap::Parser;
use console::style;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true, help = "url to download")]
    url: String,
    #[arg(short = 'q', long = "quiet", help = "quite (no output)")]
    quite: bool,
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(run()) {
        Ok(_) => println!("Done"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();

    download(&args.url, args.quite).await?;
    Ok(())
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(Cow::Owned(format!("{}", msg)));
    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .expect(msg)
                .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}

async fn download(target: &String, quiet_mode: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = Url::parse(target)?;
    let client = Client::new();

    let resp = client.get(url).send().await?;
    println!(
        "HTTP request sent... {} {}",
        style(format!("{}", resp.status())).green(),
        quiet_mode,
    );

    if resp.status().is_success() {
        let headers = resp.headers().clone();
        let ct_len = headers
            .get(header::CONTENT_LENGTH)
            .and_then(|x| x.to_str().ok())
            .and_then(|x| x.parse::<u64>().ok());

        let ct_type = headers
            .get(header::CONTENT_TYPE)
            .and_then(|x| x.to_str().ok());

        match ct_len {
            Some(x) => {
                println!(
                    "Length: {} ({}) {}",
                    style(x).green(),
                    style(format!("{}", HumanBytes(x))).red(),
                    quiet_mode,
                );
            }
            None => println!("Length: {} {}", style("unknown").red(), quiet_mode),
        };

        match ct_type {
            Some(x) => {
                println!("Type: {} {}", style(x).green(), quiet_mode);
            }
            None => (),
        }

        let fname = target.split("/").last().unwrap();
        println!("Saving to: {} {}", style(fname).green(), quiet_mode);

        let bar = create_progress_bar(quiet_mode, fname.clone(), ct_len);
        let bytes = resp.bytes().await?;
        bar.finish();

        fs::write(fname, bytes)?;
    }

    Ok(())
}
