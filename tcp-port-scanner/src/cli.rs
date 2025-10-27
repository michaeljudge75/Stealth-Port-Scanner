//This File Contains Everything Relating to Parsing, Validating, and Documenting CLI including how
//help and usage text appears
use clap::Parser;

//Different Options for Command Line Arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "TCP Port Scanner", version, author, about = "A Conccurrent TCP port scanner written in Rust")]
pub struct CliArgs{
    #[arg(help = "Target host to scan (e.g. 127.0.0.1 or example.com")]
    pub target: String,

    #[arg(short, long, default_value = "1-1024", help = "Port range to scan, e.g. 1-1024 or 80, 443")]
    pub ports: PortRange,

    #[arg(long, default_value = "connect", value_enum, help ="Scan mode: connect or timed")]
    pub mode: ScanMode,

    #[arg(long, default_value_t = 500, help = "Connection timeout in milliseconds")]
    pub timeout_ms: u64,

    #[arg(long, default_value_t = 100, help = "Max concurrent scans")]
    pub concurrency: usize,

    #[arg(long, help = "Limit scans per second (optional)")] 
    pub rate_limit: Option<u64>,

    #[arg(long, help = "Output file path for results (optional")]
    pub output_path: Option<String>,

    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,
}
//Defines a start and end for port range
pub struct PortRange{
    pub start: u16;
    pub end: u16
}

//Represets how Scanner performs its scans
pub enum ScanMode{
    Connect,
    Timed,
}

//Custom Error Type
pub enum CliError{
    InvalidPortRange,
    InvalidTarget,
    IOError,
}

//Helps Parse the CLI Arguments, wraps CliArgs::parse() and adds Validation
pub fn parse_args() -> Result<CliArgs, CliError>{
    let mut args = CliArgs::parse();

    if args.concurrency == 0{
        return Err(CliError::InvalidArgument("Concurrency must be greater than 0".into()));
    }



}

//Helps Parse the Specified Ports into Numbers
pub fn parse_port_range(input: &str) -> Result<PortRange, CliError>{


}
