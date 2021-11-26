use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{io, thread, time};

fn main() -> io::Result<()> {
    // 创建 TcpListener 监听服务器指定端口
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // 通过 Vec 存储连接线程
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("stream failed!");
        // 创建线程来处理 TcpStream 连接
        let handle = thread::spawn(move || {
            // 调用 handle_client() 来处理目标 TcpStream 连接
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });
        // 将该线程存进 Vec 中
        thread_vec.push(handle)
    }

    for handle in thread_vec {
        // 执行 handle 线程
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // 创建 buf 存储传送来的信息
    let mut buf = [0; 512];
    for _ in 0..1000 {
        // 读取流传过来的信息
        let bytes_read = stream.read(&mut buf)?;
        // 如果没有读取到信息就直接返回
        if bytes_read == 0 {
            return Ok(());
        }

        // 将读取到的信息按原路写回
        stream.write(&buf[..bytes_read])?;
        // 让该线程 sleep 1s
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
