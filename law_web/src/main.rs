pub mod types;
pub mod routes;
#[allow(unused_imports)]
use handle_errors::return_error;
use warp::{http::Method, Filter};
use law_rs::Laws;
use tracing_subscriber::fmt::format::FmtSpan;
use crate::routes::file::{delete_file, get_content_markdown, update_content};
use crate::routes::law::get_on_law;
use crate::routes::record::{get_dir_for_pop, update_note};

#[tokio::main]
async fn main() {

    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_|
        "law_web=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db_url = "postgres://dbuser:12345678@localhost:5432/law";
    let store = Laws::from_pool(&db_url).await.unwrap();
    let store_filter = warp::any().map(move || store.clone());
    let record = types::record::Records::new(&db_url).await;
    let record_filter = warp::any().map(move || record.clone());

    let files = types::file::Files::new(&db_url).await;
    let files_filter = warp::any().map(move || files.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_dir = warp::get()
        .and(warp::path("all_dir"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(record_filter.clone())
        .and_then(routes::record::get_dir);

    let delete_dir_by_name = warp::delete()
        .and(warp::path("delete_dir_by_name"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(record_filter.clone())
        .and_then(routes::record::delete_dir_by_name);

    let get_dir_for_pop = warp::get()
        .and(warp::path("dir_for_pop"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(record_filter.clone())
        .and_then(routes::record::get_dir_for_pop);

    let get_table = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::law::get_table)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_questions request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )})
        );

    let get_all_lines = warp::get()
        .and(warp::path("questions"))
        .and(warp::path("all_lines"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::law::get_all_lines);

    let get_search_chapters = warp::get()
        .and(warp::path("search"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::law::get_search_chapters);

    let get_all_chapters = warp::get()
        .and(warp::path("all_chapters"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::law::get_all_chapters);



    let add_record = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(record_filter.clone())
        .and(warp::body::json())
        .and_then(routes::record::add_record);

    let get_records_to_laws = warp::get()
        .and(warp::path("records_to_laws"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(record_filter.clone())
        .and(store_filter.clone())
        .and_then(routes::record::get_records_to_laws);

    let get_lines_by_chapter = warp::get()
        .and(warp::path("lines_by_chapter"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::law::get_lines_by_chapter);

    let update_note = warp::put()
        .and(warp::path("update_note"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(record_filter.clone())
        .and(warp::body::json())
        .and_then(routes::record::update_note);

    let add_file = warp::post()
        .and(warp::path("file"))
        .and(warp::path::end())
        .and(files_filter.clone())
        .and(warp::body::json())
        .and_then(routes::file::add_file);

    let update_content = warp::put()
        .and(warp::path("file"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(files_filter.clone())
        .and(warp::body::json())
        .and_then(routes::file::update_content);

    let get_content_markdown = warp::get()
        .and(warp::path("file_markdown"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(files_filter.clone())
        .and_then(routes::file::get_content_markdown);

    let get_content_html = warp::get()
        .and(warp::path("file_html"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(files_filter.clone())
        .and_then(routes::file::get_content_html);

    let get_one_law = warp::get()
        .and(warp::path("one_law"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::law::get_on_law);

    let delete_file = warp::delete()
        .and(warp::path("file"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(files_filter.clone())
        .and_then(routes::file::delete_file);


    let static_files = warp::fs::dir("static");


    // 新增靜態文件路由

    let routes = get_all_lines
        .or(static_files)
        .or(add_record)
        .or(get_table)
        .or(get_dir)
        .or(get_one_law)
        .or(get_content_markdown)
        .or(get_search_chapters)
        .or(get_all_chapters)
        .or(get_records_to_laws)
        .or(get_lines_by_chapter)
        .or(get_dir_for_pop)
        .or(delete_dir_by_name)
        .or(get_content_html)
        .or(update_note)
        .or(add_file)
        .or(update_content)
        .or(delete_file)
        .with(warp::trace::request())// 提供靜態文件
        .with(cors)
        .recover(return_error);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 9090))
        .await;
}
