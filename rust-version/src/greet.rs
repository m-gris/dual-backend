fn greet() {
    // NOTE: let is a STATEMENT that binds the expression "Hello" (&str) to msg
    // requires ; at the end
    let msg = "Hello, you... Wonderful world!";

    // NOTE:
    // the ! in println! indicates that is is a Macro, not a Function !!!
    // the ; at the end indicates a STATEMENT.
    //  println! treats its first argument as a format string that may contain placeholders.
    println!("{}", msg);
}
