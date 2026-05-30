use crate::app::App;
use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(app: &App, frame: &mut Frame) {
    // get current speed
    let wpm = app.calculate_wpm();

    // build string
    let text = format!(
        "WPM: {:.0}\n\n{}\n\nInput: {}\n\n(Esc to quit)",
        wpm, app.target_text, app.user_input
    );

    // print widget
    frame.render_widget(
        Paragraph::new(text)
            // add box
            .block(Block::default().title("Ratatype").borders(Borders::ALL))
            // center text
            .alignment(Alignment::Center),
        // go fullscreen
        frame.area(),
    );
}
