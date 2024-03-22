//! main.rs 是用来测试的，接口别放这里

/* use std::io;

mod draw;
use draw::*;

fn main() -> io::Result<()> {
    let mut resources = Resources::new();

    resources.add(&Bar { c: 69, d: 2131283 });

    let foo = Foo {
        a: 114514,
        bar: Bar { c: 69, d: 420 },
    };

    resources.add(&foo);

    resources.run()?;

    println!("{:?}", foo);
    Ok(())
}

#[allow(unused)]
#[derive(Drawable, Debug)]
struct Foo {
    a: i32,
    bar: Bar,
}

#[allow(unused)]
#[derive(Drawable, Debug)]
struct Bar {
    c: i32,
    d: i32,
}
 */

 use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // 设置层级菜单的状态
    let mut state = ListState::default();
    state.select(Some(0));
    let items = vec![
        ListItem::new("Foo"),
        ListItem::new("Bar"),
        ListItem::new("Baz"),
    ];

    // 设置事件处理
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            if last_tick.elapsed() >= Duration::from_millis(500) {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                last_tick = Instant::now();
            }
            // 发送输入事件
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    if tx.send(Event::Input(key)).is_err() {
                        break;
                    }
                }
            }
        }
    });

    loop {
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    // 按'q'退出
                    break;
                }
                KeyCode::Char('s') => {
                    let i = match state.selected() {
                        Some(i) => {
                            if i >= items.len() - 1 { 0 } else { i + 1 }
                        }
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Char('w') => {
                    let i = match state.selected() {
                        Some(i) => {
                            if i == 0 { items.len() - 1 } else { i - 1 }
                        }
                        None => 0,
                    };
                    state.select(Some(i));
                }
                _ => {}
            },
            Event::Tick => {
                // 每秒更新UI
                terminal.draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                        .split(f.size());

                    let list = List::new(items.clone())
                        .block(Block::default().title("Items").borders(Borders::ALL))
                        .highlight_style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD))
                        .highlight_symbol(">> ");

                    f.render_stateful_widget(list, chunks[0], &mut state);

                    if let Some(selected) = state.selected() {
                        let details = match selected {
                            0 => vec![ListItem::new("a: 1")],
                            1 => vec![ListItem::new("b: 2")],
                            _ => vec![ListItem::new("c: 3")],
                        };

                        let details_list = List::new(details)
                            .block(Block::default().title("Details").borders(Borders::ALL));

                        f.render_widget(details_list, chunks[1]);
                    }
                })?;
            }
        }
    }

    // 退出前恢复终端状态
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
