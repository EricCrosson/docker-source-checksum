use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Distributed code review system")]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Opts {
    /// Output hash in hex
    #[structopt(long = "hex")]
    pub hex: bool,

    /// Path relative to context to ignore in the checksum
    #[structopt(long = "ignore-path")]
    pub ignore_path: Vec<PathBuf>,

    /// Path relative to context to include in the checksum
    #[structopt(long = "extra-path")]
    pub extra_path: Vec<PathBuf>,

    /// String (like arguments to dockerfile) to include in the checksum
    #[structopt(long = "extra-string")]
    pub extra_string: Vec<String>,

    #[structopt(name = "file", short = "f", long = "file")]
    pub dockerfile_path: Option<PathBuf>,

    pub context_path: PathBuf,
}
