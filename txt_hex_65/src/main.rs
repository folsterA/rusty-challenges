use std::io;

fn text_to_hex(text: &str) -> String {
    // transform into bytes
    let bytes = &text.as_bytes();

    // build bytes as hex string
    let mut x: Vec<String> = Vec::new();
    for byte in bytes.iter()
    {
        let s = format!("{:#x}", byte);
        x.push(s);
    }
    x.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text()
    {
        let text = "hello";
        let hex_truth = "0x68 0x65 0x6c 0x6c 0x6f";
        let hex = &text_to_hex(text);
        assert_eq!(hex, hex_truth);
    }

    #[test]
    fn hex()
    {
        let text = "1234567890";
        let hex_truth = "0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x30";
        let hex = &text_to_hex(text);
        assert_eq!(hex, hex_truth);
    }

    #[test]
    fn whitespace()
    {
        let text = "w h i \t e \n s_p a c e";
        let hex_truth = "0x77 0x20 0x68 0x20 0x69 0x20 0x9 0x20 0x65 0x20 0xa 0x20 0x73 0x5f 0x70 0x20 0x61 0x20 0x63 0x20 0x65";
        let hex = &text_to_hex(text);
        assert_eq!(hex, hex_truth);
    }
}

fn main() {
    // create buffer
    let mut buffer = String::new();
    
    // read into buffer
    let stdin = io::stdin();
    let _bytes_read = stdin.read_line(&mut buffer).unwrap();
    buffer.pop(); // remove 'CR' from the end.
    buffer.pop(); // remove 'LF' from the end.
    
    // build bytes as hex string
    let hex = text_to_hex(&buffer);
    
    // output hex string
    println!("in hexedecimal is\n{hex}");
}
