extern crate escargot;

#[test]
fn test_bin() {
    let msgs = escargot::CargoBuild::new()
        .manifest_path("tests/fixtures/bin/Cargo.toml")
        .current_release()
        .current_target()
        .exec()
        .unwrap();
    for msg in msgs {
        let msg = msg.unwrap();
        let msg: escargot::format::Message = msg.convert().unwrap();
        println!("{:#?}", msg);
    }
}

#[test]
fn test_lib() {
    let msgs = escargot::CargoBuild::new()
        .manifest_path("tests/fixtures/lib/Cargo.toml")
        .current_release()
        .current_target()
        .exec()
        .unwrap();
    for msg in msgs {
        let msg = msg.unwrap();
        let msg: escargot::format::Message = msg.convert().unwrap();
        println!("{:#?}", msg);
    }
}

#[test]
fn test_bin_lib() {
    let msgs = escargot::CargoBuild::new()
        .manifest_path("tests/fixtures/bin_lib/Cargo.toml")
        .current_release()
        .current_target()
        .exec()
        .unwrap();
    for msg in msgs {
        let msg = msg.unwrap();
        let msg: escargot::format::Message = msg.convert().unwrap();
        println!("{:#?}", msg);
    }
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
