use scraper::{ElementRef, Html, Selector};
use std::collections::HashSet;
use std::env;
use std::io::Read;

pub fn parse(html: &String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    let parsed_html = Html::parse_document(html);
    let mut all_text = HashSet::new();

    let body_selector = Selector::parse("body").unwrap();
    let body_element = parsed_html.select(&body_selector).next().unwrap();
    let num_children = body_element.children().count();

    //println!("Number of children of <body>: {}", num_children);
    for node in parsed_html.root_element().descendants() {
        if let Some(element) = ElementRef::wrap(node) {
            if !is_js_or_css_bloat(&element.value().name()) {
                for text_node in element.text() {
                    all_text.insert(text_node.trim().to_string());
                }
            }
        }
    }

    for text in &all_text {
        if line_filter(&text) {
            for word in text.split(" ").collect::<Vec<&str>>() {
                ret.push(word.chars().filter(|&c| c.is_alphanumeric()).collect());
            }
        }
    }

    ret.sort_unstable();
    ret.dedup();
    let ret = ret.into_iter().filter(|w| w.len() < 13).collect();
    ret
}

fn line_filter(line: &String) -> bool {
    let mut ok: bool = true;

    if line.is_empty() {
        ok = false;
    }

    ok
}

fn is_js_or_css_bloat(tag: &str) -> bool {
    !matches!(
        tag,
        "li" | "p" | "h" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "strong" | "blockquote" | "em"
    )
}
/*
fn main() {
    cat path | xargs -I {} -P $(nproc) sh -c './bin/extract_words {} | ./bin/tag_match'
    let args: Vec<String> = env::args().collect();

    // Now you can access the command line arguments
    // as elements in the `args` vector.

    // For example, to print the first argument:
    if let Some(html) = args.get(1) {
        if html == "--file" || html == "-f" {
            if let Some(path) = args.get(2) {
                let text = std::fs::read_to_string(path).expect("error reading path");
                print!("{}", parse(&text).join("\n"));
            } else {
                panic!("No file provided for --file(-f) arg)");
            }
        } else {
            print!("{}", parse(html).join(" "));
        }
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("error");
        print!("{}", parse(&buffer).join(" "));
    }
}*/
