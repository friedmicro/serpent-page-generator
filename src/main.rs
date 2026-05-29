mod md;
mod template;

use chrono::{DateTime, Local};
use std::env;
use std::fs;
use std::io;
use std::path::Path;

use crate::md::process_md;
use crate::template::Replacements;
use crate::template::render;

fn main() {
    let site_path: String = env::var("SITE_PATH").unwrap_or_else(|_| "/tmp/website".to_string());

    let commit_hash = resolve_git_commit(&site_path);
    let _ = output_git_changes(&site_path);
    let now_local: DateTime<Local> = Local::now();
    let formatted_local: String = now_local.format("%Y/%m/%d %I:%M:%S %p (%Z)").to_string();

    let mut replacements: Replacements = template::Replacements::default();
    replacements.set("BUILD_VERSION", formatted_local);
    replacements.set("COMMIT", commit_hash);
    replacements.set("GEN_NAME", String::from("Serpent Page Generator"));

    // ── Load template ────────────────────────────────────────────────
    let md_job_projects: String = site_path.clone() + "/md/job_projects";
    let md_personal: String = site_path.clone() + "/md/personal";
    let md_personal_projects: String = site_path.clone() + "/md/personal_projects";
    let _ = parse_files(&md_job_projects, &replacements, true);
    let _ = parse_files(&md_personal, &replacements, false);
    let _ = parse_files(&md_personal_projects, &replacements, false);
    let _ = parse_files(&site_path, &replacements, false);
}

fn parse_files(
    site_path: &String,
    replacements: &Replacements,
    skip_index_inject: bool,
) -> io::Result<()> {
    let entries = fs::read_dir(Path::new(site_path.as_str()));
    for entry in entries? {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        let name = entry.file_name().into_string().unwrap();
        let file_type = entry.file_type().expect("Failed to get file type");

        if name.contains(".html") || name.contains(".md") {
            if !file_type.is_dir() {
                let contents =
                    fs::read_to_string(path).expect("Should have been able to read the file");

                if name.contains(".md") {
                    let html_name: String = name.replace(".md", ".html");
                    let md_content = process_md(contents.as_str(), skip_index_inject);
                    let result = render(md_content.as_str(), &replacements);
                    fs::write(String::from("out/") + &html_name, result)?;
                } else {
                    let result = render(contents.as_str(), &replacements);
                    fs::write(String::from("out/") + &name, result)?;
                }
            }
        } else {
            let blacklisted_files = [
                "main.css",
                ".gitignore",
                "package.json",
                "package-lock.json",
            ];
            if !file_type.is_dir() {
                if blacklisted_files.contains(&name.as_str()) {
                    continue;
                }
                fs::copy(path, String::from("out/") + &name)?;
            }
        }
    }
    Ok(())
}

fn resolve_git_commit(repo_root: &String) -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(repo_root)
        .output()
        .expect("failed to execute process");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn output_git_changes(repo_root: &String) -> io::Result<()> {
    let output = std::process::Command::new("git")
        .args(["--no-pager", "diff", "HEAD", "HEAD~1"])
        .current_dir(repo_root)
        .output()
        .expect("failed to execute process");
    fs::write(
        String::from("out/diff.txt"),
        String::from_utf8_lossy(&output.stdout).to_string(),
    )?;
    Ok(())
}
