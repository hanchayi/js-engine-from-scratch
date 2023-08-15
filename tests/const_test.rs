use engine::engine::run_script;

#[test]
pub fn check_econst() {
    let script = "const a = \"World\";
    a;".to_string();
    let res = run_script(script).to_string();
    assert_eq!(res, "World");
}