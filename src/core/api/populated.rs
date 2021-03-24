use super::State;

pub fn generate_goals(shared_data: State) {
    let recent_cals = shared_data.calories.lock().unwrap().first().unwrap();
    let content = format!(
    "<div class=\"cards\">
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>Weight</h2>
                <p>Overall Goal: 145 lbs</p>
            </div>
            <h2 class=\"percentage\">{}%</h2>
        </div>
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>Valorant</h2>
                <p>Target Rank: Gold 1</p>
            </div>
            <h2 class=\"percentage\">{}%</h2>
        </div>
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>Website Progress</h2>
                <p>Feature Completion</p>
            </div>
            <h2 class=\"percentage\">{}%</h2>
        </div>
    </div>", "", "", "");
}