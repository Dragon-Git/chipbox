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
            println!("filelist is{}", filelist.display());
        }
        std::process::Command::new("verdi")
            .arg("-q -nologo -dbdir simv.daidir -ssf top.fsdb")
            .current_dir(self.path.clone())
            .spawn()
            .expect("ls command failed to start");
    }
}
