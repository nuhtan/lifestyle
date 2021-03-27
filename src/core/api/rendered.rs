use super::{super::state_data::progress::ToDo, response::Response, State};

pub fn generate_goals<'a>(shared_data: State) -> Response<'a> {
    let cals = shared_data.calories.lock().unwrap().clone();
    let recent_cals = cals.last().unwrap();
    let games = shared_data.valorant.lock().unwrap().clone();
    let recent_game = games.last().unwrap();
    let progress = shared_data.progress.lock().unwrap().clone();
    let prog = progress.in_progress;
    let prog_len = prog.len().clone();
    let mut count_done: u32 = 0;
    let mut _count_not_done: u32 = 0;
    for todo in prog {
        match todo {
            ToDo::Bug(finished, _) => {
                match finished {
                    true => count_done += 1,
                    false => _count_not_done += 1,
                }
            },
            ToDo::Feature(finished, _) => {
                match finished {
                    true => count_done += 1,
                    false => _count_not_done += 1,
                }
            }
        }
    }
    let targets = shared_data.basics.lock().unwrap().clone();
    let content = format!(
    "<div class=\"cards\">
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>Weight</h2>
                <p>Overall Goal: 160 lbs</p>
            </div>
            <h2 class=\"percentage\">{:.2}%</h2>
        </div>
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>Valorant</h2>
                <p>Target Rank: Gold 1</p>
            </div>
            <h2 class=\"percentage\">{:.2}%</h2>
        </div>
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>Website Progress</h2>
                <p>Feature Completion</p>
            </div>
            <h2 class=\"percentage\">{:.2}%</h2>
        </div>
    </div>", 100.0 * (targets.weight_start - recent_cals.day_weight) / (targets.weight_start - targets.weight_goal), 100.0 * (recent_game.rank_rating_after as f32 / targets.rank_goal as f32), 100.0 / (count_done as f32 / prog_len as f32));
    Response::new(200, "text/html", content)
}

pub fn generate_calories<'a>() -> Response<'a> {
    let content = "
    <div class=\"cards\">
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>
                    Calorie Progress 
                    <button class=\"add\" id=\"add\">
                        <h2>+</h2>
                    </button>
                </h2>
                <canvas id=\"myChart\" width=\"600\" height=\"400\"></canvas>
                <iframe onload=\"fetch_cals()\" style=\"position: absolute;width:0;height:0;border:0;\"></iframe>
            </div>
        </div>
    </div>";
    Response::new(200, "text/html", String::from(content))
}

// pub fn generate_shopping<'a>(shared_data: State) -> Response<'a> {

// }

// pub fn generate_valorant<'a>(shared_data: State) -> Response<'a> {

// }

pub fn generate_progress<'a>(shared_data: State) -> Response<'a> {
    let prog = shared_data.progress.lock().unwrap().clone();
    let mut content = String::from("
    <div class=\"cards\">
        <div class=\"card\">
            <div class=\"card-info\">
                <h2>
                    Current Progress
                    <button class=\"add\" id=\"add\">
                        <h2>+</h2>
                    </button>
                </h2>
                <ul class=\"scrolly\">");
    for todo in prog.in_progress {
        let mut item = format!("<li><button onclick=\"toggleToDo(this)\"");
        match todo {
            ToDo::Bug(done, message) => {
                match done {
                    true => {
                        item.push_str(" class=\"progress progress-done\">Done</button>");
                    },
                    false => {
                        item.push_str(" class=\"progress progress-wip\">WIP</button>");
                    }
                }
                item.push_str("<h3>Bug: ");
                item.push_str(&message[..]);
                item.push_str("</h3>");
            },
            ToDo::Feature(done, message) => {
                match done {
                    true => {
                        item.push_str(" class=\"progress progress-done\">Done</button>");
                    },
                    false => {
                        item.push_str(" class=\"progress progress-wip\">WIP</button>");
                    }
                }
                item.push_str("<h3>Feature:");
                item.push_str(&message[..]);
                item.push_str("</h3>");
            },
        }
        item.push_str("</li>");
        content.push_str(&item[..]);
    }

    content.push_str("</ul>
            </div>
        </div>
        <iframe onload=\"modalSetup(1)\" style=\"position: absolute;width:0;height:0;border:0;\"></iframe>
    </div>");
    Response::new(200, "text/html", String::from(content))
}

pub fn modal_calories<'a>(shared_data: State) -> Response<'a> {
    let cals = shared_data.calories.lock().unwrap().clone();
    let index = cals.last().unwrap().index + 1;
    let content = format!("
    <input type=\"number\" value=\"{}\" id=\"index\" hidden>
    <div class=\"modal-split\">
        <input type=\"number\" id=\"day_weight\" class=\"modal-input\" placeholder=\"Start Weight\" step=\"0.1\">
        <input type=\"number\" id=\"calories_burned\" class=\"modal-input\" placeholder=\"Calories Burned\" step=\"1\">
    </div>
    <div class=\"modal-sub-div\">
        <div class=\"modal-split\">
            <input type=\"text\" class=\"modal-input\" placeholder=\"Food Name\" id=\"FoodName\">
            <input type=\"number\" class=\"modal-input\" placeholder=\"Calories\" id=\"FoodCals\">
            <button class=\"modal-add-button\" onclick=\"addFood()\">+</button>
        </div>
        <div class=\"list-div\">
            <ul id=\"itemList\">
            </ul>
        </div>
    </div>", index);
    Response::new(200, "text/html", String::from(content))
}

pub fn modal_progress<'a>() -> Response<'a> {
    let content = "
    <div class=\"modal-split\">
        <h3 class=\"wip-text\">Bug</h3>
        <label class=\"switch\">
            <input type=\"checkbox\">
            <span class=\"slider round\" id=\"checkBug\" checked=false onclick=\"toggleSwitch(this)\"></span>
        </label>
        <h3 class=\"done-text\">Feature</h3>
        <input type=\"number\" id=\"description\" class=\"modal-input\" placeholder=\"Description\" step=\"1\">
    </div>";
    Response::new(200, "text/html", String::from(content))
}