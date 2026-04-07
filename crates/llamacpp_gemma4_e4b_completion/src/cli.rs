use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
pub struct Cli {
    /// verbosity
    #[clap(flatten)]
    pub verbose: Verbosity,

    /// Do not stream model response (wait for entire response and output everything at once)
    #[clap(short, long, action)]
    pub no_stream: bool,
}
