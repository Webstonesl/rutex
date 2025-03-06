fn main() {
    // Get git commit hash
    let git_hash = match std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let hash = String::from_utf8(output.stdout).unwrap();
                hash.trim().to_string()
            } else {
                "unknown".to_string()
            }
        }
        Err(_) => "unknown".to_string(),
    };
    // Check if there is uncommitted changes
    let has_changes = match std::process::Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let status = String::from_utf8(output.stdout).unwrap();

                status.trim().len() > 0
            } else {
                true
            }
        }
        Err(_) => true,
    };
    // Check for git tag
    let git_tag = match std::process::Command::new("git")
        .args(&["describe", "--tags", "--abbrev=0"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let tag = String::from_utf8(output.stdout).unwrap();
            Some(tag.trim().to_string())
        }
        _ => None,
    };

    // Get current date and time
    let now = chrono::Local::now();
    let build_date = now.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    // Write the git hash and build date to a file
    std::fs::write(
        "src/build_info.rs",
        format!(
            r#"pub const GIT_HASH: &str = "{}";
pub const BUILD_DATE: &str = "{}";
pub const GIT_TAG: Option<&str> = {:?};
pub const HAS_CHANGES: bool = {:#?};
pub const VERSION: &str = {:?};
"#,
            git_hash,
            build_date,
            git_tag,
            has_changes,
            env!("CARGO_PKG_VERSION")
        ),
    )
    .unwrap();
    // Compile the rutex library
}
