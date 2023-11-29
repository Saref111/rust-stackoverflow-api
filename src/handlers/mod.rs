use rocket::{serde::json::Json, State, fairing::Fairing};

use crate::{models::*, persistance::{questions_dao::QuestionsDao, answers_dao::AnswersDao}};

mod handlers_inner;

use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    let question_detail = handlers_inner::create_question(question.0, questions_dao).await
                                        .map_err(|e| Into::<APIError>::into(e))?;
    Ok(Json(question_detail))
}   

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<Vec<QuestionDetail>>, APIError> { 
    let questions = handlers_inner::read_questions(questions_dao).await
                                        .map_err(|e| Into::<APIError>::into(e))?;
    Ok(Json(questions))
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<(), APIError> {
    handlers_inner::delete_question(question_uuid.0, questions_dao).await.map_err(|e| Into::<APIError>::into(e))?;
    Ok(())
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<AnswerDetail>, APIError> {
    let answer_detail = handlers_inner::create_answer(answer.0, answers_dao).await
                                                            .map_err(|e| Into::<APIError>::into(e))?;
    Ok(Json(answer_detail))
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>, 
) -> Result<Json<Vec<AnswerDetail>>, APIError>  {
    let answers = handlers_inner::read_answers(question_uuid.0, answers_dao).await
                                                            .map_err(|e| Into::<APIError>::into(e))?;
    Ok(Json (answers))
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>, 
) ->  Result<(), APIError>  {
    handlers_inner::delete_answer(answer_uuid.0, answers_dao).await
                    .map_err(|e| Into::<APIError>::into(e))?;
    Ok(())
}
