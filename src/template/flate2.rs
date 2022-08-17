use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;

///
fn compress<P, Q>(src_path: P, dst_path: Q) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
{
    let file = File::create(dst_path)?;
    let write = GzEncoder::new(file, Compression::default());
    // tar::Builder
    Builder::new(write)
        // path -> the name of the directory in the archive
        // src_path -> source files
        .append_dir_all("", src_path).unwrap();

    Ok(())
}


fn decompress<P, Q>(src_path: P, dst_path: Q) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
{
    let file = File::open(src_path)?;
    let read = GzDecoder::new(file);
    // tar::archive
    Archive::new(read).unpack(dst_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::template::flate2::{compress, decompress};

    const PATH_TMP: &str = "./_tmp";

    #[test]
    fn base() {
        let path = Path::new(PATH_TMP).join("archive.tar.gz");
        let tar = Path::new(PATH_TMP).join("tar");

        compress("./src", path.clone()).unwrap();
        decompress(path, tar).unwrap();
    }
}
