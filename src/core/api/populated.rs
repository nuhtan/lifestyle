// use super::{response::Response, State};

// pub fn generate_goals<'a>(shared_data: State) -> Response<'a> {
//     let recent_cals = shared_data.calories.lock().unwrap().first().unwrap();
//     let content = format!(
//     "<div class=\"cards\">
//         <div class=\"card\">
//             <div class=\"card-info\">
//                 <h2>Weight</h2>
//                 <p>Overall Goal: 145 lbs</p>
//             </div>
//             <h2 class=\"percentage\">{}%</h2>
//         </div>
//         <div class=\"card\">
//             <div class=\"card-info\">
//                 <h2>Valorant</h2>
//                 <p>Target Rank: Gold 1</p>
//             </div>
//             <h2 class=\"percentage\">{}%</h2>
//         </div>
//         <div class=\"card\">
//             <div class=\"card-info\">
//                 <h2>Website Progress</h2>
//                 <p>Feature Completion</p>
//             </div>
//             <h2 class=\"percentage\">{}%</h2>
//         </div>
//     </div>", "", "", "");
// }

// pub fn generate_calories<'a>(shared_data: State) -> Response<'a> {

// }

// pub fn generate_shopping<'a>(shared_data: State) -> Response<'a> {

// }

// pub fn generate_valorant<'a>(shared_data: State) -> Response<'a> {

// }

// pub fn generate_progress<'a>(shared_data: State) -> Response<'a> {

// }