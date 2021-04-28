use super::{response, state_data::state::State};

pub mod calories;
pub mod progress;
pub mod rendered;

pub fn apply_request<'a>(
    path: &'a str,
    body: String,
    shared_data: State,
) -> Result<response::Response<'a>, response::Response<'a>> {
    match path {
        "/api/calories/add/day" => calories::add_day(body, shared_data),
        "/api/progress/add" => progress::add(body, shared_data),
        _ => Err(response::Response::new(
            404,
            "application/json",
            String::from("{\'message\':\'requested resource not found\'}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_request, State};

    #[test]
    fn apply_correct() {
        let state = State::new([0, 0, 0, 0], 8020);
        let body = String::from("{\"index\":1,\"total\":200,\"day_weight\":190.1,\"date\":[2021,3,18],\"burn\":300,\"food\":[[\"Steak\",500]]}");
        let response = String::from_utf8(
            apply_request("/api/calories/add/day", body, state)
                .unwrap()
                .to_block(),
        )
        .unwrap();
        assert_eq!(response, String::from("HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\"status\": \"completed\"}"), "This should work");
    }

    #[test]
    fn apply_incorrect_path() {
        let state = State::new([0, 0, 0, 0], 8020);
        let body = String::from("{\"index\":1,\"total\":200,\"date\":[2021,3,18],\"burn\":300,\"food\":[[\"Steak\",500]]}");
        let result = apply_request("/api/calorie/add/day", body, state);
        assert!(result.is_err());
        let response =
            String::from_utf8(result.expect_err("This should be an error").to_block()).unwrap();
        assert_eq!(response, String::from("HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\n\r\n{\'message\':\'requested resource not found\'}"), "This should work");
    }
}
