fn main() {
    use std::io::{stdin, stdout, Write};

    print!("what's your name: ");
    stdout()
        .flush()
        .expect("well your girlfriend doesn't want to listen your name now.");
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("well your girlfriend doesn't want to talk with you now.");

    let name = s.trim();

    println!("Hi {} i am your virtual girlfriend! 😉", name);
}

// 하지만 여자친구쨩 이렇게라도 하지 않으면..1!!
