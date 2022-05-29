/*
 * @Description: 
 * @Version: 2.0
 * @Autor: wht
 * @Date: 2022-05-28 10:37:12
 * @LastEditors: wht
 * @LastEditTime: 2022-05-28 16:05:57
 */
use std::io::Write;

use http::{httprequest,httprequest::HttpRequest,httpresponse::HttpResponse};
use crate::handler::PageNotFoundHandler;

use super::handler::{WebServerHandler, Handler, StaticPageHandler};

pub struct Router;

impl Router {
    pub fn route(req:HttpRequest,stream:&mut impl Write) -> () {
        match req.method {
            httprequest::Method::GET => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route:Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp:HttpResponse = WebServerHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        },
                        _ => {
                            let resp:HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}