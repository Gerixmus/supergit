use std::ops::Range;
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
        is_selected: false,
    };
    create_file_tree(&mut root);

    let current_node = &mut root;

    let _files = select_files(&mut current_node.children);
    Ok(())
}

const MAX_SIZE: usize = 7;

fn select_files(items: &mut Vec<Node>) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(Hide)?;
    println!("{}{}", "? ".green(), "Select items to ignore:");

    let (_, start_row) = position()?;

    let size: usize = items.len().min(MAX_SIZE);
    let mut range = 0..size;

    print_range(start_row as usize, 0, &items[range.start..range.end])?;

    let mut selected = 0;
    let mut highlight_row = 0;

    loop {
        if let Event::Key(key_event) = read()? {
            if key_event.code == KeyCode::Char('c')
                && key_event.modifiers.contains(KeyModifiers::CONTROL)
            {
                clear_terminal(start_row, start_row + size as u16 + 1)?;
                break;
            };

            if key_event.code == KeyCode::Down
                && key_event.kind == KeyEventKind::Press
                && !items.is_empty()
            {
                selected = (selected + 1) % items.len();
                clear_terminal(start_row, start_row + size as u16 + 1)?;
                (range, highlight_row) = calculate_table(selected, size, items.len());
                print_range(
                    start_row as usize,
                    highlight_row,
                    &items[range.start..range.end],
                )?;
            }

            if key_event.code == KeyCode::Up
                && key_event.kind == KeyEventKind::Press
                && !items.is_empty()
            {
                selected = (selected + items.len() - 1) % items.len();
                clear_terminal(start_row, start_row + size as u16 + 1)?;
                (range, highlight_row) = calculate_table(selected, size, items.len());
                print_range(
                    start_row as usize,
                    highlight_row,
                    &items[range.start..range.end],
                )?;
            }

            if key_event.code == KeyCode::Right
                && key_event.kind == KeyEventKind::Press
                && !items.is_empty()
                && items[selected].path.is_dir()
            {
                clear_terminal(start_row, start_row + size as u16 + 1)?;
                stdout.execute(MoveTo(0, start_row - 1))?;
                select_files(&mut items[selected].children)?;
                print_range(
                    start_row as usize,
                    highlight_row,
                    &items[range.start..range.end],
                )?;
            }

            if key_event.code == KeyCode::Char(' ')
                && key_event.kind == KeyEventKind::Press
                && !items.is_empty()
            {
                items[selected].is_selected = !items[selected].is_selected;
                clear_terminal(start_row, start_row + size as u16 + 1)?;
                print_range(
                    start_row as usize,
                    highlight_row,
                    &items[range.start..range.end],
                )?;
            }

            if key_event.code == KeyCode::Left && key_event.kind == KeyEventKind::Press {
                clear_terminal(start_row, start_row + size as u16 + 1)?;
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

fn calculate_table(selected: usize, size: usize, len: usize) -> (Range<usize>, usize) {
    let half = MAX_SIZE / 2;
    if size < MAX_SIZE || len == MAX_SIZE || selected <= half {
        (0..size, selected)
    } else if selected >= len - half {
        (len - size..len, selected - (len - size))
    } else {
        (selected - half..size + selected - half, half)
    }
}

fn print_range(start: usize, highlight_row: usize, items: &[Node]) -> std::io::Result<()> {
    for i in 0..items.len() {
        stdout().execute(MoveTo(0, i as u16 + start as u16))?;
        let selected = if items[i].is_selected { "[x]" } else { "[ ]"};
        if highlight_row == i {
            println!("{} {} {}", ">".cyan(), selected.cyan(), items[i].file_name().cyan());
        } else {
            println!("  {} {}", selected, items[i].file_name())
        }
    }
    println!(
        "{}",
        format!("[↑↓ to move, → to expand, ← to collapse]").cyan()
    );
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
    is_selected: bool,
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

fn create_file_tree(node: &mut Node) {
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
            is_selected: false,
        });
    }

    for child in &mut node.children {
        create_file_tree(child);
    }
}
