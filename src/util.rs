use std::fs::{self, File};
use std::io::{self, Seek, SeekFrom};
use std::path::Path;
use semver::Version;

pub fn archive(version: &Version) -> String {
    format!("ruby-{}.tar.bz2", version)
}

pub fn url(version: &Version) -> String {
    format!(
        "https://cache.ruby-lang.org/pub/ruby/{major}.{minor}/ruby-{version}.tar.bz2",
        major = version.major,
        minor = version.minor,
        version = version,
    )
}

pub fn download(version: &Version, dir: &Path) -> File {
    let archive_path = dir.join(archive(&version));

    if let Ok(archive) = File::open(&archive_path) {
        archive
    } else {
        let url = url(&version);

        let mut response = ureq::get(&url).call().into_reader();

        let mut archive = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&archive_path)
            .expect("Could not create file");

        io::copy(&mut response, &mut archive).unwrap();
        archive.sync_all().unwrap();
        archive.seek(SeekFrom::Start(0)).unwrap();

        archive
    }
}

pub fn unpack(archive: &mut File, dir: &Path) {
    tar::Archive::new(bzip2::read::BzDecoder::new(archive))
        .unpack(dir)
        .unwrap_or_else(|error| panic!("{:#?}", error));
}
