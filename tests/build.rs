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
        println!("{:?}", msg);
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
        println!("{:?}", msg);
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
        println!("{:?}", msg);
    }
}

#[test]
fn test_error() {
    let msgs = escargot::CargoBuild::new()
        .manifest_path("tests/fixtures/error/Cargo.toml")
        .current_release()
        .current_target()
        .exec();
    assert!(msgs.is_err());
    println!("```{}```", msgs.err().unwrap());
}
