fn equal(a: &super::Header, b: &qoi::Header) -> bool {
    a.width == b.width
        && a.height == b.height
        && a.channels == b.channels.as_u8()
        && a.colorspace == b.colorspace.as_u8()
}

#[test]
fn decode_parity() {
    let dir = std::fs::read_dir("./benches/images").unwrap();
    for file in dir {
        if let Ok(f) = file {
            let image_data = std::fs::read(f.path()).unwrap();
            let (qoi_header, qoi_data) = qoi::decode_to_vec(&image_data).unwrap();
            let (my_header, my_data) = super::decode_to_vec(&image_data).unwrap();

            assert!(equal(&my_header, &qoi_header), "Headers don't match");
            assert_eq!(qoi_data, my_data, "Decoded data doesn't match");
        }
    }
}
