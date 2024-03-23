use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    decompress_tarball()
}

fn decompress_tarball() -> Result<(), std::io::Error> {
    let path = "file_test.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}
