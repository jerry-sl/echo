//使用std中tcp监听和流处理的依赖
use std::net::{TcpListener, TcpStream};
//处理多个tcp的连接需要后台线程
use std::thread;
// 流io处理
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    // 读取20个字节 处理后写会原来的流中
    loop {
        let mut read = [0; 1028];
        //判断读取的流数据是否正确并按情况处理
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    break;
                }
                //打印client的请求数据
                println!("Request: {}", String::from_utf8_lossy(&read[..]));
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn main() {
    // 服务端绑定ip和端口并监听
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 监听到客户端的连接请求，根据流的状态处理
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 启动后台线程处理客户的一个请求
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}