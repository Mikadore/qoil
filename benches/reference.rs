#[link(name = "wrapper")]
extern "C" {
    fn ffi_encode(data: *const u8, width: u32, height: u32, channels: u8, colorspace: u8);
    fn ffi_decode(data: *const u8, len: i32, channels: u8);
}

pub fn encode(data: &[u8], header: qoi::Header) {
    unsafe {
        ffi_encode(
            data.as_ptr(),
            header.width,
            header.height,
            header.channels.as_u8(),
            header.colorspace.as_u8(),
        )
    }
}

pub fn decode(data: &[u8], channels: u8) {
    unsafe { ffi_decode(data.as_ptr(), data.len() as i32, channels) }
}
