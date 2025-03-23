//! src/github/config.rs
use std::os::unix::process;
#[warn(unused_imports)]
use std::process::Command;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::github::env::{username, token, repo, scope_project};

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub id: u64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub container: ContainerMetadata,
}

#[derive(Debug, Deserialize)]
pub struct ContainerMetadata {
    pub tags: Vec<String>,
}

pub fn get_user() -> String {
    username()
}

pub fn get_packages() -> Vec<Package> {
    let user = username();
    let out = gh_api(&[&format!("/users/{}/packages?package_type=container", user)]);
    serde_json::from_slice(&out).unwrap_or_default()
}

pub fn get_versions(image: &str) -> Vec<Version> {
    let user = username();
    let out = gh_api(&[&format!("/users/{}/packages/container/{}/versions", user, image)]);
    serde_json::from_slice(&out).unwrap_or_default()
}

fn gh_api(args: &[&str]) -> Vec<u8> {
    let token = token();
    let auth_header = format!("Authorization: token {}", token);

    let mut full_args = vec![
        "api",
        "-H", "Accept: application/vnd.github.v3+json",
        "-H", &auth_header,
    ];
    full_args.extend_from_slice(args);

    gh(&full_args)
}

fn gh(args: &[&str]) -> Vec<u8> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .expect("❌ Не удалось вызвать `gh`");

    if !output.status.success() {
        eprintln!("❌ gh error:\n{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    output.stdout
}

pub fn gh_json<T: DeserializeOwned>(args: &[&str]) -> T {
    let out = gh(args);
    serde_json::from_slice(&out).expect("❌ Невалидный JSON из gh")
}

pub fn gh_status(args: &[&str]) -> bool {
    let status = Command::new("gh")
        .args(args)
        .status()
        .expect("❌ Не удалось вызвать `gh`");

    status.success()
}

pub fn get_packages_by_type(package_type: &str) -> Vec<Package> {
    let user = username();
    let out = gh_api(&[&format!("/users/{}/packages?package_type={}", user, package_type)]);
    serde_json::from_slice(&out).unwrap_or_default()
}

pub fn get_versions_by_type(package_type: &str, name: &str) -> Vec<Version> {
    let user = username();
    let out = gh_api(&[&format!(
        "/users/{}/packages/{}/{}/versions",
        user, package_type, name
    )]);
    serde_json::from_slice(&out).unwrap_or_default()
}

pub fn get_project_scope() -> String{
    match scope_project() {
        Some(scope) => {
            if scope.starts_with('@'){
                scope
            }else{
                format!("@{}", scope)
            }
        }
        None => {
            eprintln!("❌ PROJECT_SCOPE не установлен в .env");
            std::process::exit(1);
        }
    }
}