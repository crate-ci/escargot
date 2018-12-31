extern crate escargot;

fn test_fixture(name: &str) {
    let msgs = escargot::CargoBuild::new()
        .manifest_path(&format!("tests/fixtures/{}/Cargo.toml", name))
        .current_release()
        .current_target()
        .exec()
        .unwrap();
    for msg in msgs {
        let raw_msg = msg.unwrap();
        let msg: escargot::error::CargoResult<escargot::format::Message> = raw_msg.convert();
        match msg {
            Ok(msg) => println!("{:#?}", msg),
            Err(err) => panic!("{}\nmsg=`{:#?}`", err, raw_msg),
        }
    }
}

#[test]
fn test_bin() {
    test_fixture("bin");
}

#[test]
fn test_lib() {
    test_fixture("lib");
}

#[test]
fn test_bin_lib() {
    test_fixture("bin_lib");
}

#[test]
fn test_warn() {
    test_fixture("warn");
}

#[test]
fn test_build_script() {
    test_fixture("script");
}

#[test]
fn test_dependency() {
    test_fixture("dep");
}

#[test]
fn test_error() {
    let msgs: Vec<_> = escargot::CargoBuild::new()
        .manifest_path("tests/fixtures/error/Cargo.toml")
        .current_release()
        .current_target()
        .exec()
        .unwrap()
        .collect();
    assert!(1 < msgs.len());
    let error_idx = msgs.len() - 1;
    for msg in &msgs[0..error_idx] {
        let msg = msg.as_ref().unwrap();
        let msg: escargot::format::Message = msg.convert().unwrap();
        println!("{:#?}", msg);
    }
    assert!(msgs[error_idx].is_err());
    println!("```{}```", msgs[error_idx].as_ref().err().unwrap());
}
