use chrono::{DateTime, Datelike, Utc, Weekday};
use core::slice;
use rustygit::Repository;
use std::env;
use std::fs;
use std::io::Write;
use std::mem;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time;

// TODO: add help flag
fn main() {
    let args: Vec<String> = env::args().collect();
    let repo_name = &args[1];

    let repo_path = Path::new(repo_name.as_str());
    let file_name = "file.txt";
    let file_path_rel = repo_path.join(PathBuf::from(&file_name));
    let file_path = fs::canonicalize(file_path_rel).unwrap();

    println!("Repo path set to: {}", repo_path.to_str().unwrap());
    println!("File path set to: {}", file_path.to_str().unwrap());

    let days_to_commit: i32 = args[2].parse().unwrap();

    let now = time::SystemTime::now();

    // Initialize RNG
    let seed = now.duration_since(time::UNIX_EPOCH).unwrap().as_secs();
    let mut rng = oorandom::Rand32::new(seed);

    // TODO: make daily commit count a parameter
    let daily_commit_count: Range<u32> = Range { start: 0, end: 5 };

    // Create folder if needed
    match fs::create_dir_all(repo_path) {
        Ok(_) => (),
        Err(_) => {
            println!("Error creating directory.");
            return;
        }
    };

    // Initialize repository (safe to run if it already exists)
    let repo: Repository = match Repository::init(repo_path) {
        Ok(repo) => repo,
        Err(_) => {
            println!("Error initializing repository.");
            return;
        }
    };

    // Generate commits
    // TODO: improve commit times, avoid them all having the same hour and minute
    for day in 0..days_to_commit {
        let commit_count = rng.rand_range(daily_commit_count.clone());
        for commit_index in 0..commit_count {
            let system_date = now
                - time::Duration::from_secs(
                    (86400 * (days_to_commit - 1 - day) as u64)
                        + (commit_count - 1 - commit_index) as u64,
                );
            let date_time: DateTime<Utc> = system_date.into();

            // Skip most weekends
            match date_time.weekday() {
                Weekday::Sat | Weekday::Sun => {
                    if rng.rand_float() < 0.9 {
                        continue;
                    }
                }
                _ => (),
            }

            // Generate random file contents and write file
            let rn: i32 = rng.rand_i32();
            let rn_ptr: *const i32 = &rn;
            let rn_byte_ptr: *const u8 = rn_ptr as *const _;
            let rn_bytes = unsafe { slice::from_raw_parts(rn_byte_ptr, mem::size_of::<i32>()) };

            // Open file to write
            let mut file = match fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&file_path)
            {
                Ok(file) => file,
                Err(_) => {
                    println!("Error opening file to write.");
                    return;
                }
            };
            match file.write_all(rn_bytes) {
                Ok(_) => (),
                Err(_) => {
                    println!("Error writing to file.");
                    return;
                }
            };

            // Add to repo, set date, then commit
            let file_vec = vec![file_name];
            repo.add(file_vec).unwrap();
            repo.add(vec!["."]).unwrap();

            let date_string = date_time.to_rfc3339();
            env::set_var("GIT_AUTHOR_DATE", &date_string);
            env::set_var("GIT_COMMITTER_DATE", &date_string);

            match repo.commit_all("Fake commit.") {
                Ok(_) => (),
                Err(err) => {
                    println!("{}", err.to_string());
                    return;
                }
            }

            println!("Generated commit for {}.", &date_string);
        }
    }
}
