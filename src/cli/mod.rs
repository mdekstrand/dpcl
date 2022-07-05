//! DPCL command-line interface.

use structopt::StructOpt;

/// Run operations on a DPCL pipeline.
#[derive(Debug, StructOpt)]
#[structopt(name="dpcl")]
pub struct DPCLRootCommand {
}
