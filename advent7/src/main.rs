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

#[derive(Debug)]
enum FilesystemEntry {
    Dir { contents: HashMap<String, FilesystemEntry> },
    File { size: i64 },
}

impl FilesystemEntry {
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

    fn size(&self) -> i64 {
        match self {
            FilesystemEntry::File { size } => *size,
            FilesystemEntry::Dir { contents } => {
                contents.values().map(|entry| entry.size()).sum()
            }
        }
    }

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

    fn size_of_dir_to_delete(&self, space_needed: i64) -> i64 {
        let min_size: i64 = self.size() - space_needed;
        self.find_dir_to_delete(min_size).size()
    }
}



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
