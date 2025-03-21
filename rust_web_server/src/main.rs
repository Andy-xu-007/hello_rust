use rust_web_server::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // handle_connection_slow_request(stream);

        // 线程
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });

        // 线程池
        pool.execute(|| {
            handle_connection_slow_request(stream);
        });

        // 连接成功打印
        // println!("Connection established!");
    }

    println!("Shutting sown.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // 打印请求内容
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // 友好debug，打印请求数据
    // println!("Request: {http_request:#?}");

    // 第一个unwrap处理Option，在迭代没有选项时停止程序
    // 第二个unwrap处理Result，与上面增加的mp中的unwrap有相同的效果
    // 加 next() 是只看请求的第一行
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // 验证请求并有选择的进行响应
    if request_line == "GET / HTTP/1.1" {
        // 对浏览器请求所响应的内容
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        // CRLF 就是回车和换行，即\r\n
        let response = format!("{status_line}\r\nConnect-length: {length}\r\n\r\n{contents}");
        // unwrap是简单操作，实际需要添加错误处理
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }

}

// 少量代码重构
// 合并if .. else .. 中一些重复的代码
fn handle_connection_new(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

// 模拟慢请求
fn handle_connection_slow_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
