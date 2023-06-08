use rodio::Sink;
use rodio::Decoder;
use std::fs::File;
use std::io::{BufReader, Write};
use std::time::Duration;

fn generate_speech(input: &str) {
    let mut path = String::from("../sounds/A.wav");

    for c in input.chars() {
        if c.is_alphanumeric() {
            path.replace_range(10..11, &c.to_string().to_uppercase());

            let file = File::open(path.clone()).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();

            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            sink.append(source);
            sink.play();
            sink.sleep_until_end();

            print!("{}", c);
            std::io::stdout().flush().unwrap();
        } else if c.is_whitespace() {
            print!(" ");
            std::io::stdout().flush().unwrap();
        } else {
            print!("{}", c);
            std::io::stdout().flush().unwrap();
        }
    }
}


fn main() {
    print!("Enter your phrase of choice: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
        }
        Err(error) => println!("Error: {}", error),
    }

    print! ("\x1B[2J\x1B[1;1H");
    
    generate_speech(&input);

    std::thread::sleep(Duration::from_millis(2000));

    print! ("\x1B[2J\x1B[1;1H");
}