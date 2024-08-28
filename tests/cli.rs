use assert_cmd::Command;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_input_text_argument() {
    let mut cmd = Command::cargo_bin("khmercut").unwrap();

    // Test with input_text argument
    cmd.arg("--text")
        .arg("សួស្តីពិភពលោក")
        .arg("--delimiter")
        .arg("|")
        .assert()
        .success()
        .stdout("សួស្តី|ពិភពលោក");
}

#[test]
fn test_stdin_input() {
    let mut cmd = Command::cargo_bin("khmercut").unwrap();

    // Test with stdin input
    cmd.arg("--delimiter")
        .arg("|")
        .write_stdin("សួស្តីពិភពលោក")
        .assert()
        .success()
        .stdout("សួស្តី|ពិភពលោក");
}

#[test]
fn test_input_file_argument() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("input.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "សួស្តីពិភពលោក").unwrap();

    let mut cmd = Command::cargo_bin("khmercut").unwrap();

    // Test with input_file argument
    cmd.arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--delimiter")
        .arg("|")
        .assert()
        .success()
        .stdout("សួស្តី|ពិភពលោក");
}
