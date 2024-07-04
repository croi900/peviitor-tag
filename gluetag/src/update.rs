/*
remove tags from id
{"id":"", "tags":{"removeregex":".*"}}

add tags to id
{"id":"", "tags":{"add": []}}

mimic curl request curl -X POST -H 'Content-Type: application/json' -d @data.json http://localhost:8983/solr/<collection>/update?commit=true

*/
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE};
use std::{process::Command, sync::Mutex};

lazy_static! {
    static ref BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn update(id: &str, tags: &Vec<String>) {
    let json = format!(
        "{{\"id\":\"{}\",\"tags\":{{\"removeregex\":\".*\"}}}},{{\"id\":\"{}\",\"tags\":{{\"add\":[\"{}\"]}}}}",
        id,id,
        tags.join("\",\"")
    );
    let mut buf = BUFFER.lock().unwrap();
    buf.push(json);
    //if buf.len() > 100 {
    //    send();
    //    buf.clear();
    //}
}

pub fn send() {
    let json = format!("[{}]", BUFFER.lock().unwrap().join(","));

    std::fs::write("./json_to_send", &json).unwrap();

    let _curl = Command::new("curl")
        .arg("-d @./json_to_send")
        .arg("-X POST")
        .arg("-H \'Accept: application/json, text/plain, */*\'")
        .arg("-H \'Accept-Encoding: application/json\'")
        .arg("-v")
        .arg("--compressed")
        .arg("https://solr.peviitor.ro/solr/shaqodoon/update?commitWithin=1000&overwrite=true&wt=json");
    //.output()
    //.expect("failed to send POST request to peviitor.");
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response: Response = client.post("https://solr.peviitor.ro/solr/shaqodoon/update?commitWithin=1000&overwrite=true&wt=json")
        .body(json)
        .headers(headers)
        .send()
        .expect("Failed to send request");

    println!("{}", response.text().expect("error getting response text"));

    //println!("Result: {}", String::from_utf8_lossy(&curl.stdout));
    //println!("Error: {}", String::from_utf8_lossy(&curl.stderr));

    //BUFFER.lock().unwrap().clear();
    //std::fs::remove_file("./json_to_send").expect("couldnt write to file");
}
