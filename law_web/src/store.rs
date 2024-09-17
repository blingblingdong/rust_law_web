use std::string::String;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use law_rs::*;


use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LawRecord {
    pub chapter: String,
    pub num: String,
}


impl LawRecord {
    pub fn new(chapter: String, num: String) -> Self {
        LawRecord {chapter, num}
    }
}

#[derive(Debug, Clone)]
pub struct Records {
    pub records: Arc<RwLock<Vec<LawRecord>>>
}

impl Records {

    pub fn new() -> Self {
        Records{records: Arc::new(RwLock::new(Vec::new()))}
    }

    pub async fn push_records(self, law_record: LawRecord) {
        self.records.write().await.push(law_record)
    }

    pub async fn show_records(self) -> String {
        let res = self.records.read().await;
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

    pub async fn get_laws(&self, laws:Laws) -> Laws {
        let res = self.records.read().await;
        let map = laws.categories(0);
        let mut new_laws = Laws::new();
        for r in res.iter() {
            let chapter = &r.chapter;
            let num = &r.num;
            if let Some(l) = map.get(chapter) {
                let x: Vec<law>= l.clone().lines.into_iter().filter(|x| x.num == *num).collect();
                if !x.is_empty() {
                    new_laws.lines.push(x.first().unwrap().clone());
                }
            }
        }
        new_laws
    }
}



#[derive(Clone)]
pub struct Store {
    pub laws: Arc<RwLock<Laws>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            laws: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> Laws {
        Laws::from_csv("../law_rs/new_all.csv".to_string())
    }
}