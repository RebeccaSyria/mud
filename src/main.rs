use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:36201").unwrap();

    let addr = listener.local_addr()
        .expect("unable to get the local port?");

    println!("Listening on port {}", addr.port());

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => panic!(e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 16];
    if let Err(_) = stream.write("Hello from Rust\n".as_bytes()) {
        return;
    }
    println!("connection accepted");
    loop{
        if let Err(_) = stream.write("\n> ".as_bytes()){
            return;
        }
        if let Ok(read) = stream.read(&mut buffer){
            let s = String::from_utf8_lossy(&buffer);
            println!("{}",s);
            if read == 0{
                break;
            }
            let mut output = "";
            match s[0..read].trim(){
                "a" | "A"  => output = "Ayy\n",
                "b" => output = "bark\n",
                _ => output = &s[0..read],
            }
            if let Err(_) = stream.write_all(output.as_bytes()){
                    break;
                }
        }else{
            break;
        }
    }
    println!("disconnected")

}
