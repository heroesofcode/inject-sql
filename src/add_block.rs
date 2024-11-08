use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct AddBlock;

impl AddBlock {
    pub fn add_info_text() -> Paragraph<'static> {
        let text = "🌐 🛢️ Command Line Tools to check for SQL Injection vulnerability.";

        let info_text = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Green));

        info_text
    }

    pub fn add_url_block(url: &str) -> Paragraph<'_> {
        let url_block = Paragraph::new(url)
            .block(
                Block::default()
                    .title("Enter the URL")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        url_block
    }

    pub fn add_type_payload_text() -> Paragraph<'static> {
        let text = "\
        \n1 - classical 1\n\
        2 - classical 2\n\
        3 - time-based\n\
        4 - blind 1\n\
        5 - blind 2\n\
        6 - boolean 1\n\
        7 - boolean 2\n\
        8 - Get Database\n";

        let type_payload_text = Paragraph::new(text).style(Style::default().fg(Color::Green));
        type_payload_text
    }

    pub fn add_payload_block(payload_type: &str) -> Paragraph<'_> {
        let payload_block = Paragraph::new(payload_type)
            .block(
                Block::default()
                    .title("Enter the payload type")
                    .borders(Borders::all()),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        payload_block
    }

    pub fn add_result_block(result_text: &str) -> Paragraph<'_> {
        let result_block = Paragraph::new(result_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Result")
                    .borders(Borders::TOP)
                    .style(Style::default().fg(Color::Yellow)),
            )
            .wrap(Wrap { trim: true });

        result_block
    }

    pub fn add_help_text() -> Paragraph<'static> {
        let text = "Use TAB to switch between fields. Press ENTER to validate.";

        let help_text = Paragraph::new(text)
            .style(Style::default().fg(Color::White));

        help_text
    }
}