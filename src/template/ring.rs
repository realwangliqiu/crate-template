use ring::digest::{Context, Digest, SHA256};
use std::io::{BufRead, BufReader, Read};
use crate::{debug, ResultX};
use ring::{digest, hmac, rand};
use ring::hmac::Tag;
use data_encoding::HEXUPPER;
use ring::{pbkdf2};
use std::num::NonZeroU32;

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
    // consumes the context and returns the digest value.
    let d = cx.finish();
    debug!(format!("digest: {}", HEXUPPER.encode(d.as_ref())));

    Ok(d)
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
    debug!(hmac::verify(&key, data, tag.as_ref()).is_ok());

    Ok(tag)
}


/// PBKDF2: Password-Based Key Derivation Function 2
///
/// `DK = PBKDF2(PRF, Password, Salt, c, DK_LEN)`
///
/// | Parameter   | Definition
/// |-------------|-------------------------------------------
/// | PRF         | HMAC here
/// | Password    | password
/// | Salt        | salt
/// | c           | iteration count
/// | DK_LEN      | derived key length
/// | DK          | derived key
///
pub fn pbkdf2(password: &[u8]) -> ResultX<()> {
    const DK_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();

    let rng = rand::SystemRandom::new();
    let salt: [u8; DK_LEN] = rand::generate(&rng)?.expose();
    debug!(format!("Salt: {}", HEXUPPER.encode(&salt)));

    let mut dk = [0u8; DK_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512, // PRF
        n_iter, // iteration count
        &salt,  // salt
        password,    // password
        &mut dk,   // derived key
    );
    debug!(format!("DK: {}", HEXUPPER.encode(&dk)));

    debug!(
        pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password,
            &dk,
        ).is_ok()
    );

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::ring::{hmac, pbkdf2, sha256};

    #[test]
    fn test_sha256() {
        let file = File::open("./src/lib.rs").unwrap();
        let _ = sha256(file).unwrap();
    }

    #[test]
    fn test_hmac() {
        let _ = hmac("data".as_bytes()).unwrap();
    }

    #[test]
    fn test_pbkdf2() {
        pbkdf2("password".as_bytes()).unwrap();
    }
}
