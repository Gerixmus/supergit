use std::{env::current_dir, fs::read_dir, io::stdout, path::PathBuf, vec};

use crossterm::{
    cursor::{position, Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    style::Stylize,
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

    let _files = select_files(&current_node.children);
    Ok(())
}

fn select_files(items: &Vec<Node>) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(Hide)?;
    println!("{}{}", "? ".green(), "Select items to ignore:");

    let (_, start_row) = position()?;

    let size: usize = if items.len() > 7 { 7 } else { items.len() };
    let mut start = 0;
    let mut end = size;

    print_range(start_row as usize, 0, &items[start..end])?;
    // println!("{}", format!("[↑↓ to move]").cyan());

    let mut selected = 0;
    let mut current = 0;

    loop {
        if let Event::Key(key_event) = read()? {
            if key_event.code == KeyCode::Char('c')
                && key_event.modifiers.contains(KeyModifiers::CONTROL)
            {
                clear_terminal(1 + start_row, items.len() as u16 + start_row)?;
                break;
            };

            if key_event.code == KeyCode::Down
                && key_event.kind == KeyEventKind::Press
                && !items.is_empty()
            {
                selected = (selected + 1) % items.len();
                clear_terminal(start_row, start_row + size as u16)?;
                (start, end, current) = calculate_table(selected, size, items.len());
                print_range(start_row as usize, current, &items[start..end])?;
            }

            if key_event.code == KeyCode::Up
                && key_event.kind == KeyEventKind::Press
                && !items.is_empty()
            {
                selected = (selected + items.len() - 1) % items.len();
                clear_terminal(start_row, start_row + size as u16)?;
                (start, end, current) = calculate_table(selected, size, items.len());
                print_range(start_row as usize, current, &items[start..end])?;
            }

            if key_event.code == KeyCode::Right
                && key_event.kind == KeyEventKind::Press
                && !items.is_empty()
                && items[selected].path.is_dir()
            {
                clear_terminal(start_row, items.len() as u16 + start_row)?;
                stdout.execute(MoveTo(0, start_row - 1))?;
                select_files(&items[selected].children)?;
                print_range(start_row as usize, current, &items[start..end])?;
            }

            if key_event.code == KeyCode::Left && key_event.kind == KeyEventKind::Press {
                clear_terminal(0 + start_row, items.len() as u16 + start_row)?;
                stdout.execute(MoveTo(0, start_row - 1))?;
                return Ok(());
            }
        }
    }
    stdout.execute(MoveTo(0, start_row))?;
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn calculate_table(selected: usize, size: usize, len: usize) -> (usize, usize, usize) {
    if size < 7 {
        (0, size, selected)
    } else if selected + 4 <= len && selected >= 3 {
        (selected - 3, size + selected - 3, 3)
    } else if selected < 3 {
        (0, size, selected)
    } else {
        (len - size, len, selected - size + 1)
    }
}

fn print_range(start: usize, current: usize, items: &[Node]) -> std::io::Result<()> {
    for i in 0..items.len() {
        stdout().execute(MoveTo(0, i as u16 + start as u16))?;
        if current == i {
            println!("{}{}", "> ".cyan(), items[i].file_name().cyan());
        } else {
            println!("  {}", items[i].file_name())
        }
    }
    // println!("{}", format!("[↑↓ to move]").cyan());
    Ok(())
}

fn clear_terminal(start: u16, end: u16) -> std::io::Result<()> {
    for i in start..end {
        stdout().execute(MoveTo(0, i))?;
        stdout().execute(Clear(ClearType::CurrentLine))?;
    }
    Ok(())
}

#[derive(Debug)]
struct Node {
    path: PathBuf,
    children: Vec<Node>,
}

impl Node {
    fn file_name(&self) -> &str {
        self.path
            .file_name()
            .expect("no file name")
            .to_str()
            .expect("invalid UTF-8 in file name")
    }
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
