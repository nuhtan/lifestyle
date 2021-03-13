use std::io::Error;

use tracking::core::server;

fn main() -> Result<(), Error> {
    let server = server::Server::initialize([0, 0, 0, 0].into(), 8020);
    server.listen().unwrap();
    Ok(())
}
