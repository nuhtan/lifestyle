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

pub fn gather<'a>(shared_data: State) -> response::Response<'a> {
    let calories = shared_data.calories.lock().unwrap().clone();
    match serde_json::to_string(&calories) {
        Ok(body) => response::Response::new(200, "application/json", body),
        Err(_) => response::Response::new(
            500,
            "application/json",
            String::from("{\"status\":\"Could not serialize vector of calories\""),
        ),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::{add_day, State};

//     #[test]
//     fn test_correct_same_ordering() {
//         let state = State::new([0, 0, 0, 0], 8020);
//         let body = String::from("{\"index\":1,\"total\":200,\"date\":[2021,3,18],\"burn\":300,\"food\":[[\"Steak\",500]]}");
//         let response = String::from_utf8(add_day(body, state).unwrap().to_block()).unwrap();
//         assert_eq!(response, String::from("HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\'status\': \'completed\'}"), "Error found");
//     }

//     #[test]
//     fn test_correct_incorrect_ordering() {
//         let state = State::new([0, 0, 0, 0], 8020);
//         let body = String::from("{\"index\":1,\"date\":[2021,3,18],\"food\":[[\"Steak\",500]],\"burn\":300,\"total\":200}");
//         let response = String::from_utf8(super::add_day(body, state).unwrap().to_block()).unwrap();
//         assert_eq!(response, String::from("HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\'status\': \'completed\'}"), "Error found");
//     }

//     #[test]
//     fn test_incorrect() {
//         let state = State::new([0, 0, 0, 0], 8020);
//         let body = String::from("{\"index\":1,\"dudes\":[2021,3,18],\"food\":[[\"Steak\",500]],\"burn\":300,\"total\":200}");
//         let result = super::add_day(body, state);
//         assert!(result.is_err());
//         let response =
//             String::from_utf8(result.expect_err("This should be an error").to_block()).unwrap();
//         assert_eq!(response, String::from("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{\'message\': \'invalid data, could not deserialize\'}"));
//     }
// }
