use std::io::{self, stdout};
pub use tsuika_draw_derive::*;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub trait Drawable {
    fn draw(&self) -> String;
}

#[derive(Default)]
pub struct Resources<'a> {
    resources: Vec<&'a (dyn Drawable)>,
    page: usize,
    cursor: usize,
    choose: usize,
    mode: TsuikaMode,
}

impl<'a> Resources<'a> {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            ..Default::default()
        }
    }

    pub fn add<T: Drawable>(&mut self, resource: &'a T) {
        self.resources.push(resource);
    }

    pub fn draw(&self) -> String {
        self.resources
            .iter()
            .fold(String::new(), |acc, resource| acc + &resource.draw() + "\n")
    }

    fn handle_events(&self) -> io::Result<TsuikaEvent> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(TsuikaEvent::Quit),
                        KeyCode::Char('e') => return Ok(TsuikaEvent::Enter),
                        KeyCode::Char('w') => match self.mode {
                            TsuikaMode::Normal => return Ok(TsuikaEvent::RollUp),
                            TsuikaMode::Edit => return Ok(TsuikaEvent::Up),
                        },
                        KeyCode::Char('s') => match self.mode {
                            TsuikaMode::Normal => return Ok(TsuikaEvent::RollDown),
                            TsuikaMode::Edit => return Ok(TsuikaEvent::Down),
                        },
                        _ => return Ok(TsuikaEvent::default()),
                    }
                }
            }
        }
        Ok(TsuikaEvent::default())
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        loop {
            terminal.draw(|frame| self.ui(frame))?;
            match self.handle_events()? {
                TsuikaEvent::Quit => match self.mode {
                    TsuikaMode::Edit => self.mode = TsuikaMode::Normal,
                    TsuikaMode::Normal => break,
                },
                TsuikaEvent::Enter => {
                    if self.mode == TsuikaMode::Normal {
                        self.mode = TsuikaMode::Edit;
                    }
                }
                TsuikaEvent::Up => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                }
                TsuikaEvent::Down => {
                    if self.cursor < self.resources.len() - 1 {
                        self.cursor += 1;
                    }
                }
                TsuikaEvent::RollUp => {
                    if self.page > 0 {
                        self.page -= 1;
                    }
                }
                TsuikaEvent::RollDown => {
                    self.page += 1;
                }
                TsuikaEvent::Nothing => (),
            }
        }
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame) {
        let draw = self.draw();
        let draw = draw.lines().collect::<Vec<_>>();
        if self.page > draw.len() - 1 {
            self.page = draw.len() - 1;
        }
        frame.render_widget(
            Paragraph::new(draw[self.page..].join("\n"))
                .block(Block::default().borders(Borders::ALL)),
            frame.size(),
        )
    }
}

#[derive(Default)]
pub enum TsuikaEvent {
    #[default]
    Nothing,
    Quit,
    Enter,
    Up,
    Down,
    RollUp,
    RollDown,
}

#[derive(Default, PartialEq)]
pub enum TsuikaMode {
    #[default]
    Normal,
    Edit,
}
