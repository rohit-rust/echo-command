use assert_cmd::Command;
use predicates::prelude::*;

type Test = Result<(), Box<dyn std::error::Error>>;

#[test]
fn check_no_args() -> Test {
    let mut cmd = Command::cargo_bin("echoc")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> Test {
    let mut cmd = Command::cargo_bin("echoc")?;
    cmd.args(["Hello,World!", "Hello", "This World Is Amazing!"])
        .assert()
        .success();
    Ok(())
}
