use engine::engine::run_script;


#[test]
fn check_new() {
    let script = "function People() {}
    new People()".to_string();
    run_script(script);
}