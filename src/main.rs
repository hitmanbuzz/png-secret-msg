#[allow(dead_code)]
#[derive(Debug)]
struct Chunk {
    length: u32,
    c_type: Vec<u8>,
    c_type_str: String,
    data: Vec<u8>,
    crc: Vec<u8>,
}

impl Chunk {
    fn new(length: u32, c_type: Vec<u8>, c_type_str: &str, data: Vec<u8>, crc: Vec<u8>) -> Self {
        Self {
            length,
            c_type,
            c_type_str: c_type_str.to_string(),
            data,
            crc,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        eprintln!("USAGE: program <png-image>");
        return;
    }

    let png_file = unsafe { args.get_unchecked(1) };
    let byte_content = std::fs::read(std::path::Path::new(png_file));

    let mut chunks = Vec::new();

    match byte_content {
        Ok(data) => {
            let sig = unsafe { &data.get_unchecked(0..=7) };
            let good_sig: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
            assert_eq!(good_sig, *sig, "check if it is valid PNG image");

            let data_len_bytes = unsafe { &data.get_unchecked(8..=11) };
            let chunk_type_bytes = unsafe { &data.get_unchecked(12..=15) };
            let chunk_type = unsafe { String::from_utf8_unchecked(chunk_type_bytes.to_vec()) };

            let mut bits = Vec::with_capacity(4);
            for i in 12..=15 {
                let byte = unsafe { data.get_unchecked(i) };
                bits.push(get_bit(*byte, 5));
            }

            println!("Chunk Length: {:?}", data_len_bytes);
            println!("Chunk Type: {}", chunk_type);
            println!("Chunk Type Bits: {:?}", bits);

            let chunk = Chunk::new(0, bits, chunk_type.as_str(), vec![], vec![]);
            chunks.push(chunk);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    for chunk in chunks.iter() {
        println!("{:#?}", chunk);
    }
}

fn get_bit(byte: u8, index: u8) -> u8 {
    byte & (1 << index)
}
