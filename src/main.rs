use crc::{CRC_32_ISO_HDLC, Crc};

#[allow(dead_code)]
#[derive(Debug)]
struct Chunk {
    length: u32,
    c_type_str: Vec<u8>,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    fn new(length: u32, c_type_str: Vec<u8>, data: Vec<u8>, crc: u32) -> Self {
        Self {
            length,
            c_type_str,
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

    let png_file = &args[1];
    let byte_content = std::fs::read(std::path::Path::new(png_file));

    let mut chunks = Vec::new();

    match byte_content {
        Ok(data) => {
            let sig = &data[0..8];
            let good_sig: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
            assert_eq!(good_sig, *sig, "check if it is valid PNG image");

            let mut idx = 8;
            while idx < data.len() {
                let data_len_bytes = &data[idx..idx + 4];
                let length = u32::from_be_bytes([
                    data_len_bytes[0],
                    data_len_bytes[1],
                    data_len_bytes[2],
                    data_len_bytes[3],
                ]);
                idx += 4;

                let chunk_type_bytes = &data[idx..idx + 4];
                let chunk_type = String::from_utf8_lossy(chunk_type_bytes).into_owned();
                idx += 4;

                let data_stride: usize = idx + length as usize;
                let chunk_data = &data[idx..data_stride];

                let crc_bytes = &data[data_stride..data_stride + 4];

                let stored_crc =
                    u32::from_be_bytes([crc_bytes[0], crc_bytes[1], crc_bytes[2], crc_bytes[3]]);

                let crc = calculate_crc(chunk_type_bytes, chunk_data);

                assert_eq!(stored_crc, crc, "CRC mismatch in chunk: {}", chunk_type);

                let chunk = Chunk::new(length, chunk_type_bytes.to_vec(), chunk_data.to_vec(), crc);
                chunks.push(chunk);

                idx = data_stride + 4;

                if chunk_type == "IEND" {
                    println!("Successfully reached IEND chunk. Stopping parser.");
                    break;
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    for chunk in chunks.iter() {
        println!("{:#?}", chunk);
    }

    println!("Total Chunks: {}", chunks.len());
}

fn calculate_crc(chunk_type: &[u8], chunk_data: &[u8]) -> u32 {
    const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

    let mut crc_data = Vec::with_capacity(chunk_type.len() + chunk_data.len());
    crc_data.extend_from_slice(chunk_type);
    crc_data.extend_from_slice(chunk_data);

    PNG_CRC.checksum(&crc_data)
}
