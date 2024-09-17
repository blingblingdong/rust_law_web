use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use csv::{Reader, Writer};
use anyhow::Result;
use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;

#[derive(Clone)]
pub struct Laws {
    pub lines: Vec<crate::law>
}

impl crate::Laws {
    pub fn new() -> Self {
        crate::Laws {
            lines: Vec::new()
        }
    }

    pub fn from_csv(path:String) -> Self{
        let mut vec = Vec::new();
        let file = File::open(path).expect("打開不能");
        let mut rdr = Reader::from_reader(file);

        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: crate::law = result.unwrap();
            vec.push(record);
        }

        crate::Laws {
            lines:vec
        }

    }

    pub fn filter_by_cate(&self, cate: String, num: String) -> String {
        let map = self.categories(0);
        if let Some(laws) = map.get(&cate) {
            let x: Vec<law>= laws.clone().lines.into_iter().filter(|x| x.num == num).collect();
            if !x.is_empty() {
                let name = &x.get(0).unwrap().num;
                let chapter:Vec<&str> = x.get(0).unwrap().chapter.split("/").collect();
                let l: String = x.get(0).unwrap().line.iter().enumerate()
                    .map(|(i, s)| format!("{}:{s}\n",i+1)).collect();
                format!("{}：{name}\n{l}", chapter.first().unwrap())
            } else {
                format!("查無項目，考慮下列類似選項")
            }
        } else {
            format!("沒有這個項目，考慮下列類似選項")
        }
    }

    pub fn filter_by_cate2(&self, cate: String, num: String) -> String {
        let map = self.categories(0);
        if let Some(laws) = map.get(&cate) {
            let x: Vec<law>= laws.clone().lines.into_iter().filter(|x| x.num == num).collect();
            if !x.is_empty() {
                let name = &x.get(0).unwrap().num;
                let chapter:Vec<&str> = x.get(0).unwrap().chapter.split("/").collect();
                let l: String = x.get(0).unwrap().line.iter().enumerate()
                    .map(|(i, s)| format!("<h3>{}:{s}</h3><br>",i+1)).collect();
                format!("<h2>{}第{name}條：</h2><br>{l}", chapter.first().unwrap())
            } else {
                format!("<h2>查無項目，考慮下列類似選項</h2>")
            }
        } else {
            format!("<h2>沒有這個項目，考慮下列類似選項</h2>")
        }
    }

    pub fn categories(&self, index:usize) -> IndexMap<String, crate::Laws> {
        let vec: Vec<String> = self.lines
            .iter()
            .map(|x| {
                let name_vec = x.chapter.split('/').collect::<Vec<&str>>();
                name_vec.get(index).unwrap().to_string()
            })
            .collect();

        let mut map = IndexMap::new();
        for (law, name) in self.lines.iter().zip(vec) {
            map.entry(name).or_insert_with(crate::Laws::new).lines.push(law.clone());
        }
        map


    }

    pub fn search_in_html_chapter(&self, chapter: String) -> String {
        let binding = self.categories(0);
        let l = binding.get(&chapter).unwrap();
        let chapter_num = self.lines.first().unwrap().chapter.split("/").count();
        let mut html_text = String::new();
        l.print_all_chapter_html(1, 3, &mut html_text);
        html_text
    }

    pub fn print_all_chapter_html(&self, level: usize, max_level: usize, html_text: &mut String) {
        let map = self.categories(level);

        for (s, l) in &map {
            // 只在 level 為 1 的時候加入外層 <ul>
            if level == 1 {
                html_text.push_str(&format!("<ul class='chapter-ul-{}'>", level));
            }

            // <li> 標籤
            html_text.push_str(&format!("<li class='chapter-li-{}'><a>{}</a>", level, s));

            // 只有在還有子項時才遞歸繼續產生 <ul> 結構
            if level < max_level - 1 {
                html_text.push_str(&format!("<ul class='chapter-ul-{}'>", level + 1));
                l.print_all_chapter_html(level + 1, max_level, html_text);
                html_text.push_str("</ul>");
            }

            html_text.push_str("</li>"); // 關閉 <li>

            // 在 level == 1 時關閉外層 <ul>
            if level == 1 {
                html_text.push_str("</ul>");
            }
        }
    }

    pub fn chapter_lines_in_html(&self, chapter1:String, num: String, chapter2: String) ->  String{
        let binding = self.categories(0);
        println!("{chapter1}");
        let l = binding.get(&chapter1).expect("找無");
        let mut html_text = String::new();
        let binding2 = l.categories(num.parse().unwrap());
        let l2 = binding2.get(&chapter2).unwrap();
        l2.print_all_html(num.parse().unwrap(), 3, &mut html_text);
        html_text
    }


    pub fn all_in_html(&self, chapter:String) -> String {
        let binding = self.categories(0);
        let l = binding.get(&chapter).unwrap();
        let chapter_num = self.lines.first().unwrap().chapter.split("/").count();
        let mut html_text = String::new();
        l.print_all_html(0, 3, &mut html_text);
        html_text
    }

    pub fn print_all_html(&self, level: usize, max_level: usize, html_text: &mut String) {
        if level == max_level {
            for l in &self.lines {
                html_text.push_str(&l.law_block());
            }
        }else {
            let map = self.categories(level);
            for (s, l) in map {
                let s = format!("<div 'in-chapter'><h2>{}</h2></div>", s);
                html_text.push_str(&s);
                l.print_all_html(level + 1, max_level, html_text);
            }
        }
    }

    pub fn table_of_cate(&self) {
        let chapter_num = self.lines.first().unwrap().chapter.split("/").count();
        self.print_categories(0, chapter_num);
    }

    fn print_categories(&self, level: usize, max_level: usize) {
        if level == max_level {
            let chapter = self.lines.first().unwrap().chapter.split("/").last().unwrap();
            for l in &self.lines {
                println!("{}", l.num);
            }
        }else {
            let map = self.categories(level);
            for (s, l) in map {
                print!("{}", "--".repeat(level));
                println!("{s}");
                l.print_categories(level + 1, max_level);
            }
        }
    }


    pub fn view(&self) {
        println!("本章節總共有：{}", self.lines.len());
        println!("first element is:{:?}", self.lines.first());
    }

}

