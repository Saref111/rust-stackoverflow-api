#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use dotenvy::dotenv;

use persistance::{questions_dao::{QuestionsDaoImpl, QuestionsDao}, answers_dao::{AnswersDaoImpl, AnswersDao}};
use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;
mod persistance;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao =  QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool.clone());

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                update_question,
                create_answer,
                read_answers,
                delete_answer,
                update_answer,
            ],
        )
        .attach(CORS)
        .manage(Box::new(questions_dao) as Box<dyn QuestionsDao + Sync + Send>)
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Sync + Send>)
}
