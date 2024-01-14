use crate::Event;
use crate::Tui;
use crate::X_OFFSET;
use crossterm::{event::KeyCode};

// NOTE: might be important
// use crossterm::event::KeyEvent;
use crate::editor::{Editor, Mode};

#[warn(unused_imports)]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Padding, List, ListItem, Paragraph, Wrap},
    Frame,
};

// TODO: create layouts for each section


pub fn ui(f: &mut Frame<'_>, editor: &mut Editor){
    // NOTE: IDEA
        // have 3 sections -> Text, Line nums, Command/Status
        // create a thread for each section to be rendered, semaphores

    let wrapper_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
                     Constraint::Min(1),
                     Constraint::Length(2)
        ])
        .split(f.size());

    let num_text_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Constraint::Length(4),
                     Constraint::Min(1)
        ])
        .split(wrapper_layout[0]);
    
    // loop to make text for line nums and file text
    let mut line_nums = "".to_string();
    let mut text_string = "".to_string();

    for (i, line) in editor.lines.lines.iter().enumerate() {
        let mut i_str = (i + 1).to_string();
        i_str.push('\n');
        i_str.push('\r');

        for char in i_str.chars() {
            line_nums.push(char);
        }

        for char in line.text.chars() {
            text_string.push(char);
        }

        text_string.push('\n');
        text_string.push('\r');
    }

    // NOTE: borders are temp

    f.render_widget(Paragraph::new("Normal")
                    .block(Block::default()
                           .borders(Borders::TOP)),
                    wrapper_layout[1]);

    f.render_widget(Paragraph::new(line_nums)
                    .alignment(ratatui::layout::Alignment::Right)
                    .style(Style::default().fg(Color::DarkGray)),
                    num_text_layout[0]);
    
    f.render_widget(Paragraph::new(text_string)
                    .block(Block::default()
                       .padding(Padding::new(1, 0, 0, 0))),
                    num_text_layout[1]);
}



pub fn update(editor: &mut Editor, event: Event, tui: &mut Tui){
    match event {
        Event::Init => {println!("init found");},
        Event::Quit => {println!("quit found");},
        Event::Error => {println!("Error found");},
        Event::Closed => {println!("Closed found");},
        Event::Tick => {},
        Event::Render => {},
        Event::FocusGained => {println!("FocusGained found");},
        Event::FocusLost => {println!("FocusLost found");},
        Event::Paste(_) => {println!("Paste found");},
        Event::Key(key) => {
            // FIX: fix this to allow for more flexability
            match editor.mode {
                Mode::Insert => todo!(),

                Mode::Normal => {
                    match key.code {
                        KeyCode::Char(value) => {
                            // FIX: change to ctrl + q
                            match value {
                                'Q' => editor.should_quit = true,
                                'j' => editor.cursor.move_down(),
                                'k' => editor.cursor.move_up(),
                                'h' => editor.cursor.move_left(),
                                'l' => editor.cursor.move_right(),
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                },
            }
        },
        Event::Mouse(_) => {println!("Mouse found");},
        Event::Resize(x, y) => {
            println!("Resize found");
            tui.size = (x, y);
        },
    }
}

#[test]
fn test_cursor(){
    let filename = "./Cargo.toml";
    let filename = std::path::Path::new(&filename);

    let mut editor = Editor::new(filename).unwrap();
    assert_eq!(editor.cursor.current.1, 0);
    
    editor.cursor.move_down();

    assert_eq!(editor.cursor.current.1, 1);
    
    editor.cursor.move_up();
    assert_eq!(editor.cursor.current.1, 0);

    editor.cursor.move_up();
    assert_eq!(editor.cursor.current.1, 0);
}
