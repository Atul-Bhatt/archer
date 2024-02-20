use eval::eval;
use std::io;

fn read() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => input,
        Err(e) => {
            println!("Cannot read string: {e}");
            "".to_string()
        },
    }
}

fn write(input: String) {
    println!("{input}");
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
