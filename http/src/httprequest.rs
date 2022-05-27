use std::collections::HashMap;

/*
 * @Description: 
 * @Version: 2.0
 * @Autor: wht
 * @Date: 2022-05-27 22:47:09
 * @LastEditors: wht
 * @LastEditTime: 2022-05-27 23:59:48
 */
#[derive(Debug,PartialEq)]
pub enum  Method {
    POST,
    GET,
    Uninitialized,
}

#[derive(Debug,PartialEq)]
pub enum  Version {
    V1_0,
    V2_0,
    Uninitialized,
}

#[derive(Debug,PartialEq)]
pub enum Resource {
    Path(String)
}

#[derive(Debug,PartialEq)]
pub struct HttpRequest {
    pub method:Method,
    pub version:Version,
    pub resource:Resource,
    pub headers:HashMap<String,String>,
    pub msg_body:String
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match  s {
            "GET" =>  Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match  s {
            "HTTP/1.1" =>  Version::V1_0,
            _ => Version::Uninitialized,
        }
    }
}

fn process_req_line(s:&str) ->  (Method,Resource,Version) {
    let mut words = s.split_whitespace();
    let  method = words.next().unwrap();
    let  resource = words.next().unwrap();
    let  version = words.next().unwrap();
    (method.into(),Resource::Path(resource.into()),version.into())
}

fn process_header_line(s:&str) -> (String,String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value:String = String::from("");
    
    if let Some(k) = header_items.next() {
        key = k.into();
    }

    if let Some(v) = header_items.next() {
        value = v.into();
    }
    (key,value)
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut  parse_method = Method::GET;
        let mut parse_version = Version::V1_0;
        let mut parse_resource:Resource = Resource::Path("".to_string());
        let mut parse_msg_body= "";
        let mut parse_headers:HashMap<String,String> = HashMap::new();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method,resource,version) = process_req_line(line);
                parse_method = method;
                parse_version = version;
                parse_resource = resource;
            } else if line.contains(":"){
                let (key,value) = process_header_line(line);
                parse_headers.insert(key, value);
                
            } else {
                parse_msg_body = line
            }
        }
        HttpRequest {
            method:parse_method,
            version:parse_version,
            resource:parse_resource,
            headers:parse_headers,
            msg_body:parse_msg_body.into(),
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mehod_into() {
        let m:Method = "GET".into();
        assert_eq!(m,Method::GET);
    }

    #[test]
    fn test_version_into() {
        let v:Version  = "HTTP/1.1".into();
        assert_eq!(v,Version::V1_0);
    }

    #[test]
    fn test_read_http() {
        let s = String::from("GET /greeting HTTP/1.1 \r\nHost:localhost:3000\r\nAccept:*/*\r\nUser-Agent:Mozilla/5.0");
        
        let mut headers_expected= HashMap::new();
        headers_expected.insert("User-Agent".into(), "Mozilla/5.0".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("Host".into(), "localhost:3000".into());

        let req:HttpRequest = s.into();
        assert_eq!(req.method,Method::GET);
        assert_eq!(req.headers,headers_expected)
    }
}