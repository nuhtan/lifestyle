use std::{
    io::Error,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, TcpListener, TcpStream},
    thread,
};

use super::{
    constant::serve_file::generate_response,
    response::Response,
};

pub const HTML_PATH: &str = "www";

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

    pub fn listen(self) -> Result<(), Error> {
        for stream_res in self.listener.incoming() {
            let stream = stream_res?;
            let handle = thread::spawn(move || Server::handle_stream(stream));
            let _exit = handle.join().unwrap();
        }
        Ok(())
    }

    pub fn handle_stream(mut stream: TcpStream) -> Result<(), Error> {
        let mut stream_reader = BufReader::new(stream.try_clone()?);
        let mut line = String::new();
        stream_reader.read_line(&mut line).unwrap();
        let method = &line[0..line.find(" ").unwrap()];
        let request = &line[line.find("/").unwrap()..line.find("HTTP").unwrap() - 1];
        println!("[{}]: {} {}", stream.peer_addr().unwrap(), method, request,);
        let response = gather_response(method, request);
        stream.write_all(response.to_block().as_slice()).ok();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        Ok(())
    }
}

pub fn gather_response<'a>(method: &str, request: &'a str) -> Response<'a> {
    match (method, request) {
        ("GET", req) => match generate_response(
            match req {
                "/" => "index.html",
                "/api" => "explanation.html",
                _ => req,
            },
            HTML_PATH,
        ) {
            Ok(res) => res,
            Err(res) => res,
        },
        _ => generate_response("not_found.html", HTML_PATH).unwrap(),
    }
}
