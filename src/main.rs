use std::io;

enum Object {
    FIXNUM(i64),
}

// Read from terminal
fn read_string() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => input,
        Err(e) => {
            println!("Cannot read string: {e}");
            "".to_string()
        },
    }
}

fn read() {
}

fn write(obj: Object) {
    match obj {
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
        write(read());
        println!();
    }
}
