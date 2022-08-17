

use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

fn dd() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;

const TMP_PATH: &str = "./_tmp";

#[test]
fn main() -> Result<(), std::io::Error> {
    let file = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}

#[cfg(test)]
mod tests {

}
