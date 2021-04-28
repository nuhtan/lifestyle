use crate::core::response;

use super::{super::state_data::progress, response::Response, State};

pub fn add<'a>(body: String, shared_data: State) -> Result<Response<'a>, Response<'a>> {
    match serde_json::from_str(body.as_str()) {
        Ok(bod) => {
            let mut state = shared_data.progress.lock().unwrap();
            match &bod {
                progress::ToDo::Bug(a, b) => state.in_progress.push(progress::ToDo::Bug(*a, b.clone())),
                progress::ToDo::Feature(a, b) => state.in_progress.push(progress::ToDo::Feature(*a, b.clone()))
            }
            Ok(response::Response::<'a>::new(
                201,
                "application/json",
                String::from("{\"status\": \"completed\"}"),
            ))
        },
        Err(_) => Err(response::Response::<'a>::new(
            400,
            "application/json",
            String::from("{\"message\": \"invalid data, could not deserialize\"}"),
        )),
    }
}
