use std::convert::TryInto;
use hacspec_lib::*;
use hacspec_aes::*;

fn format_func(a: &ByteSeq, n: &ByteSeq, p: &ByteSeq, t: u8, alen: u64, nlen: u8, plen: u64) -> ByteSeq {
    let mut r = 0;
    let mut tmp = 10;

    if alen < 0x800000 {
        tmp = 2;
    } else if alen < 0x100000000 {
        tmp = 6;
    }

    r = r + ((tmp+alen+15)/16)+((plen+15)/16); // ceiling operation used
    let mut b = ByteSeq::new((16*(r+1)).try_into().unwrap());

    // creation of b(0)
    let qlen: u8 = 15 - nlen;
    let mut flags: u8 = 0;

    if alen > 0 {
        flags = 0x40;
    }

    flags = flags | (((t-2)/2) << 3);
    flags = flags | (qlen-1);
    let f = ByteSeq::from_public_slice(&[flags]);

    b = b.set_exact_chunk(1, 0, &f);

    for i in 0..nlen {
        let tmp2 = n.get_exact_chunk(1, i.into());
        b = b.set_exact_chunk(1, (i+1).into(), &tmp2);
    }

    let andy: u64 = 255; // 0xFF
    let zero = ByteSeq::from_public_slice(&[0x0]);
    let mut copy: u64 = plen;

    for i in (16-qlen..16).rev() {
        let smth = ByteSeq::from_public_slice(&[(copy & andy) as u8]);
        b = b.set_exact_chunk(1, i.into(), &smth);
        copy = copy >> 8;
    }

    // creation of b(1) to b(u)
    let x = ByteSeq::from_public_slice(&[0xff]);
    let y = ByteSeq::from_public_slice(&[0xfe]);
    let mut k = 16;
    let mut copy2 = alen;

    if alen >= 0x800000 {
        if alen < 0x100000000 {
            b = b.set_exact_chunk(1, 16, &x);
            b = b.set_exact_chunk(1, 17, &y);
        } else {
            b = b.set_exact_chunk(1, 16, &x);
            b = b.set_exact_chunk(1, 17, &x);
        }

        k = k + 2;
    }

    for i in (k..k+tmp).rev() {
        let smth2 = ByteSeq::from_public_slice(&[(copy2 & andy) as u8]);
        b = b.set_exact_chunk(1, i.try_into().unwrap(), &smth2);
        copy2 = copy2 >> 8;
    }

    k = k + tmp;

    for i in 0..alen {
        let tmp2 = a.get_exact_chunk(1, i.try_into().unwrap());
        b = b.set_exact_chunk(1, (i+k).try_into().unwrap(), &tmp2);
    }

    k = k + alen-1;

    while k % 16 != 15 {
        // add zero padding for Associated Data
        k = k + 1;
        b = b.set_exact_chunk(1, k.try_into().unwrap(), &zero);
    }

    // creation of b(u+1) to b(r)
    for i in 0..plen {
        let tmp2 = p.get_exact_chunk(1, i.try_into().unwrap());
        b = b.set_exact_chunk(1, (i+k+1).try_into().unwrap(), &tmp2);
    }

    k = k + plen;

    while k % 16 != 15 {
        // add zero padding for Payload
        k = k + 1;
        b = b.set_exact_chunk(1, k.try_into().unwrap(), &zero);
    }

    b
}

fn get_t(b: &ByteSeq, key: Key128, num: usize) -> ByteSeq {
    let b0 = b.get_exact_chunk(16, 0);
    let bloc = Block::from_seq(&b0);
    let mut y_curr = aes128_encrypt_block(key, bloc);

    for i in 1..b.len()/16 {
        let mut b_curr = Block::from_seq(&b.get_exact_chunk(16, i));
        b_curr = y_curr ^ b_curr;
        y_curr = aes128_encrypt_block(key, b_curr);
    }

    ByteSeq::from_seq(&(y_curr.slice(0, num)))
}

