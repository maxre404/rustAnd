// use mosq::Mosquitto;
// use std::str::from_utf8;
extern crate futures;
extern crate paho_mqtt as mqtt;

// The topics to which we subscribe.
// const QOS: &[i32] = &[1, 1];
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
fn main() {
    println!("Hello, world!");
   // let result= get_ip();
    init_socket();
    // init_udp();
    // init_mqtt()
}

#[no_mangle]
#[allow(non_snake_case)]
fn get_ip()-> Result<(), Box<dyn std::error::Error>>{
    let  res = reqwest::blocking::get("https://raw.githubusercontent.com/maxre404/openApi/master/check.json")?.text()?;
    println!("数据请求成功了哦");
    // logPirnt("Status: {}", res.status());
    // println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    println!("++++++++++++++good luch++++++++++++++++");
    println!("result{}",res);
    // println!("\n\nDone.");
    Ok(())
}
fn init_socket(){
    let mut stream = TcpStream::connect("192.168.1.68:8888")
        .expect("Couldn't connect to the server...");
    // stream.set_nonblocking(true).expect("set_nonblocking call failed");
    loop {
        // stream.read
        // println!("head is:{}",test);
        let mut head=[0 as u8; 1];
        stream.read(&mut head).unwrap();
        if head[0]==b'|' {
            let mut length=[0 ;4];
            stream.read(&mut length).unwrap();
           let len= i8_toi32(length);
            let mut body=vec![Default::default(); len as usize];
            let mut end=[0;2]; //结尾是两个&&  换成u8是36 36
            stream.read(&mut body).unwrap();
            stream.read(&mut end).unwrap();
            for index in 0..len{
                body[index as usize]^=255
            }
            let mut pos=0;
           let cmd= get_int(&body,&pos);
            println!(".....................收到cmd:..........{}.......................",cmd);
            pos+=4;
            match cmd {
                101=>{
                    let bb= get_int(&body,&pos);
                    println!("101 协议值{}",bb);
                    // stream.write(b"Nice");
                }
                102=>{
                    let msg_len=get_int(&body,&pos);
                    pos+=4;
                    let msg_buf=get_buf(&body,&pos,msg_len);
                   let msg = from_utf8(msg_buf.as_slice()).unwrap();
                    println!(" 来自服务器消息:{}++++++++++++++++++",msg);
                    stream.write(b"client send hello").unwrap();

                }
                _ => {
                    println!("未知协议")
                }
            }

        }
    }
}
fn get_int(buf: &[u8],position:&i32)->i32{
    let mut int_array=[0 as u8;4];
    for index in 0..4{
        let pos=index as i32+ *position;
        int_array[index as usize]=buf[pos as usize]
    }
    i8_toi32(int_array)
}
fn get_buf<'a>(buf: &[u8], position:&i32, len:i32) -> Vec<u8> {
    let end=len as usize;
    let mut data=vec![Default::default(); end ];

    for i in 0..end {
        let tmp=position+(i as i32);
        data[i]=buf[tmp as usize]
    }
    return data;
}
fn i8_toi32(v: [u8; 4]) -> i32 {
    if v.len() < 4 {
        return 0
    }
    unsafe {
        let i32_ptr: *const i32 = v.as_ptr() as *const i32;
        return *i32_ptr;
    }
    // return 0
}