pub fn group(mut map:IndexMap<String, Laws>) -> Laws {
    let key = map.iter().last().unwrap();
    let mut num : usize;
    for (i, law) in key.1.lines.first().unwrap().chapter.split("/").enumerate() {
        if key.0 == law {
            num = i;
            println!("group by {}{}", num, law)
        }
    }
    let mut l = Laws::new();
    for x in map.into_iter(){
        l.lines.extend(x.1.lines);
    }

    l
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct law {
    pub num: String,
    #[serde(deserialize_with = "deserialize_line")]
    pub line: Vec<String>,
    pub href: String,
    pub chapter: String,
}

fn deserialize_line<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split('/').map(|s| s.to_string()).collect())
}


impl crate::law {
    pub fn new(num: String, line:Vec<String> , href: String, chapter: String) -> Self {
        crate::law {
            num,
            line,
            href,
            chapter
        }
    }

    pub fn format_chapter(&self) -> String {
        let chapter: Vec<&str> = self.chapter.split("/").collect();
        let c = chapter.first().unwrap();
        format!("{}第{}條", c, self.num)
    }

    pub fn law_block(&self) -> String {
        let mut s = String::new();
        s.push_str("<div class='law-content'>");
        let chapter = format!("<div class='law-chapter'>{}</div>", self.format_chapter());
        s.push_str(&chapter);
        let line: String = self.line.iter().enumerate()
            .map(|(i, s)| format!("<div 'law-line'>{}:{s}</div>",i+1)).collect();
        let lines = format!("<div 'law-lines'>{}</div>", line);
        s.push_str(&lines);
        s.push_str("</div>");
        s
    }

    pub fn update_chapter(&mut self, chapter: String) {
        self.chapter = chapter;
    }
}


pub fn write_law(path: String, vec: Vec<crate::law>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(&["num", "line", "href", "chapter"])?;

    for law in vec {
        wtr.write_record(&[law.num, law.line.join("/"), law.href, law.chapter])?;
    }
    println!("寫入成功");
    wtr.flush()?;
    Ok(())
}

