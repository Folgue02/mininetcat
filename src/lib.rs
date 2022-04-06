use std::{
    error,
    net,
    thread
};
use std::io::{Read, Write};

pub struct Config {
    pub ip: String,
    pub port: String
}

impl Config {
    pub fn new(arguments: &[String]) -> Result<Self, &'static str> {
        if arguments.len() < 3 {
            return Err("Not enough arguments.");
        }
        Ok(Self {
            ip: arguments[1].clone(),
            port: arguments[2].clone()
        })
    }
}

pub fn run(arguments: &[String]) -> Result<(), Box<dyn error::Error>>{
    let config = Config::new(arguments)?;
    let socket = net::TcpStream::connect(format!("{}:{}", &config.ip, &config.port)).unwrap();
    let listen_socket = socket.try_clone().unwrap();
    let echo_socket = socket.try_clone().unwrap();
    thread::spawn(move || listen(listen_socket));
    echo(echo_socket)?;


    Ok(())
}

pub fn listen(mut stream: net::TcpStream) -> Result<(), ()> {
    let mut buffer: Vec<u8> = Vec::new();
    loop {
        if stream.read(&mut buffer).unwrap() == 0 {
            break;
        }
        else {
            println!("->{}<-", std::str::from_utf8(&buffer).unwrap());
        }
    }
    Ok(())
}


pub fn echo(mut stream: net::TcpStream) -> Result<(), &'static str> {
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        match stream.write(&buffer[..buffer.len()-1].as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Connection closed.")
        };
        println!("Sending data.")
    }
}

