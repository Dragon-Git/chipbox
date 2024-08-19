use clap::Args;
use std::{fs, path::PathBuf};

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
            let filelist_str = match filelist.to_str() {
                Some(s) => s,
                None => panic!("Failed to convert PathBuf to &str"),
            };
            std::process::Command::new("vericom")
                .args(["-sv", "-ntb_opts", "uvm", "-f", filelist_str])
                .current_dir(self.path.clone())
                .spawn()
                .expect("vericom command failed to start\n");
            std::process::Command::new("elabcom")
                .args(["-lib", "work.lib++", "-elab", "kdb"])
                .current_dir(self.path.clone())
                .spawn()
                .expect("elabcom command failed to start\n");
        }
        // 搜索子目录，直到找到以 .daidir 结尾的目录并且该目录包含 kdb.elab++ 目录
        let mut daidir = "simv.daidir".to_string();
        if let Ok(entries) = fs::read_dir(self.path.clone()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() && entry_path.extension().unwrap() == "daidir" {
                        let kdb_path = entry_path.join("kdb.elab++");
                        if kdb_path.exists() && kdb_path.is_dir() {
                            daidir = entry_path.to_str().unwrap().to_string();
                            break;
                        }
                    }
                }
            }
        }
        std::process::Command::new("verdi")
            .args(["-q", "-nologo", "-dbdir", daidir.as_ref(), "-ssf", "top.fsdb"])
            .current_dir(self.path.clone())
            .spawn()
            .expect("verdi command failed to start\n");
    }
}
