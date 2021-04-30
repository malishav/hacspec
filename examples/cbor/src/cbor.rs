#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use backtrace::Backtrace;

// Import hacspec and all needed definitions.
use hacspec_lib::*;

bytes!(Bytes1, 1);
bytes!(Bytes2, 2);
bytes!(Bytes5, 5);
bytes!(Bytes32, 32);

#[derive(Debug)]
pub enum error_code {
   not_implemented
}

pub use error_code::*;

pub type Res<T> = Result<T, error_code>;
pub fn err<T>(x: error_code) -> Res<T> {
    let bt = Backtrace::new();
    println!("{:?}", bt);
    Err(x)
}

pub type Bytes = ByteSeq;
pub fn empty() -> Bytes {
    return Seq::new(0);
}

pub fn zeros(u: usize) -> Bytes {
    return Seq::new(u);
}

pub fn bytes<T: SeqTrait<U8>>(x: &T) -> Bytes {
    return Seq::from_seq(x);
}

pub fn bytes1(x: u8) -> Bytes {
    bytes(&Bytes1([U8(x)]))
}

pub fn bytes2(x: u8, y: u8) -> Bytes {
    bytes(&Bytes2([U8(x), U8(y)]))
}

pub fn bytes5(x0: u8, x1: u8, x2:u8, x3:u8, x4:u8) -> Bytes {
    bytes(&Bytes5([U8(x0), U8(x1), U8(x2), U8(x3), U8(x4)]))
}

pub fn bytes32(x0: u8, x1: u8, x2: u8, x3: u8, x4: u8, x5: u8, x6: u8, x7: u8, x8: u8, x9: u8, x10: u8, x11: u8, x12: u8, x13 :u8, x14 : u8, x15 : u8, x16 : u8, x17 : u8,x18 : u8, x19 : u8, x20 : u8, x21 : u8, x22 : u8, x23 : u8, x24 : u8, x25 : u8, x26 : u8, x27 : u8, x28 : u8, x29 : u8, x30 : u8, x31 : u8) -> Bytes {
    bytes(&Bytes32([U8(x0), U8(x1), U8(x2), U8(x3), U8(x4), U8(x5), U8(x6), U8(x7), U8(x8), U8(x9), U8(x10), U8(x11), U8(x12), U8(x13), U8(x14), U8(x15), U8(x16), U8(x17), U8(x18), U8(x19), U8(x20), U8(x21), U8(x22), U8(x23), U8(x24), U8(x25), U8(x26), U8(x27), U8(x28), U8(x29), U8(x30), U8(x31)]))
}

fn encode_null() -> Res<Bytes> {
    Ok(bytes1(0xf6))
}

fn encode_unsigned_u8(num: u8) -> Res<Bytes> {
    if num < 24 {
        Ok(bytes1(num))
    } else {
        Ok(bytes2(0x18, num))
    }
}

fn encode_bytes(bytes: Bytes) -> Res<Bytes> {
    let len = bytes.len();

    if len <= 23 {
        let buf = bytes1(0x40 | len as u8);
        Ok(buf.concat(&bytes))
    } else if bytes.len() > 23 && len <= 255 {
        let buf = bytes2(0x58, len as u8);
        Ok(buf.concat(&bytes))
    } else {
        Err(not_implemented)
    }
}

fn encode_array(num_elems: u8) -> Res<Bytes> {
    if num_elems <= 15 {
        Ok(bytes1(0x80 | num_elems))
    } else {
        Err(not_implemented)
    }
}

fn encode_map(num_elems: u8) -> Res<Bytes> {
    if num_elems <= 15 {
        Ok(bytes1(0xA0 | num_elems))
    } else {
        Err(not_implemented)
    }
}

fn main() {
}
