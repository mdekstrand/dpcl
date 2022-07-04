//! DPCL command-line interface.

use structopt::StructOpt;

/// Run operations on a DPCL pipeline.
#[derive(Debug, SturctOpt)]
#[structopt(name="dpcl")]
pub struct DPCLRootCommand {
}
