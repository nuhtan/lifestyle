use std::{
    io::Error,
    io::{BufRead, BufReader, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    thread,
};

use super::{api::{self}, content::serve_file, response::Response, state_data::state::State};

pub const HTML_PATH: &str = "www";

pub struct Server {
    listener: TcpListener,
    shared_data: State,
}

impl Server {
    /// Binds the listener to a port and address and sets nonblocking to true, afterwards returning the struct.
    pub fn initialize(addr: IpAddr, port: u16, shared_data: State) -> Server {
        let listener = TcpListener::bind((addr, port)).unwrap();
        listener.set_nonblocking(true).unwrap(); // Used to better determine when the server should shutdown
        Server {
            listener,
            shared_data,
        }
    }

    /// Listens for incoming connections and spawns a thread to handle each.
    pub fn listen(self) -> Result<(), Error> {
        for stream_res in self.listener.incoming() {
            // If the ui has been exited shutdown the listener, any ongoing threads should finish first, i think.
            if *self.shared_data.clone().running.lock().unwrap() == false {
                return Ok(());
            }
            match stream_res {
                Ok(stream) => {
                    let data = self.shared_data.clone();
                    let handle = thread::spawn(move || Server::handle_stream(stream, data));
                    let exit = handle.join();
                    match exit {
                        // TODO This should be changed
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Ok(())
    }

    // TODO simplify this method, all of the read lines should happen in the loop.
    pub fn handle_stream(mut stream: TcpStream, shared_data: State) -> Result<(), Error> {
        let mut stream_reader = BufReader::new(stream.try_clone()?);
        let mut lines = String::new();
        stream_reader.read_line(&mut lines).unwrap();
        let mut lines2 = lines.clone();
        let line = &mut lines[..lines2.len() - 2];

        let method = &line[0..line.find(" ").unwrap()];
        let request = &line[line.find("/").unwrap()..line.find("HTTP").unwrap() - 1];
        loop {
            let count = stream_reader.read_line(&mut lines2).unwrap();
            if count == 2 {
                break;
            }
        }

        let mut body = String::new();
        if method == "POST" {
            let split = lines2.split("\r\n");
            for l in split {
                if l.len() >= 17 {
                    if &l[0..16] == "Content-Length: " {
                        let length: usize = l[16..].parse().unwrap();
                        let mut temp_buf = vec![0; length];
                        stream_reader.read_exact(&mut temp_buf).unwrap();
                        body = String::from_utf8(temp_buf).unwrap();
                    }
                }
            }
        }

        shared_data.add_request((
            stream.peer_addr().unwrap(),
            request.clone().to_owned(),
            method.clone().to_owned(),
        ));
        let response = gather_response(method, request, body, shared_data);
        stream.write_all(response.to_block().as_slice()).ok();
        Ok(())
    }
}

/// Depending on the method and request return a Response for the request.
pub fn gather_response<'a>(
    method: &str,
    request: &'a str,
    body: String,
    shared_data: State,
) -> Response<'a> {
    match (method, request) {
        ("GET", req) => {
            match req {
                // "/api/pages/goals" => generate_goals(shared_data),
                // "/api/pages/calories" => generate_goals(shared_data),
                // "/api/pages/shopping" => generate_goals(shared_data),
                // "/api/pages/valorant" => generate_goals(shared_data),
                // "/api/pages/progress" => generate_goals(shared_data),
                "/" | "/api" | _ => match serve_file::generate_response(req, HTML_PATH) {
                    Ok(res) => res,
                    Err(res) => res
                }
            }
        },
        ("POST", req) => match api::apply_request(req, body, shared_data) {
            Ok(res) => res,
            Err(res) => res,
        },
        _ => serve_file::generate_response("not_found.html", HTML_PATH).unwrap(),
    }
}
