#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap().lines().next().unwrap().to_owned();
    let exp = String::from("<!doctype html>");
    assert_eq!(act, exp);
}
