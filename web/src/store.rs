use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use handle_errors::CustomError;

use crate::types::{
    account::{Account, AccountId},
    answer::{Answer, AnswerId, NewAnswer},
    question::{NewQuestion, Question, QuestionId},
};
/// The Store object represents the connection and interaction 
/// with a PostgreSQL database.
#[derive(Debug, Clone)]
pub struct Store {
    /// Connection to the PostgreSQL database.
    pub connection: PgPool, 
}

impl Store {
    /// This function creates a new Store object from the given database URL.
    /// # Example usage
    /// ```rust
    /// let store = store::Store::new(postgres://username@password/host:port/databse)
    ///             .await?;
    /// ```
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        tracing::warn!("{}", db_url);
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        Ok(Store {
            connection: db_pool,
        })
    }
    
    /// This function retrieves a list of questions from the database with
    /// optional limits and offsets.
    pub async fn get_questions(
        self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, CustomError> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError(e))
            }
        }
    }

    /// This function checks if a user is the owner of a question.
    pub async fn is_question_owner(
        &self,
        question_id: i32,
        account_id: &AccountId,
    ) -> Result<bool, CustomError> {
        match sqlx::query(
            "SELECT * from questions where id = $1 and account_id = $2",
        )
        .bind(question_id)
        .bind(account_id.0)
        .fetch_optional(&self.connection)
        .await
        {
            Ok(question) => Ok(question.is_some()),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError(e))
            }
        }
    }

    /// This function adds a new question to the database.
    pub async fn add_question(
        self,
        new_question: NewQuestion,
        account_id: AccountId,
    ) -> Result<Question, CustomError> {
        match sqlx::query("INSERT INTO questions (title, content, tags, account_id) VALUES ($1, $2, $3, $4) RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .bind(account_id.0)
            .map(|row: PgRow| Question {
			    id: QuestionId(row.get("id")),
                title: row.get("title"),
			    content: row.get("content"),
                tags: row.get("tags"),
		    })
            .fetch_one(&self.connection)
            .await {
                Ok(question) => Ok(question),
                Err(error) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", error);
                    Err(CustomError::DatabaseQueryError(error))
                },
            }
    }

    /// This function updates an existing question in the database.
    pub async fn update_question(
        self,
        question: Question,
        id: i32,
        account_id: AccountId,
    ) -> Result<Question, CustomError> {
        match sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3
        WHERE id = $4 AND account_id = $5
        RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(id)
        .bind(account_id.0)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(CustomError::DatabaseQueryError(error))
            }
        }
    }

    /// This function deletes a question from the database.
    pub async fn delete_question(
        self,
        id: i32,
        account_id: AccountId,
    ) -> Result<bool, CustomError> {
        
        match self.clone().delete_all_question_answers(id).await {
            Ok(_) => {
                match sqlx::query("DELETE FROM questions WHERE id = $1 AND account_id = $2",)
                    .bind(id)
                    .bind(account_id.0)
                    .execute(&self.connection)
                    .await
                {
                    Ok(_) => Ok(true),
                    Err(e) => {
                        tracing::event!(tracing::Level::ERROR, "{:?}", e);
                        Err(CustomError::DatabaseQueryError(e))
                    }
                }
            },
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)   
            }
        }
    }

    /// This function deletes all answers associated with a specific 
    /// question from the database.
    async fn delete_all_question_answers(
        self,
        id: i32,
    ) -> Result<bool, CustomError> {
        match sqlx::query(
            "DELETE FROM answers WHERE corresponding_question = $1",
        )
        .bind(id)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError(e))
            }
        }
    }

    /// This function adds a new answer to the database.
    pub async fn add_answer(
        self,
        new_answer: NewAnswer,
        account_id: AccountId,
    ) -> Result<Answer, CustomError> {

        match sqlx::query(
            "INSERT INTO answers (content, corresponding_question, account_id) VALUES ($1, $2, $3)
            RETURNING id, content, corresponding_question",
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .bind(account_id.0)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("corresponding_question")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(answer) => Ok(answer),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = error.as_database_error().unwrap().message(),
                    constraint = error.as_database_error().unwrap().constraint().unwrap()
                );

                Err(CustomError::DatabaseQueryError(error))
            }
        }
    }

    /// This function retrieves a list of answers for a specific question 
    /// from the database with optional limits and offsets.
    pub async fn get_question_answers(
        self,
        limit: Option<u32>,
        offset: u32,
        question_id: i32,
    ) -> Result<Vec<Answer>, CustomError> {
        match sqlx::query("SELECT * from answers where corresponding_question = $1 LIMIT $2 OFFSET $3")
            .bind(question_id)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(question_id),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(answers) => Ok(answers),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError(e))
            }
        }
    }

    /// This function adds a new account to the database.
    pub async fn add_account(
        self,
        account: Account,
    ) -> Result<bool, CustomError> {
        match sqlx::query(
            "INSERT INTO accounts (email, password) VALUES ($1, $2)",
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message =
                        error.as_database_error().unwrap().message(),
                    constraint = error
                        .as_database_error()
                        .unwrap()
                        .constraint()
                        .unwrap()
                );
                Err(CustomError::DatabaseQueryError(error))
            }
        }
    }

    /// This function retrieves an account from the database by its email address.
    pub async fn get_account(
        self,
        email: String,
    ) -> Result<Account, CustomError> {
        match sqlx::query("SELECT * from accounts where email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(CustomError::DatabaseQueryError(error))
            }
        } 
    }  

}
