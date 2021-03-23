use std::{fs, path::Path};

use super::super::response;

pub fn generate_response<'a>(
    file: &'a str,
    dir_prepend: &'a str,
) -> Result<response::Response<'a>, response::Response<'a>> {
    match fs::read_to_string(format!("{}/{}", dir_prepend, &file)) {
        Ok(contents) => Ok(response::Response::new(200, content_type(file), contents)),
        _ => Err(response::Response::new(
            404,
            "text/html",
            match fs::read_to_string(format!("{}/not_found.html", dir_prepend)) {
                Ok(body) => body,
                Err(_) => String::from("<html><head><meta charset='utf-8'><title>Not Found</title></head><body>Check that root directory is correct.</body></html>")
            },
        )),
    }
}

fn content_type(file: &str) -> &str {
    match Path::new(file).extension().unwrap().to_str().unwrap() {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "ico" => "image/x-icon",
        _ => "text/plain",
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Read};

    use super::{content_type, generate_response};

    #[test]
    fn generate_response_bad_file() {
        let result = generate_response("indexer.html", "www");
        assert!(result.is_err());
        let response = String::from_utf8(result.unwrap_err().to_block()).unwrap();
        let mut contents = String::new();
        fs::File::open("www/not_found.html")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert_eq!(
            response,
            format!(
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n{}",
                contents
            ),
            "File should not exist"
        );
    }

    #[test]
    fn generate_response_bad_base_dir() {
        let result = generate_response("index.html", "wwwtest");
        assert!(result.is_err());
        let response = String::from_utf8(result.unwrap_err().to_block()).unwrap();
        let mut contents = String::new();
        fs::File::open("www/not_found.html")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert_eq!(
            response,
            format!(
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n{}",
                String::from("<html><head><meta charset='utf-8'><title>Not Found</title></head><body>Check that root directory is correct.</body></html>")
            ),
            "File should not exist"
        );
    }

    #[test]
    fn generate_good_response() {
        let response = String::from_utf8(generate_response("sample.html", "www").unwrap().to_block()).unwrap();
        assert_eq!(response, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html>\n<html>\n<head>\n    <meta charset=\'utf-8\'>\n    <title>Sample</title>\n</head>\n<body>\n    This is just a sample html file.\n</body>\n</html>", "Did not get proper response");
    }

    #[test]
    fn content_type_exists() {
        assert_eq!(content_type("test.html"), "text/html", "html should work");
        assert_eq!(content_type("test.css"), "text/css", "css should work");
        assert_eq!(
            content_type("test.js"),
            "application/javascript",
            "js should work"
        );
        assert_eq!(content_type("test.ico"), "image/x-icon", "ico should work");
    }

    #[test]
    fn content_type_other() {
        assert_eq!(
            content_type("test.test"),
            "text/plain",
            "anything should work"
        );
    }
}
