use clap::Args;

#[derive(Args, Debug)]
pub struct SimCommand {
    /// testname, default is base_test
    pub testname: String,

    /// fpga sim
    #[arg(short, long)]
    pub fpga: Option<bool>,

    /// upf sim
    #[arg(short, long)]
    pub upf: Option<bool>,

    /// gls sim
    #[arg(short, long)]
    pub gls: Option<String>,

    /// pg sim
    #[arg(short, long)]
    pub pg: Option<String>,
}

impl SimCommand {
    pub fn run(&self) {
        println!("Running simulation with testname: {}", self.testname);
        if let Some(fpga) = self.fpga {
            println!("Sim arg is {}", fpga);
        }
        if let Some(gls) = &self.gls {
            println!("gls is {}", gls);
        }
        if let Some(pg) = &self.pg {
            println!("pg is {}", pg);
        }
        if let Some(upf) = self.upf {
            println!("upf is {}", upf);
        }
    }
}
