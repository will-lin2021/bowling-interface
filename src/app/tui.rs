use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::prelude::Game;

#[derive(Default, Clone, Copy)]
#[repr(u8)]
enum Mode {
    #[default]
    Main = 0,
    NewGame = b'n',
    ModGame = b'm',
    ViewGame = b'v',
    DeleteGame = b'd',
    Exiting(u8) = b'e',
}

#[derive(Default)]
enum Message {
    #[default]
    Noop,
    ModeChange(u8),
    KeyEnter,
    KeyPress(event::KeyEvent),
    Cancel,
    Exit,
}

pub struct App {
    mode: Mode,
    input: Input,
    exit: bool,
    updated: bool,

    game: Option<Game>,
}

impl Default for App {
    fn default() -> Self {
        App {
            mode: Mode::default(),
            input: Input::default(),
            exit: false,
            updated: false,

            game: None,
        }
    }
}

pub fn run(app: &mut App) -> io::Result<()> {
    ui::install_panic_hook();
    let mut terminal = ui::init_terminal()?;

    while !app.exit {
        terminal.draw(|f| ui(f, app))?;

        handle_events(app);
    }

    ui::restore_terminal()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let mut constraints: Vec<Constraint> = vec![Constraint::Max(1), Constraint::Fill(1)];

    let mut main_layout = Layout::default().direction(Direction::Vertical);

    let title_block = Block::new()
        .borders(Borders::TOP)
        .title("Bowling Score Tracker");
    let input_block =
        Paragraph::new(app.input.value()).block(Block::new().borders(Borders::ALL).title("Input"));

    match app.mode {
        Mode::Main => {
            let main_layout = main_layout.constraints(constraints).split(f.size());

            let sub_layout = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ],
            )
            .split(main_layout[1]);

            f.render_widget(title_block, main_layout[0]);
            f.render_widget(
                Block::new().borders(Borders::ALL).title("This Month"),
                sub_layout[0],
            );
            f.render_widget(
                Block::new().borders(Borders::ALL).title("This Year"),
                sub_layout[1],
            );
            f.render_widget(
                Block::new().borders(Borders::ALL).title("Overall"),
                sub_layout[2],
            );
        }
        Mode::NewGame => {
            constraints.push(Constraint::Max(3));

            let main_layout = main_layout.constraints(constraints).split(f.size());

            f.render_widget(title_block, main_layout[0]);
            if app.game.is_none() {
                f.render_widget(
                    Paragraph::new("No game selected: Enter Date")
                        .block(Block::new().borders(Borders::ALL).title("New Game")),
                    main_layout[1],
                )
            } else {
                f.render_widget(
                    Block::new().borders(Borders::ALL).title("New Game"),
                    main_layout[1],
                );
            }
            f.render_widget(input_block, main_layout[2]);
        }
        Mode::ModGame => {
            constraints.push(Constraint::Max(3));

            let main_layout = main_layout.constraints(constraints).split(f.size());

            f.render_widget(title_block, main_layout[0]);
            f.render_widget(
                Block::new().borders(Borders::ALL).title("Modify Game"),
                main_layout[1],
            );
            f.render_widget(input_block, main_layout[2]);
        }
        Mode::ViewGame => {
            constraints.push(Constraint::Max(3));

            let main_layout = main_layout.constraints(constraints).split(f.size());

            f.render_widget(title_block, main_layout[0]);
            f.render_widget(
                Block::new().borders(Borders::ALL).title("View Game"),
                main_layout[1],
            );
            f.render_widget(input_block, main_layout[2]);
        }
        Mode::DeleteGame => {
            constraints.push(Constraint::Max(3));

            let main_layout = main_layout.constraints(constraints).split(f.size());

            f.render_widget(title_block, main_layout[0]);
            f.render_widget(
                Block::new().borders(Borders::ALL).title("Delete Game"),
                main_layout[1],
            );
            f.render_widget(input_block, main_layout[2]);
        }
        Mode::Exiting(_) => {
            let main_layout = main_layout.constraints(constraints).split(f.size());

            let sub_layout = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(20),
                    Constraint::Fill(1),
                    Constraint::Percentage(20),
                ],
            )
            .split(main_layout[1]);

            f.render_widget(title_block, main_layout[0]);
            f.render_widget(
                Paragraph::new("Are you sure?")
                    .block(Block::new().borders(Borders::ALL).title("Exiting...")),
                sub_layout[1],
            );
        }
    }
}

