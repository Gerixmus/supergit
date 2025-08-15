use std::{
    env::current_dir,
    fs::{read_dir}, path::PathBuf,
};

use inquire::Select;

pub fn run_ignore() -> Result<(), String> {
    let dir = current_dir().unwrap();
    println!("Path: {}", dir.display());
    let mut root = Node {
        path: dir,
        children: vec![],
    };
    get_children(&mut root);

    let mut pointer = &root;

    loop {
        let files = pointer.children.iter().filter_map(|child| child.path.file_name()?.to_str()).collect();
        let ans = Select::new("Select item:", files).prompt().unwrap();
        for node in &pointer.children {
            if let Some(name) = node.path.file_name() {
                if name == ans {
                    pointer = &node;
                }
            }
        }
        if !pointer.path.metadata().unwrap().is_dir() {
            break;
        };
    }
    Ok(())
}

#[derive(Debug)]
struct Node {
    path: PathBuf,
    children: Vec<Node>,
}

fn get_children(node: &mut Node) {
    let path = &node.path;
    if  !path.metadata().unwrap().is_dir() {
        return;
    }
    let contents = read_dir(path).unwrap();
    for content in contents { 
        let entry = content.unwrap();
        node.children.push(Node { path: entry.path(), children: vec![] });
    }

    for child in &mut node.children {
        get_children(child);
    }
}