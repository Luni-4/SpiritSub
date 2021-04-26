use std::path::Path;

use iced::{
    button, scrollable, text_input, Button, Color, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Scrollable, Settings, Space, Text, TextInput,
};

pub fn main() -> iced::Result {
    AudioViewer::run(Settings::default())
}

pub struct AudioViewer {
    steps: Steps,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
}

impl Sandbox for AudioViewer {
    type Message = Message;

    fn new() -> AudioViewer {
        AudioViewer {
            steps: Steps::new(),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        self.steps.title().to_string()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let AudioViewer {
            steps,
            scroll,
            back_button,
            next_button,
        } = self;

        let mut controls = Row::new();

        if steps.has_previous() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button(next_button, "Next")
                    .on_press(Message::NextPressed)
                    .style(style::Button::Primary),
            );
        }

        let content: Element<_> = Column::new()
            .spacing(20)
            .padding(20)
            .push(steps.view().map(Message::StepMessage))
            .push(controls)
            .into();

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

struct Steps {
    steps: [Step; 2],
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: [
                Step::PathsInput {
                    ass_path: String::new(),
                    ass_state: text_input::State::new(),
                    is_ass_path_ok: false,
                    audio_path: String::new(),
                    audio_state: text_input::State::new(),
                    is_audio_path_ok: false,
                },
                Step::Viewer,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&mut self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&mut self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

enum Step {
    PathsInput {
        ass_path: String,
        ass_state: text_input::State,
        is_ass_path_ok: bool,
        audio_path: String,
        audio_state: text_input::State,
        is_audio_path_ok: bool,
    },
    Viewer,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    AssPathChanged(String),
    AudioPathChanged(String),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::AssPathChanged(new_ass_path) => {
                if let Step::PathsInput { ass_path, .. } = self {
                    *ass_path = new_ass_path;
                }
            }
            StepMessage::AudioPathChanged(new_audio_path) => {
                if let Step::PathsInput { audio_path, .. } = self {
                    *audio_path = new_audio_path;
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::PathsInput { .. } => "Input Paths",
            Step::Viewer => "Viewer",
        }
    }

    fn check_path(path_str: &str, extension: &str) -> bool {
        let path = Path::new(path_str);
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == extension {
                    return true;
                }
            }
        }
        false
    }

    fn can_continue(&mut self) -> bool {
        match self {
            Step::PathsInput {
                ass_path,
                is_ass_path_ok,
                audio_path,
                is_audio_path_ok,
                ..
            } => {
                *is_ass_path_ok = Self::check_path(ass_path, "ass");
                *is_audio_path_ok = Self::check_path(audio_path, "wav");
                *is_ass_path_ok && *is_audio_path_ok
            }
            Step::Viewer => false,
        }
    }

    fn view(&mut self) -> Element<StepMessage> {
        match self {
            Step::PathsInput {
                ass_path,
                ass_state,
                is_ass_path_ok,
                audio_path,
                audio_state,
                is_audio_path_ok,
            } => Self::paths_input(
                ass_path,
                ass_state,
                *is_ass_path_ok,
                audio_path,
                audio_state,
                *is_audio_path_ok,
            ),
            Step::Viewer => unimplemented!(),
        }
        .into()
    }

    fn show_path(path_str: &str, is_path_correct: bool) -> Text {
        if is_path_correct {
            Text::new(path_str)
                .color(Color::from_rgb8(0x22, 0x8B, 0x22))
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
        } else {
            Text::new(path_str)
                .color(Color::from_rgb8(0xFA, 0x80, 0x72))
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
        }
    }

    fn paths_input(
        ass_path: &str,
        ass_state: &'a mut text_input::State,
        is_ass_path_ok: bool,
        audio_path: &str,
        audio_state: &'a mut text_input::State,
        is_audio_path_ok: bool,
    ) -> Column<'a, StepMessage> {
        let ass_path_input = text_input(
            ass_state,
            "Type the path of an ass file to continue...",
            ass_path,
            StepMessage::AssPathChanged,
        );
        let audio_path_input = text_input(
            audio_state,
            "Type the path of a wav file to continue...",
            audio_path,
            StepMessage::AudioPathChanged,
        );
        Column::new()
            .spacing(20)
            .push(Text::new("Input Paths").size(50))
            .push(ass_path_input)
            .push(Self::show_path(ass_path, is_ass_path_ok))
            .push(audio_path_input)
            .push(Self::show_path(audio_path, is_audio_path_ok))
    }
}

fn text_input<'a, F, Message: Clone>(
    state: &'a mut text_input::State,
    label: &str,
    value: &str,
    step_message: F,
) -> TextInput<'a, Message>
where
    F: 'static + Fn(String) -> Message,
{
    TextInput::new(state, label, value, step_message)
        .padding(10)
        .size(30)
}

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
