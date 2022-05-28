/*
 * @Description: 
 * @Version: 2.0
 * @Autor: wht
 * @Date: 2022-05-28 00:06:16
 * @LastEditors: wht
 * @LastEditTime: 2022-05-28 10:19:10
 */
use std::{collections::HashMap, io::{Result, Write}};

#[derive(Debug,PartialEq,Clone)]
pub struct HttpResponse<'a> {
    version:&'a str,
    status_code:&'a str,
    status_text:&'a str,
    headers:Option<HashMap<&'a str,&'a str>>,
    body: Option<String>,
}

impl<'a> From<HttpResponse<'a>> for String{
    fn from(res: HttpResponse) -> Self {
        let res1 = res.clone();

        format!("{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",&res1.version(),&res1.status_code(),&res1.status_text(),&res1.headers(),&res1.body().len(),&res1.body())
    }
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version:"HTTP/1.1".into(),
            status_code:"200".into(),
            status_text:"OK".into(),
            headers:None,
            body:None
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(status_code:&'a str,headers:Option<HashMap<&'a str,&'a str>>,body:Option<String>) -> Self {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        }

        response.headers = match &headers {
            Some(_h) => headers,
            _ => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match  response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internet server error".into(),
            _ => "Unknown error".into(),
        };
        response.body = body;
        
        return response;
    }

    pub fn send_response(&self,write_stream:&mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string:String = res.into();
        let _  = write!(write_stream,"{}",response_string);
        Ok(())
    }

    pub fn version(&self) -> &str {
        self.version
    }
    
    pub fn status_code(&self) -> &str {
        self.status_code
    }

    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string = "".into();
        for (k,v) in map.into_iter() {
            header_string = format!("{}{}:{}\r\n",header_string,k,v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(body) => body.as_str(),
            None => ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_response_struct_asseration_200() {
        let response_actual = HttpResponse::new("200", None, Some("hello".into()));
        
        let response_expected = HttpResponse {
            version:"HTTP/1.1",
            status_code:"200",
            status_text:"OK",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("hello".into()),
        };

        assert_eq!(response_actual,response_expected);
    }

    #[test]
    fn test_response_struct_asseration_404() {
        let response_actual = HttpResponse::new("404", None, Some("hello".into()));
        
        let response_expected = HttpResponse {
            version:"HTTP/1.1",
            status_code:"404",
            status_text:"Not Found",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("hello".into()),
        };

        assert_eq!(response_actual,response_expected);
    }
}