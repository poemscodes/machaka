use clap::{Parser, Subcommand};
use plistt::json;
use plistt::{BufReader, BufWriter};
use std::fs::OpenOptions;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;
// use xq::{module_loader::PreludeLoader, run_query, InputError, Value};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // #[command(arg_required_else_help(true))]
    Lsusb {
        //#[arg(default_value_t = Some("-".to_string()))]
        // action: Option<String>,
    },
}
const DEFAULT_DESTINATION_FILENAME: &'static str = "/dev/stdout";

fn default_destination_filename() -> String {
    String::from(DEFAULT_DESTINATION_FILENAME)
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Lsusb { .. } => {
            let ioreg = Command::new("ioreg")
                .arg("-c")
                .arg("IOUSB")
                .arg("-a")
                .output()
                .unwrap();
            let input = BufReader::new(Cursor::new(ioreg.stdout));

            let stdout = OpenOptions::new()
                .write(true)
                .open(Path::new(&default_destination_filename()))
                .unwrap();

            let output = BufWriter::new(stdout);
            // let module_loader = PreludeLoader();
            // let result_iterator = run_query(&query, context, input, &module_loader)
            //     .map_err(|e| anyhow!("{:?}", e))
            //     .with_context(|| "compile query")?;

            json::transcode_from_xml_reader(input, output)
        }
    }
}
