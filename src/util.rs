use std::cmp;
use std::io;
use std::mem;
use std::slice;

/// Takes a value and rounds it up to be aligned % 4
#[macro_export]
macro_rules! align4 {
    ($e:expr) => {
        ($e) + ((4 - (($e) % 4)) % 4)
    };
}

/// Takes a value and rounds it up to be aligned % 8
#[macro_export]
macro_rules! align8 {
    ($e:expr) => {
        ($e) + ((8 - (($e) % 8)) % 8)
    };
}

/// How much needs to be added to get a value aligned % 4
#[macro_export]
macro_rules! align4needed {
    ($e:expr) => {
        (4 - (($e) % 4)) % 4
    };
}

pub fn do_pad(output: &mut io::Write, length: usize) -> io::Result<()> {
    let mut pad = length;
    let zero_buf = [0u8; 512];
    while pad > 0 {
        let amount_to_write = cmp::min(zero_buf.len(), pad);
        pad -= output.write(&zero_buf[..amount_to_write])?;
    }
    Ok(())
}

pub unsafe fn as_byte_slice<'a, T: Copy>(input: &'a T) -> &'a [u8] {
    slice::from_raw_parts(input as *const T as *const u8, mem::size_of::<T>())
}
