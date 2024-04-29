use rocket::{request::FromParam, serde::{Deserialize, Serialize}};
use rocket::serde::json::{Json, Value, json};

use crate::sorting::{selection_sort, bubble_sort, insertion_sort, merge_sort, quick_sort, heap_sort};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum AlgorithmType {
    Selection,
    Bubble,
    Insertion,
    Merge,
    Quick,
    Heap,
}

impl<'r> FromParam<'r> for AlgorithmType {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "selection" => Ok(AlgorithmType::Selection),
            "bubble" => Ok(AlgorithmType::Bubble),
            "insertion" => Ok(AlgorithmType::Insertion),
            "merge" => Ok(AlgorithmType::Merge),
            "quick" => Ok(AlgorithmType::Quick),
            "heap" => Ok(AlgorithmType::Heap),
            _ => Err("Invalid algorithm type."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct SortRequest {
    numbers: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SortResult {
    pub result: Vec<i64>,
    pub array_accesses: i64,
    pub duration: std::time::Duration,
}

#[post("/<algorithm_type>", format = "json", data = "<sort_request>")]
fn get(sort_request: Json<SortRequest>, algorithm_type: Result<AlgorithmType, &str>) -> Json<Value> {
    match algorithm_type {
    Ok(algorithm) => {
        let sort_request_length = sort_request.numbers.len();
        let result = match algorithm { 
            AlgorithmType::Selection => selection_sort(sort_request.numbers.clone()),
            AlgorithmType::Bubble => bubble_sort(sort_request.numbers.clone()),
            AlgorithmType::Insertion => insertion_sort(sort_request.numbers.clone()),
            AlgorithmType::Merge => merge_sort(sort_request.numbers.clone()),
            AlgorithmType::Quick => quick_sort(sort_request.numbers.clone()),
            AlgorithmType::Heap => heap_sort(sort_request.numbers.clone()),
        };
        Json(json!({
            "status": "success",
            "result": result.result,
            "array_accesses": result.array_accesses,
            "duration": result.duration,
            "results_length": sort_request_length,
        }))
    },
    Err(err) => {
        Json(json!({
            "status": "error",
            "reason": err
        }))
    }
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/sort", routes![get]).register("/sort", catchers![not_found])
    })
}
