use aeroapi::data::{commands::Controller, commons::Vec3d};
use ratatui::{
    buffer::Buffer, layout::{
        Alignment, 
        Constraint, 
        Direction, 
        Layout, Rect,
    }, style::{Color, Style, Stylize}, symbols::border, text::{
        Line, Span, Text
    }, widgets::{
        block::{
            Position, 
            Title
        }, Block, Borders, Gauge, Padding, Paragraph, Widget
    }, Frame
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

    // let instrument_widget = instrument_widget(&app.sensors);
    let controls_widget = ControlsWidget::new(&app.controller);
    //         instrument_widget,
    //         outer_layout[0]);

    let instrument_widget = InstrumentWidget::new(
        app.sensors.magnetometer(),
        app.sensors.altimeter(),
        None, 
        None, 
    );
    
    frame.render_widget(instrument_widget, outer_layout[0]);
    // ins.render(outer_layout[0], frame.buffer_mut());
    frame.render_widget(main_window,
        inner_layout[0]);
        frame.render_widget(controls_widget, inner_layout[1]);    // frame.render_widget(

}


pub struct ControlsWidget<'a> {
    block: Block<'a>,
    controller: &'a Controller,
}

impl<'a> ControlsWidget<'a> {
    pub fn new(controller: &'a Controller) -> Self {
        Self {
            block: Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title("Flight Controls"),
            controller,
        }
    }

    fn create_gauge(label: &str, value: f32, color: Color) -> Gauge {
        Gauge::default()
            .block(Block::default().title(label))
            .gauge_style(Style::default().fg(color))
            .percent((value * 100.0) as u16)
    }
}

impl Widget for ControlsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_area = self.block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(inner_area);

        let throttle_gauge = Self::create_gauge("Throttle", self.controller.channels.throttle as f32 / 100 as f32, Color::Green);
        let yaw_gauge = Self::create_gauge("Yaw", self.controller.channels.yaw as f32 / 100 as f32, Color::Blue);
        let pitch_gauge = Self::create_gauge("Pitch", self.controller.channels.pitch as f32 / 100 as f32, Color::Magenta);
        let roll_gauge = Self::create_gauge("Roll", self.controller.channels.roll as f32 / 100 as f32, Color::Cyan);

        throttle_gauge.render(chunks[0], buf);
        yaw_gauge.render(chunks[1], buf);
        pitch_gauge.render(chunks[2], buf);
        roll_gauge.render(chunks[3], buf);

        let instructions = Line::from(vec![
            Span::styled("▲ Increase", Style::default().fg(Color::Green)),
            Span::raw(" | "),
            Span::styled("▼ Decrease", Style::default().fg(Color::Red)),
            Span::raw(" | "),
            Span::styled("Tab", Style::default().fg(Color::Yellow)),
            Span::raw(": Switch Control"),
        ]);

        Paragraph::new(instructions)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[4], buf);

        self.block.render(area, buf);
    }
}



pub struct InstrumentWidget<'a> {
    block: Option<Block<'a>>,
    magnetometer: Option<Vec3d>,
    altimeter: Option<f32>,
    heading: Option<f32>,
    airspeed: Option<f32>,
}

impl<'a> InstrumentWidget<'a> {
    pub fn new(
        magnetometer: Option<Vec3d>,
        altimeter: Option<f32>,
        heading: Option<f32>,
        airspeed: Option<f32>,
    ) -> Self {
        Self {
            block: Some(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))),
            magnetometer,
            altimeter,
            heading,
            airspeed,
        }
    }

    fn format_value<T: std::fmt::Debug>(value: Option<T>, unit: &str) -> Span<'a> {
        match value {
            Some(v) => Span::styled(format!("{:?} {}", v, unit), Style::default().fg(Color::Green)),
            None => Span::styled("N/A", Style::default().fg(Color::Red)),
        }
    }
}

impl Widget for InstrumentWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.block.unwrap_or_default();
        let inner_area = block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(inner_area);

        let title = Line::from(vec![
            Span::styled("Flight Instruments", Style::default().fg(Color::Yellow).bold()),
        ]);

        let altitude = Line::from(vec![
            Span::raw("Altitude: "),
            Self::format_value(self.altimeter, "m"),
        ]);

        let heading = Line::from(vec![
            Span::raw("Heading: "),
            Self::format_value(self.heading, "°"),
        ]);

        let airspeed = Line::from(vec![
            Span::raw("Airspeed: "),
            Self::format_value(self.airspeed, "m/s"),
        ]);

        let magnetometer = match self.magnetometer {
            Some(mag) => Line::from(vec![
                Span::raw("Magnetometer: "),
                Span::styled(
                    format!("X:{:.2} Y:{:.2} Z:{:.2}", mag.x, mag.y, mag.z),
                    Style::default().fg(Color::Magenta),
                ),
            ]),
            None => Line::from(vec![
                Span::raw("Magnetometer: "),
                Span::styled("N/A", Style::default().fg(Color::Red)),
            ]),
        };

        Paragraph::new(title).render(chunks[0], buf);
        Paragraph::new(altitude).render(chunks[1], buf);
        Paragraph::new(heading).render(chunks[2], buf);
        Paragraph::new(airspeed).render(chunks[3], buf);
        Paragraph::new(magnetometer).render(chunks[4], buf);

        block.render(area, buf);
    }
}
