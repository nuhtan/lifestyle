use std::{io::Error, thread};

use tracking::core::{server, state_data::state::State, ui};

fn main() -> Result<(), Error> {
    let addr = [0, 0, 0, 0];
    let port = 8020;
    let shared_data = State::new(addr, port);
    let shared_data_ui = shared_data.clone();
    let server = server::Server::initialize(addr.into(), port, shared_data.clone());
    let server_handle = thread::spawn(move || server.listen().unwrap());
    let ui_handle = thread::spawn(move || ui::run(shared_data_ui));

    ui_handle.join().unwrap().unwrap();
    // Once the UI thread end the server should begin shutting down
    *shared_data.running.lock().unwrap() = false;
    server_handle.join().unwrap();
    shared_data.save();
    Ok(())
}
