use std::{fs, path::Path};

use super::super::response;

pub fn generate_response<'a>(
    file: &'a str,
    dir_append: &'a str,
) -> Result<response::Response<'a>, response::Response<'a>> {
    match fs::read_to_string(format!("{}/{}", dir_append, &file)) {
        Ok(contents) => Ok(response::Response::new(200, content_type(file), contents)),
        _ => Err(response::Response::new(
            404,
            "text/html",
            fs::read_to_string(format!("{}/not_found.html", dir_append)).unwrap(),
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
