pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//https://course.rs/advance-practice1/web-server.html
pub mod server {
    use std::{
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    //建立链接
    pub fn start() {
        let tcp = "127.0.0.1:8900";
        // 监听地址: 127.0.0.1:7878
        let listener = TcpListener::bind(tcp).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("连接已确认");
            handle_connection(stream);
        }
    }

    //请求读取和响应
    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let status_line  = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("http_server/hello.html").unwrap();
        let length = contents.len();
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();

        println!("Request: {:#?}", http_request);
    }
}


