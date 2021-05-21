#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_parens)]

// A module that for the formatting code needed by TLS 1.3
use crate::cryptolib::*;

// Import hacspec and all needed definitions.
use hacspec_lib::*;


//pub const label_iv: Bytes2 = Bytes2(secret_bytes!([105, 118]));
//pub const label_key: Bytes3 = Bytes3(secret_bytes!([107, 101, 121]));

pub fn check(b:bool) -> Res<()> {
    if b {Ok(())}
    else {Err(parse_failed)}
}

pub fn eq1(b1: U8, b2: U8) -> bool {
    b1.declassify() == b2.declassify()
}
pub fn check_eq1(b1: U8, b2: U8) -> Res<()> {
    if eq1(b1,b2) {Ok(())}
    else {Err(parse_failed)}
}

pub fn eq(b1: &Bytes, b2: &Bytes) -> bool {
    if b1.len() != b2.len() {
        false
    } else {
        for i in 0..b1.len() {
            if !eq1(b1[i],b2[i]) {return false;};
        }
        true
    }
}

pub fn check_eq(b1: &Bytes, b2: &Bytes) -> Res<()> {
    if b1.len() != b2.len() {
        err(parse_failed)
    } else {
        for i in 0..b1.len() {
            check_eq1(b1[i],b2[i])?;
        }
        Ok(())
    }
}

pub fn check_mem(b1: &Bytes, b2: &Bytes) -> Res<()> {
    if b2.len() % b1.len() != 0 {
        err(parse_failed)
    } else {
        for i in 0..(b2.len() / b1.len()) {
            let snip = b2.slice_range(i * b1.len()..(i + 1) * b1.len());
            if eq(b1, &snip) {return Ok(());}
            }
        err(parse_failed)
    }
}

pub fn lbytes1(b: &Bytes) -> Res<Bytes> {
    let len = b.len();
    if len >= 256 {
        Err(payload_too_long)
    } else {
        let mut lenb = Seq::new(1);
        lenb[0] = U8(len as u8);
        Ok(lenb.concat(b))
    }
}

pub fn lbytes2(b: &Bytes) -> Res<Bytes> {
    let len = b.len();
    if len >= 65536 {
        Err(payload_too_long)
    } else {
        let len: u16 = len as u16;
        let lenb = Seq::from_seq(&U16_to_be_bytes(U16(len)));
        Ok(lenb.concat(b))
    }
}

pub fn lbytes3(b: &Bytes) -> Res<Bytes> {
    let len = b.len();
    if len >= 16777216 {
        Err(payload_too_long)
    } else {
        let lenb = U32_to_be_bytes(U32(len as u32));
        Ok(lenb.slice_range(1..4).concat(b))
    }
}

pub fn check_lbytes1(b: &Bytes) -> Res<usize> {
    if b.len() < 1 {
        err(parse_failed)
    } else {
        let l = (b[0] as U8).declassify() as usize;
        if b.len() - 1 < l {
            err(parse_failed)
        } else {
            Ok(l)
        }
    }
}

pub fn check_lbytes2(b: &Bytes) -> Res<usize> {
    if b.len() < 2 {
        err(parse_failed)
    } else {
        let l0 = (b[0] as U8).declassify() as usize;
        let l1 = (b[1] as U8).declassify() as usize;
        let l = l0 * 256 + l1;
        if b.len() - 2 < l as usize {
            err(parse_failed)
        } else {
            Ok(l)
        }
    }
}

pub fn check_lbytes3(b: &Bytes) -> Res<usize> {
    if b.len() < 3 {
        err(parse_failed)
    } else {
        let l0 = (b[0] as U8).declassify() as usize;
        let l1 = (b[1] as U8).declassify() as usize;
        let l2 = (b[2] as U8).declassify() as usize;
        let l = l0 * 65536 + l1 * 256 + l2;
        if b.len() - 3 < l {
            err(parse_failed)
        } else {
            Ok(l)
        }
    }
}

pub fn check_lbytes1_full(b: &Bytes) -> Res<()> {
    if check_lbytes1(b)? + 1 != b.len() {
        err(parse_failed)
    } else {
        Ok(())
    }
}

pub fn check_lbytes2_full(b: &Bytes) -> Res<()> {
    if check_lbytes2(b)? + 2 != b.len() {
        err(parse_failed)
    } else {
        Ok(())
    }
}

pub fn check_lbytes3_full(b: &Bytes) -> Res<()> {
    if check_lbytes3(b)? + 3 != b.len() {
        err(parse_failed)
    } else {
        Ok(())
    }
}


pub fn make_msg1(method: METHOD, corr:usize, suite:CIPHERSUITE, pk:&KEMPK, c_i:&Bytes, ad_1:&Bytes) -> Res<Bytes> {
    encode_null()                                                   // mandatory C_1
        .concat(encode_unsigned_u8((4 * method as u8 + corr) as u8))// METHOD_CORR
        .concat(encode_unsigned_u8(suite.0))                        // SINGLE SELECTED SUITE for now
        .concat(encode_bytes(pk))                                   // G_X
        .concat(encode_bytes(c_i))                                  // C_I
        .concat(encode_bytes(ad_1))                                 // AD_1
}

pub fn make_msg2(data2: &Bytes, ciphertext2: &Bytes) -> Res<Bytes> {
    Err(parse_failed)
}

pub fn make_info(aea: &AEADAlgorithm, th: &Bytes, len: usize) -> Res<Bytes> {
    Err(parse_failed)
}

pub fn make_plaintext(id: &Bytes, sg: &Bytes, ad: &Bytes) -> Res<Bytes> {
    Err(parse_failed)
}

pub fn check_msg1(m:&Bytes) -> Res<()> {
    Err(parse_failed)
}

pub fn parse_msg1(suite:CIPHERSUITE, msg: &Bytes) -> Res<(corr:usize, c_i: Bytes, pk_i: Bytes)> {
    Err(parse_failed)
}

pub fn make_data_2(c_i: &Bytes, pk_r: &Bytes, c_r: &Bytes) -> Res<Bytes> {
    Err(parse_failed)
}

pub fn make_aad(label: &Bytes, id_cred_r: &Bytes,th_2: &Bytes,cred_r: &Bytes, ad_2: &Bytes) -> Res<()> {
    Err(parse_failed)
}

#[derive(Clone, Copy, PartialEq)]
pub enum METHOD {
    SIG_SIG = 0,
    SIG_STATIC = 1,
    STATIC_SIG = 2,
    STATIC_STATIC = 3
}

#[derive(Clone, Copy, PartialEq)]
pub enum CIPHERSUITEID {
    SUITE_0 = 0, // find better reference to the suites, use of alg ids is too long
    SUITE_1 = 1,
    SUITE_2 = 2,
    SUITE_3 = 3,
    SUITE_4 = 4,
    SUITE_5 = 5
}

#[derive(Clone, Copy, PartialEq)]
pub struct CIPHERSUITE(
    pub CIPHERSUITEID,
    pub AEADAlgorithm,
    pub HashAlgorithm,
    pub KEMScheme,
    pub SignatureScheme
);


