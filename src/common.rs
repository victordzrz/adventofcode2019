use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;
use std::vec::Vec;

pub fn get_input(filepath: &str, separator: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let mut buf_file = BufReader::new(&file);
    let mut in_string = String::new();
    let res = buf_file.read_to_string(&mut in_string);
    let mut output = Vec::new();
    match res {
        Ok(_) => {
            for element in in_string.split(separator) {
                output.push(String::from_str(element).unwrap());
            }
        }
        Err(_) => {
            println!("Couldn't read the string");
        }
    }

    return output;
}

pub fn to_int_vec(string_vec: Vec<String>) -> Vec<i64> {
    let mut int_vec = Vec::new();
    for element in string_vec {
        int_vec.push(element.parse::<i64>().unwrap());
    }
    return int_vec;
}
