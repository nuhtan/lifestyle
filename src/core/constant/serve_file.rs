use std::{fs, path::Path};

use super::super::response;

pub fn generate_request<'a>(
    file: &'static str,
    dir_append: &'static str,
) -> Result<response::Response<'static>, response::Response<'static>> {
    match fs::read_to_string(format!("{}/{}", dir_append, &file)) {
        Ok(contents) => Ok(response::Response::new(200, content_type(file), contents)),
        _ => Err(response::Response::new(
            404,
            content_type(file),
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
