use std::{fs, io::{BufReader, Read}};

use super::{super::state_data::progress::ToDo, response::Response, State};

//TODO there should be another {} in the html that gets replaced for the weight. The same should happen for the valorant rank.
pub fn generate_goals<'a>(shared_data: State) -> Response<'a> {
    let cals = shared_data.calories.lock().unwrap().clone();
    let recent_cals = cals.last();
    let games = shared_data.valorant.lock().unwrap().clone();
    let recent_game = games.last();
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
    let file = fs::File::open("www/pages/goals.html").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let day_weight = match recent_cals {
        Some(a) => a.day_weight,
        None => 0.0
    };
    let rank_rating_after = match recent_game {
        Some(a) => a.rank_rating_after,
        None => 0
    };
    let replacements = [100.0 * (targets.weight_start - day_weight) / (targets.weight_start - targets.weight_goal), 100.0 * (rank_rating_after as f32 / targets.rank_goal as f32), 100.0 / (count_done as f32 / prog_len as f32)];
    for replace in replacements.iter() {
        contents = contents.replacen("{}", format!("{:.2}", replace).as_str(), 1);
    }
    Response::new(200, "text/html", contents)
}

pub fn generate_calories<'a>() -> Response<'a> {
    let file = fs::File::open("www/pages/calories.html").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    Response::new(200, "text/html", String::from(contents))
}

// pub fn generate_shopping<'a>(shared_data: State) -> Response<'a> {

// }

// pub fn generate_valorant<'a>(shared_data: State) -> Response<'a> {

// }

pub fn generate_progress<'a>(shared_data: State) -> Response<'a> {
    let file = fs::File::open("www/pages/progress.html").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let prog = shared_data.progress.lock().unwrap().clone();
    let mut list_items = String::from("");
    for todo in prog.in_progress {
        let mut item = String::from("<li><button onclick=\"toggleToDo(this)\"");
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
        list_items.push_str(&item[..]);
    }

    contents = contents.replacen("{}", list_items.as_str(), 1);
    Response::new(200, "text/html", String::from(contents))
}

pub fn modal_calories<'a>(shared_data: State) -> Response<'a> {
    let file = fs::File::open("www/modals/calories.html").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let cals = shared_data.calories.lock().unwrap().clone();
    // let index = cals.last().unwrap().index + 1;
    let index = match cals.last() {
        Some(a) => a.index + 1,
        None => 0
    };
    contents = contents.replacen("{}", index.to_string().as_str(), 1);
    Response::new(200, "text/html", String::from(contents))
}

pub fn modal_progress<'a>() -> Response<'a> {
    let file = fs::File::open("www/modals/progress.html").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    Response::new(200, "text/html", String::from(contents))
}