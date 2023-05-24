use chrono::Utc;
use colored::*;
use dialoguer::Input;
use figlet_rs::FIGfont;
use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::fmt::Display;
use std::sync::Mutex;
use thiserror::Error;

const GITLAB_API_URL: &str = "https://gitlab.com/api/v4";

lazy_static! {
    static ref PROJECT_ID: Mutex<Option<String>> = Mutex::new(None);
    static ref PRIVATE_TOKEN: Mutex<Option<String>> = Mutex::new(None);
}

#[derive(Debug, Deserialize)]
struct Commit {
    created_at: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct MergeRequest {
    iid: i64,
    merged_at: String,
}

#[derive(Debug, Error)]
pub enum GitLabError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Request to GitLab API failed with status: {0}")]
    ApiError(StatusCode),
}

#[tokio::main]
async fn main() -> Result<(), GitLabError> {
    match run_app().await {
        Ok(_) => println!("{}", "Execution completed successfully!".green().bold()),
        Err(e) => eprintln!("{}", format!("Error: {}", e).bright_red()),
    }
    Ok(())
}

async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    display_title();

    let source_branch = prompt("Name of the origin branch", "main");
    let target_branch = prompt("Destination branch name", "production");
    let token = prompt("Your Gitlab Token", "");
    let project_id = prompt("Project GitLab ID", "");

    *PROJECT_ID.lock().unwrap() = Some(project_id);
    *PRIVATE_TOKEN.lock().unwrap() = Some(token);

    let client = Client::new();

    let merge_requests = get_merged_requests(&client, &source_branch, &target_branch).await?;
    let (total_duration, mr_count_staging) =
        process_merge_requests(&client, &merge_requests, &source_branch, &target_branch).await?;
    print_average_duration(total_duration, mr_count_staging, &target_branch, &source_branch);

    Ok(())
}

async fn process_merge_requests(
    client: &Client,
    merge_requests: &[MergeRequest],
    source_branch: &str,
    target_branch: &str,
) -> Result<(chrono::Duration, i32), GitLabError> {
    let mut total_duration = chrono::Duration::zero();
    let mut mr_count = 0;

    for mr in merge_requests {
        let time_difference = calculate_time_difference(client, mr, source_branch).await?;
        total_duration = total_duration + time_difference;
        mr_count += 1;

        print_time_difference(mr.iid, &time_difference, source_branch, target_branch);
    }

    Ok((total_duration, mr_count))
}

async fn calculate_time_difference(
    client: &Client,
    mr: &MergeRequest,
    source_branch: &str,
) -> Result<chrono::Duration, GitLabError> {
    let merged_at = mr.merged_at.parse::<chrono::DateTime<Utc>>()?;
    let commits = get_commit(client, &mr.iid).await?;

    let main_date = {
        let merge_commits: Vec<&Commit> = commits
            .iter()
            .filter(|commit| {
                let message_lower = commit.message.to_lowercase();
                let merge_into_target =
                    format!("merge branch '.*' into '{}'", source_branch).to_lowercase();
                let re = regex::Regex::new(&merge_into_target).unwrap();
                re.is_match(&message_lower)
            })
            .collect();

        if merge_commits.is_empty() {
            commits
                .first()
                .unwrap()
                .created_at
                .parse::<chrono::DateTime<Utc>>()?
        } else {
            merge_commits
                .last()
                .unwrap()
                .created_at
                .parse::<chrono::DateTime<Utc>>()?
        }
    };

    Ok(merged_at - main_date)
}

fn print_time_difference(
    id: i64,
    time_difference: &chrono::Duration,
    source_branch: &str,
    target_branch: &str,
) {
    let time_difference_seconds = time_difference.num_seconds();
    let hours = time_difference_seconds.div_euclid(3600);
    let remainder = time_difference_seconds.rem_euclid(3600);
    let minutes = remainder.div_euclid(60);
    let seconds = remainder.rem_euclid(60);

    println!(
        "{}{}{}{}{}{}{}{}{}",
        random_color("Merge Request #"),
        random_color(id.to_string()),
        random_color(": The time it took to merge "),
        random_color(source_branch),
        random_color(" to "),
        random_color(target_branch),
        random_color(" was "),
        random_color(format!(
            "{} hours and {} minutes {} seconds",
            hours, minutes, seconds
        )),
        random_color(".")
    );
}

