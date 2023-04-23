#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap();
    let number = act.trim_end().parse::<i32>().unwrap();
    assert!((1..=100).contains(&number));
}
