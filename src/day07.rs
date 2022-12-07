use crate::util::get_lines;

struct Directory {
    name: String,
    files: Vec<usize>,
    directories: Vec<Directory>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    fn get_size(&self) -> usize {
        let mut result = 0;
        for file in self.files.iter() {
            result += file;
        }
        for directory in self.directories.iter() {
            result += directory.get_size();
        }
        result
    }

    fn get_sizes(&self) -> Vec<usize> {
        let mut results: Vec<usize> = Vec::new();
        results.push(self.get_size());
        for directory in self.directories.iter() {
            for result in directory.get_sizes() {
                results.push(result);
            }
        }
        results
    }
}

struct VirtualMachine<'a> {
    lines: Vec<&'a str>,
    line_index: usize,
    root_directory: Directory,
    working_directory: String,
}

impl<'a> VirtualMachine<'a> {
    fn new(lines: Vec<&str>) -> VirtualMachine {
        VirtualMachine {
            lines,
            line_index: 0,
            root_directory: Directory::new(String::from("")),
            working_directory: String::from(""),
        }
    }

    fn execute(&mut self)-> bool {
        if let Some(line) = self.lines.get(self.line_index) {
            self.line_index += 1;
            match line[2..4].trim() {
                "cd" => self.execute_change_directory(line[5..].trim()),
                "ls" => self.execute_listing(),
                _ => panic!("invalid command: {}", &line[2..]),
            }
            true
        } else {
            false
        }
    }

    fn execute_change_directory(&mut self, arguments: &str) {
        self.working_directory = match arguments {
            "/" => String::from(""),
            ".." => {
                let mut results = self.working_directory.split("/").collect::<Vec<&str>>();
                results.pop();
                results.join("/")
            },
            _ => {
                let mut results = self.working_directory.split("/").collect::<Vec<&str>>();
                results.push(arguments);
                results.join("/")
            },
        }
    }

    fn execute_listing(&mut self) {
        loop {
            if let Some(line) = self.lines.get(self.line_index) {
                if let Some('$') = line.chars().next() {
                    break;
                }
                self.line_index += 1;

                // get reference to directory object from string representation
                let mut working_directory = &mut self.root_directory;
                let path = self.working_directory.split("/").collect::<Vec<&str>>();
                let mut index = 1;
                while index < path.len() {
                    let path_item = path[index];
                    let mut dir_index = 0;
                    while dir_index < working_directory.directories.len() {
                        if working_directory.directories[dir_index].name == path_item {
                            working_directory = &mut working_directory.directories[dir_index];
                        }
                        dir_index += 1;
                    }
                    index += 1;
                }

                let mut split = line.trim().split(' ');
                let size_str = split.next().unwrap();
                let name = String::from(split.next().unwrap());
                if size_str == "dir" {
                    working_directory.directories.push(Directory::new(name));
                } else {
                    let size: usize = size_str.parse().unwrap();
                    working_directory.files.push(size);
                }
            } else {
                break;
            }
        }
    }
}

pub fn answer1(input: &str) -> String {
    let lines = get_lines(input);
    let mut vm = VirtualMachine::new(lines);
    loop {
        if !vm.execute() {
            break;
        }
    }

    let mut results = 0;
    let sizes = vm.root_directory.get_sizes();
    for size in sizes.iter() {
        if *size <= 100_000usize {
            results += size;
        }
    }
    results.to_string()
}

pub fn answer2(input: &str) -> String {
    let lines = get_lines(input);
    let mut vm = VirtualMachine::new(lines);
    loop {
        if !vm.execute() {
            break;
        }
    }

    let used_size = vm.root_directory.get_size();
    let mut results = used_size;
    let sizes = vm.root_directory.get_sizes();
    for size in sizes.iter() {
        if 70_000_000usize - used_size + size >= 30_000_000usize && *size < results {
            results = *size;
        }
    }
    results.to_string()
}