fn counter_func(n: &ByteSeq, nlen: u64, m: u64) -> ByteSeq {
    let qlen: u8 = (15 - nlen).try_into().unwrap();
    let flag = ByteSeq::from_public_slice(&[qlen-1]);
    let mut ctr = ByteSeq::new((16 * (m+1)).try_into().unwrap());
    let high: u64 = 255; // 0xFF

    for i in 0..m+1 {
        let k = 16*i;
        ctr = ctr.set_exact_chunk(1, k.try_into().unwrap(), &flag);

        for j in 0..nlen {
            let tmp2 = n.get_exact_chunk(1, j.try_into().unwrap());
            ctr = ctr.set_exact_chunk(1, (k+j+1).try_into().unwrap(), &tmp2);
        }

        let mut icopy = i;

        for x in (k+nlen+1..k+16).rev() {
            let curr = ByteSeq::from_public_slice(&[(icopy & high) as u8]);
            ctr = ctr.set_exact_chunk(1, x.try_into().unwrap(), &curr);
            icopy = icopy >> 8;
        }
    }

    ctr
}

fn ctr_cipher(ctr: &ByteSeq, key: Key128, m: u64) -> (ByteSeq, ByteSeq) {
    let ctr_zero = Block::from_seq(&ctr.get_exact_chunk(16, 0));
    let s0 = ByteSeq::from_seq(&aes128_encrypt_block(key, ctr_zero));
    let mut s = ByteSeq::new((16*m).try_into().unwrap());

    for i in 1..m+1 {
        let ctr_block = Block::from_seq(&ctr.get_exact_chunk(16, i.try_into().unwrap()));
        let s_curr = aes128_encrypt_block(key, ctr_block);
        let seq_s = ByteSeq::from_seq(&s_curr);
        s = s.set_exact_chunk(16, (i-1).try_into().unwrap(), &seq_s);
    }

    (s0, s)
}

pub fn encrypt_ccm(a: ByteSeq, n: ByteSeq, pay: ByteSeq, key: Key128, tlen: u8, alen: u64, nlen: u8, plen: u64) -> ByteSeq {
    let b = format_func(&a, &n, &pay, tlen, alen, nlen, plen); // step 1
    let t = get_t(&b, key, tlen.into()); // steps 2 to 4

    let m = (plen+15)/16; // round up
    let counter = counter_func(&n, nlen.into(), m.into());
    let (s0, s) = ctr_cipher(&counter, key, m.into());

    let cipherlen = t.len()+pay.len(); let pl = pay.len();
    let mut ciphertext = ByteSeq::new(cipherlen);

    let pay_xor = pay ^ s.get_exact_chunk(plen.try_into().unwrap(), 0);
    ciphertext = ciphertext.set_exact_chunk(plen.try_into().unwrap(), 0, &pay_xor);

    let t_xor = t ^ s0.get_exact_chunk(tlen.into(), 0);

    for i in pl..cipherlen {
        let curr_chunk = t_xor.get_exact_chunk(1, i-pl);
        ciphertext = ciphertext.set_exact_chunk(1, i, &curr_chunk);
    }

    ciphertext
}

pub fn decrypt_ccm(adata: ByteSeq, nonce: ByteSeq, ciph: ByteSeq, clen: u8, key: Key128, tlen: u8, nlen: u8) -> ByteSeq {
    if clen > tlen {
        let m = (clen-tlen+15) / 16;
        let counter = counter_func(&nonce, nlen.into(), m.into());
        let (s0, s) = ctr_cipher(&counter, key, m.into());

        let x = clen-tlen;
        let p = ciph.get_exact_chunk(x.into(), 0) ^ s.get_exact_chunk(x.into(), 0);
        p
    } else {
        ByteSeq::from_public_slice(&[]) // TODO: Return "Invalid" instead
    }
}