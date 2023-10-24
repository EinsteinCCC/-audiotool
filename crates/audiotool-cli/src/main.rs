use rx::prelude::*;
use rx::clap::{self, Parser as _};
use std::path::PathBuf;
use std::fs;

mod convert;
mod split;

fn main() -> AnyResult<()> {
    rx::extras::init();

    let cli = Cli::parse();
    cli.run()?;

    Ok(())
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
    #[command(flatten)]
    args: Args,
}

#[derive(clap::Subcommand)]
enum Command {
    Convert(ConvertCommand),
}

#[derive(clap::Args)]
struct Args {
}

#[derive(clap::Args)]
struct ConvertCommand {
    config: PathBuf,
}

impl Cli {
    fn run(&self) -> AnyResult<()> {
        match &self.cmd {
            Command::Convert(cmd) => cmd.run(&self.args),
        }
    }
}

impl ConvertCommand {
    fn run(&self, _args: &Args) -> AnyResult<()> {
        use audiotool::convert as cvt;

        let config = fs::read_to_string(&self.config)?;
        let config: cvt::config::Config = rx::toml::from_str(&config)?;

        let (_tx, rx) = cvt::exec::spawn(config);

        // todo handle cancellation with tx

        loop {
            let resp = rx.recv()?;

            match resp {
                cvt::exec::Response::NextResult(res) => {
                    println!("{res:#?}");
                }
                cvt::exec::Response::Done => {
                    break;
                }
            }
        }

        Ok(())
    }
}
