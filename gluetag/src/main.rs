use rayon::prelude::*;
use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use libc::{fcntl, F_SETFL, O_NONBLOCK};
use std::os::unix::io::{AsRawFd, RawFd};
mod extract_words;
mod tag_match;
mod update;

fn set_nonblocking(fd: RawFd) {
    unsafe {
        let flags = fcntl(fd, F_SETFL, O_NONBLOCK);
        if flags == -1 {
            // Something went wrong, handle the error
            // Minimal suggestion: Panic or print an error message
            panic!("Failed to set file descriptor to non-blocking mode");
        }
    }
}

fn main() {
    if cfg!(target_os = "windows") {
        panic!("glue not implemented on windows machines, please run under WSL");
    } else {
        let paths = std::fs::read_dir("./bin/job_sources")
            .expect("no bin directory or job_sources folder is missing in bin");
        let names = paths
            .filter_map(|entry| {
                let fullpath = match entry {
                    Ok(entry) => entry.path().to_str().unwrap().to_string(),
                    Err(_) => "error".to_string(),
                };

                Some(fullpath)
            })
            .collect::<Vec<String>>();

        let tag_map = tag_match::load_tags_from_folder("./bin/tags");

        names.par_iter().for_each(|path| {
            if path != "error" {
                let id = path.split(".").collect::<Vec<&str>>();

                if id.len() > 1 {
                    let id: &str = id[id.len() - 2];
                    let id: &str = id.split("/").collect::<Vec<&str>>().last().unwrap();

                    let code = std::fs::read_to_string(path).unwrap();
                    let words = extract_words::parse(&code).join(" ");
                    let tags = tag_match::match_all(words.as_str(), &tag_map);

                    if tags.len() > 0 {
                        update::update(&id, &tags);
                    }
                } else {
                    return;
                }
            }
        });

        update::send();
    };
}

/*let output = Command::new("cat")
    .arg(path)
    //  .stdout(Stdio::piped())
    //  .spawn()
    .output()
    .expect("failed to execute process");

/*if let Some(stdin) = output.stdin.as_mut() {
    set_nonblocking(stdin.as_raw_fd());
}*/

let extracted_words = Command::new("./bin/extract_words")
    //   .stdin(Stdio::from(output.stdout.unwrap()))
    //   .stdout(Stdio::piped())
    //   .spawn()
    .arg(String::from_utf8_lossy(&output.stdout).to_string())
    .output()
    .expect("failed to execute extract_words");

/* if let Some(stdin) = extracted_words.stdin.as_mut() {
    set_nonblocking(stdin.as_raw_fd());
}*/

let tags = Command::new("./bin/tag_match")
    //.stdin(Stdio::from(extracted_words.stdout.unwrap()))
    .arg(String::from_utf8_lossy(&extracted_words.stdout).to_string())
    .output()
    .expect("failed to tag match");

let tags = String::from_utf8_lossy(&tags.stdout);
if !tags.is_empty() {
    println!("{}", tags);
}*/
