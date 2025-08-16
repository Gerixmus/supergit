use std::{
    env::current_dir,
    fs::read_dir,
    io::{stdout, Write},
    path::PathBuf,
    vec,
};

use crossterm::{
    cursor::{position, Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

pub fn run_ignore() -> Result<(), String> {
    let dir = current_dir().unwrap();
    println!("Path: {}", dir.display());
    let mut root = Node {
        path: dir,
        children: vec![],
    };
    get_children(&mut root);

    let current_node = &root;

    let files: Vec<&str> = current_node
        .children
        .iter()
        .filter_map(|child| child.path.file_name()?.to_str())
        .collect();

    let _files = select_files(files);
    Ok(())
}

fn select_files(items: Vec<&str>) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(Hide)?;

    let (_, start_row) = position()?;

    for (_i, item) in items.iter().enumerate() {
        println!("  {}", item);
    }
    println!("[↑↓ to move]");

    let mut selected = 0;
    stdout.execute(MoveTo(0, start_row + selected as u16))?;
    print!("> {}", items[selected]);
    stdout.flush()?;

    loop {
        if let Event::Key(key_event) = read()? {
            if key_event.code == KeyCode::Char('c')
                && key_event.modifiers.contains(KeyModifiers::CONTROL)
            {
                stdout.execute(MoveTo(0, 0))?;

                for i in 1..items.len() + 1 {
                    stdout.execute(MoveTo(0, start_row + i as u16))?;
                    stdout.execute(Clear(ClearType::CurrentLine))?;
                }

                stdout.execute(MoveTo(0, start_row))?;
                break;
            };

            let prev_selected = selected;

            if key_event.code == KeyCode::Down && key_event.kind == KeyEventKind::Press {
                selected = (selected + 1) % items.len();
            }
            if key_event.code == KeyCode::Up && key_event.kind == KeyEventKind::Press {
                selected = (selected + items.len() - 1) % items.len();
            }
            if prev_selected != selected {
                stdout.execute(MoveTo(0, start_row + prev_selected as u16))?;
                print!("  {}", items[prev_selected]);

                stdout.execute(MoveTo(0, start_row + selected as u16))?;
                print!("> {}", items[selected]);

                stdout.flush()?;
            }
        }
    }
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

#[derive(Debug)]
struct Node {
    path: PathBuf,
    children: Vec<Node>,
}

fn get_children(node: &mut Node) {
    let path = &node.path;
    if !path.metadata().unwrap().is_dir() {
        return;
    }
    let contents = read_dir(path).unwrap();
    for content in contents {
        let entry = content.unwrap();
        node.children.push(Node {
            path: entry.path(),
            children: vec![],
        });
    }

    for child in &mut node.children {
        get_children(child);
    }
}
