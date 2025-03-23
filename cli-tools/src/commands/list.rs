use crate::github::config::{get_user, get_packages};





pub fn run() {
    let user = get_user();

    let packages = get_packages();

    if packages.is_empty() {
        println!("✅ No GHCR packages found.");
        return;
    }

    println!("📦 GHCR packages:");
    for (i, pkg) in packages.iter().enumerate() {
        println!("  [{}] {}", i + 1, pkg.name);
    }
}