fn print_average_duration(total_duration: chrono::Duration, mr_count: i32, target_branch: &str, source_branch: &str) {
    let average_duration = if mr_count > 0 {
        total_duration / mr_count
    } else {
        chrono::Duration::zero()
    };

    let average_duration_seconds = average_duration.num_seconds();
    let average_hours = average_duration_seconds.div_euclid(3600);
    let remainder = average_duration_seconds.rem_euclid(3600);
    let average_minutes = remainder.div_euclid(60);
    let average_seconds = remainder.rem_euclid(60);

    println!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        random_color("The average duration of the merge request from "),
        random_color(source_branch),
        random_color(" to "),
        random_color(target_branch),
        random_color(" is "),
        random_color(average_hours.to_string()),
        random_color(" hours "),
        random_color(average_minutes.to_string()),
        random_color(" minutes, and "),
        random_color(average_seconds.to_string()),
        random_color(" seconds."),
    );
}

async fn get_commit(client: &Client, iid: &i64) -> Result<Vec<Commit>, GitLabError> {
    let (project_id_str, private_token_str) = get_project_id_and_token();

    let url = format!(
        "{}/projects/{}/merge_requests/{}/commits",
        GITLAB_API_URL, project_id_str, iid
    );

    let response = client
        .get(&url)
        .header("Private-Token", private_token_str)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(GitLabError::ApiError(response.status()));
    }

    let commit: Vec<Commit> = response.json().await?;
    Ok(commit)
}

async fn get_merged_requests(
    client: &Client,
    source_branch: &str,
    target_branch: &str,
) -> Result<Vec<MergeRequest>, GitLabError> {
    let (project_id_str, private_token_str) = get_project_id_and_token();

    let url = format!(
        "{}/projects/{}/merge_requests?state=merged&source_branch={}&target_branch={}",
        GITLAB_API_URL, project_id_str, source_branch, target_branch
    );

    let response = client
        .get(&url)
        .header("Private-Token", private_token_str)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(GitLabError::ApiError(response.status()));
    }

    let merge_requests: Vec<MergeRequest> = response.json().await?;
    Ok(merge_requests)
}

fn display_title() {
    let standard_font = FIGfont::standard().unwrap();
    let title = standard_font.convert("CRONOS CLI").unwrap().to_string();
    let title_lines: Vec<&str> = title.split('\n').collect();

    let colored_title = format!(
        "{}\n{}\n{}\n{}\n{}",
        title_lines[0].red(),
        title_lines[1].green(),
        title_lines[2].blue(),
        title_lines[3].yellow(),
        title_lines[4].cyan()
    );

    println!("{}", colored_title);
}

fn prompt(message: &str, default: &str) -> String {
    println!("{}", random_color(message));
    Input::new()
        .default(default.to_string())
        .interact_text()
        .unwrap()
}

fn random_color<T: Display>(msg: T) -> colored::ColoredString {
    let msg_str = format!("{}", msg);
    let mut rng = thread_rng();
    let color_range = Uniform::from(1..=5);

    let random_number = rng.sample(color_range);

    match random_number {
        1 => msg_str.red(),
        2 => msg_str.green(),
        3 => msg_str.blue(),
        4 => msg_str.yellow(),
        5 => msg_str.cyan(),
        _ => msg_str.white(),
    }
    .bold()
}

fn get_project_id_and_token() -> (String, String) {
    let project_id = PROJECT_ID.lock().unwrap();
    let project_id_str = project_id.as_ref().unwrap().clone();

    let private_token = PRIVATE_TOKEN.lock().unwrap();
    let private_token_str = private_token.as_ref().unwrap().clone();

    (project_id_str, private_token_str)
}
