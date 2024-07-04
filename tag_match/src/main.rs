use aho_corasick::{AhoCorasick, PatternID};
use rayon::prelude::*;
use std::collections::HashSet;
use std::env;
use std::io::Read;
fn match_tag(tag: &str, text: &str) -> f32 {
    let confidence: f32;
    let binding = std::fs::read_to_string(format!("tags/{}.tag", tag)).unwrap();
    let patterns = binding
        .split("\n")
        .filter(|t| !t.is_empty())
        .map(|t| {
            let mut s = t.to_string();
            s.push(' ');
            return s;
        })
        .collect::<Vec<String>>()
        .to_vec();
    let num_patterns = patterns.len() as f32;
    //println!("{}", text);
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(patterns.clone())
        .unwrap();

    let mut matches = vec![];
    for mat in ac.find_iter(text) {
        //println!("{}", mat.pattern().as_u32());
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }

    /*  println!(
        "{} {} {} {} {:#?}",
        format!("tags/{}.tag", tag),
        matches.len(),
        num_patterns,
        text.len(),
        patterns.clone()
    );*/
    confidence = matches.len() as f32;

    confidence
}
fn get_names(folder: &str) -> Vec<String> {
    let paths = std::fs::read_dir(folder).unwrap();
    let names = paths
        .filter_map(|entry| {
            let fullpath = match entry {
                Ok(entry) => entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                Err(_) => "error".to_string(),
            };
            let fullpath = fullpath.split(".").next().expect("bad_file").to_string();
            Some(fullpath)
        })
        .collect::<Vec<String>>();

    names
}

fn get_paths(folder: &str) -> Vec<String> {
    let paths = std::fs::read_dir(folder).unwrap();
    let names = paths
        .filter_map(|entry| {
            let fullpath = match entry {
                Ok(entry) => entry.path().to_str().unwrap().to_string(),
                Err(_) => "error".to_string(),
            };

            Some(fullpath)
        })
        .collect::<Vec<String>>();

    names
}

fn match_all(buffer: &str) -> Vec<String> {
    let mut matched_tags: Vec<String> = vec![];

    get_names("./tags").iter().for_each(|tag| {
        let conf = match_tag(tag, &buffer);
        if conf > 0.0 {
            matched_tags.push(tag.to_string());
        }
    });


    matched_tags
}

fn match_tag_folder(folder: &str) {
    get_paths(folder).par_iter().for_each(|path| {
        if path != "error" {
            let buffer = std::fs::read_to_string(path).unwrap().to_string();
            let matched_tags: Vec<String> = match_all(&buffer);

            matched_tags.iter().for_each(|x| println!("{}", x));
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(pos) = args.iter().position(|x| x == "-f") {
        if args.len() > pos + 1 {
            match_tag_folder(&args[pos + 1]);
        }
    } else if let Some(text) = args.get(1) {
        let matched_tags = match_all(&text);
        matched_tags.iter().for_each(|x| println!("{}", x));
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("error");
        let matched_tags = match_all(&buffer);
        matched_tags.iter().for_each(|x| println!("{}", x));
    }
}
