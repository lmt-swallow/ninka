use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {}

pub fn run(up_opts: Opts) -> i32 {
    log::debug!("build subcommand called");
    0
}
