use lazy_static::lazy_static;
use std::sync::Mutex;

use rocket::{serde::json::Json};

use crate::models::*;

struct TempDB {
    questions: Vec<QuestionDetail>,
    answers: Vec<AnswerDetail>,
}

lazy_static! {
    static ref DB: Mutex<TempDB> = Mutex::new(TempDB {
        questions: Vec::new(),
        answers: Vec::new(),
    });
}
// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    let mut db = DB.lock().unwrap();
    let new_question = QuestionDetail {
        question_uuid: "123".to_string(),
        title: question.title.clone(),
        description: question.description.clone(),
        created_at: "2021-08-25T00:00:00Z".to_string(),
    };
    db.questions.push(QuestionDetail {
        question_uuid: "123".to_string(),
        title: question.title.clone(),
        description: question.description.clone(),
        created_at: "2021-08-25T00:00:00Z".to_string(),
    });
    Json(new_question)
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    let db = DB.lock().unwrap();
    Json(db.questions.clone())
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
    let mut db = DB.lock().unwrap();
    db.questions.retain(|q| q.question_uuid != question_uuid.question_uuid);
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub fn create_answer(
    answer: Json<Answer>,
) -> Json<Vec<AnswerDetail>> {
    let mut db = DB.lock().unwrap();
    let new_answer = AnswerDetail {
        answer_uuid: "123".to_string(),
        question_uuid: answer.question_uuid.clone(),
        content: answer.content.clone(),
        created_at: "2021-08-25T00:00:00Z".to_string(),
    };
    db.answers.push(new_answer.clone());
    Json(vec![new_answer])
}

#[get("/answers", data = "<question_uuid>")]
pub fn read_answers(
    question_uuid: Json<QuestionId>
) -> Json<Vec<AnswerDetail>> {
    let db = DB.lock().unwrap();
    let answers = db.answers.clone();
    Json(answers.into_iter().filter(|a| a.question_uuid == question_uuid.question_uuid).collect())
}

#[delete("/answer", data = "<answer_uuid>")]
pub fn delete_answer(
    answer_uuid: Json<AnswerId>
) {
    let mut db = DB.lock().unwrap();
    db.answers.retain(|a| a.answer_uuid != answer_uuid.answer_uuid);
}
