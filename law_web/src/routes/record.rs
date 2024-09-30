#[allow(unused_imports)]
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use warp::http::StatusCode;
use law_rs::Laws;
use crate::types::record;
use crate::types::record::Records;
use tracing::{instrument, info};

#[derive(Deserialize)]
pub struct NoteUpdate {
    note: String,
}

pub async fn add_record(records: record::Records, law_record: record::LawRecord) -> Result<impl warp::Reply, warp::Rejection> {
    match records.add_records(law_record).await {
        Ok(record) => {
            info!("成功新增：{}",record.id);
            Ok(warp::reply::with_status("Records added", StatusCode::OK))
        },
        Err(e) => Err(warp::reject::custom(e))
    }
}

pub async fn update_note(id: String, records: Records , note: NoteUpdate) -> Result<impl warp::Reply, warp::Rejection> {
    let id = percent_decode_str(&id).decode_utf8_lossy();
    let res = match records.update_note(id.to_string(), note.note).await {
        Ok(record) => {
            info!("成功更新筆記：{}",record.id);
            record
        },
        Err(e) => return Err(warp::reject::custom(e))
    };
    Ok(warp::reply::json(&res))
}



pub async fn get_records_to_laws(user_name: String, directory: String,records: record::Records, laws: Laws) -> Result<impl warp::Reply, warp::Rejection> {
    let mut s = String::new();
    let user_name = percent_decode_str(&user_name).decode_utf8_lossy();
    let directory = percent_decode_str(&directory).decode_utf8_lossy();
    let res = records.get_by_user(&user_name.to_owned()).await?;
    let map = res.get_by_dir(directory.to_string())?;
    if map.vec_record.len() == 1 {
        s.push_str("<h2>尚無加入任何法條</h2>");
    } else {
        for (law, note) in map.get_laws(laws) {
            let block = law.law_block_delete(note);
            s.push_str(&block);
        }
    }
    // 製作一個新增的card
    s.push_str("<div class='law-card'>");
    s.push_str("<div class='card-law-up'>");
    s.push_str("<div class='card-law-content'>");
    s.push_str("<div class='card-law-chapter'>新增法條</div>");
    s.push_str("<div class='card-law-lines'>");
    s.push_str("<form class='card-add-form'><input list='law-name-data' id='card-form-chapter'></input><input id='card-form-num' placeholder='條目' required></input><button type='submit'>新增</button></form>");
    s.push_str("</div></div></div></div>");
    Ok(warp::reply::html(s))
}

pub async fn get_dir(user_name: String, records: record::Records) -> Result<impl warp::Reply, warp::Rejection> {
    let mut s = String::new();
    let user_name = percent_decode_str(&user_name).decode_utf8_lossy();
    println!("{user_name}");
    let records = records.get_by_user(&user_name.to_owned()).await?;
    let map = records.categorize_by_dir()?;
    map.keys()
        .map(|k| {format!("<li class='the-dir'><a>{}<a></li>", k)})
        .for_each(|str| {
            s.push_str(&str);
        });
    Ok(warp::reply::html(s))
}

pub async fn get_dir_for_pop(user_name: String, records: record::Records) -> Result<impl warp::Reply, warp::Rejection> {
    let mut s = String::new();
    let user_name = percent_decode_str(&user_name).decode_utf8_lossy();
    println!("{user_name}");
    let records = records.get_by_user(&user_name.to_owned()).await?;
    let map = records.categorize_by_dir()?;
    map.keys()
        .map(|k| {format!("<div class='option'><input type='checkbox' id='option-{}'>
                            <label for='option-{}'>{}</label></div>", k, k, k)})
        .for_each(|str| {
            println!("{str}");
            s.push_str(&str);
        });
    Ok(warp::reply::html(s))
}

pub async fn delete_dir_by_name(dir: String, records: Records)-> Result<impl warp::Reply, warp::Rejection> {
    let dir = percent_decode_str(&dir).decode_utf8_lossy();
    println!("刪除{dir}");
    let x = records.delete_by_dir(&dir.to_owned()).await?;
    Ok(warp::reply::with_status("Records delete", StatusCode::OK))
}




