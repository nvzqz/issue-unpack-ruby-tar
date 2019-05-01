extern crate semver;
extern crate ureq;
extern crate tar;
extern crate bzip2;

use std::env;
use std::path::PathBuf;
use semver::Version;

mod util;

fn main() {
    let dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let version = match env::var("RUBY_VERSION") {
        Ok(version) => version.parse::<Version>().unwrap(),
        Err(_) => Version::new(2, 6, 0),
    };

    let mut file = util::download(&version, &dir);
    util::unpack(&mut file, &dir);
}
