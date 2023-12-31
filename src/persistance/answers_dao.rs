use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
    async fn update_answer(&self, updated_answer: Answer, answer_uuid: String) -> Result<AnswerDetail, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {

        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid)
            .map_err(|_| DBError::InvalidUUID(answer.question_uuid.clone()))?;

        let record = sqlx::query!(
            "INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *",
            uuid,
            answer.content
        ).fetch_one(&self.db).await.map_err(|e| {
            if e.as_database_error().map(|e| e.code().expect("Error reading &dyn DatabaseError code").to_string()) == Some(postgres_error_codes::FOREIGN_KEY_VIOLATION.to_string()) {
                DBError::InvalidUUID(answer.question_uuid.clone())
            } else {
                DBError::Other("Error creating answer".into())
            }
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid) 
                        .map_err(|_| DBError::InvalidUUID(answer_uuid.clone()))?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db).await.map_err(|_| DBError::Other("Error deleting answer".into()))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|_| DBError::InvalidUUID(question_uuid.clone()))?;

        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db).await.map_err(|_| DBError::Other("Error getting answers".into()))?;

        let answers = records.iter().map(|record| {
            AnswerDetail {
                answer_uuid: record.answer_uuid.to_string(),
                question_uuid: record.question_uuid.to_string(),
                content: record.content.clone(),
                created_at: record.created_at.to_string(),
            }
        }).collect();

        Ok(answers)
    }

    async fn update_answer(&self, updated_answer: Answer, answer_uuid: String) -> Result<AnswerDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid).map_err(|_| DBError::InvalidUUID(answer_uuid.clone()))?;

        let record = sqlx::query!(
            "UPDATE answers SET content = $1 WHERE answer_uuid = $2 RETURNING *",
            updated_answer.content,
            uuid
        ).fetch_one(&self.db).await.map_err(|_| DBError::Other("Error updating answer".into()))?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }
}
