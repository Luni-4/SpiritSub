//mod ass_renderer;
mod subs_viewer;

use iced::{
    executor, keyboard, pane_grid, Application, Command, Container, Element, Length, PaneGrid, Row,
    Settings, Subscription, Text,
};

use iced_native::{event, subscription, Event};

use subs_viewer::SubsViewer;

pub fn main() -> iced::Result {
    SpiritSub::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct SpiritSub {
    panes_state: pane_grid::State<PaneState>,
    focus: pane_grid::Pane,
}

#[derive(Debug, Clone)]
enum Message {
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    SubsViewer(subs_viewer::Message, pane_grid::Pane),
}

impl Application for SpiritSub {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (mut panes_state, main_pane) =
            pane_grid::State::new(PaneState::AudioViewer("Audio Viewer!".to_owned()));

        let focus = if let Some((pane, _)) = panes_state.split(
            pane_grid::Axis::Horizontal,
            &main_pane,
            PaneState::SubsViewer(SubsViewer::new()),
        ) {
            pane
        } else {
            main_pane
        };

        (SpiritSub { panes_state, focus }, Command::none())
    }

    fn title(&self) -> String {
        String::from("SpiritSub Editor")
    }

    fn update(&mut self, message: Message, _clipboard: &mut iced::Clipboard) -> Command<Message> {
        match message {
            Message::FocusAdjacent(direction) => {
                if let Some(adjacent) = self.panes_state.adjacent(&self.focus, direction) {
                    self.focus = adjacent;
                }
            }
            Message::Clicked(pane) => {
                self.focus = pane;
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes_state.resize(&split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes_state.swap(&pane, &target);
            }
            Message::Dragged(_) => {}
            Message::SubsViewer(message, pane) => {
                if let Some(PaneState::SubsViewer(subs)) = self.panes_state.get_mut(&pane) {
                    subs.update(message);
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) if modifiers.is_command_pressed() => handle_hotkey(key_code),
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {
        let focus = self.focus;
        let pane_grid = PaneGrid::new(&mut self.panes_state, |pane, state| {
            let is_focused = focus == pane;

            let title_bar = pane_grid::TitleBar::new(Row::new())
                .padding(5)
                .style(style::TitleBar { is_focused });

            pane_grid::Content::new(match state {
                PaneState::AudioViewer(text) => Container::new(Text::new(text.as_str()))
                    .style(style::Pane { is_focused: true })
                    .into(),
                PaneState::SubsViewer(subs_viewer) => subs_viewer
                    .view()
                    .map(move |message| Message::SubsViewer(message, pane)),
            })
            .title_bar(title_bar)
            .style(style::Pane { is_focused })
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<Message> {
    use keyboard::KeyCode;
    use pane_grid::Direction;

    let direction = match key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        _ => None,
    };

    direction.map(Message::FocusAdjacent)
}

enum PaneState {
    AudioViewer(String),
    SubsViewer(SubsViewer),
}

mod style {
    use iced::{container, Background, Color};

    const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
    );

    const TITLE_BAR_BG: Color = Color::from_rgb(
        0xE8 as f32 / 255.0,
        0xEC as f32 / 255.0,
        0xF4 as f32 / 255.0,
    );

    const TITLE_BAR_BG_FOCUS: Color = Color::from_rgb(
        0xAC as f32 / 255.0,
        0xB3 as f32 / 255.0,
        0xBE as f32 / 255.0,
    );

    pub struct TitleBar {
        pub is_focused: bool,
    }

    impl container::StyleSheet for TitleBar {
        fn style(&self) -> container::Style {
            let pane = Pane {
                is_focused: self.is_focused,
            }
            .style();

            container::Style {
                background: Some(pane.border_color.into()),
                ..Default::default()
            }
        }
    }

    pub struct Pane {
        pub is_focused: bool,
    }

    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(SURFACE)),
                border_width: 2.0,
                border_color: if self.is_focused {
                    TITLE_BAR_BG_FOCUS
                } else {
                    TITLE_BAR_BG
                },
                ..Default::default()
            }
        }
    }
}
