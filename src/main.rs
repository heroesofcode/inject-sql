use clap::Command;
use crossterm::{
	execute,
	terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::io::stdout;

mod add_block;
mod app;
mod response;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
	check_version();
	enable_raw_mode()?;

	let mut stdout = stdout();
	execute!(stdout, EnterAlternateScreen)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let result = app::run_app(&mut terminal).await;

	disable_raw_mode()?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
	terminal.show_cursor()?;

	if let Err(err) = result {
		println!("Error: {:?}", err);
	}

	Ok(())
}

fn check_version() {
	let _app = Command::new("injectsql")
		.version("0.7.0")
		.ignore_errors(true)
		.get_matches();
}
