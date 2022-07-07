use anyhow::Result;
use dpcl::cli::DPCLRootCommand;

fn main() -> Result<()> {
  DPCLRootCommand::exec_main()
}
