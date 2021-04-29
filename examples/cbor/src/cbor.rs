#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::cmp::Ordering;

// Import hacspec and all needed definitions.
use hacspec_lib::*;

bytes!(Bytes1, 1);
bytes!(Bytes2, 2);
bytes!(Bytes5, 5);
bytes!(Bytes32, 32);

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

trait Unsigned {
    fn encode(&self) -> Bytes
        where
            Self: Sized;
}

impl Unsigned for u8 {
    fn encode(&self) -> Bytes {
        match 24.cmp(self) {
            Ordering::Greater => bytes1(*self),
            Ordering::Equal => bytes2(0x18, *self),
            Ordering::Less => bytes2(0x18, *self),
        }
    }
}

fn encode_unsigned<U>(elem: U) -> Bytes
where
    U: Unsigned,
{
    elem.encode()
}

fn encode_null() -> Bytes {
    bytes1(0xf6)
}

fn encode_bytes(bytes: Bytes) -> Bytes {
    let len = bytes.len();
    let buf;

    if len <= 23 {
        buf = bytes1(0x40 | len as u8);
    } else if bytes.len() > 23 && len <= 255 {
        buf = bytes2(0x58, len as u8);
    } else {
        unimplemented!()
    }

    buf.concat(&bytes)
}

fn encode_array(num_elems: u8) -> Bytes {
    match 15.cmp(&num_elems) {
        Ordering::Less => unimplemented!(),
        Ordering::Equal => bytes1(0x80 | num_elems),
        Ordering::Greater => bytes1(0x80 | num_elems),
    }
}

fn encode_map(num_elems: u8) -> Bytes {
    match 15.cmp(&num_elems) {
        Ordering::Less => unimplemented!(),
        Ordering::Equal => bytes1(0xA0 | num_elems),
        Ordering::Greater => bytes1(0xA0 | num_elems),
    }
}

fn main() {
    let x = 23u8;
    let y = 100u8;

    println!("Byte string for 24 {:?}", encode_unsigned(x));
    println!("Byte string for 100 {:?}", encode_unsigned(y));
    println!("Null string {:?}", encode_null());
    println!("Array 3 elems {:?}", encode_array(15));
    println!("Bytes [1,2,3,4,5..25] {:?}", encode_bytes(bytes32(0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55)));
}
