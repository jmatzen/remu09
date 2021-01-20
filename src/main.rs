
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use hex::FromHex;

struct Machine {
    reg_a: u8,
    reg_b: u8,
    reg_cc: u8,
    reg_dp: u8,
    reg_x: u16,
    reg_y: u16,
    reg_s: u16,
    reg_d: u16,
    reg_pc: u16,
    memory: Vec<u8>
}

impl Default for Machine {
    fn default()->Self {
        Self {
            reg_a: 0,
            reg_b: 0,
            reg_cc: 0,
            reg_dp: 0,
            reg_x: 0,
            reg_y: 0,
            reg_s: 0,
            reg_d: 0,
            reg_pc: 0,
            memory: vec![0; 1024*512],
        }
    }
}

impl Machine {
    fn new()->Self {
        Machine {
            ..Default::default()
        }
    }

    fn run() {

    }

    fn load_hex<P:AsRef<Path>>(&mut self, filename: P) -> &mut Machine {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                    self.load_hex_line(ip);
                }
            }
        }
        self
    }

    fn load_hex_line(&mut self, line: String) -> &mut Self {
        let start_code = &line[0..1];
        let byte_count = &line[1..3];
        let address = &line[3..7];
        let record_type = &line[7..9];
        let data = &line[9..line.len()-2];
        println!("start={} byte_count={} address={} record_type={} data={}",
                 start_code,
                 hex::decode(byte_count).unwrap()[0],
                 to_short(hex::decode(address).unwrap()),
                 record_type,
                 data);
        self
    }
}
fn main() {
    println!("Hello, world!");

    let mut machine = Box::new(Machine { ..Default::default() });
    machine.load_hex("as9/ExBasROM.hex");

}

fn to_short(vec: AsRef<[u8]>) {

}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}