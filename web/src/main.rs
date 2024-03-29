#![warn(clippy::all)]
use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use dotenv;

mod controllers;
mod store;
mod types;
mod config;
#[tokio::main]
async fn main() -> Result<(), handle_errors::CustomError>{

    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Cannot load Config file.");

    let log_filter = format!(
        "handle_errors={},rust_web_dev={},warp={}",
        config.log_level, config.log_level, config.log_level
    );

    // create store.
    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name
    ))
    .await
    .map_err(|e| handle_errors::CustomError::DatabaseQueryError(e))?;

    // migrate database
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await.map_err(|e| {
            handle_errors::CustomError::MigrationError(e) 
        })?;

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(controllers::question::get_questions);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(controllers::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(controllers::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(controllers::authentication::auth())
        .and(store_filter.clone())
        .and_then(controllers::question::delete_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(controllers::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(controllers::question::add_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(controllers::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(controllers::answer::add_answer);
   
    let get_question_answers = warp::get()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(controllers::answer::get_question_answers);
    

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(controllers::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(controllers::authentication::login);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(add_answer)
        .or(get_question_answers)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);


    tracing::info!("Q&A service build ID {}", env!("RUST_WEB_DEV_VERSION"));
    
    /// run server
    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;

    Ok(())
}
