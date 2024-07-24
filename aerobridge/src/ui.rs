use ratatui::{
    layout::{
        Alignment, 
        Constraint, 
        Direction, 
        Layout,
    }, 
    style::Stylize, 
    symbols::border, 
    text::{
        Line, 
        Text
    }, 
    widgets::{
        block::{
            Position, 
            Title
        }, 
        Block, 
        Paragraph
    }, 
    Frame,
};

use crate::app::App;

pub fn instrument_widget(sensors: &aeroapi::data::sensors::Sensors) -> Paragraph {
    let title = Title::from("Instruments".bold());
    let quit_instruction = vec!["[".into(),"Quit ".into(),"<Ctrl-C>".blue().bold(), "]".into()];
    let decrement_instruction = vec!["[".into(), "Decrement ".into(),"<Left>".blue().bold(), "]".into()];
    let increment_instruction = vec!["[".into(), "Increment ".into(),"<Right>".blue().bold(), "]".into()];
    
    let instructions = Title::from(Line::from(vec![
        quit_instruction,
        decrement_instruction,
        increment_instruction,
    ].concat()));

    let sensors_text = Text::from(vec![Line::from(vec![
        "Sensors: ".into(),
        format!("Magnetometer: {:?}\n Altitude: {:?}", sensors.magnetometer(), sensors.altimeter()).yellow(),
    ])]);

    let block = Block::bordered()
    .title(title.alignment(Alignment::Center))
    .title(
        instructions
            .alignment(Alignment::Center)
            .position(Position::Bottom),
    )
    .border_set(border::ROUNDED);

    Paragraph::new(sensors_text)
                .block(block)
}

fn controls_widget(controls: &aeroapi::data::commands::Controller) -> Paragraph {
    let title = Title::from("Controls".bold());
    let quit_instruction = vec!["[".into(),"Quit ".into(),"<Ctrl-C>".blue().bold(), "]".into()];
    let decrement_instruction = vec!["[".into(), "Decrement ".into(),"<Left>".blue().bold(), "]".into()];
    let increment_instruction = vec!["[".into(), "Increment ".into(),"<Right>".blue().bold(), "]".into()];
    
    let instructions = Title::from(Line::from(vec![
        quit_instruction,
        decrement_instruction,
        increment_instruction,
    ].concat()));

    let controls_text = Text::from(vec![Line::from(vec![
        "Controls: ".into(),
        format!("Throttle: {:?}\n Yaw: {:?}\n Pitch: {:?}\n Roll: {:?}", controls.channels.throttle, controls.channels.yaw, controls.channels.pitch, controls.channels.roll).yellow(),
    ])]);

    let block = Block::bordered()
    .title(title.alignment(Alignment::Center))
    .title(
        instructions
            .alignment(Alignment::Center)
            .position(Position::Bottom),
    )
    .border_set(border::ROUNDED);

    Paragraph::new(controls_text)
                .block(block)
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(frame.size());

    let inner_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(25),
        Constraint::Percentage(75),
    ])
    .split(outer_layout[1]);

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

    let main_window = Paragraph::new("Main Window")
        .centered()
        .block(block.clone());

    let instrument_widget = instrument_widget(&app.sensors);
    let controls_widget = controls_widget(&app.controller);
    frame.render_widget(
            instrument_widget,
            outer_layout[0]);
    frame.render_widget(main_window,
        inner_layout[0]);
    frame.render_widget(
        controls_widget,
        inner_layout[1]);
}
