use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{PgPool, Row};
use crate::types::record::{LawRecord, LawRecords, Records};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    pub id: String,
    pub content: String,
    pub css: String,
    pub user_name: String,
    pub directory: String,
}





#[derive(Clone)]
pub struct Files {
    pub connection: PgPool, //設定一個連接池
}


impl Files {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)// 最多可以同時連接5個
            .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("無法連接上池：{e}"),
        };
        Files {
            connection: db_pool
        }
    }

    pub async fn add_file(&self, file: File) -> Result<File, handle_errors::Error> {
        match sqlx::query(
            "INSERT INTO file (id, content, css, user_name, directory)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, content, css, user_name, directory"
        ).bind(file.id)
            .bind(file.content)
            .bind(file.css)
            .bind(file.user_name)
            .bind(file.directory)
            .map(|row: PgRow| File {
                id: row.get("id"),
                content: row.get("content"),
                css: row.get("css"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
            })
            .fetch_one(&self.connection)
            .await{
            Ok(record) => Ok(record),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }

    pub async fn get_file(&self, id: String) -> Result<File, handle_errors::Error>{
        match sqlx::query("SELECT id, content, css, user_name, directory
        FROM file
        WHERE id = $1;")
            .bind(id)
            .map(|row: PgRow| File {
                id: row.get("id"),
                content: row.get("content"),
                css: row.get("css"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
            })
            .fetch_one(&self.connection)
            .await{
            Ok(file) => Ok(file),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }

    pub async fn update_content(&self, id:String, content: String) -> Result<File, handle_errors::Error> {
        match sqlx::query(
            "UPDATE file
            SET content = $1
            WHERE id = $2
            RETURNING id, content, css, user_name, directory;"
        )
            .bind(content)
            .bind(id)
            .map(|row: PgRow| File {
                id: row.get("id"),
                content: row.get("content"),
                css: row.get("css"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
            })
            .fetch_one(&self.connection)
            .await{
            Ok(file) => Ok(file),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }



}