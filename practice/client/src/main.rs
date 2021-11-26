use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::{io, str};

fn main() -> io::Result<()> {
    // 创建 TcpStream 连接目标服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    for _ in 0..100 {
        let mut input = String::new();
        // 从命令行读取输入的内容并通过流传送到服务器端
        io::stdin().read_line(&mut input).expect("failed to read!");
        stream.write(input.as_bytes()).expect("failed to write!");

        // 创建 BufReader 连接
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        // 从 TcpStream 中读取流传递的信息
        reader
            .read_until(b'\n', &mut buffer)
            .expect("failed to read into buffer!");

        // 将字节数组转为字符串输出
        println!("sever return message: {}", str::from_utf8(&buffer).unwrap());
    }
    Ok(())
}
