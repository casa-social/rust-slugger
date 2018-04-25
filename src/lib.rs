extern crate walkdir;
use std::path::{Path, PathBuf};
use std::{cmp, io};
use std::cmp::Ordering;
use walkdir::{DirEntry, WalkDir};
#[macro_use(slugify)]
extern crate slugify;
use slugify::slugify;
use std::fs::FileType;

#[derive(Debug)]
pub struct Slug<'a> {
    pub from: &'a Path,
    pub to: PathBuf,
}

// pub fn slug1(path: &Path) -> io::Result<PathBuf> {
//     let to = slugify!(path.file_name().unwrap().to_str().unwrap());
//     Ok(PathBuf::from(to))
// }

// #[test]
// fn slug1_base_case() {
//     assert_eq!(slug1(&PathBuf::from("a b")).unwrap(), PathBuf::from("a-b"));
// }

fn sort_depth_then_directories(path_a: &Path, path_b: &Path) -> Ordering {
    // deepest first
    path_a
        .components()
        .count()
        .cmp(&path_b.components().count())
        .reverse()
    // directories first (rust considers true>false)
        .then(path_a.is_dir().cmp(&path_b.is_dir()).reverse())
    // then files sorted by name
       .then(path_a.cmp(&path_b))
}

#[test]
fn sort_by_name() {
    let p1 = PathBuf::from("a");
    let p2 = PathBuf::from("b");
    assert_eq!(sort_depth_then_directories(&p1, &p2), Ordering::Less);
}

#[test]
fn sort_by_depth() {
    let p1 = PathBuf::from("b/b");
    let p2 = PathBuf::from("a");
    assert_eq!(sort_depth_then_directories(&p1, &p2), Ordering::Less);
}

#[test]
fn sort_directories_first() {
    let src_dir = PathBuf::from(file!());
    let src_dir = src_dir.parent().unwrap();
    let src_path = PathBuf::from("s"); // earlier same name but file not dir
    println!("true.cmp(false) {:?}", true.cmp(&false));
    println!("src_dir components {:?}", src_dir.components().count());
    println!("src_dir is_dir {:?}", src_dir.is_dir());
    println!("src_path components {:?}", src_path.components().count());
    println!("src_path is_dir {:?}", src_path.is_dir());
    assert_eq!(
        sort_depth_then_directories(&src_dir, &src_path),
        Ordering::Less
    );
}

/// Scans depth first then by path name within
pub fn scan(path: &Path) -> io::Result<Vec<walkdir::DirEntry>> {
    let mut entries: Vec<walkdir::DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    entries.sort_by(|a, b| {
        b.path()
            .components()
            .count()
            .cmp(&a.path().components().count())
            .then(a.path().cmp(b.path()))
    });
    Ok(entries)
}

#[test]
fn scan_is_depth_first_sorted() {
    let dir = PathBuf::from(file!());
    let mut dir = PathBuf::from(dir.parent().unwrap().parent().unwrap()); // project root
    dir.push(PathBuf::from("unit-test"));

    let entries = scan(&dir).unwrap();
    let bulk: Vec<String> = entries
        .iter()
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect();
    let bulk: String = bulk.join("\n");
    // Note you can rebuild this on the command line with `find unit-test -depth`
    let expected = "unit-test/sub1/a?b
unit-test/sub1/x y z
unit-test/sub1
unit-test/sub2
unit-test";
    assert_eq!(bulk, expected);
}

pub fn slug(path: &Path) -> io::Result<()> {
    let mut entries: Vec<walkdir::DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    entries.sort_by(|a, b| {
        b.path()
            .components()
            .count()
            .cmp(&a.path().components().count())
    });
    for entry in entries.iter() {
        println!("{:?}", entry.path());
        if entry.path().is_file() {
            match entry.path().file_name() {
                Some(name) => {
                    println!("{:?}", slugify!(&name.to_string_lossy()));
                }
                None => {
                    println!("NONE");
                }
            }
        }
    }
    /*    for result in WalkDir::new(path).into_iter() {
        match result {
            Ok(entry) => {
                if entry.path().is_file() {
                    println!("{:?}", entry.path().file_name()?);
                }
                let slug = Slug {
                    from: &entry.path(),
                    to: PathBuf::from(slugify!(&entry.path().to_string_lossy())),
                };
                println!("{:?}", slug);
                // println!("{}", slugify!(&entry.path().to_string_lossy()));
            }
            Err(message) => {
                eprintln!("ERR: {}", message);
            }
        }
    }
    */
    Ok(())
}

/*
#[test]
fn identity() {
    let input = PathBuf::from("/");
    assert!(slug(input.as_path()) == input);
    let input = PathBuf::from("/foo");
    assert!(slug(input.as_path()) == input);
    let input = PathBuf::from("/foo/bar/baz-dash-bux.txt");
    assert!(slug(input.as_path()) == input);
}

#[test]
fn space_in_word() {
    let input = PathBuf::from("/space here.txt");
    assert!(slug(input.as_path()) == PathBuf::from("/space-here.txt"));
}
*/
