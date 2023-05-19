fn test_fixture(name: &str) {
    let temp = assert_fs::TempDir::new().unwrap();

    let cmd = escargot::CargoBuild::new()
        .manifest_path(format!("tests/fixtures/{}/Cargo.toml", name))
        .current_release()
        .current_target()
        .target_dir(temp.path())
        .run()
        .unwrap();
    let output = cmd.command().output().unwrap();
    assert!(output.status.success());
}

#[test]
fn test_bin() {
    test_fixture("bin");
}

#[test]
fn test_warn() {
    test_fixture("warn");
}

#[test]
fn test_error() {
    let result = escargot::CargoBuild::new()
        .manifest_path("tests/fixtures/error/Cargo.toml")
        .current_release()
        .current_target()
        .run();
    assert!(result.is_err());
    println!("```{}```", result.err().unwrap());
}
