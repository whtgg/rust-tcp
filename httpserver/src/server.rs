/*
 * @Description: 
 * @Version: 2.0
 * @Autor: wht
 * @Date: 2022-05-28 10:37:04
 * @LastEditors: wht
 * @LastEditTime: 2022-05-28 16:20:33
 */
use http::httprequest::HttpRequest;
use std::{net::TcpListener, io::Read};

use crate::router::Router;


pub struct Server<'a> {
    socket_addr:&'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr:&'a str) -> Self {
        Self { socket_addr}
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        
        println!("Running on {}",self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            
            let mut read_buffer = [0;400];
            stream.read(&mut read_buffer).unwrap();
            
            let req:HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            println!("url {:?}",req.resource);
            Router::route(req,&mut stream);
        }
    }
}