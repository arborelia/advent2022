// An example to build from each day
use std::fs;
use std::error::Error;
use std::collections::HashMap;
use parse_display::{Display, FromStr};

pub const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

/// This struct represents each line in the elf's shell session, and provides a parse-display
/// implementation for each type of line, telling us how to parse it.
#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum ElfShellLine {
    #[display("$ ls")]
    ListFiles,
    #[display("$ cd {destination}")]
    ChangeDir { destination: String },
    #[display("dir {name}")]
    DirEntry { name: String },
    #[display("{size} {name}")]
    FileEntry { name: String, size: i64 },
}

/// An entry in the filesystem is either a directory, mapping names to entries, or a file, which
/// has a size.
#[derive(Debug)]
enum FilesystemEntry {
    Dir { contents: HashMap<String, FilesystemEntry> },
    File { size: i64 },
}

impl FilesystemEntry {
    /// In the shell session, we've seen a file at a particular path, and we know its size.
    /// Put it into the filesystem in that path, creating more directories if necessary.
    fn put_in_path(&mut self, path: &[String], filename: &str, size: i64) {
        match self {
            FilesystemEntry::File { size: _ } => {
                panic!("did we cd into a file?");
            },
            FilesystemEntry::Dir { contents } => {
                if path.len() == 0 {
                    contents.insert(filename.to_owned(), FilesystemEntry::File { size });
                } else {
                    let path_component: &str = &path[0];
                    if !(contents.contains_key(path_component)) {
                        contents.insert(
                            path_component.to_owned(), 
                            FilesystemEntry::Dir { contents: HashMap::new() }
                        );
                    }
                    match contents.get_mut(path_component) {
                        Some(entry) => {
                            entry.put_in_path(&path[1..], filename, size);
                        },
                        None => {
                            panic!("I swear we just added this directory");
                        }
                    }
                }
            }
        }
    }

    /// The total size of a file or directory
    fn size(&self) -> i64 {
        match self {
            FilesystemEntry::File { size } => *size,
            FilesystemEntry::Dir { contents } => {
                contents.values().map(|entry| entry.size()).sum()
            }
        }
    }

    /// A list of directories in this filesystem, found recursively in depth-first order.
    fn depth_first_dirs(&self) -> Vec<&FilesystemEntry> {
        let mut dirs = Vec::new();
        match self {
            FilesystemEntry::File { size: _ } => dirs,
            FilesystemEntry::Dir { contents } => {
                dirs.push(&self);
                for entry in contents.values() {
                    dirs.extend(entry.depth_first_dirs().iter());
                }
                dirs
            }
        }
    }

    /// An implementation of part 1: find all the small enough directories and add up their size.
    fn size_of_small_dirs(&self, cutoff: i64) -> i64 {
        match self {
            FilesystemEntry::File { size: _ } => 0,
            FilesystemEntry::Dir { contents: _ } => {
                let mut total: i64 = 0;
                for dir in self.depth_first_dirs() {
                    if dir.size() <= cutoff {
                        total += dir.size();
                    }
                }
                total
            }
        }
    }

    /// Used in part 2: find the smallest directory with size at least min_size.
    fn find_dir_to_delete(&self, min_size: i64) -> &FilesystemEntry {
        let mut dir_to_delete: Option<&FilesystemEntry> = None;
        let mut smallest_size: i64 = i64::MAX;
        for dir in self.depth_first_dirs() {
            let size = dir.size();
            if size >= min_size && size < smallest_size {
                dir_to_delete = Some(dir);
                smallest_size = size;
            }
        }
        dir_to_delete.unwrap()
    }

    /// Implementation of part 2: we need the size of the filesystem to be no more than
    /// max_size. Find the smallest directory to delete that will make this work.
    fn size_of_dir_to_delete(&self, max_size: i64) -> i64 {
        let min_size: i64 = self.size() - max_size;
        self.find_dir_to_delete(min_size).size()
    }
}


/// Read the elf's shell session and build a filesystem based on what we see.
fn read_dir_tree(input: &str) -> Result<FilesystemEntry, Box<dyn Error>> {
    let mut path: Vec<String> = Vec::new();
    let mut filesystem = FilesystemEntry::Dir { contents: HashMap::new() };
    for line in input.lines() {
        let shell: ElfShellLine = line.parse()?;
        match shell {
            ElfShellLine::ChangeDir { destination } => {
                if destination == ".." {
                    path.pop();
                } else if destination == "/" {
                    path.drain(..);
                } else {
                    path.push(destination.clone());
                }
            },
            ElfShellLine::FileEntry { name, size } => {
                filesystem.put_in_path(&path, &name, size);
            },
            // We don't do anything with the 'dir' entries -- we'll create the directories when we
            // see a file inside them.
            ElfShellLine::DirEntry { name: _ } => {},
            ElfShellLine::ListFiles => {}
        }
    }
    Ok(filesystem)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let filesystem = read_dir_tree(&input)?;
    println!("sum of small directories: {}", filesystem.size_of_small_dirs(100_000));
    // We can subtract numbers. We need room for a 30 MB thing on a 70 MB filesystem, so the rest of
    // the filesystem has a max size of 40 MB.
    println!("size of dir to delete: {}", filesystem.size_of_dir_to_delete(40_000_000));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let sh: ElfShellLine = "$ ls".parse().unwrap();
        assert_eq!(sh, ElfShellLine::ListFiles);
        let sh: ElfShellLine = "$ cd here".parse().unwrap();
        assert_eq!(sh, ElfShellLine::ChangeDir { destination: "here".to_owned() });
        let sh: ElfShellLine = "123 filename".parse().unwrap();
        assert_eq!(sh, ElfShellLine::FileEntry { name: "filename".to_owned(), size: 123 });
    }

    #[test]
    fn test_full_parse() {
        let filesystem = read_dir_tree(TEST_INPUT).unwrap();
    }

    #[test]
    fn test_example() {
        let filesystem = read_dir_tree(TEST_INPUT).unwrap();
        assert_eq!(filesystem.size_of_small_dirs(100_000), 95437);
    }

    #[test]
    fn test_example_2() {
        let filesystem = read_dir_tree(TEST_INPUT).unwrap();
        assert_eq!(filesystem.size_of_dir_to_delete(40_000_000), 24933642);
    }
}
