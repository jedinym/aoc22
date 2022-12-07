use crate::solution::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, io::Lines, str::FromStr, iter::FromIterator};

lazy_static! {
    static ref CD_RGX: Regex = Regex::new(r#"\$ cd (.*)"#).unwrap();
}

#[derive(Debug)]
struct Directory {
    name: String,
    items: Vec<Item>,
    size: usize,
}

impl FromStr for Directory {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matched = CD_RGX.captures(s);
        match matched {
            None => Err("upsik"),
            Some(captured) => {
                let name = captured.get(1).unwrap().as_str().to_string();

                Ok(Directory { name, items: Vec::new(), size: 0 })
            }
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum Item {
    Dir(Directory),
    File(File),
}

pub struct FileSystem {
    root: Item,
}

impl FileSystem {
    fn change_directory<'a, I>(mut cwd: Directory, lines: &mut I) -> Directory
        where I: Iterator<Item = &'a str>
    {
        // I mutably borrow the iterator later, so I cannot use for
        while let Some(line) = lines.next() {
            if line == "$ cd .." {
                // searching this directory is finished
                return cwd;
            } else if line == "$ ls" {
                // just skip line
            } else {
                let matched = CD_RGX.captures(line);
                if matched.is_some() {
                    // cd to new dir
                    let new_dir = Directory::from_str(line).unwrap();
                    let resolved = Self::change_directory(new_dir, lines);
                    cwd.size += resolved.size;
                    cwd.items.push(Item::Dir(resolved));
                } else {
                    // add a file to current dir
                    Self::add_item(&mut cwd, line);
                }
            }
        }

        return cwd;
    }

    fn parse(input: String) -> Item {
        let mut iterator = input.lines();
        iterator.next().expect("Empty input!"); // skip root (cd /)

        let root = Directory {name: "/".to_string(), items: Vec::new(), size: 0};

        return Item::Dir(Self::change_directory(root, &mut iterator));
    }

    fn add_item(cwd: &mut Directory, line: &str) {
        let (prefix, raw_name) = line.split_once(" ").expect("Invalid file/dir input");
        let name = raw_name.to_string();

        // only add files as directories are added through "cd"
        if prefix != "dir" {
            let size: usize = prefix.parse().expect("Size NaN");
            let new_file = File { name, size };
            cwd.size += size;
            cwd.items.push(Item::File(new_file));
        }
    }

    pub fn new(input: String) -> FileSystem {
        return FileSystem { root: Self::parse(input) };
    }

    fn sum_dirs(item: &Item) -> usize {
        match item {
            Item::Dir(dir) => {
                let subitems: usize = dir.items.iter()
                                         .map(|itm| Self::sum_dirs(itm))
                                         .sum();

                if dir.size <= 100000 {
                    dir.size + subitems
                } else {
                    subitems
                }
            },
            _ => 0
        }
    }

    fn flatten_sizes(&self) -> Vec<usize> {
        let mut vc = Vec::new();
        Self::flatten_sizes_rec(&mut vc, &self.root);

        return vc;
    }

    fn flatten_sizes_rec(vec: &mut Vec<usize>, item: &Item) {
        match item {
            Item::Dir(dir) => {
                vec.push(dir.size);
                for itm in &dir.items {
                    Self::flatten_sizes_rec(vec, itm)
                }
            },
            _ => ()
        }
    }
}

impl Solution for FileSystem {
    fn part_one(&self) -> String {
        return Self::sum_dirs(&self.root).to_string();
    }

    fn part_two(&self) -> String {
        let flattened_sizes = self.flatten_sizes();
        let root_size;

        if let Item::Dir(root) = &self.root {
            root_size = root.size;
        } else {
            panic!("Root is a file?")
        }

        let needed = root_size - 40000000;

        return flattened_sizes.into_iter()
                              .filter(|size| size > &needed)
                              .min()
                              .expect("Empty input?")
                              .to_string();
    }
}
