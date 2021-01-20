
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use hex::FromHex;
use std::mem::size_of;
use std::ops::Shl;
use std::borrow::Borrow;
use num::traits::{Unsigned, PrimInt};
use num::CheckedSub;

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
const VEC_RESET : u16 = 0xfffe;
const OP_LDS : u16 = 0x10ce;

impl Machine {

    fn new()->Self {
        Machine {
            ..Default::default()
        }
    }

    fn reset(&mut self) -> &mut Self {
        self.reg_pc = self.load(VEC_RESET);
        self.exec();
        self
    }

    fn exec(&mut self)->&mut Self {
        let opcode : u16 = self.load(self.reg_pc);
        match opcode >> 8  {
            0x10 => {
                match opcode {
                    OP_LDS => {
                        self.reg_s = self.load(self.reg_pc+2);
                    }
                    _ => {}
                }
            }
            _ => panic!("unexpected opcode")
        }
        self
    }

    fn load<T: PrimInt + Unsigned + PartialOrd>(&self, addr: u16) -> T {
        let l = size_of::<T>();
        let mut res = T::zero();
        for i in 0..size_of::<T>() {
            if  size_of::<T>() >=2 {
                res = res.unsigned_shl(8);
            }

            res = res
                .bitor(T::from(self.memory[addr as usize + i]).unwrap());
        }
        res
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
        let address = to_short(&hex::decode(&line[3..7]).unwrap()) as usize;
        let record_type = &line[7..9];
        let data = &line[9..line.len()-2];
        let byte_count = hex::decode(byte_count).unwrap()[0];

        println!("start={} byte_count={} address={} record_type={} data={}",
                 start_code,
                 byte_count,
                 address,
                 record_type,
                 data);
        match &hex::decode(record_type).unwrap()[0] {
            0 => {
                for i in 0..byte_count {
                    let offset = 2 * i as usize;
                    let byte = hex::decode(&data[offset..offset+2]).unwrap()[0];
                    self.memory[address + i as usize] = byte;
                }
            },
            _ => ()
        }
        self
    }
}
fn main() {
    println!("Hello, world!");

    let mut machine = Box::new(Machine { ..Default::default() });
    machine
        .load_hex("ExBasROM.hex")
        .reset();

}

fn to_short(vec: &Vec<u8>) -> u16 {
    ((vec[0] as u16) << 8) | (vec[1] as u16)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}