use super::{response, State};

pub fn add_day<'a>(
    body: String,
    shared_data: State,
) -> Result<response::Response<'a>, response::Response<'a>> {
    match serde_json::from_str(body.as_str()) {
        Ok(cal) => {
            let mut state = shared_data.calories.lock().unwrap();
            state.push(cal);
            Ok(response::Response::<'a>::new(
                201,
                "application/json",
                String::from("{\'status\': \'completed'}"),
            ))
        }
        Err(_) => Err(response::Response::<'a>::new(
            400,
            "application/json",
            String::from("{\'message\': \'invalid data, could not deserialize\'}"),
        )),
    }
}
