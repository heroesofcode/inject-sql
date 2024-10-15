use clap::Command;
use std::io;
use tokio;

use ratatui::{
	backend::CrosstermBackend,
	layout::{Alignment, Constraint, Direction, Layout},
	style::{Color, Style},
	widgets::{Block, Borders, Paragraph, Wrap},
	Terminal,
};

use crossterm::{
	event::{self, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::stdout;

mod response;
use response::validation_exist_sql_injection;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
	check_version();
	enable_raw_mode()?;
	let mut stdout = stdout();
	execute!(stdout, EnterAlternateScreen)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let result = run_app(&mut terminal).await;

	disable_raw_mode()?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
	terminal.show_cursor()?;

	if let Err(err) = result {
		println!("{:?}", err);
	}

	Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
	let mut url = String::new();
	let mut payload_type = String::new();
	let mut result_text = String::new();
	let mut current_field = 0;

	loop {
		terminal.draw(|f| {
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints(
					[
                        Constraint::Percentage(10),
						Constraint::Percentage(10),
                        Constraint::Percentage(30),
						Constraint::Percentage(10),
						Constraint::Percentage(10),
						Constraint::Percentage(10),
					]
					.as_ref(),
				)
				.split(f.area());

			let info_text = Paragraph::new(
                "🌐 🛢️ Command Line Tools to check for SQL Injection vulnerability.\n👨‍💻 https://github.com/heroesofcode/inject-sql")
                .block(
					Block::default()
						.borders(Borders::ALL),
				)
				.style(Style::default().fg(Color::Green));
			f.render_widget(info_text, chunks[0]);

			let url_block = Paragraph::new(&url[..])
				.block(
					Block::default()
						.title("Enter the URL")
						.borders(Borders::ALL),
				)
				.style(Style::default().fg(Color::White))
				.wrap(Wrap { trim: true });
			f.render_widget(url_block, chunks[1]);

            let type_payload_text = Paragraph::new(
                "\n1 - classical 1\n2 - classical 2\n3 - time-based\n4 - blind 1\n5 - blind 2\n6 - boolean 1\n7 - boolean 2\n")
				.style(Style::default().fg(Color::Green));
			f.render_widget(type_payload_text, chunks[2]);

			let payload_block = Paragraph::new(&payload_type[..])
				.block(
					Block::default()
						.title("Enter the payload type")
						.borders(Borders::all())
				)
                .style(Style::default().fg(Color::White))
				.wrap(Wrap { trim: true });
			f.render_widget(payload_block, chunks[3]);

			let result_block = Paragraph::new(&result_text[..])
				.alignment(Alignment::Center)
				.block(
					Block::default()
						.title("Result")
						.borders(Borders::TOP)
						.style(Style::default().fg(Color::Yellow)),
				)
				.wrap(Wrap { trim: true });
			f.render_widget(result_block, chunks[4]);

			let help_text = Paragraph::new(
                "Use TAB to switch between fields. Press ENTER to validate.")
				.style(Style::default().fg(Color::White));
			f.render_widget(help_text, chunks[5]);
		})?;

		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Char(c) => {
					if current_field == 0 {
						url.push(c);
					} else {
						payload_type.push(c);
					}
				}
				KeyCode::Backspace => {
					if current_field == 0 {
						url.pop();
					} else {
						payload_type.pop();
					}
				}
				KeyCode::Tab => {
					current_field = (current_field + 1) % 2;
				}
				KeyCode::Enter => {
					result_text = show_result(&url, &payload_type).await;
				}
				KeyCode::Esc => return Ok(()),
				_ => {}
			}
		}
	}
}

async fn show_result(url: &str, payload_type: &str) -> String {
    if !url.is_empty() && !payload_type.is_empty() {
        match validation_exist_sql_injection(&url, &payload_type).await {
            Ok(value) => {
                return format!("{}", value);
            }
            Err(error) => {
               return  format!("{}", error);
            }
        }
    } else {
        return "Fields cannot be empty".to_string();
    }
}

fn check_version() {
	let _app = Command::new("injectsql")
		.version("0.2.0")
		.ignore_errors(true)
		.get_matches();
}
