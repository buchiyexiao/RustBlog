extern crate regex;

use comrak::{ComrakOptions, markdown_to_html};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use self::regex::Regex;
use std::fs;

static HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <meta name="author" content="{{ author }}" />
    <title>{{ title }}</title>
</head>
<body>
    {{ content }}
</body>
</html>"#;

pub fn md_to_html(path: &Path){
    let mut md_file = match fs::File::open(&path){
        Ok(md_file) => md_file,
        Err(error) => panic!("{}",error.description()),
    };
    let mut md_content = String::new();
    match md_file.read_to_string(&mut md_content){
        Err(error) => panic!("{}",error.description()),
        Ok(_) => println!("ok"),
    }
    let re = Regex::new(r"^---([\s\S]*?)---([\s\S]*)").unwrap();
    let pipei = re.captures(md_content.as_str()).unwrap();
    let head = pipei.get(1).unwrap().as_str();
    //title author
    //println!("{:?}",pipei);
    //println!("{:?}",head);
    let re_2 = Regex::new(r"^\r\ntitle: ([\s\S]*?)\r\nauthor: ([\s\S]*?)\r\n").unwrap();
    let head_content = re_2.captures(head).unwrap();
    let title = head_content.get(1).unwrap().as_str();
    //println!("{:?}",title);
    let author = head_content.get(2).unwrap().as_str();
    //println!("{:?}",author);
    let md_str = pipei.get(2).unwrap().as_str();
    let md_str = markdown_to_html(md_str,&ComrakOptions::default());
    let re_author = Regex::new(r"\{\{\s*author\s*\}\}").unwrap();
    let re_title  = Regex::new(r"\{\{\s*title\s*\}\}").unwrap();
    let re_content = Regex::new(r"\{\{\s*content\s*\}\}").unwrap();
    
    let mut html_content = String::new();
    html_content = String::from(re_author.replace_all(HTML_TEMPLATE, author));
    html_content = String::from(re_title.replace_all(html_content.as_str(),title));
    html_content = String::from(re_content.replace_all(html_content.as_str(),md_str.as_str()));

    let html_t = Path::new("test.html");
    let mut file_html = match File::create(html_t) {
        Err(error) => panic!("{}",error.description()),
        Ok(file) => file,
    };
    match file_html.write_all(html_content.as_bytes()) {
        Err(error) => panic!("{}",error.description()),
        Ok(_) => println!("OKkkkkkkkk"),
    };
}

fn main(){
    let test_path = Path::new("test.md");
    md_to_html(test_path);
}