use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect(); // skip the program name argument
    let mut output = String::new();

    let mut print_newline = true; // print a newline by default
    let mut escape_chars = false; // disable escape characters by default

    for arg in &args {
        if arg == "-n" {
            print_newline = false;
        } else if arg == "-E" {
            escape_chars = false;
        } else if arg == "-e" {
            escape_chars = true;
        } else {
            output += arg;
            output += " ";
        }
    }

    if escape_chars {
        output = interpret_escape_chars(&output);
    }

    if print_newline {
        println!("{}", output);
    } else {
        print!("{}", output);
    }
}

fn interpret_escape_chars(string: &str) -> String {
    let mut result = String::new();
    let mut chars = string.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('a') => result.push('\x07'), // audible bell
                Some('b') => result.push('\x08'), // backspace
                Some('c') => break, // end output
                Some('e') => result.push('\x1B'), // escape
                Some('f') => result.push('\x0C'), // form feed
                Some('n') => result.push('\n'), // newline
                Some('r') => result.push('\r'), // carriage return
                Some('t') => result.push('\t'), // horizontal tab
                Some('v') => result.push('\x0B'), // vertical tab
                Some('x') => { // hexadecimal byte value
                    let byte = parse_hex_byte(&mut chars);
                    result.push(byte);
                }
                Some(ch) => result.push(ch), // other character
                None => result.push('\\'), // backslash at end of string
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn parse_hex_byte(chars: &mut std::str::Chars) -> char {
    let mut byte = String::new();
    byte.push(chars.next().unwrap_or('0'));
    byte.push(chars.next().unwrap_or('0'));
    byte.push(chars.next().unwrap_or('0'));

    u8::from_str_radix(&byte, 16)
        .ok()
        .map(|byte| byte as char)
        .unwrap_or('?')
}
