use ring::digest::{Context, Digest, SHA256};
use std::io::{BufRead, BufReader, Read};
use crate::{debug, ResultX};
use ring::{digest, hmac, rand};
use ring::hmac::Tag;

/// SHA-256
///
/// for large content to read
pub fn sha256<R: Read>(read: R) -> ResultX<Digest> {
    let mut cx = Context::new(&SHA256);
    let mut bufread = BufReader::new(read);

    loop {
        let buf = bufread.fill_buf()?;
        let len = buf.len();
        if len == 0 {
            break;
        }

        // `update` can be called many times until finish is called
        cx.update(&buf[..len]);

        // `consume` must be called to be paired with the `fill_buf`
        bufread.consume(len);
    }

    Ok(
        // consumes the context and returns the digest value.
        cx.finish()
    )
}



/// HMAC: Hash-based Message Authentication Code
///
pub fn hmac(data: &[u8]) -> ResultX<Tag> {
    let rng = rand::SystemRandom::new();
    let raw_key: [u8; digest::SHA256_OUTPUT_LEN] = rand::generate(&rng)?.expose();
    // generate i_pad_key and o_pad_key, then store them in Key's inner and outer Context
    let key = hmac::Key::new(hmac::HMAC_SHA256, &raw_key);
    let tag = hmac::sign(&key, data);

    // calculates data by key and verifies whether the result equals tag
    debug!(hmac::verify(&key, data, tag.as_ref())?);

    Ok(tag)
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use data_encoding::HEXUPPER;
    use crate::ring::{hmac, sha256};

    #[test]
    fn test_sha256() {
        let file = File::open("./src/lib.rs").unwrap();
        let digest = sha256(file).unwrap();

        println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));
    }

    #[test]
    fn test_hmac() {
        let _ = hmac("data".as_bytes()).unwrap();
    }
}
