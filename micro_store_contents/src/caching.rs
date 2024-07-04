use crate::{fetching::get_jobs, Job};
use futures::{stream, StreamExt};
use sha256::{digest, try_async_digest};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub async fn job_hash(job: &Job) -> String {
    let url: Vec<String> = job.job_link.clone().unwrap_or(Vec::new());
    if url.is_empty() {
        "nullhash".to_string()
    } else {
        digest(&url[0])
    }
}

pub async fn cache_job(job: Job) -> std::io::Result<()> {
    fs::create_dir("job_sources").ok();

    fs::write(
        format!("job_sources/{}.txt", &job.id),
        &job.blob.as_ref().unwrap_or(&"".to_string()).as_str(),
    )
    .expect("Error saving file");
    Ok(())
}

pub async fn cache_jobs(jobs: Vec<Job>) -> std::io::Result<()> {
    fs::create_dir("job_sources").ok();
    for job in jobs.into_iter() {
        //let hash = job_hash(&job).await;

        fs::write(
            format!("job_sources/{}.txt", &job.id),
            &job.blob.as_ref().unwrap_or(&"".to_string()).as_str(),
        )
        .expect("Error saving file");
    }
    Ok(())
}

pub async fn update_jobs_html(beg: u64, size: u64, key: &String) -> Result<(), &str> {
    if key == "testkey" {
        let jobs = get_jobs(beg, size).await;
        cache_jobs(jobs).await.unwrap();
        Ok(())
    } else {
        Err("bad key")
    }
}
