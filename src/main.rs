use std::collections::HashMap;
use std::time::Duration;

use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Focus {
    Keys,
    Items,
}

struct App {
    data: HashMap<String, Vec<String>>,
    keys: Vec<String>,
    keys_state: ListState,
    item_state: ListState,
    focus: Focus,
    keys_filter: String,
    items_filter: String,
}

impl App {
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert(
            "Fruits".to_string(),
            vec!["Apple".into(), "Orange".into(), "Banana".into()],
        );
        data.insert(
            "Vegetables".to_string(),
            vec!["Letuce".into(), "Ruccola".into(), "Potatoe".into()],
        );
        data.insert(
            "Lacteo".to_string(),
            vec!["Milk".into(), "Yogurt".into(), "Cheese".into()],
        );

        let mut keys: Vec<_> = data.keys().cloned().collect();
        keys.sort();

        let mut keys_state = ListState::default();
        keys_state.select(Some(0));

        let mut item_state = ListState::default();
        item_state.select(Some(0));

        Self {
            data,
            keys,
            keys_state,
            item_state,
            focus: Focus::Keys,
            keys_filter: String::new(),
            items_filter: String::new(),
        }
    }

    fn filtered_keys(&self) -> Vec<String> {
        let needle = self.keys_filter.to_lowercase();
        self.keys
            .iter()
            .filter(|k| k.to_lowercase().contains(&needle))
            .cloned()
            .collect()
    }

    fn selected_key(&self) -> Option<String> {
        let filtered_keys = self.filtered_keys();
        self.keys_state
            .selected()
            .and_then(|i| filtered_keys.get(i).cloned())
    }

    fn filtered_items(&self) -> Vec<String> {
        let needle = self.items_filter.to_lowercase();
        self.selected_key()
            .and_then(|k| self.data.get(&k))
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|item| item.to_lowercase().contains(&needle))
            .collect()
    }

    fn toogle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Items => Focus::Keys,
            Focus::Keys => Focus::Items,
        }
    }

    fn next(&mut self) {
        match self.focus {
            Focus::Keys => {
                let len = self.filtered_keys().len();
                if len == 0 {
                    return;
                }
                let i = self.keys_state.selected().map_or(0, |i| (i + 1) % len);
                self.keys_state.select(Some(i));
                self.item_state.select(Some(0));
            }
            Focus::Items => {
                let len = self.filtered_items().len();
                if len == 0 {
                    return;
                }
                let i = self.item_state.selected().map_or(0, |i| (i + 1) % len);
                self.item_state.select(Some(i));
            }
        }
    }

    fn previous(&mut self) {
        match self.focus {
            Focus::Keys => {
                let len = self.filtered_keys().len();
                if len == 0 {
                    return;
                }
                let i = self
                    .keys_state
                    .selected()
                    .map_or(0, |i| (i + len - 1) % len);
                self.keys_state.select(Some(i));
                self.item_state.select(Some(0));
            }
            Focus::Items => {
                let len = self.filtered_items().len();
                if len == 0 {
                    return;
                }
                let i = self
                    .item_state
                    .selected()
                    .map_or(0, |i| (i + len - 1) % len);
                self.item_state.select(Some(i));
            }
        }
    }

    fn push_filter_char(&mut self, c: char) {
        match self.focus {
            Focus::Keys => {
                self.keys_filter.push(c);
                self.keys_state.select(Some(0));
                self.item_state.select(Some(0));
            }
            Focus::Items => {
                self.items_filter.push(c);
                self.item_state.select(Some(0));
            }
        }
    }

    fn pop_filter_char(&mut self) {
        match self.focus {
            Focus::Keys => {
                self.keys_filter.pop();
                self.keys_state.select(Some(0));
                self.item_state.select(Some(0));
            }
            Focus::Items => {
                self.items_filter.pop();
                self.item_state.select(Some(0));
            }
        }
    }

    fn current_filter_is_active(&self) -> bool {
        match self.focus {
            Focus::Keys => !self.keys_filter.is_empty(),
            Focus::Items => !self.items_filter.is_empty(),
        }
    }

    fn clear_current_filter(&mut self) {
        match self.focus {
            Focus::Keys => self.keys_filter.clear(),
            Focus::Items => self.items_filter.clear(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            break;
                        }
                        KeyCode::Esc => {
                            if app.current_filter_is_active() {
                                app.clear_current_filter();
                            } else {
                                break;
                            }
                        }
                        KeyCode::Backspace => app.pop_filter_char(),
                        KeyCode::Char(c) => app.push_filter_char(c),
                        KeyCode::Tab => app.toogle_focus(),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        _ => {}
                    }
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(5),
        ])
        .split(f.area());

    render_header(f, rows[0]);
    render_body(f, rows[1], app);
    render_footer(f, rows[2], app);
}

fn render_header(f: &mut Frame, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let title = Paragraph::new("My Ratatui App")
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        )
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, cols[0]);

    let date = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
    let date_widget = Paragraph::new(date)
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(date_widget, cols[1]);
}

fn render_body(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    let keys_focused = app.focus == Focus::Keys;
    let keys_items: Vec<ListItem> = app.filtered_keys().into_iter().map(ListItem::new).collect();

    let keys_title = filter_title("Keys", &app.keys_filter, keys_focused);

    let keys_list = List::new(keys_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(keys_title)
                .border_style(border_style(keys_focused)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(keys_list, cols[0], &mut app.keys_state);

    // Render the values of the keys
    let items_focused = app.focus == Focus::Items;
    let items: Vec<ListItem> = app
        .filtered_items()
        .into_iter()
        .map(ListItem::new)
        .collect();

    let title = app
        .selected_key()
        .map(|k| format!("Elements of: {}", k))
        .unwrap_or_else(|| "Elements".to_string());

    let items_title = filter_title(&title, &app.items_filter, items_focused);

    let items_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(items_title)
                .border_style(border_style(items_focused)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("-> ");

    f.render_stateful_widget(items_list, cols[1], &mut app.item_state);
}

fn border_style(focused: bool) -> Style {
    if focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let focus_txt = match app.focus {
        Focus::Keys => "KEYS",
        Focus::Items => "VALUES",
    };

    let line = Line::from(vec![
        Span::raw(
            "Tab: Change Column | ↑/↓: Move up/down | write to apply a filter | Backspace to delete from filter | ESC to remove filter or exit | Current focus: ",
        ),
        Span::styled(
            focus_txt,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let footer = Paragraph::new(line).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}

fn filter_title(base: &str, filter: &str, focused: bool) -> String {
    if filter.is_empty() {
        base.to_string()
    } else {
        let cursor = if focused { "|" } else { "" };
        format!("{}[/{}{}]", base, filter, cursor)
    }
}
