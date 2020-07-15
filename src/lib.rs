#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/array_ops.rs"));

pub fn fast_array_copy(dst: &mut [u8], src: &[u8]) {
    debug_assert!(dst.len() == src.len());

    let len = src.len() as u32;
    
    unsafe {
        let dst = dst.as_mut_ptr();
        let src = src.as_ptr();
        array_copy(dst, src, len);
    }
}

pub fn fast_array_xor(dst: &mut [u8], src: &[u8]) {
    debug_assert!(dst.len() == src.len());
    let len = src.len() as u32;
    debug_assert!(len % 64 == 0);
    
    unsafe {
        let dst = dst.as_mut_ptr();
        let src = src.as_ptr();
        array_xor(dst, src, len);
    }
}
