use std::fs;
use async_std::prelude::*;
use std::time::Duration;
use async_std::{io::{Read, Write}, task};
use futures::stream::StreamExt;
use async_std::net::{TcpListener,TcpStream};
use async_std::task::spawn;
use std::marker::Unpin;


#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    listener
    .incoming()
    .for_each_concurrent(None , |tcpstream| async move{
        let tcpstream = tcpstream.unwrap();
        spawn(handle_connection(tcpstream));
    }).await;
}

async fn handle_connection(mut stream:impl Read + Write + Unpin){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).await.unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    //处理HTTP协议头
    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    }else if buffer.starts_with(sleep){
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).await.unwrap();
    // 使用flush刷新缓冲区
    stream.flush().await.unwrap();
}