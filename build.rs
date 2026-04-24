use std::process::Command;

fn run_git(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn git_tag_exact_match() -> Option<String> {
    run_git(&["describe", "--tags", "--exact-match"])
}

fn git_tag_latest_by_version() -> Option<String> {
    run_git(&["tag", "--sort=-version:refname"])
        .and_then(|tags| tags.lines().next().map(|tag| tag.trim().to_string()))
        .filter(|tag| !tag.is_empty())
}

fn git_tag_latest_reachable() -> Option<String> {
    run_git(&["describe", "--tags", "--abbrev=0"])
}

fn main() {
    println!("cargo:rerun-if-env-changed=GITHUB_REF_NAME");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");

    let ci_tag = std::env::var("GITHUB_REF_NAME")
        .ok()
        .filter(|tag| !tag.is_empty());

    let version = ci_tag
        .or_else(git_tag_exact_match)
        .or_else(git_tag_latest_by_version)
        .or_else(git_tag_latest_reachable)
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

    let normalized = version.strip_prefix('v').unwrap_or(&version);

    println!("cargo:rustc-env=APP_VERSION={normalized}");
}
