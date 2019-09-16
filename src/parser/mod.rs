use nom::IResult;
use crate::http::request::{Method, URI, HTTPVersion, Request};
use crate::http::request::{METHOD_MAP, HTTP_VERSION_MAP};
use nom::branch::alt;
use nom::bytes::streaming::{tag, take_until, take};
use nom::combinator::{map, recognize, opt};
use nom::multi::{many0, separated_list};
use nom::sequence::tuple;
use nom::character::complete::{alphanumeric1, none_of, alpha1};
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn method(input: &str) -> IResult<&str, Method> {
    map(alt((tag("OPTIONS"),
             tag("GET"),
             tag("HEAD"),
             tag("POST"),
             tag("PUT"),
             tag("DELETE"),
             tag("TRACE"),
             tag("CONNECT"))),
        |s: &str| {
            *METHOD_MAP.get(s).unwrap()
        })(input)
}

pub(crate) fn uri(input: &str) -> IResult<&str, URI> {
    map(tuple((
        recognize(many0(none_of("? \n"))),
        opt(tuple((
            tag("?"),
            separated_list(
                tag("&"),
                tuple((
                    alphanumeric1,
                    tag("="),
                    opt(alphanumeric1)
                )),
            )
        )))
    )), |(path, query_raw): (&str, Option<(&str, Vec<(&str, &str, Option<&str>)>)>)| {
        let path = path.to_owned();
        let mut query = HashMap::new();
        match query_raw {
            Some(query_raw) => {
                let (_, query_list) = query_raw;
                for it in query_list {
                    query.insert(it.0.to_string(), it.2.unwrap_or("").to_string());
                }
            }
            _ => ()
        }
        URI {
            path,
            query,
        }
    })(input)
}

pub(crate) fn http_version(input: &str) -> IResult<&str, HTTPVersion> {
    map(tuple((
        tag("HTTP/"),
        alt((
            tag("1.0"),
            tag("1.1"),
            tag("2.0"),
        )))), |(_, s): (_, &str)| {
        *HTTP_VERSION_MAP.get(s).unwrap()
    })(input)
}

pub(crate) fn header_entry(input: &str) -> IResult<&str, (&str, &str)> {
    map(tuple((
        recognize(many0(alt((tag("-"), tag("_"), alpha1, alphanumeric1)))),
        tag(":"),
        take_until("\r\n")
    )), |(key, _, value): (&str, _, &str)| (key.trim(), value.trim()))(input)
}

pub(crate) fn headers(input: &str) -> IResult<&str, HashMap<String, String>> {
    map(many0(tuple((header_entry, tag("\r\n")))),
        |header_vec: Vec<((&str, &str), _)>| {
            let mut result = HashMap::new();
            for ((key, value), _) in header_vec {
                result.insert(key.to_owned(), value.to_owned());
            }
            result
        })(input)
}

pub(crate) fn parse(input: &str) -> IResult<&str, Request> {
    let except_body = tuple((method, tag(" "), uri, tag(" "), http_version, tag("\r\n"), headers, tag("\r\n")))(input);
    match except_body {
        Ok((rest, (method, _, uri, _, http_version, _, headers, _))) => {
            let content_length: Option<&String> = headers.get("Content-Length");
            match content_length {
                Some(content_length) => {
                    let length = usize::from_str(content_length.as_str()).unwrap();
                    let content_body = take::<_, &str, ()>(length)(rest);
                    match content_body {
                        Ok((rest, body)) => {
                            Ok((rest, Request {
                                method,
                                uri,
                                headers,
                                body: body.to_string(),
                            }))
                        }
                        Err(_) => Err(nom::Err::Error((input, nom::error::ErrorKind::Eof)))
                    }
                }
                None => Ok((rest, Request {
                    method,
                    uri,
                    headers,
                    body: "".to_string(),
                }))
            }
        }
        Err(_) => Err(nom::Err::Error((input, nom::error::ErrorKind::Eof)))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{method, uri, headers, header_entry, parse};
    use crate::http::request::Method;

    #[test]
    fn parse_method() {
        assert_eq!(method("GET").unwrap().1, Method::GET);
        assert_eq!(method("POST").unwrap().1, Method::POST);
    }

    #[test]
    fn parse_abs_path() {
        assert_eq!(uri("/ ").unwrap().1.path, "/");
        assert_eq!(uri("asdf?id=1 ").unwrap().1.path, "asdf");
        assert_eq!(uri("asdf?id=1 ").unwrap().1.query.get("id").unwrap(), "1");
        assert_eq!(uri("asdf/gh?id=1 ").unwrap().1.path, "asdf/gh");
        assert_eq!(uri("asdf/gh?id=1&faq=0 ").unwrap().1.query.get("faq").unwrap(), "0");
        assert_eq!(uri("/?faq=1&shit=2&rubbish=&fuck= ").unwrap().1.query.get("rubbish").unwrap(), "");
        assert_eq!(uri("/?faq=1&shit=2&rubbish=&fuck= ").unwrap().1.path, "/");
        assert_eq!(uri("/?faq=1&shit=2&rubbish=&fuck= ").unwrap().0, " ");
    }

    #[test]
    fn parse_header() {
        let (key, value) = header_entry("Content-Type: application/json\n").unwrap().1;
        assert_eq!(value, "application/json");
    }

    #[test]
    fn parse_headers() {
        let content = "Host: 127.0.0.1:8000
Content-Type: application/json
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTaGFuZ0hhaSBVbml2ZXJpc3R5IiwiZXhwIjoxNTY3NzMwMDAwLCJ1c2VybmFtZSI6IjE3MDAxMjM0In0.OaSOhr9rzwfv3NU1z32cso2nfZeu0y7wRP0Qn9pwfaq
User-Agent: PostmanRuntime/7.16.3
Accept: */*
Cache-Control: no-cache
Postman-Token: 9487479c-d98d-4862-8bf1-82e68efaaf00,12f4e67e-845c-4e33-b33b-c3c799c3c1fe
Host: 127.0.0.1:8000
Accept-Encoding: gzip, deflate
Connection: keep-alive
cache-control: no-cache

faq
";
        assert_eq!(headers(content).unwrap().1.get("Content-Type"), Some(&("application/json".to_string())));
        assert_eq!(headers(content).unwrap().1.get("Connection"), Some(&("keep-alive".to_string())));
    }

    #[test]
    fn parse_all() {
        let content = "POST /faq HTTP/1.1
Host: 127.0.0.1:8000
User-Agent: PostmanRuntime/7.16.3
Accept: */*
Cache-Control: no-cache
Postman-Token: 9487479c-d98d-4862-8bf1-82e68efaaf00,f3955246-0450-4a78-955b-a8d12cce9231
Host: 127.0.0.1:8000
Accept-Encoding: gzip, deflate
Connection: keep-alive
cache-control: no-cache
Content-Length: 12

asdfasdfasdf";
        let (_rest, context) = parse(content).unwrap();
        assert_eq!(context.body, "asdfasdfasdf");
        assert_eq!(context.uri.path, "/faq");
    }
}
