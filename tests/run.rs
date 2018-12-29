extern crate escargot;

#[test]
fn test_run() {
    let cmd = escargot::CargoBuild::new()
        .manifest_path("tests/fixtures/bin/Cargo.toml")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let output = cmd
        .command()
        .output()
        .unwrap();
    assert!(output.status.success());
}
