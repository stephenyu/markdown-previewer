use comrak::{markdown_to_html, ComrakOptions};
use hotwatch::{Event, Hotwatch};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn get_html(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Problem opening the file");
    let html = markdown_to_html(&contents, &ComrakOptions::default());
    return html;
}

fn handle_connection(mut stream: TcpStream, contents: &str) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch(filename, move |event: Event| {
            println!("File changed {}", filename);
        })
        .expect("failed to watch file!");

    // move || println!("{}", filename);
    // let html = get_html(&filename);
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Started Server at http://localhost:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // handle_connection(stream, &html);
    }
    // })
}
