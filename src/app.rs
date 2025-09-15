use crate::add_block::AddBlock;
use crate::response::Response;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
	Terminal,
	layout::{Constraint, Direction, Layout},
};
use std::io;

pub async fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
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

			let info_text = AddBlock::add_info_text();
			f.render_widget(info_text, chunks[0]);

			let url_block = AddBlock::add_url_block(&url);
			f.render_widget(url_block, chunks[1]);

			let type_payload_text = AddBlock::add_type_payload_text();
			f.render_widget(type_payload_text, chunks[2]);

			let payload_block = AddBlock::add_payload_block(&payload_type);
			f.render_widget(payload_block, chunks[3]);

			let result_block = AddBlock::add_result_block(&result_text);
			f.render_widget(result_block, chunks[4]);

			let help_text = AddBlock::add_help_text();
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
		match Response::validation_exist_sql_injection(url, payload_type).await {
			Ok(value) => value.to_string(),
			Err(error) => {
				format!("Request error: {}", error)
			}
		}
	} else {
		"Fields cannot be empty".to_string()
	}
}