fn handle_events(app: &mut App) {
    if event::poll(Duration::from_millis(0)).unwrap() {
        let msg = match event::read().unwrap() {
            Event::Key(key) => handle_key(key),
            _ => Message::Noop,
        };

        match app.mode {
            Mode::Main => match msg {
                Message::Noop => (),
                Message::ModeChange(mode) => match mode {
                    b'n' => app.mode = Mode::NewGame,
                    b'm' => app.mode = Mode::ModGame,
                    b'v' => app.mode = Mode::ViewGame,
                    b'd' => app.mode = Mode::DeleteGame,
                    _ => (),
                },
                Message::Exit => {
                    app.exit = true;
                }
                _ => (),
            },
            Mode::NewGame => {
                match msg {
                    Message::Noop => (),
                    Message::KeyEnter => {
                        // TODO: Enter score and parse, etc
                        app.input.reset();
                    }
                    Message::KeyPress(key) => {
                        app.input.handle_event(&Event::Key(key));
                    }
                    Message::Cancel => {
                        // TODO: Erase entry in input
                        app.input.reset();
                    }
                    Message::Exit => {
                        app.mode = Mode::Exiting(b'n');
                    }
                    _ => (),
                }
            }
            Mode::ModGame => {
                match msg {
                    Message::Noop => (),
                    Message::KeyEnter => {
                        // TODO
                        app.input.reset();
                    }
                    Message::KeyPress(key) => {
                        app.input.handle_event(&Event::Key(key));
                    }
                    Message::Cancel => {
                        // TODO
                        app.input.reset();
                    }
                    Message::Exit => {
                        app.mode = Mode::Exiting(b'm');
                    }
                    _ => (),
                }
            }
            Mode::ViewGame => {
                match msg {
                    Message::Noop => (),
                    Message::KeyEnter => {
                        // TODO: Go to date entered
                        app.input.reset();
                    }
                    Message::KeyPress(key) => {
                        app.input.handle_event(&Event::Key(key));
                    }
                    Message::Cancel => {
                        // TODO
                        app.input.reset();
                    }
                    Message::Exit => {
                        app.mode = Mode::Exiting(b'v');
                    }
                    _ => (),
                }
            }
            Mode::DeleteGame => {
                match msg {
                    Message::Noop => (),
                    Message::KeyEnter => {
                        // TODO
                        app.input.reset();
                    }
                    Message::KeyPress(key) => {
                        app.input.handle_event(&Event::Key(key));
                    }
                    Message::Cancel => {
                        // TODO
                        app.input.reset();
                    }
                    Message::Exit => {
                        app.mode = Mode::Exiting(b'd');
                    }
                    _ => (),
                }
            }
            Mode::Exiting(mode) => match msg {
                Message::Noop => (),
                Message::Cancel => match mode {
                    b'n' => app.mode = Mode::NewGame,
                    b'm' => app.mode = Mode::ModGame,
                    b'v' => app.mode = Mode::ViewGame,
                    b'd' => app.mode = Mode::DeleteGame,
                    _ => (),
                },
                Message::Exit => {
                    app.mode = Mode::Main;
                }
                _ => (),
            },
        }
    }
}

fn handle_key(key: event::KeyEvent) -> Message {
    match key.code {
        KeyCode::Char('q') => Message::Exit,
        KeyCode::Char('n') => Message::ModeChange(b'n'),
        KeyCode::Char('m') => Message::ModeChange(b'm'),
        KeyCode::Char('v') => Message::ModeChange(b'v'),
        KeyCode::Char('d') => Message::ModeChange(b'd'),
        KeyCode::Char('0'..='9') | KeyCode::Char(' ') | KeyCode::Backspace => {
            Message::KeyPress(key)
        }
        KeyCode::Enter => Message::KeyEnter,
        KeyCode::Esc => Message::Cancel,
        _ => Message::Noop,
    }
}

// TODO: Rename module?
mod ui {
    use crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    };
    use ratatui::prelude::*;
    use std::{
        io::{stdout, Result},
        panic,
    };

    pub fn init_terminal() -> Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
