use std::{
    env, fs,
    io::{self, Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let instructions: Vec<u8> = fs::read(&args[1]).unwrap();

    interpret(instructions, &mut io::stdin(), &mut io::stdout());
}

fn interpret(instructions: Vec<u8>, reader: &mut impl Read, writer: &mut impl Write) {
    let mut data: Vec<u8> = vec![0; instructions.len()];
    let mut dp: usize = 0;
    let mut ip: usize = 0;

    while ip < instructions.len() {
        match instructions[ip] {
            b'>' => dp += 1,
            b'<' => dp -= 1,
            b'+' => data[dp] += 1,
            b'-' => data[dp] -= 1,
            b'.' => {
                let _ = writer.write(&[data[dp]]);
            }
            b',' => {
                let input_bytes: &mut [u8; 1] = &mut [0; 1];
                reader.read_exact(input_bytes).unwrap();
                data[dp] = input_bytes[0];
            }
            b'[' => {
                if data[dp] == 0 {
                    let mut scopes: i32 = 1;
                    while scopes > 0 {
                        ip += 1;
                        if instructions[ip] == b'[' {
                            scopes += 1;
                        } else if instructions[ip] == b']' {
                            scopes -= 1;
                        }
                    }
                }
            }
            b']' => {
                if data[dp] != 0 {
                    let mut scopes: i32 = 1;
                    while scopes > 0 {
                        ip -= 1;
                        if instructions[ip] == b']' {
                            scopes += 1;
                        } else if instructions[ip] == b'[' {
                            scopes -= 1;
                        }
                    }
                }
            }
            _ => (),
        };
        ip += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_hello_world() {
        let program: String = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();
        let mut reader: &[u8] = &[];
        let mut writer: BufWriter<Vec<u8>> = BufWriter::new(Vec::new());

        interpret(program.into_bytes(), &mut reader, &mut writer);

        let bytes: Vec<u8> = writer.into_inner().unwrap();
        let output: String = String::from_utf8(bytes).unwrap();

        assert_eq!("Hello World!\n", output);
    }
}
