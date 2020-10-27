#[cfg(test)]
mod integration_test {
    use std::process::Command;

    #[test]
    fn test_run_with_unknown_option() {
        let output = Command::new("./target/debug/overlap")
            .arg("--unknown")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("error:"));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    }

    #[test]
    fn test_run_with_help() {
        let output = Command::new("./target/debug/overlap")
            .arg("--help")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
        assert!(String::from_utf8_lossy(&output.stdout).contains("USAGE"));
    }
}
