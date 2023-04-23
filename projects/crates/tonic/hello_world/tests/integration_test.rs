#[test]
fn cargo_run_server() {
    let output = ::std::process::Command::new("cargo").args(["run", "--bin", "helloworld-server"]).output().expect("failure");
    assert!(output.status.success());
    let act = String::from("TODO\n");
    let exp = String::from_utf8(output.stdout).unwrap();
    assert_eq!(act, exp);
}

#[test]
fn cargo_run_client() {
    let output = ::std::process::Command::new("cargo").args(["run", "--bin", "helloworld-client"]).output().expect("failure");
    assert!(output.status.success());
    let act = String::from("TODO\n");
    let exp = String::from_utf8(output.stdout).unwrap();
    assert_eq!(act, exp);
}
