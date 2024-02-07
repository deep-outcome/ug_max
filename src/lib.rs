#![no_std]
//! UGLY-MAXIMAL font.
//!
//! Each character representation is defined as series of columns consisting of 5 bits
//! at maximum. Bits are ordered down-top and columns left-right. So 0 index bit of 0 index column
//! is top left corner. For example, for 0x1e = 0b11_110 which is 1ˢᵗ column of A this is `0`.

#[rustfmt::skip]
/// English alphabet. 26 letters.
pub const LETTERS: [&[u8]; 26] = 
[ 
 /*A*/   & [0x1e, 0x9, 0x9, 0x9, 0x1e], 
 /*B*/   & [0x1f, 0x15, 0x15, 0x15, 0xa],
 /*C*/   & [0xe, 0x11, 0x11, 0x11, 0x11],
 /*D*/   & [0x1f, 0x11, 0x11, 0x11, 0xe],
 /*E*/   & [0x1f, 0x15, 0x15, 0x15, 0x11],
 /*F*/   & [0x1f, 0x5, 0x5, 0x5, 0x1],
 /*G*/   & [0xe, 0x11, 0x15, 0x15, 0x9],
 /*H*/   & [0x1f, 0x4, 0x4, 0x4, 0x1f],
 /*I*/   & [0x1f],
 /*J*/   & [0xc, 0x10, 0x10, 0x10, 0xf],
 /*K*/   & [0x1f, 0x4, 0x4, 0xa, 0x11],
 /*L*/   & [0x1f, 0x10, 0x10, 0x10, 0x8],
 /*M*/   & [0x1f, 0x1, 0x6, 0x1, 0x1f],
 /*N*/   & [0x1f, 0x1, 0xe, 0x10, 0x1f],
 /*O*/   & [0xe, 0x11, 0x11, 0x11, 0xe],
 /*P*/   & [0x1f, 0x5, 0x5, 0x5, 0x2],
 /*Q*/   & [0xe, 0x11, 0x15, 0x9, 0x16],
 /*R*/   & [0x1f, 0x5, 0x5, 0xd, 0x16],
 /*S*/   & [0x16, 0x15, 0x15, 0x15, 0xd],
 /*T*/   & [0x1, 0x1, 0x1f, 0x1, 0x1],
 /*U*/   & [0xf, 0x10, 0x10, 0x10, 0xf],
 /*V*/   & [0x7, 0x8, 0x10, 0x8, 0x7],
 /*W*/   & [0xf, 0x10, 0x1f, 0x10, 0xf],
 /*X*/   & [0x11, 0xa, 0x4, 0xa, 0x11],
 /*Y*/   & [0x1, 0x2, 0x1c, 0x2, 0x1],
 /*Z*/   & [0x11, 0x19, 0x15, 0x13, 0x11],
];

#[rustfmt::skip]
/// From '0' to '9'.
pub const DIGITS: [[u8; 4]; 10] = 
[
/*0*/   [0x1f, 0x11, 0x11, 0x1f],
/*1*/   [0x4, 0x2, 0x1, 0x1f],
/*2*/   [0x1d, 0x15, 0x15, 0x17],
/*3*/   [0x15, 0x15, 0x15, 0x1f],
/*4*/   [0x7, 0x4, 0x4, 0x1f],
/*5*/   [0x17, 0x15, 0x15, 0x1d],
/*6*/   [0x1f, 0x15, 0x15, 0x1d],
/*7*/   [0x3, 0x1, 0x1, 0x1f],
/*8*/   [0x1f, 0x15, 0x15, 0x1f],
/*9*/   [0x7, 0x5, 0x5, 0x1f],
];

#[rustfmt::skip]
/// Set of chosen symbols.
pub const SYMBOLS: [&[u8]; 7] = 
[
  /* ! */ & [0x17], 
  /* . */ & [0x10],
  /*   */ & [0x0],
  /* - */ & [0x4, 0x4, 0x4],  
  /* = */ & [0xa, 0xa, 0xa],
  /* # */ & [0xa, 0x1f, 0xa, 0x1f, 0xa],
  /* + */ & [0x4, 0xe, 0x4],
];

/// Special matrix for unsupported `char` mapping.
pub const UNSUPPORTED: [u8; 5] = [0x11, 0x13, 0x15, 0x19, 0x11];
/// One column of '0's.
pub const SPACING: [u8; 1] = [0u8; 1];

