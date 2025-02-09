use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Tabs},
    DefaultTerminal, Frame,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetEvent, TuiWidgetState};

static TAB_NAMES: &[&str] = &["Logger", "Peers"];

pub struct SpoutTui {
    pub state: TuiWidgetState,
    pub selected_tab: usize,
    pub is_running: bool,
}

impl SpoutTui {
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        loop {
            tokio::task::yield_now().await;
            self.handle_ui_event();
            if !self.is_running {
              break;
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ]);

        let [title_area, tab_area, body_area, help_area] = vertical.areas(frame.area());
        let title = Line::from("🌈 Spout Social 🌈").centered().bold();

        let logger = TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(&self.state);

        let tabs = Tabs::new(TAB_NAMES.iter().cloned())
            .block(Block::default().title("States").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .select(self.selected_tab);

        frame.render_widget(title, title_area);
        frame.render_widget(tabs, tab_area);
        frame.render_widget(logger, body_area);

        if help_area.width > 40 {
            let help = Text::from(vec![
                "Q: Quit | Tab: Switch state | ↑/↓: Select target | f: Focus target".into(),
                "←/→: Display level | +/-: Filter level | Space: Toggle hidden targets".into(),
                "h: Hide target selector | PageUp/Down: Scroll | Esc: Cancel scroll".into(),
            ])
            .style(Color::Gray)
            .centered();
            frame.render_widget(help, help_area);
        }
    }

    fn handle_ui_event(&mut self) {
        let Ok(event) = event::poll(Duration::from_millis(10)) else {
            return;
        };
        let event = if event { event::read() } else { return };
        let Ok(event) = event else { return };

        if let Event::Key(key) = event {
            let code = key.code;

            match code.into() {
                KeyCode::Char('Q') => self.is_running = false,
                KeyCode::Char(' ') => self.state.transition(TuiWidgetEvent::SpaceKey),
                KeyCode::Tab => self.next_tab(),
                KeyCode::Esc => self.state.transition(TuiWidgetEvent::EscapeKey),
                KeyCode::PageUp => self.state.transition(TuiWidgetEvent::PrevPageKey),
                KeyCode::PageDown => self.state.transition(TuiWidgetEvent::NextPageKey),
                KeyCode::Up => self.state.transition(TuiWidgetEvent::UpKey),
                KeyCode::Down => self.state.transition(TuiWidgetEvent::DownKey),
                KeyCode::Left => self.state.transition(TuiWidgetEvent::LeftKey),
                KeyCode::Right => self.state.transition(TuiWidgetEvent::RightKey),
                KeyCode::Char('+') => self.state.transition(TuiWidgetEvent::PlusKey),
                KeyCode::Char('-') => self.state.transition(TuiWidgetEvent::MinusKey),
                KeyCode::Char('h') => self.state.transition(TuiWidgetEvent::HideKey),
                KeyCode::Char('f') => self.state.transition(TuiWidgetEvent::FocusKey),
                _ => (),
            }
        }
    }

    fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % TAB_NAMES.len();
    }
}
