use std::{sync::Arc, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListDirection, ListState, Tabs},
    DefaultTerminal, Frame,
};
use tokio::sync::RwLock;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetEvent, TuiWidgetState};

use super::protocol::{
    node::{OceanProtocol, PostWatchers, ProfileWatchers},
    watchers::profiles::{ProfileChangeWatcher, ProfileEventStream, ProfileWatcherEvent},
};

static TAB_NAMES: &[&str] = &["Logger", "Watched Profiles", "Watched Posts"];

pub struct SpoutTui {
    pub logger_state: TuiWidgetState,
    pub profiles_list_state: ListState,
    pub posts_list_state: ListState,

    pub selected_tab: usize,
    pub is_running: bool,
    pub ocean_protocol: OceanProtocol,
}

impl SpoutTui {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        let mut profile_event_streams = {
            let locked = self.ocean_protocol.profile_watchers.blocking_read();
            let res = locked
                .iter()
                .map(|v| v.rx.resubscribe())
                .collect::<Vec<_>>();
            drop(locked);
            println!("dropped locked");
            res
        };
        let mut profile_events = vec![];
        loop {
            self.handle_ui_event();
            if !self.is_running {
                break;
            }

            for stream in profile_event_streams.iter_mut() {
                match stream.try_recv() {
                    Ok(event) => profile_events.push(event),
                    Err(_) => continue,
                }
            }

            terminal.draw(|frame| self.draw(frame, &profile_events))?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, profile_events: &Vec<ProfileWatcherEvent>) {
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
            .state(&self.logger_state);

        let tabs = Tabs::new(TAB_NAMES.iter().cloned())
            .block(Block::default().title("States").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .select(self.selected_tab);

        frame.render_widget(title, title_area);
        frame.render_widget(tabs, tab_area);

        if self.selected_tab == 0 {
            frame.render_widget(logger, body_area);
        } else if self.selected_tab == 1 {
            let [left, right] =
                Layout::vertical(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                    .areas(body_area);

            let locked = self.ocean_protocol.profile_watchers.blocking_read();
            let profile_list = locked
                .iter()
                .map(|v| v.namespace_id.to_string())
                .collect::<Vec<_>>();
            let profile_list = List::new(profile_list)
                .block(Block::bordered().title("Profiles"))
                .style(Style::new().white())
                .highlight_style(Style::new().bg(Color::Blue).italic())
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);
            let profile_event_list = profile_events
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>();

            let profile_event_list = List::new(profile_event_list)
                .block(Block::bordered().title("Events"))
                .style(Style::new().white())
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);
            // let list = List::new()
            // .block(Block::bordered().title("Events"))
            // .style(Style::new().white())
            // .highlight_style(Style::new().bg(Color::Blue).italic())
            // .highlight_symbol(">>")
            // .repeat_highlight_symbol(true)
            // .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(profile_list, left, &mut self.profiles_list_state);
            frame.render_widget(profile_event_list, right);
        } else if self.selected_tab == 2 {
            let [left, right] =
                Layout::vertical(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                    .areas(body_area);

            let locked = self.ocean_protocol.post_watchers.blocking_read();
            let list = locked
                .iter()
                .map(|v| {
                    let last_active = v
                        .last_active
                        .blocking_read()
                        .map(|v| format!("Seen {}", v.format("%d/%m %H:%M").to_string()))
                        .unwrap_or("Not seen this session".to_string());

                    format!("{} - {}", v.namespace_id.to_string(), last_active)
                })
                .collect::<Vec<_>>();
            let list = List::new(list)
                .block(Block::bordered().title("List"))
                .style(Style::new().white())
                .highlight_style(Style::new().bg(Color::Blue).italic())
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(list, left, &mut self.posts_list_state);
        }

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

        if self.selected_tab == 0 {
            self.handle_event_for_logger(event);
        } else if self.selected_tab == 1 {
            self.handle_event_for_profiles(event);
        } else if self.selected_tab == 2 {
            self.handle_event_for_posts(event);
        }
    }

    fn handle_event_for_profiles(&mut self, event: Event) {
        if let Event::Key(key) = event {
            let code = key.code;

            match code.into() {
                KeyCode::Char('Q') => self.is_running = false,
                KeyCode::Tab => self.next_tab(),
                KeyCode::Esc => self.profiles_list_state.select(None),
                KeyCode::Up => self.profiles_list_state.select_previous(),
                KeyCode::Down => self.profiles_list_state.select_next(),
                _ => (),
            }
        }
    }

    fn handle_event_for_posts(&mut self, event: Event) {
        if let Event::Key(key) = event {
            let code = key.code;

            match code.into() {
                KeyCode::Char('Q') => self.is_running = false,
                KeyCode::Tab => self.next_tab(),
                KeyCode::Esc => self.posts_list_state.select(None),
                KeyCode::Up => self.posts_list_state.select_previous(),
                KeyCode::Down => self.posts_list_state.select_next(),
                _ => (),
            }
        }
    }

    fn handle_event_for_logger(&mut self, event: Event) {
        if let Event::Key(key) = event {
            let code = key.code;

            match code.into() {
                KeyCode::Char('Q') => self.is_running = false,
                KeyCode::Tab => self.next_tab(),
                KeyCode::Char(' ') => self.logger_state.transition(TuiWidgetEvent::SpaceKey),
                KeyCode::Esc => self.logger_state.transition(TuiWidgetEvent::EscapeKey),
                KeyCode::PageUp => self.logger_state.transition(TuiWidgetEvent::PrevPageKey),
                KeyCode::PageDown => self.logger_state.transition(TuiWidgetEvent::NextPageKey),
                KeyCode::Up => self.logger_state.transition(TuiWidgetEvent::UpKey),
                KeyCode::Down => self.logger_state.transition(TuiWidgetEvent::DownKey),
                KeyCode::Left => self.logger_state.transition(TuiWidgetEvent::LeftKey),
                KeyCode::Right => self.logger_state.transition(TuiWidgetEvent::RightKey),
                KeyCode::Char('+') => self.logger_state.transition(TuiWidgetEvent::PlusKey),
                KeyCode::Char('-') => self.logger_state.transition(TuiWidgetEvent::MinusKey),
                KeyCode::Char('h') => self.logger_state.transition(TuiWidgetEvent::HideKey),
                KeyCode::Char('f') => self.logger_state.transition(TuiWidgetEvent::FocusKey),
                _ => (),
            }
        }
    }

    fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % TAB_NAMES.len();
    }
}
