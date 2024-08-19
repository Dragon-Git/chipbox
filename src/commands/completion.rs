use clap::{Args, Command};
use std::io;

#[derive(Args)]
pub struct CompletionCommand {
    /// completion_shell
    #[arg(value_enum)]
    pub shell: clap_complete::Shell,
}

impl CompletionCommand {
    pub fn run(&self, app: &mut Command) {
        clap_complete::generate(self.shell, app, "chipbox", &mut io::stdout());
    }
}
