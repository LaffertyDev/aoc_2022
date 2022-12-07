use std::env;
use std::fs;

struct AocDirectory {
    name: String,
    files: Vec<AocFile>,
    directories: Vec<AocDirectory>
}

struct AocFile {
    name: String,
    size: u32
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
    CdParent
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Entry {
    File((u32, String)),
    Dir(String)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let aoc_fs = build_input(&contents);

    let mut all_dir_sizes: u32 = 0;
    let _ = get_directory_size_with_max(&aoc_fs, &mut all_dir_sizes);
    println!("Problem 1: {}", all_dir_sizes);

    let total_dir_space = get_directory_size(&aoc_fs);
    let free_space = 70000000 - total_dir_space;

    println!("Problem 2: {}", get_smallest_directory_size(&aoc_fs, 30000000 - free_space).1);
}

fn build_input(input: &str) -> AocDirectory {
    let mut root = AocDirectory {
        name: "/".to_string(),
        files: vec![],
        directories: vec![]
    };

    build_tree(&mut root, &mut input.split('\n').into_iter().skip(1));

    root
}

fn build_tree<'a>(current: &mut AocDirectory, input_iter: &mut dyn Iterator<Item = &str>) {
    while let Some(input) = input_iter.next() {
        if input.starts_with('$') {
            match parse_command(input) {
                Command::Ls => (), // ignore, what follows is for the current directory
                Command::CdParent => return,
                Command::Cd(path) => {
                    let child = current.directories.iter_mut().find(|d| d.name == path).unwrap();
                    build_tree(child, input_iter);
                }
            }
        } else {
            // we have data for the current directory
            match parse_entry(input) {
                Entry::File((size, name)) => {
                    current.files.push(AocFile { name: name, size: size });
                },
                Entry::Dir(path) => {
                    current.directories.push(AocDirectory { name: path, directories: vec![], files: vec![] });
                }
            }
        }
    }
}


fn get_directory_size_with_max(directory: &AocDirectory, cur_max: &mut u32) -> u32 {
    // compute raw size of this directory, from all children
    // if this directories raw size is less than the max value, increment the total directory size
    let file_sizes: u32 = directory.files.iter().map(|f| f.size).sum();
    let sub_dir_sizes: u32 = directory.directories.iter().map(|d| get_directory_size_with_max(&d, cur_max)).sum();

    let size_of_directory = file_sizes + sub_dir_sizes;
    if size_of_directory <= 100000 {
        *cur_max += size_of_directory;
    }

    size_of_directory
}

fn get_smallest_directory_size(directory: &AocDirectory, target_min: u32) -> (u32, u32) {
    let sizes = directory.directories.iter().map(|d| get_smallest_directory_size(&d, target_min)).collect::<Vec<(u32, u32)>>();

    let my_size: u32 = sizes.iter().map(|s| s.0).sum::<u32>() + directory.files.iter().map(|f| f.size).sum::<u32>();

    let smallest_child_dir = sizes.iter().map(|s| s.1).filter(|s| *s >= target_min).min_by(|a, b| a.cmp(b)).unwrap_or(u32::MAX);

    let smallest = std::cmp::min(my_size, smallest_child_dir);

    let ret = if smallest >= target_min { smallest} else { u32::MAX };

    return (my_size, ret)
}

fn get_directory_size(directory: &AocDirectory) -> u32 {
    let file_sizes: u32 = directory.files.iter().map(|f| f.size).sum();
    let sub_dir_sizes: u32 = directory.directories.iter().map(|d| get_directory_size(&d)).sum();
    file_sizes + sub_dir_sizes
}

fn parse_command(line: &str) -> Command {
    if line.starts_with("$ ls") {
        return Command::Ls;
    } else if line.starts_with ("$ cd ..") {
        return Command::CdParent;
    } else {
        return Command::Cd(line.chars().skip(5).collect::<String>());
    }
}

fn parse_entry(line: &str) -> Entry {
    if line.starts_with("dir") {
        return Entry::Dir(line.chars().skip(4).collect::<String>());
    } else {
        let mut line = line.split(' ');
        return Entry::File((line.next().unwrap().parse::<u32>().unwrap(), line.next().unwrap().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_command_ls() {
        assert_eq!(Command::Ls, parse_command("$ ls"));
    }

    #[test]
    fn parse_command_cd_parent() {
        assert_eq!(Command::CdParent, parse_command("$ cd .."));
    }

    #[test]
    fn parse_command_cd_path() {
        assert_eq!(Command::Cd(String::from("foobar")), parse_command("$ cd foobar"));
    }

    #[test]
    fn parse_entry_dir() {
        assert_eq!(Entry::Dir(String::from("1234")), parse_entry("dir 1234"));
    }

    #[test]
    fn parse_entry_file() {
        assert_eq!(Entry::File((54321, "john_jackson.txt".to_string())), parse_entry("54321 john_jackson.txt"));
    }

    #[test]
    fn build_tree_builds() {
        let inputs = "\
$ ls
dir a
123 b.txt
$ cd a
$ ls
1 c.txt
2 c.txt";
        let mut root = AocDirectory {
            name: "/".to_string(),
            files: vec![],
            directories: vec![]
        };
        build_tree(&mut root, &mut inputs.split('\n').into_iter());
        assert_eq!(1, root.files.len());
        assert_eq!(1, root.directories.len());
        assert_eq!(2, root.directories.get(0).unwrap().files.len());
    }
}