pub mod store;
use crate::store::{LawRecord, Records, Store};
use percent_encoding::percent_decode_str;
use warp::http::StatusCode;

pub async fn get_table(cate: String, num: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let cate = percent_decode_str(&cate).decode_utf8_lossy();
    let num = percent_decode_str(&num).decode_utf8_lossy();
    let res= store.laws.read().await;
    let n = res.filter_by_cate2(cate.to_string(), num.to_string());
    Ok(warp::reply::html(n))
}

pub async fn add_record(records: Records, law_record: LawRecord) -> Result<impl warp::Reply, warp::Rejection> {
    records.push_records(law_record).await;
    Ok(warp::reply::with_status("Records added", StatusCode::OK))
}

pub async fn get_records(records: Records) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::html(records.show_records().await))
}

pub async fn get_all_lines(cate: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let cate = percent_decode_str(&cate).decode_utf8_lossy();
    let res= store.laws.read().await;
    let n = res.all_in_html(cate.to_string());
    Ok(warp::reply::html(n))
}

pub async fn get_all_chapters(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res= store.laws.read().await;
    let mut s = String::new();
    for key in res.categories(0).keys() {
        let format_key = format!("<li class='chapter-li'><a>{}</a></li>", key);
        s.push_str(&format_key);
    }
    Ok(warp::reply::html(s))
}

pub async fn get_search_chapters(cate: String, store: Store)-> Result<impl warp::Reply, warp::Rejection> {
    let cate = percent_decode_str(&cate).decode_utf8_lossy();
    let res= store.laws.read().await;
    let n = res.search_in_html_chapter(cate.to_string());
    Ok(warp::reply::html(n))
}

pub async fn get_records_to_laws(records: Records, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store.laws.read().await;
    let mut s = String::new();
    let laws = records.get_laws(res.to_owned());
    for l in laws.await.lines {
        s.push_str(&l.law_block());
    }
    Ok(warp::reply::html(s))
}

pub async fn get_lines_by_chapter(chapter1: String, num: String, chapter2: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let chapter1 = percent_decode_str(&chapter1).decode_utf8_lossy();
    let num = percent_decode_str(&num).decode_utf8_lossy();
    let chapter2 = percent_decode_str(&chapter2).decode_utf8_lossy();
    println!("{chapter1}{num}{chapter2}");
    let res = store.laws.read().await;
    let s = res.chapter_lines_in_html(chapter1.into_owned(), num.into_owned(), chapter2.into_owned());
    Ok(warp::reply::html(s))
}


use handle_errors::return_error;
use warp::{http::Method, Filter};



#[tokio::main]
async fn main() {

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let record = store::Records::new();
    let record_filter = warp::any().map(move || record.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_table = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_table);

    let get_all_lines = warp::get()
        .and(warp::path("questions"))
        .and(warp::path("all_lines"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_all_lines);

    let get_search_chapters = warp::get()
        .and(warp::path("search"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_search_chapters);

    let get_all_chapters = warp::get()
        .and(warp::path("all_chapters"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_all_chapters);

    let get_records = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(record_filter.clone())
        .and_then(get_records);

    let add_record = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(record_filter.clone())
        .and(warp::body::json())
        .and_then(add_record);

    let get_records_to_laws = warp::get()
        .and(warp::path("records_to_laws"))
        .and(warp::path::end())
        .and(record_filter.clone())
        .and(store_filter.clone())
        .and_then(get_records_to_laws);

    let get_lines_by_chapter = warp::get()
        .and(warp::path("lines_by_chapter"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_lines_by_chapter);

    let static_files = warp::fs::dir("static");


    // 新增靜態文件路由

    let routes = get_all_lines
        .or(static_files)
        .or(add_record)
        .or(get_records)
        .or(get_table)
        .or(get_search_chapters)
        .or(get_all_chapters)
        .or(get_records_to_laws)
        .or(get_lines_by_chapter)// 提供靜態文件
        .with(cors)
        .recover(return_error);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 9090))
        .await;
}
