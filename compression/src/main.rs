use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    decompress_tarball()?;
    compress_directory_into_tarball()?;
    decompress_tarball_while_removing_prefix_from_paths()
}

fn decompress_tarball() -> Result<(), std::io::Error> {
    let path = "file_test.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

fn compress_directory_into_tarball() -> Result<(), std::io::Error> {
    unimplemented!()
}

fn decompress_tarball_while_removing_prefix_from_paths() -> Result<(), std::io::Error>{
    unimplemented!()
}
