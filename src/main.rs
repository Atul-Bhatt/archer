use std::io;
use std::io::Write;

enum Object {
    BOOLEAN(String),
    FIXNUM(i64),
}

// Read from terminal
fn read_string() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(e) => {
            println!("Cannot read string: {e}");
            "".to_string()
        },
    }
}

fn read() -> Object {
    let input_string = read_string().trim().to_owned();
    if input_string == "true" || input_string == "false" {
        return Object::BOOLEAN(input_string.to_string());
    }
    Object::FIXNUM(input_string.parse::<i64>().unwrap())
}

fn eval(expr: Object) -> Object {
    expr
}

fn write(obj: Object) {
    match obj {
        Object::BOOLEAN(val) => print!("{}", val),
        Object::FIXNUM(n) => print!("{}", n),
        _ => println!("cannot read input"),
    }
}

fn main() {
    println!(r#"
        Welcome to Archer.
        Use ctrl-c to exit."#);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        write(eval(read()));
        println!();
    }
}
