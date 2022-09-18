use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("File Magic ({0:?}) doesn't match expected magic 'qoif'")]
    IncorrectMagic([u8; 4]),
    #[error("The supplied buffer isn't big enough ({size} < {required})")]
    BufferTooSmall { required: usize, size: usize },
}

macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if $cond { return Err($err) }
    };
}

impl Error {
    fn buff(size: usize, required: usize) -> Error {
        Error::BufferTooSmall { required, size }
    }
}

#[derive(Clone, Debug)]
pub struct Header {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8,
}

#[inline]
fn hash(r: u8, g: u8, b: u8, a: u8) -> u8 {
    //index_position = (r * 3 + g * 5 + b * 7 + a * 11) % 64
    let r = r.wrapping_mul(3);
    let g = g.wrapping_mul(5);
    let b = b.wrapping_mul(7);
    let a = a.wrapping_mul(11);

    let sum = r.wrapping_add(g).wrapping_add(b).wrapping_add(a);

    sum % 64
}

pub fn decode_header(data: impl AsRef<[u8]>) -> Result<Header, Error> {
    let slice = data.as_ref();
    ensure!(slice.len() < 14, Error::buff(slice.len(), 14));

    let magic = &slice[0..4];
    ensure!(magic == b"qoif", Error::IncorrectMagic(magic.try_into().unwrap()));

    let width = u32::from_be_bytes(slice[4..8].try_into().unwrap());
    let height = u32::from_be_bytes(slice[8..12].try_into().unwrap());
    let channels = slice[12];
    let colorspace = slice[13];
    
    Ok(Header {
        width,
        height,
        channels,
        colorspace
    })
}

/// Decodes the given QOI file data into a supplied buffer. Uses the 
/// same number of channels as defined in the header. If the channels
/// count isn't 3 or 4, it defaults to 3. The given buffer must be at 
/// least (width * height * channels) big. 
pub fn decode_to_buf(data: impl AsRef<[u8]>, buf: impl AsMut<[u8]>) {

}


pub fn decode_to_buf_rgb(data: impl AsRef<[u8]>, buf: impl AsMut<[u8]>) {
}
