use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Copy file from <source> to <dest>")]
pub struct CpCli {
    #[arg(short, long)]
    pub source: String,
    #[arg(short, long)]
    pub dest: String,
}

#[derive(Parser)]
#[command(version, about = "Return lines from text file <file> including <query> phrase. \
Setting <ignore_case> will impose case insensitivity")]
pub struct GrepCli {
    #[arg(short, long)]
    pub query: String,
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub ignore_case: bool,
}

#[derive(Parser)]
#[command(version, about = "List files and directories inside <dir>")]
pub struct LsCli {
    #[arg(short, long)]
    pub dir: String,
}

#[derive(Parser)]
#[command(version, about = "Return <num_lines> first lines from a text file <file>")]
pub struct HeadCli {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub num_lines: u32,
}
