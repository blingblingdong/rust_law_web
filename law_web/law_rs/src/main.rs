use law_rs::Laws;

fn main() {
    let l = Laws::from_csv("all.csv".to_string());
    let x = l.categories(1);
    x.keys().for_each(|s| println!("{s}"));
    x.get("第一編總則").unwrap().view();


    let s = l.chapter_lines_in_html("民法".to_string(), "1".to_string(), "第一編總則".to_string());
    println!("{s}");

}
