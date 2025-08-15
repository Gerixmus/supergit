use std::{
    env::current_dir,
    fs::{read_dir}, path::PathBuf,
};

pub fn run_ignore() -> Result<(), String> {
    let dir = current_dir().unwrap();
    println!("Path: {}", dir.display());
    let mut root = Node {
        value: dir,
        children: vec![],
    };
    get_children(&mut root);
    println!("{:?}", root);
    Ok(())
}

#[derive(Debug)]
struct Node {
    value: PathBuf,
    children: Vec<Node>,
}

fn get_children(node: &mut Node) {
    let value = &node.value;
    if  !value.metadata().unwrap().is_dir() {
        return;
    }
    let contents = read_dir(value).unwrap();
    for content in contents { 
        let entry = content.unwrap();
        node.children.push(Node { value: entry.path(), children: vec![] });
    }

    for child in &mut node.children {
        get_children(child);
    }
}