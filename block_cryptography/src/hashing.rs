use ring::digest::{Context, Digest};
use std::io::{Read, Result};

pub fn hash_digest<R: Read>(reader: &mut R, mut context: Context) -> Result<Digest> {
    let mut buffer = [0; 1024];
    
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use ring::digest::{Context, SHA256};
    use super::*;

    #[test]
    fn file_sha256hash_tests() {
        let mut file : File;
        let filenames : [&str; 5] = [
            "tests/test1.txt",
            "tests/test2.txt",
            "tests/test3.txt",
            "tests/test4.txt",
            "tests/test5.txt"
        ];
        let results : [&str; 5] = [
            "SHA256:532eaabd9574880dbf76b9b8cc00832c20a6ec113d682299550d7a6e0f345e25",
            "SHA256:a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e",
            "SHA256:d4461e1f33db9190c72806ea55a693497b564cef03adbe9e1d59519b77eaff49",
            "SHA256:76bda7359a9990b99fe390f51d7310ab3505e26f02e5d4b049ee75a49d058896",
            "SHA256:f8284373d0b9e8076e7164c1468f841a1b046d8d4c185142aee58841e676cc4f"
        ];

        assert_eq!(filenames.len(), results.len());

        for i in 0..filenames.len() {
            let context = Context::new(&SHA256);
            file = File::open(filenames[i]).unwrap();

            match hash_digest(&mut file, context) {
                Ok(hash) => { assert_eq!(format!("{:?}", hash), results[i])},
                _ => { panic!() }
            };
        }   
    }
}
