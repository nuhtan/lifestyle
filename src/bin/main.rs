use std::{io:: Error, thread};

use tracking::core::{server, state_data::state::State, ui};

fn main() -> Result<(), Error> {
    let shared_data = State::new();
    let server = server::Server::initialize([0, 0, 0, 0].into(), 8020, shared_data.clone());
    let server_handle = thread::spawn(move || server.listen().unwrap());
    let ui_handle = thread::spawn(move || ui::initialize());

    server_handle.join().unwrap();
    ui_handle.join().unwrap();
    Ok(())
}
