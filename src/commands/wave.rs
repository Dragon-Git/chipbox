use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct WaveCommand {
    /// get a filelist
    #[arg(short, long)]
    pub filelist: Option<PathBuf>,

    /// verdi database path
    pub path: PathBuf,
}

impl WaveCommand {
    pub fn run(&self) {
        if let Some(filelist) = &self.filelist {
        std::process::Command::new("vericom")
            .args(["-sv", "-ntb_opts", "uvm", "-f", filelist])
            .current_dir(self.path.clone())
            .spawn()
            .expect("vericom command failed to start");
        std::process::Command::new("elabcom")
            .args(["-lib", "work.lib++", "-elab", "kdb"])
            .current_dir(self.path.clone())
            .spawn()
            .expect("elabcom command failed to start");
        }
        std::process::Command::new("verdi")
            .arg(["-q", "-nologo", "-dbdir", "simv.daidir", "-ssf", "top.fsdb"])
            .current_dir(self.path.clone())
            .spawn()
            .expect("verdi command failed to start");
    }
}
