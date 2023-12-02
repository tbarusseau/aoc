pub struct Day7;

crate::impl_day!("7", true);

#[derive(Debug)]
struct Directory {
    pub dirs: Vec<Directory>,
    pub files: Vec<File>,
}

#[derive(Debug)]
struct File {
    pub size: usize,
}

fn process_input(input: &str) -> Directory {
    let mut root = Directory {
        dirs: vec![],
        files: vec![],
    };

    let mut lines = input.lines().skip(1);

    parse_lines_rec(&mut root, &mut lines);

    root
}

fn parse_lines_rec<'a, I>(parent_dir: &mut Directory, lines: &mut I)
where
    I: Iterator<Item = &'a str>,
{
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            let dirname = line.split(' ').nth(2).unwrap();

            match dirname {
                ".." => {
                    break;
                }
                _ => {
                    let mut subdir = Directory {
                        dirs: vec![],
                        files: vec![],
                    };

                    parse_lines_rec(&mut subdir, &mut *lines);
                    parent_dir.dirs.push(subdir);
                }
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
            /* Nothing to do */
        } else {
            // File entry
            let mut split = line.split(' ');
            let size = split.next().unwrap().parse().unwrap();

            let new_file = File { size };

            parent_dir.files.push(new_file);
        }
    }
}

fn walk_dir_rec(dir: &Directory, sizes: &mut Vec<usize>) {
    for subdir in &dir.dirs {
        sizes.push(get_dir_size_rec(subdir));
        walk_dir_rec(subdir, sizes);
    }
}

fn get_dir_size_rec(dir: &Directory) -> usize {
    let mut result = 0;

    for subdir in &dir.dirs {
        result += get_dir_size_rec(subdir);
    }

    for file in &dir.files {
        result += file.size;
    }

    result
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let root_dir = process_input(input);
    let mut sizes = vec![];

    walk_dir_rec(&root_dir, &mut sizes);

    sizes.retain(|s| *s <= 100_000);
    let res: usize = sizes.iter().sum();
    Box::new(res)
}

const TOTAL_DISK_SPACE: usize = 70_000_000;
const REQUIRED_UNUSED_SPACE: usize = 30_000_000;

fn find_smallest_dir_rec(root_dir: &Directory, unused_space: usize) -> usize {
    let mut smallest_valid = usize::MAX;

    for dir in &root_dir.dirs {
        let size = get_dir_size_rec(dir);
        if unused_space + size > REQUIRED_UNUSED_SPACE {
            smallest_valid = smallest_valid.min(size);
        }

        smallest_valid = smallest_valid.min(find_smallest_dir_rec(dir, unused_space));
    }

    smallest_valid
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let root_dir = process_input(input);
    let unused_space = TOTAL_DISK_SPACE - get_dir_size_rec(&root_dir);

    let smallest = find_smallest_dir_rec(&root_dir, unused_space);
    println!("unused_space: {unused_space}");
    println!("smallest: {smallest}");

    Box::new(smallest)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"$ cd /
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

    #[test]
    fn test_part1() {
        assert_eq!(95437.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(24_933_642.to_string(), *solve_part2(INPUT).to_string());
    }
}
