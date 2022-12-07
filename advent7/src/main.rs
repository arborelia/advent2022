// An example to build from each day
use std::fs;
use std::error::Error;
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
#[display("{size} {name}")]  
struct File {
    name: String,
    size: i64
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum ElfShellLine {
    #[display("$ ls")]
    ListFiles,
    #[display("$ cd {destination}")]
    ChangeDir { destination: String },
    #[display("dir {name}")]
    DirEntry { name: String },
    #[display("{file}")]  
    FileEntry { file: File },
}

#[derive(Debug)]
enum FilesystemEntry {
    DirEntry { contents: Vec<FilesystemEntry> },
    FileEntry { file: File },
}

impl FilesystemEntry {
    fn put_in_path(&mut self, path: &[String], file: &File) {
        match self {
            FilesystemEntry::FileEntry { file: _ } => {
                panic!("did we cd into a file?");
            },
            FilesystemEntry::DirEntry { contents } => {
                if path.len() == 0 {
                    contents.push(FilesystemEntry::FileEntry { file: file.clone() });
                } else {
                    self.put_in_path(&path[1..], file)
                }
            }
        }
    }

    fn size(&self) -> i64 {
        match self {
            FilesystemEntry::FileEntry { file } => file.size,
            FilesystemEntry::DirEntry { contents } => {
                contents.iter().map(|entry| entry.size()).sum()
                /* let mut total: i64 = 0;
                for entry in contents {
                    total += entry.size();
                }
                total */
            }
        }
    }
}

fn read_dir_tree(input: &str) -> Result<FilesystemEntry, Box<dyn Error>> {
    let mut path: Vec<String> = Vec::new();
    let mut filesystem = FilesystemEntry::DirEntry { contents: Vec::new() };
    for line in input.lines() {
        let shell: ElfShellLine = line.parse()?;
        match shell {
            ElfShellLine::ChangeDir { destination } => {
                if destination == ".." {
                    path.pop();
                } else {
                    path.push(destination.clone());
                }
            },
            ElfShellLine::FileEntry { file } => {
                filesystem.put_in_path(&path, &file);
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
        assert_eq!(sh, ElfShellLine::FileEntry { file: File { name: "filename".to_owned(), size: 123 }});
    }

    #[test]
    fn test_full_parse() {
        read_dir_tree(TEST_INPUT).unwrap();
    }
}
