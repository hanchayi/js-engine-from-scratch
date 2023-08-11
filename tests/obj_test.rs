
use engine::engine::run_script;


#[test]
fn check_new() {
    let script = "a = { a: 1 };
    a;".to_string();
    let res = run_script(script);
    println!("{}", res)
}