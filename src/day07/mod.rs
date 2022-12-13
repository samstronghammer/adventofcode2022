use crate::util::advent;
use std::collections::HashMap;

// I went a little overboard on today's puzzle. I created an entire pointer-based
// filesystem-like thing, when I really only needed to track sizes. Ah well.

fn path_to_string(path: Vec<String>) -> String {
  if path.len() == 0 {
    return String::from("/");
  }
  let mut return_string = String::from("");
  for part in path {
    return_string.push('/');
    return_string.push_str(&part);
  }
  return return_string;
}

struct File {
  path: String,
  is_dir: bool,
  size: i32,
  children: Vec<String>,
  parent: String,
}

pub fn run() {
  let contents = advent::parse_input(7);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut file_system: HashMap<String, File> = HashMap::new();
  let mut file_list: Vec<String> = [].to_vec();
  let root_file = File {
    path: String::from("/"),
    is_dir: true,
    size: 0,
    children: [].to_vec(),
    parent: String::from(""),
  };
  file_system.insert(String::from("/"), root_file);
  file_list.push(String::from("/"));
  let mut curr_path: Vec<String> = [].to_vec();

  for line in lines {
    if line.starts_with("$ cd") {
      if line == "$ cd /" {
        curr_path = [].to_vec();
      } else if line == "$ cd .." {
        curr_path.pop();
      } else {
        curr_path.push(String::from(line.split(' ').nth(2).unwrap()));
      }
    } else if line.starts_with("$ ls") {
    } else {
      let toks: Vec<&str> = line.split(' ').collect();
      let is_dir = toks[0] == "dir";
      let size = if is_dir {
        0
      } else {
        toks[0].parse::<i32>().unwrap()
      };
      let parent_path_string: String = path_to_string(curr_path.clone());
      let mut file_path = curr_path.clone();
      file_path.push(String::from(toks[1]));
      let file_path_string = path_to_string(file_path.clone());
      let file = File {
        path: file_path_string.clone(),
        is_dir,
        size: size,
        children: [].to_vec(),
        parent: parent_path_string.clone(),
      };
      let parent = file_system.get_mut(&parent_path_string).unwrap();
      parent.children.push(file_path_string.clone());
      file_system.insert(file_path_string.clone(), file);
      file_list.push(file_path_string.clone());
    }
  }

  for key in file_list.clone() {
    let mut parent_path: String;
    let size: i32;
    {
      let curr_file = file_system.get(&key).unwrap();
      if curr_file.is_dir {
        continue;
      }
      parent_path = curr_file.parent.clone();
      size = curr_file.size;
    }
    loop {
      let parent_file = file_system.get_mut(&parent_path).unwrap();
      if parent_file.is_dir {
        parent_file.size += size;
      }
      if parent_file.path == "/" {
        break;
      }
      parent_path = parent_file.parent.clone();
    }
  }
  let mut sum = 0;
  let space_needed = file_system.get("/").unwrap().size - (70000000 - 30000000);
  let mut min_dir = file_system.get("/").unwrap().size;
  for key in file_list.clone() {
    let curr_file = file_system.get(&key).unwrap();
    if curr_file.is_dir && curr_file.size <= 100000 {
      sum += curr_file.size;
    }
    if curr_file.is_dir && curr_file.size >= space_needed && curr_file.size < min_dir {
      min_dir = curr_file.size;
    }
  }
  println!("Part 1:");
  println!("{}", sum);
  println!("Part 2:");
  println!("{}", min_dir);
}
