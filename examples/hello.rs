use chissa;

fn main() {
    let source = r#"
        >
        ^1+4
        [
            ^5+4
            c>p-
            v5-4
        j]

        ^5+4

        [o<j]
    "#
    .to_string();

    let mut interpreter = chissa::Interpreter::new();
    interpreter.set_source(source);

    for _ in interpreter {}
}
