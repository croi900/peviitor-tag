use core::panic;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    time::Duration,
};

use futures::{self, stream, StreamExt};
use reqwest::{self, header};
use serde::{de::Error, Deserialize, Serialize};
use serde_json::{json, Result, Value};
use std::collections::HashSet;

use crate::caching;
const DB_CORE: &str = "jobs";

#[derive(Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: String,
    pub blob: Option<String>, //TODO make option
    pub words: Option<Vec<String>>,
    pub job_title: Option<Vec<String>>,
    pub job_link: Option<Vec<String>>,
}

pub async fn get_batch(start: u64, sz: u64) -> Result<Vec<Job>> {
    let body = reqwest::get(
        format!("https://solr.peviitor.ro/solr/{}/select?indent=true&q.op=OR&q=*%3A*&rows={}&start={}&useParams=",DB_CORE,sz,start))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    let solr_root: Value = serde_json::from_str(body.as_str())?;
    let solr_jobs = solr_root.get("response").unwrap().get("docs").unwrap();

    let jobs: Vec<Job> = serde_json::from_value(solr_jobs.clone()).unwrap();

    Ok(jobs)
}

pub async fn download_jobs(start: u64, sz: u64) {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
        ),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let jobs: Vec<Job> = get_batch(start, sz).await.unwrap();

    let tasks = stream::iter(jobs)
        .map(|mut job: Job| {
            let client = &client;
            async move {
                let url: Vec<String> = match job.job_link.as_ref() {
                    Some(vec) => vec.to_vec(),
                    None => Vec::new(),
                };
                if !url.is_empty() {
                    if !std::fs::metadata(format!("{}.txt", &job.id)).is_ok() {
                        let resp = client
                            .get(url[0].clone())
                            .timeout(Duration::from_secs(5 * 60))
                            .send()
                            .await?;
                        let blob = resp.text().await;

                        match blob {
                            Ok(blob) => {
                                println!("{}", job.id);

                                job.blob = Some(blob);

                                caching::cache_job(job.clone()).await.unwrap();
                                return Ok(job);
                            }
                            Err(e) => return Err(e),
                        }
                    } else {
                        println!("job cached already");
                    }
                }
                Ok(job)
            }
        })
        .buffer_unordered(16);

    tasks
        .for_each(|b| async {
            //let downloaded = Arc::clone(&downloaded_clone);
            match b {
                Ok(b) => {
                    //println!("{:#?} {:#?} {:#?}", b.id, b.blob.len(), downloaded.lock().unwrap().len());
                    println!("success {:#?} ", b.id);
                }
                Err(_e) => println!("error"),
                //Err(e) => println!("error {:#?}", e),
            }
        })
        .await;
}

pub async fn get_jobs(start: u64, sz: u64) -> Vec<Job> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
        ),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let jobs: Vec<Job> = get_batch(start, sz).await.unwrap();

    let tasks = stream::iter(jobs)
        .map(|mut job: Job| {
            let client = &client;
            async move {
                let url: Vec<String> = match job.job_link.as_ref() {
                    Some(vec) => vec.to_vec(),
                    None => Vec::new(),
                };
                if !url.is_empty() {
                    if !std::fs::metadata(format!("{}.txt", &job.id)).is_ok() {
                        let resp = client
                            .get(url[0].clone())
                            .timeout(Duration::from_secs(5 * 60))
                            .send()
                            .await?;
                        let blob = resp.text().await;

                        match blob {
                            Ok(blob) => {
                                println!("{}", job.id);

                                job.blob = Some(blob);
                                return Ok(job);
                            }
                            Err(e) => return Err(e),
                        }
                    } else {
                        println!("job cached already");
                    }
                }
                Ok(job)
            }
        })
        .buffer_unordered(10);

    let downloaded: Arc<Mutex<Vec<Job>>> = Arc::new(Mutex::new(Vec::new()));
    let downloaded_clone = Arc::clone(&downloaded);

    tasks
        .for_each(|b| async {
            let downloaded = Arc::clone(&downloaded_clone);
            match b {
                Ok(b) => {
                    //println!("{:#?} {:#?} {:#?}", b.id, b.blob.len(), downloaded.lock().unwrap().len());
                    downloaded.lock().unwrap().push(b);
                }
                Err(e) => println!("error"),
                //Err(e) => println!("error {:#?}", e),
            }
        })
        .await;

    let downloaded_lock = downloaded.lock().unwrap();
    let downloaded_vec = downloaded_lock.deref().clone();

    downloaded_vec
}

/*STORE FOR OTHER SERVICE
let html = blob.clone();
                        // let parsed_html = Html::parse_document(html.as_str());

                        // let mut all_text = HashSet::new();
                        // for node in parsed_html.root_element().descendants() {
                        //     if let Some(element) = ElementRef::wrap(node) {
                        //         if !is_js_or_css_bloat(&element.value().name()) {
                        //             for text_node in element.text() {
                        //                 all_text.insert(text_node.trim().to_string());
                        //             }
                        //         }
                        //     }
                        // }

                        // for text in &all_text {
                        //     if !text.is_empty() {
                        //         job.blob += text;
                        //     }
                        // }
fn is_js_or_css_bloat(tag: &str) -> bool {
    tag == "p"
}
*/
