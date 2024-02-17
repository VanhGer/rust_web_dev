
use warp::{
    filters::cors::CorsForbidden, http::{Method, StatusCode}, reject::Reject, reply::with_status, Filter, Rejection, Reply
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, CustomError> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params.get("start").unwrap()
                .parse::<usize>()
                .map_err(CustomError::ParseError)?,
            
            end: params.get("end").unwrap()
                .parse::<usize>()
                .map_err(CustomError::ParseError)?,
        });
    }

    Err(CustomError::MissingParameters)
}


#[derive(Debug)]
enum CustomError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CustomError::ParseError(ref err) => {
                write!(f, "Cannot parse paramenter: {}", err)
            },
            CustomError::MissingParameters => write!(f, "Missing paramenter"),
        }
    }
}
impl Reject for CustomError {}

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("cant read questions.json")
    }   
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
} 

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);

async fn get_questions(
    params: HashMap<String, String>,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res : Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res : Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    
    if let Some(error) = r.find::<CustomError>() {
        Ok(warp::reply::with_status(
            error.to_string(), 
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(), 
            StatusCode::NOT_FOUND)
        )
    }
}

#[tokio::main] 
async fn main() {

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());


    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(
            &[Method::PUT, Method::DELETE, Method::GET, Method::POST]
        );

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);
    // let hello = warp::path("hello").map(|| format!("Hello, VanhG"));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}


