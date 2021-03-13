use std::{
    io::Error,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, TcpListener, TcpStream},
    thread,
};

use super::constant::serve_file::generate_request;

pub const HTML_PATH: &str = "public";

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn initialize(addr: IpAddr, port: u16) -> Server {
        println!("Server listening on http://{}:{}", addr, port);
        Server {
            listener: TcpListener::bind((addr, port)).unwrap(),
        }
    }

    pub fn listen(self) -> Result<ExitState, Error> {
        for stream_res in self.listener.incoming() {
            let stream = stream_res?;
            let handle = thread::spawn(move || Server::handle_stream(stream));
            let exit = handle.join().unwrap();
        }
        Ok(ExitState::Close)
    }

    pub fn handle_stream(mut stream: TcpStream) -> Result<ExitState, Error> {
        let mut stream_reader = BufReader::new(stream.try_clone()?);
        let mut line = String::new();
        stream_reader.read_line(&mut line)?;
        let method = &line[0..line.find(" ").unwrap()];
        let request = &line[line.find("/").unwrap()..line.find("HTTP").unwrap() - 1];
        println!("[{}]: {} {}", stream.peer_addr().unwrap(), method, request,);
        let response = generate_request(
            match (method, request) {
                ("GET", req) => match req {
                    "/" => "index.html",
                    _ => "not_found.html",
                },
                _ => "not_found.html",
            },
            HTML_PATH,
        )
        .unwrap();
        stream.write_all(response.to_block().as_slice())?;
        return Ok(ExitState::Close);
    }
}

pub enum ExitState {
    Close,
    Crash,
}