/// Provides mappings for input `text` into 5x5 matrixes.
/// Each `char` mapping is followed by `SPACING` with exception for last one.
/// Whole `text` is optinally followed by `final_sp` count of `SPACING`s.
/// For details see `col_def`.
pub fn col_defs(text: &str, final_sp: usize, out: &mut [&[u8]]) {
    if text.len() == 0 {
        return;
    }

    let mut ix = 0;
    let mut chars = text.chars();
    let mut c = chars.next();

    loop {
        out[ix] = col_def(c.unwrap());
        ix += 1;

        c = chars.next();
        if c.is_none() {
            break;
        }

        out[ix] = &SPACING;
        ix += 1;
    }

    for _ in 0..final_sp {
        out[ix] = &SPACING;
        ix += 1;
    }
}

/// Provides `c` into 5×5 matrix defined projection.
/// Some `char`s are unsuitable for exact 5×5 representation.
/// For instance '!' which is only 1 column wide.
/// All letters are capitalized.
pub fn col_def(mut c: char) -> &'static [u8] {
    if c.is_ascii() {
        if c.is_lowercase() {
            c = c.to_ascii_uppercase()
        }
    }

    let code = c as usize;
    if code > 64 && code < 91 {
        return &LETTERS[code - 65];
    }

    if code > 47 && code < 58 {
        return &DIGITS[code - 48];
    }

    match c {
        '!' => SYMBOLS[0],
        '.' => SYMBOLS[1],
        ' ' => SYMBOLS[2],
        '-' => SYMBOLS[3],
        '=' => SYMBOLS[4],
        '#' => SYMBOLS[5],
        '+' => SYMBOLS[6],
        _ => &UNSUPPORTED,
    }
}

/// Computes expected buffer size for `text` provided.
/// Goes with `str.len()` thus size is computed from size in 
/// bytes not `char`s nor _graphemes_. Optionally extends size with `extra`.
///
/// Supports _const_ context. 
/// ```
/// use ug_max::buff_size;
/// const TEXT: &str = "abc123";
/// let buffer = [&[0u8; 0]; buff_size(TEXT, 0)];
/// assert_eq!(11, buffer.len());
/// ```
pub const fn buff_size(text: &str, extra: usize) -> usize {
    let len = text.len();
    len * 2 - 1 + extra
}

#[cfg(test)]
mod tests_of_units {
    use super::{buff_size, col_defs, DIGITS, LETTERS, SPACING, SYMBOLS, UNSUPPORTED};

    #[test]
    fn letters_symbols() {
        let alphabet1 = "abcdefghijklmnopqrstuvwxyz";
        let alphabet2 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let symbols = "!. -=#+";

        test::<51>(alphabet1, &LETTERS);
        test::<51>(alphabet2, &LETTERS);
        test::<13>(symbols, &SYMBOLS);

        fn test<const OUTSIZE: usize>(source: &str, sample: &[&[u8]]) {
            let boundary = sample.len() - 1;
            let mut out = [&[0u8; 0] as &[u8]; OUTSIZE];

            col_defs(&source, 0, &mut out);
            let mut out_iter = out.into_iter();

            for i in 0..=boundary {
                let letter = sample[i];

                let mut next = out_iter.next().unwrap();
                assert_eq!(letter, next);

                if i < boundary {
                    next = out_iter.next().unwrap();
                    assert_eq!(SPACING, next);
                }
            }

            assert_eq!(None, out_iter.next());
        }
    }
    #[test]
    fn numbers() {
        let numbers = "0123456789";

        let mut out = [&[0u8; 0] as &[u8]; 19];

        col_defs(&numbers, 0, &mut out);
        let mut out_iter = out.into_iter();

        for i in 0..10usize {
            let digit = DIGITS[i];

            let mut next = out_iter.next().unwrap();
            assert_eq!(digit, next);

            if i < 9 {
                next = out_iter.next().unwrap();
                assert_eq!(SPACING, next);
            }
        }

        assert_eq!(None, out_iter.next());
    }

    #[test]
    fn unsupported() {
        let mut out = [&[0u8; 0] as &[u8]; 1];
        let text = ">";

        col_defs(&text, 0, &mut out);
        assert_eq!(UNSUPPORTED, out[0]);
    }

    #[test]
    fn final_spacing() {
        let mut out = [&[0u8; 0] as &[u8]; 6];
        let text = " ";

        col_defs(&text, 5, &mut out);

        for i in 1..6 {
            assert_eq!(SPACING, out[i]);
        }
    }

    #[test]
    fn buff_size_const() {
        const TEXT: &str = "abc123";

        let buffer = [&[0u8; 0]; buff_size(TEXT, 2)];
        assert_eq!(13, buffer.len());
    }

    #[test]
    fn buff_size_dynam() {
        let text: &str = "abc123";
        assert_eq!(11, buff_size(text, 0));
    }
}
