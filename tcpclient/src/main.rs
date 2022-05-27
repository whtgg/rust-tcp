/*
 * @Description: 
 * @Version: 2.0
 * @Autor: wht
 * @Date: 2022-05-27 21:34:59
 * @LastEditors: wht
 * @LastEditTime: 2022-05-27 22:33:49
 */
use std::{net::TcpStream, io::{Read, Write}};
use std::str;

fn main() {
    let mut  stream = TcpStream::connect("localhost:3000").unwrap();
    let mut buffer = [0;5];
    
    stream.write("hello".as_bytes()).unwrap();
    stream.read(&mut buffer).unwrap();
    println!("receive from server:{:?}",str::from_utf8(&buffer).unwrap())
    
}
