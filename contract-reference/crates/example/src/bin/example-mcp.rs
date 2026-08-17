use transport_harness::{
    HarnessError,
    mcp::rmcp::ServerHandler,
};

#[derive(Clone, Default)]
struct ExampleServer;

impl ServerHandler for ExampleServer {}

fn main() -> Result<(), HarnessError> {
    transport_harness::mcp::run(ExampleServer)
}
