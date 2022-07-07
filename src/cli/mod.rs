//! DPCL command-line interface.

use structopt::StructOpt;
use anyhow::Result;

/// Run operations on a DPCL pipeline.
#[derive(Debug, StructOpt)]
#[structopt(name="dpcl")]
pub struct DPCLRootCommand {
}

impl DPCLRootCommand {
  pub fn exec_main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
  }
}
