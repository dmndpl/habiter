#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_command1() {
        let mut cmd = Command::cargo_bin("my_cli_tool").unwrap();
        cmd.arg("command1")
            .arg("test_input")
            .assert()
            .success()
            .stdout("Running command1 with input: test_input\n");
    }

    #[test]
    fn test_command2() {
        let mut cmd = Command::cargo_bin("my_cli_tool").unwrap();
        cmd.arg("command2")
            .arg("--verbose")
            .assert()
            .success()
            .stdout("Running command2 in verbose mode\n");
    }
}

