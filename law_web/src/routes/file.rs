#[allow(unused_imports)]
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use warp::http::StatusCode;
use tracing::{instrument, info};
use crate::types::file::{File, Files};
use crate::types::record;
use pulldown_cmark::{html, Options, Parser};
use crate::routes::record::NoteUpdate;
use crate::types::record::Records;

pub async fn add_file(files: Files, file: File) -> Result<impl warp::Reply, warp::Rejection> {
    match files.add_file(file).await {
        Ok(file) => {
            info!("成功新增：{}", file.id);
            Ok(warp::reply::with_status("file added", StatusCode::OK))
        },
        Err(e) => Err(warp::reject::custom(e))
    }
}

pub async fn get_content_markdown(id: String, files: Files) -> Result<impl warp::Reply, warp::Rejection> {
    let id = percent_decode_str(&id).decode_utf8_lossy();
    match files.get_file(id.to_string()).await {
        Ok(file) => {
            info!("成功獲取：{}", file.id);
            Ok(warp::reply::html(file.content))
        },
        Err(e) => Err(warp::reject::custom(e))
    }
}

pub async fn get_content_html(id: String, files: Files) -> Result<impl warp::Reply, warp::Rejection> {
    let id = percent_decode_str(&id).decode_utf8_lossy();
    match files.get_file(id.to_string()).await {
        Ok(file) => {
            info!("成功獲取：{}", file.id);
            let parser = Parser::new_ext(&file.content, Options::all());
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            Ok(warp::reply::html(html_output))
        },
        Err(e) => Err(warp::reject::custom(e))
    }
}

#[derive(Deserialize)]
pub struct UpdateContent {
    content: String,
}

pub async fn update_content(id: String, files: Files , contnet: UpdateContent) -> Result<impl warp::Reply, warp::Rejection> {
    let id = percent_decode_str(&id).decode_utf8_lossy();
    let res = match files.update_content(id.to_string(), contnet.content).await {
        Ok(file) => {
            info!("成功更新筆記：{}",file.id);
            file
        },
        Err(e) => return Err(warp::reject::custom(e))
    };
    Ok(warp::reply::json(&res))
}