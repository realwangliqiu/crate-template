use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use crate::ResultX;

/// for large content to read
pub fn sha256<R: Read>(mut read: R) -> ResultX<Digest> {
    let mut cx = Context::new(&SHA256);
    let mut buf = [0; 1024];

    let mut read = BufReader::new(read );

    read.read_exact();

    loop {
        let count = read.read(&mut buf)?;
        if count == 0 {
            break;
        }
        cx.update(&buf[..count]);
    }

    Ok(
        // consumes the context and returns the digest value.
        cx.finish()
    )
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use data_encoding::HEXUPPER;
    use crate::ResultX;
    use crate::ring::sha256;

    #[test]
    fn main() -> ResultX<()> {
        let path = "./src/lib.rs";

        let read = BufReader::new(File::open(path)?);
        let digest = sha256(read)?;

        println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

        Ok(())
    }
}
