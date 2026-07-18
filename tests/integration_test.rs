use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("rust-cp-toolkit").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A feature-rich Rust CLI for competitive programming"));
}

#[test]
fn test_init_command() {
    let dir = tempdir().unwrap();
    let project_name = "test_problem";
    let project_path = dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("rust-cp-toolkit").unwrap();
    cmd.current_dir(dir.path())
        .arg("init")
        .arg(project_name)
        .assert()
        .success();

    assert!(project_path.exists());
    assert!(project_path.join("main.cpp").exists());
    assert!(project_path.join("tests").exists());
    assert!(project_path.join("tests").join("sample1.in").exists());
    assert!(project_path.join("tests").join("sample1.out").exists());
}
