#![warn(clippy::all, clippy::pedantic)]
use octocrab::Octocrab;
use structopt::StructOpt;
use confy;
use config::GhConfig;
use github::GhRunner;

mod config;
mod github;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    config: std::path::PathBuf
}

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args = Cli::from_args();

    let config : GhConfig = confy::load_path(args.config).expect("Cannot load config");
    let octocrab = Octocrab::builder().personal_token(config.token.clone()).build().expect("Failed to create octocrab client");
    let gh_runner = GhRunner::new(octocrab);
    let results = gh_runner.query(&config).await;
    for result in results {
        println!("{}", result)
    }
    return Result::Ok(())
}
