use transport_harness::{
    HarnessError,
    Output,
    cli::clap::{
        self,
        Parser,
    },
};

#[derive(Parser)]
#[command(name = "example")]
struct ExampleCommand {}

fn main() -> Result<(), HarnessError> {
    transport_harness::cli::run::<ExampleCommand, _>(|_command| {
        Ok(Output::Text(format!("{}-cli", example::domain_name())))
    })
}
