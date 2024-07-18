use ratatui::{
    layout::Alignment, style::{Color, Style, Stylize}, symbols::border, text::{Line, Text}, widgets::{block::{Position, Title}, Block, BorderType, Paragraph, Widget}, Frame
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let title = Title::from(" AeroBridge ".bold());
    let quit_instruction = vec!["[".into(),"Quit ".into(),"<Ctrl-C>".blue().bold(), "]".into()];
    let decrement_instruction = vec!["[".into(), "Decrement ".into(),"<Left>".blue().bold(), "]".into()];
    let increment_instruction = vec!["[".into(), "Increment ".into(),"<Right>".blue().bold(), "]".into()];
    
    let instructions = Title::from(Line::from(vec![
        quit_instruction,
        decrement_instruction,
        increment_instruction,
    ].concat()));

    let block = Block::bordered()
    .title(title.alignment(Alignment::Center))
    .title(
        instructions
            .alignment(Alignment::Center)
            .position(Position::Bottom),
    )
    .border_set(border::ROUNDED);

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        app.message.clone().yellow(),
    ])]);

    let main_window = Paragraph::new(counter_text)
        .centered()
        .block(block);

    frame.render_widget(main_window,
        frame.size(),
    )
}