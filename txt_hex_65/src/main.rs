use std::io;

fn main() {
    // create buffer
    let mut buffer = String::new();
    
    // read into buffer
    let stdin = io::stdin();
    let _bytes_read = stdin.read_line(&mut buffer).unwrap();
    buffer.pop(); // remove '\n' from the end.

    // transform into bytes
    let bytes = &buffer.clone().into_bytes();

    // build bytes as hex string
    let mut hex = String::new();
    for byte in bytes.iter()
    {
        let f = format!("{:#x} ", byte);
        hex += &f;
    }
 
    // output hex string
    println!("in hexedecimal is\n{hex}");
}
