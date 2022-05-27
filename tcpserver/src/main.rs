/*
 * @Description: 
 * @Version: 2.0
 * @Autor: wht
 * @Date: 2022-05-27 21:35:15
 * @LastEditors: wht
 * @LastEditTime: 2022-05-27 22:26:10
 */
use std::{net::TcpListener, io::{Read, Write}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("connecting 3000");

    for stream  in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0;1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();
        println!("connection is established!")
    }
    // println!("Hello, world!");
}
