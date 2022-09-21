use crate::{Error, Header, ensure};

#[inline(always)]
fn hash(r: u8, g: u8, b: u8, a: u8) -> u8 {
    //index_position = (r * 3 + g * 5 + b * 7 + a * 11) % 64
    let r = r.wrapping_mul(3);
    let g = g.wrapping_mul(5);
    let b = b.wrapping_mul(7);
    let a = a.wrapping_mul(11);

    let sum = r.wrapping_add(g).wrapping_add(b).wrapping_add(a);

    sum % 64
}

#[derive(Copy, Clone, Debug)]
struct Pix {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pix {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[inline(always)]
    fn hash(&self) -> u8 {
        hash(self.r, self.g, self.b, self.a)
    }
}

pub fn decode_impl<const ALPHA: bool>(src: &[u8], dest: &mut [u8], header: Header) -> Result<(), Error> {
    let minlen = (header.width * header.height * 3) as usize;
    ensure!(dest.len() >= minlen, Error::buff(dest.len(), minlen));
    
    let mut prev = Pix::new(0,0,0,255);
    let mut array = [Pix::new(0,0,0,0); 64];
    
    let mut ix = 0;
    let mut ox = 0;

    while ix < src.len() {
        match src[ix] {
            // QOI_OP_RGB 
            0b11111110 => {
                let r = src[ix + 1];
                let g = src[ix + 2];
                let b = src[ix + 3];
                ix += 4;

                if !ALPHA {
                    dest[ox + 0] = r;
                    dest[ox + 1] = g;
                    dest[ox + 2] = b;
                    ox += 3;
                } else {
                    dest[ox + 0] = r;
                    dest[ox + 1] = g;
                    dest[ox + 2] = b;
                    dest[ox + 3] = prev.a;
                    ox += 4;
                }
                                
                prev = Pix { r, g, b, a: prev.a};           
                array[prev.hash() as usize] = prev;     
            }
            // QOI_OP_RGBA
            0b11111111 => {
                let r = src[ix + 1];
                let g = src[ix + 2];
                let b = src[ix + 3];
                let a = src[ix + 4];
                ix += 5;

                if !ALPHA {
                    dest[ox + 0] = r;
                    dest[ox + 1] = g;
                    dest[ox + 2] = b;                
                    ox += 3;
                } else {
                    dest[ox + 0] = r;
                    dest[ox + 1] = g;
                    dest[ox + 2] = b;                
                    dest[ox + 3] = a;                
                    ox += 4;   
                }

                prev = Pix { r, g, b, a };
                array[prev.hash() as usize] = prev;
            }
            b => {
                match (b & 0xC0) >> 6 {
                    // QOI_OP_INDEX
                    0b00 => {
                        let qoi_ix = (b & 0x3F) as usize;
                        prev = array[qoi_ix];

                        if !ALPHA {
                            dest[ox + 0] = prev.r;
                            dest[ox + 1] = prev.g;
                            dest[ox + 2] = prev.b;
                            ox += 3;
                        } else {
                            dest[ox + 0] = prev.r;
                            dest[ox + 1] = prev.g;
                            dest[ox + 2] = prev.b;
                            dest[ox + 3] = prev.a;
                            ox += 4;
                        }
                        
                        ix += 1;
                    },
                    // QOI_OP_DIFF
                    0b01 => {
                        let dr = (b & 0x30) as i8 - 2;
                        let dg = (b & 0x0C) as i8 - 2;
                        let db = (b & 0x03) as i8 - 2;

                        prev = Pix {
                            r: (prev.r as i8).wrapping_add(dr) as u8,
                            g: (prev.g as i8).wrapping_add(dg) as u8,
                            b: (prev.b as i8).wrapping_add(db) as u8,
                            a: prev.a,
                        };

                        if !ALPHA {
                            dest[ox + 0] = prev.r;
                            dest[ox + 1] = prev.g;
                            dest[ox + 2] = prev.b;
                            ox += 3;
                        } else {
                            dest[ox + 0] = prev.r;
                            dest[ox + 1] = prev.g;
                            dest[ox + 2] = prev.b;
                            dest[ox + 3] = prev.a;
                            ox += 4;
                        }
                        ix += 1;                        
                        array[prev.hash() as usize] = prev;
                    },
                    // QOI_OP_LUMA
                    0b10 => {
                        let dg = (b & 0x3F) as i8 - 32;
                        let dr_dg = ((src[ix+1] & 0xF0) as i8).wrapping_sub(8);
                        let db_dg = ((src[ix+1] & 0x0F) as i8).wrapping_sub(8);
                        
                        //dr_dg = (cur_px.r - prev_px.r) - (cur_px.g - prev_px.g)
                        //db_dg = (cur_px.b - prev_px.b) - (cur_px.g - prev_px.g)
                        let g = (prev.g as i8).wrapping_add(dg) as u8;
                        let r = dr_dg.wrapping_add(dg).wrapping_add(prev.r as i8) as u8;
                        let b = db_dg.wrapping_add(dg).wrapping_add(prev.b as i8) as u8;

                        if !ALPHA {
                            dest[ox + 0] = r;
                            dest[ox + 1] = g;
                            dest[ox + 2] = b;
                            ox += 3;
                        } else {
                            dest[ox + 0] = r;
                            dest[ox + 1] = g;
                            dest[ox + 2] = b;
                            dest[ox + 3] = prev.a;
                            ox += 4;
                        }

                        prev = Pix {
                            r,
                            g,
                            b,
                            a: prev.a
                        };

                        ix += 2;
                        array[prev.hash() as usize] = prev;
                    },
                    0b11 => {
                        let run = (b & 0x3F) as usize;

                        if !ALPHA {
                            for i in (ox..ox + run*3).step_by(3) {
                                dest[i + 0] = prev.r;
                                dest[i + 1] = prev.g;
                                dest[i + 2] = prev.b;
                            }
                            ox += 3 * run;
                        } else {
                            for i in (ox..ox + run*4).step_by(4) {
                                dest[i + 0] = prev.r;
                                dest[i + 1] = prev.g;
                                dest[i + 2] = prev.b;
                                dest[i + 3] = prev.a;
                            }
                            ox += 4 * run;
                        }

                        ix += 1;
                    },
                    _ => unreachable!()
                }
            }
        }
    }
    
    if ox != minlen {
        return Err(Error::IncompleteImage);
    }
    
    Ok(())
}