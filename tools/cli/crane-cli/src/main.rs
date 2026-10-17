use clap::Parser;

use crane_cli::{
    CraneCli,
    run,
};

fn main() {
    let cli = CraneCli::parse();
    match run(cli) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        },
    }
}
