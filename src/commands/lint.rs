use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct LintCommand {
    /// set a tool
    #[arg(short, long)]
    pub tool: Option<String>,

    /// script path
    pub path: PathBuf,
}

impl LintCommand {
    pub fn run(&self) {
        std::process::Command::new("jg")
            .args([
                "-superlint",
                "-nogui",
                self.path.to_str().expect("Invalid path"),
            ])
            .current_dir(".")
            .spawn()
            .expect("jaspergold command failed to start\n");
    }
}
