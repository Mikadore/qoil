use thiserror::Error;

mod detail;
#[cfg(test)]
mod test;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("File Magic ({0:?}) doesn't match expected magic 'qoif'")]
    IncorrectMagic([u8; 4]),
    #[error("The supplied buffer isn't big enough ({size} < {required})")]
    BufferTooSmall { required: usize, size: usize },
    #[error("The source ran out of bytes before the image could be completed")]
    IncompleteImage,
}

macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}
pub(crate) use ensure;

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

pub fn decode_header(data: impl AsRef<[u8]>) -> Result<Header, Error> {
    let slice = data.as_ref();
    ensure!(slice.len() >= 14, Error::buff(slice.len(), 14));

    let magic = &slice[0..4];
    ensure!(
        magic == b"qoif",
        Error::IncorrectMagic(magic.try_into().unwrap())
    );

    let width = u32::from_be_bytes(slice[4..8].try_into().unwrap());
    let height = u32::from_be_bytes(slice[8..12].try_into().unwrap());
    let channels = slice[12];
    let colorspace = slice[13];

    Ok(Header {
        width,
        height,
        channels,
        colorspace,
    })
}

/// Decodes the given QOI file data into a supplied buffer. Uses the
/// same number of channels as defined in the header. If the channels
/// count isn't 3 or 4, it defaults to 3. The given buffer must be at
/// least (width * height * channels) big.
pub fn decode_to_buf(data: impl AsRef<[u8]>, mut buf: impl AsMut<[u8]>) -> Result<Header, Error> {
    let header = decode_header(&data)?;
    match header.channels {
        4 => detail::decode_impl::<true>(data.as_ref(), buf.as_mut(), header.clone()),
        _ => detail::decode_impl::<false>(data.as_ref(), buf.as_mut(), header.clone()),
    }
    .map(|_| header)
}

/// Decodes the given QOI file data into a supplied buffer.
/// Always decodes to 3 channels (RGB). The given buffer
///  must be at least (width * height * 3) big.
pub fn decode_to_buf_rgb(
    data: impl AsRef<[u8]>,
    mut buf: impl AsMut<[u8]>,
) -> Result<Header, Error> {
    let header = decode_header(&data)?;
    detail::decode_impl::<false>(data.as_ref(), buf.as_mut(), header.clone())?;

    Ok(header)
}

/// Decodes the given QOI file data into a supplied buffer.
/// Always decodes to 4 channels (RGBA). The given buffer
///  must be at least (width * height * 4) big.
pub fn decode_to_buf_rgba(
    data: impl AsRef<[u8]>,
    mut buf: impl AsMut<[u8]>,
) -> Result<Header, Error> {
    let header = decode_header(&data)?;
    detail::decode_impl::<true>(data.as_ref(), buf.as_mut(), header.clone())?;

    Ok(header)
}

/// Decodes the given QOI file data into a newly allocated Vec.
/// Uses the same number of channels as defined in the header.
/// If the channels count isn't 3 or 4, it defaults to 3.
pub fn decode_to_vec(data: impl AsRef<[u8]>) -> Result<(Header, Vec<u8>), Error> {
    let header = decode_header(&data)?;
    let channels = if header.channels == 4 { 4 } else { 3 };
    let mut v = vec![0; header.width as usize * header.height as usize * channels as usize];
    match header.channels {
        4 => detail::decode_impl::<true>(data.as_ref(), &mut v, header.clone()),
        _ => detail::decode_impl::<false>(data.as_ref(), &mut v, header.clone()),
    }
    .map(|_| (header, v))
}

/// Decodes the given QOI file data into a newly allocated Vec.
/// Always uses 3 channels (RGB), irrespective of the header.
pub fn decode_to_vec_rgb(data: impl AsRef<[u8]>) -> Result<(Header, Vec<u8>), Error> {
    let header = decode_header(&data)?;
    let mut v = vec![0; header.width as usize * header.height as usize * 3];
    detail::decode_impl::<false>(data.as_ref(), &mut v, header.clone()).map(|_| (header, v))
}

/// Decodes the given QOI file data into a newly allocated Vec.
/// Always uses 4 channels (RGBA), irrespective of the header.
pub fn decode_to_vec_rgba(data: impl AsRef<[u8]>) -> Result<(Header, Vec<u8>), Error> {
    let header = decode_header(&data)?;
    let mut v = vec![0; header.width as usize * header.height as usize * 4 as usize];
    detail::decode_impl::<true>(data.as_ref(), &mut v, header.clone()).map(|_| (header, v))
}
