#[allow(unused_imports)]
use std::string::String;
use law_rs::*;
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::{Row};
use indexmap::{IndexMap};

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LawRecord {
    pub id: String,// primary key(user+directory+chapter+num)
    pub chapter: String,
    pub num: String,
    pub user_name: String,
    pub directory: String,
    pub note: String,
}




impl LawRecord {
    pub fn new(chapter: String, num: String, user_name: String, directory: String) -> Self {
        let id = format!("{}-{}-{}-{}", user_name, directory, chapter, num);
        LawRecord {id, chapter, num, user_name, directory, note: "新增筆記".to_string()}
    }
}


#[derive(Debug, Clone)]
pub struct LawRecords {
    pub vec_record: Vec<LawRecord>
}

impl LawRecords {

    pub fn categorize_by_dir(&self)  -> Result<IndexMap<String, Vec<LawRecord>>, handle_errors::Error> {
        let mut map = IndexMap::new();
        for r in self.vec_record.iter() {
            map.entry(r.directory.clone()).or_insert_with(Vec::new).push(r.clone());
        }
        Ok(map)
    }

    pub fn get_by_dir(&self, dir: String) -> Result<LawRecords, handle_errors::Error> {
        let map = self.categorize_by_dir()?;
        let records = map.get(&dir).unwrap();
        Ok(LawRecords { vec_record: records.clone() })
    }

    pub async fn show_records(&self) -> String {
        let res = self.vec_record.clone();
        let mut table = String::new();
        table.push_str("<h2>查詢記錄</h2>");
        table.push_str("<ul>");
        for law in res.iter() {
            table.push_str(&format!(
                "<li class='record-button'>{}-{}</li>",
                law.chapter, law.num
            ));
        }
        table.push_str("</ul>");
        table
    }

    pub  fn get_laws(&self, laws:Laws) -> Vec<(law, String)> {
        let res = self.vec_record.clone();
        let res: Vec<LawRecord> = res.iter().filter(|&x| x.chapter != "創建").map(|x| x.clone()).collect();
        let map = laws.categories(0);
        let mut new_vec = Vec::new();
        for r in res.iter() {
            let chapter = &r.chapter;
            let num = &r.num;
            if let Some(l) = map.get(chapter) {
                if let Some(law) = l.clone().lines.into_iter().find(|law| law.num == *num) {
                    new_vec.push((law,r.note.clone()));
                }
            }
        }
        new_vec
    }
}

#[derive(Clone)]
pub struct Records {
    pub connection: PgPool, //設定一個連接池
}

impl Records {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)// 最多可以同時連接5個
            .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("無法連接上池：{e}"),
        };
        Records {
            connection: db_pool
        }
    }

    pub async fn get_all_records(&self) -> Result<LawRecords, handle_errors::Error>{
        match sqlx::query("SELECT * from records")
            .map(|row: PgRow| LawRecord {
                id: row.get("id"),
                chapter: row.get("chapter"),
                num: row.get("num"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
                note: row.get("note"),
            })
            .fetch_all(&self.connection)
            .await{
            Ok(records) => Ok(LawRecords{vec_record: records}),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }

    pub async fn get_by_user(&self, user_name: &str) -> Result<LawRecords, handle_errors::Error>{
        match sqlx::query("SELECT * from records WHERE user_name = $1")
            .bind(user_name)
            .map(|row: PgRow| LawRecord {
                id: row.get("id"),
                chapter: row.get("chapter"),
                num: row.get("num"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
                note: row.get("note"),
            })
            .fetch_all(&self.connection)
            .await{
            Ok(records) => Ok(LawRecords{vec_record: records}),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }

    pub async fn add_records(&self, record:LawRecord) -> Result<LawRecord, handle_errors::Error> {
        match sqlx::query(
            "INSERT INTO records (id, chapter, num, user_name, directory, note)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, chapter, num, user_name, directory, note"
        ).bind(record.id)
            .bind(record.chapter)
            .bind(record.num)
            .bind(record.user_name)
            .bind(record.directory)
            .bind(record.note)
            .map(|row: PgRow| LawRecord {
                id: row.get("id"),
                chapter: row.get("chapter"),
                num: row.get("num"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
                note: row.get("note"),
            })
            .fetch_one(&self.connection)
            .await{
            Ok(record) => Ok(record),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }

    pub async fn update_note(&self, id:String, note: String) -> Result<LawRecord, handle_errors::Error> {
        match sqlx::query(
            "UPDATE records
            SET note = $1
            WHERE id = $2
            RETURNING id, chapter, num, user_name, directory, note;"
        )
            .bind(note)
            .bind(id)
            .map(|row: PgRow| LawRecord {
                id: row.get("id"),
                chapter: row.get("chapter"),
                num: row.get("num"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
                note: row.get("note"),
            })
            .fetch_one(&self.connection)
            .await{
            Ok(record) => Ok(record),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }

    pub async fn delete_by_dir(&self, dir: &str) -> Result<LawRecords, handle_errors::Error> {
        match sqlx::query(
            "DELETE FROM records
            Where directory = $1;"
        )
            .bind(dir)
            .map(|row: PgRow| LawRecord {
                id: row.get("id"),
                chapter: row.get("chapter"),
                num: row.get("num"),
                user_name: row.get("user_name"),
                directory: row.get("directory"),
                note: row.get("note"),
            })
            .fetch_all(&self.connection)
            .await {
            Ok(records) => Ok(LawRecords{vec_record: records}),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e))
        }
    }

}

