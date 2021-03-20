use super::{response, state_data::state::State};

pub mod calories;

pub fn apply_request<'a>(
    path: &'a str,
    body: String,
    shared_data: State,
) -> Result<response::Response<'a>, response::Response<'a>> {
    match path {
        "/api/calorie/add/day" => calories::add_day(body, shared_data),
        _ => Err(response::Response::new(
            404,
            "application/json",
            String::from("{\'message\':\'requested resource not found\'}"),
        )),
    }
}
