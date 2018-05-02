extern crate slugger;
#[macro_use(slugify)]
extern crate slugify;
extern crate walkdir;

use slugify::slugify;
use std::env;
use walkdir::WalkDir;
use slugger::Slug;
use std::path::PathBuf;
fn main() {
    for path in env::args().skip(1) {
        for path in slugger::scan(&PathBuf::from(path)).unwrap() {
            println!("{}", path.display());
        }
    }
}
