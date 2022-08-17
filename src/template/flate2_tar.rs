use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;
use crate::ResultX;


pub fn compress<P, Q>(src_path: P, dst_path: Q) -> ResultX<()>
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
        .append_dir_all("", src_path)?;

    Ok(())
}


pub fn decompress<P, Q>(src_path: P, dst_path: Q) -> ResultX<()>
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


pub fn decompress_strip_prefix<P, Q>(src_path: P, dst_path: Q) -> ResultX<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
{
    let file = File::open(src_path)?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "template";

    // cannot use the `?` operator in `for_each` loop
    for r in archive.entries()? {
        let mut entry = r.unwrap(); // must panic

        let path = entry.path()?;
        // ignore prefix does not exist in entry path
        let path = path.strip_prefix(prefix).or::<String>(Ok(&path))?;
        let s = dst_path.as_ref().join(path);

        entry.unpack(&s)?;
    }


    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::flate2_tar::{compress, decompress, decompress_strip_prefix};

    const PATH_TMP: &str = "./_tmp";

    #[test]
    fn base() {
        let path = Path::new(PATH_TMP).join("archive.tar.gz");
        let tar = Path::new(PATH_TMP).join("tar");

        compress("./src", path.clone()).unwrap();
        decompress(path.clone(), tar.clone()).unwrap();

        decompress_strip_prefix(path, tar).unwrap();
    }
}
