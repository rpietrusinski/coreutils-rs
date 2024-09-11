use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CpCli {
    #[arg(short, long)]
    pub source: String,
    #[arg(short, long)]
    pub dest: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct GrepCli {
    #[arg(short, long)]
    pub query: String,
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub ignore_case: bool,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct LsCli {
    #[arg(short, long)]
    pub dir: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct HeadCli {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub num_lines: u32,
}
