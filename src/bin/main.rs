use std::io::Error;

use tracking::core::{server, state_data::state::State};

fn main() -> Result<(), Error> {
    let shared_data = State::new();
    let server = server::Server::initialize([0, 0, 0, 0].into(), 8020, shared_data.clone());
    server.listen().unwrap();
    Ok(())
}
