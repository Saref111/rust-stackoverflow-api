use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
    async fn update_question(&self, updated_question: Question, question_uuid: String) -> Result<QuestionDetail, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            "INSERT INTO questions ( title, description )
            VALUES ( $1, $2 )
            RETURNING *",
            question.title,
            question.description
        ).fetch_one(&self.db).await.map_err(|_| DBError::Other("Error creating question".into()))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|_| DBError::InvalidUUID(question_uuid.clone()))?;

        sqlx::query!("DELETE FROM questions WHERE question_uuid = $1", uuid)
            .execute(&self.db).await.map_err(|_| DBError::Other("Error deleting question".into()))?;
        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!("SELECT * FROM questions")
            .fetch_all(&self.db).await.map_err(|_| DBError::Other("Error getting questions".into()))?;

        let questions = records.iter().map(|record| {
            QuestionDetail {
                question_uuid: record.question_uuid.to_string(),
                title: record.title.clone(),
                description: record.description.clone(),
                created_at: record.created_at.to_string(),
            }
        }).collect();

        Ok(questions)
    }

    async fn update_question(&self, updated_question: Question, question_uuid: String) -> Result<QuestionDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|_| DBError::InvalidUUID(question_uuid.clone()))?;

        let record = sqlx::query!(
            "UPDATE questions SET title = $1, description = $2 WHERE question_uuid = $3 RETURNING *",
            updated_question.title,
            updated_question.description,
            uuid
        ).fetch_one(&self.db).await.map_err(|_| DBError::Other("Error updating question".into()))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }
}
