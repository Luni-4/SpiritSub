use iced::pick_list;
use iced::{
    text_input, tooltip, Align, Checkbox, Column, Element, Length, PickList, Row, TextInput,
    Tooltip,
};

use iced_aw::number_input::{self, NumberInput};

use super::SubsControlsValues;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleList {
    r#Default,
}

impl Default for StyleList {
    fn default() -> Self {
        Self::r#Default
    }
}

impl From<StyleList> for &'static str {
    fn from(val: StyleList) -> Self {
        match val {
            StyleList::r#Default => "Default",
        }
    }
}

static ALL_STYLES: &[StyleList] = &[StyleList::r#Default];

impl std::fmt::Display for StyleList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: &str = (*self).into();
        write!(f, "{}", val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorList {
    Undefined,
}

impl Default for ActorList {
    fn default() -> Self {
        Self::Undefined
    }
}

impl From<ActorList> for &'static str {
    fn from(val: ActorList) -> Self {
        match val {
            ActorList::Undefined => "",
        }
    }
}

static ALL_ACTORS: &[ActorList] = &[ActorList::Undefined];

impl std::fmt::Display for ActorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: &str = (*self).into();
        write!(f, "{}", val)
    }
}

const ROW_SPACING: u16 = 6;
const COLUMN_SPACING: u16 = 6;
const ROW_MARGIN_SPACING: u16 = 4;

#[derive(Default)]
pub struct SubsControls {
    style_list: pick_list::State<StyleList>,
    actor_list: pick_list::State<ActorList>,
    layer: number_input::State,
    margin_left: number_input::State,
    margin_vertical: number_input::State,
    margin_right: number_input::State,
    start_time: text_input::State,
    end_time: text_input::State,
    duration: text_input::State,
    text: text_input::State,
    notes: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    CommentToggled(bool),
    StyleListPicked(StyleList),
    ActorListPicked(ActorList),
    LayerPicked(usize),
    StartTimeChanged(String),
    EndTimeChanged(String),
    DurationChanged(String),
    MarginLeftPicked(usize),
    MarginVerticalPicked(usize),
    MarginRightPicked(usize),
    TextChanged(String),
    NotesChanged(String),
}

impl SubsControls {
    pub fn update(&mut self, message: Message, subs_controls: &mut SubsControlsValues) {
        match message {
            Message::CommentToggled(is_comment) => subs_controls.is_comment = is_comment,
            Message::StyleListPicked(style_list) => subs_controls.style_list = style_list,
            Message::ActorListPicked(actor_list) => subs_controls.actor_list = actor_list,
            Message::LayerPicked(layer) => subs_controls.layer = layer,
            Message::StartTimeChanged(start_time) => subs_controls.start_time = start_time,
            Message::EndTimeChanged(end_time) => subs_controls.end_time = end_time,
            Message::DurationChanged(duration) => subs_controls.duration = duration,
            Message::MarginLeftPicked(margin_left) => subs_controls.margin_left = margin_left,
            Message::MarginVerticalPicked(margin_vertical) => {
                subs_controls.margin_vertical = margin_vertical
            }
            Message::MarginRightPicked(margin_right) => subs_controls.margin_right = margin_right,
            Message::TextChanged(text) => subs_controls.text = text,
            Message::NotesChanged(notes) => subs_controls.notes = notes,
        }
    }

    pub fn view(&mut self, values: &SubsControlsValues) -> Element<Message> {
        let first_row = Row::new()
            .spacing(ROW_SPACING)
            .align_items(Align::Center)
            .push(Self::tooltip(
                "Comment this line out.",
                Checkbox::new(values.is_comment, "Comment", Message::CommentToggled).into(),
                tooltip::Position::Bottom,
            ))
            .push(Self::tooltip(
                "Style for the selected line",
                Self::picklist(
                    &mut self.style_list,
                    ALL_STYLES,
                    values.style_list,
                    Message::StyleListPicked,
                )
                .into(),
                tooltip::Position::Bottom,
            ))
            .push(Self::tooltip(
                "Actor name for this speech",
                Self::picklist(
                    &mut self.actor_list,
                    ALL_ACTORS,
                    values.actor_list,
                    Message::ActorListPicked,
                )
                .into(),
                tooltip::Position::Bottom,
            ));

        let second_row = Row::new()
            .spacing(ROW_SPACING)
            .align_items(Align::Center)
            .push(
                NumberInput::new(&mut self.layer, values.layer, 999, Message::LayerPicked)
                    .step(1)
                    .min(0),
            )
            .push(TextInput::new(
                &mut self.start_time,
                "00:00:00",
                &values.start_time,
                Message::StartTimeChanged,
            ))
            .push(TextInput::new(
                &mut self.end_time,
                "00:00:00",
                &values.end_time,
                Message::EndTimeChanged,
            ))
            .push(TextInput::new(
                &mut self.duration,
                "00:00:00",
                &values.duration,
                Message::DurationChanged,
            ))
            .push(Self::text_margins(
                &mut self.margin_left,
                &mut self.margin_vertical,
                &mut self.margin_right,
                values.margin_left,
                values.margin_vertical,
                values.margin_right,
            ));

        let third_row = Row::new()
            .spacing(ROW_SPACING)
            .align_items(Align::Center)
            .push(
                TextInput::new(
                    &mut self.text,
                    "Sub text",
                    &values.text,
                    Message::TextChanged,
                )
                .width(Length::Fill),
            )
            .push(
                TextInput::new(
                    &mut self.notes,
                    "Notes",
                    &values.notes,
                    Message::NotesChanged,
                )
                .width(Length::Fill),
            );

        Column::new()
            .spacing(COLUMN_SPACING)
            .align_items(Align::Center)
            .push(first_row)
            .push(second_row)
            .push(third_row)
            .into()
    }

    #[inline(always)]
    fn text_margins<'a>(
        margin_left_state: &'a mut number_input::State,
        margin_vertical_state: &'a mut number_input::State,
        margin_right_state: &'a mut number_input::State,
        margin_left: usize,
        margin_vertical: usize,
        margin_right: usize,
    ) -> Element<'a, Message> {
        Row::new()
            .spacing(ROW_MARGIN_SPACING)
            .push(
                NumberInput::new(
                    margin_left_state,
                    margin_left,
                    999,
                    Message::MarginLeftPicked,
                )
                .step(1)
                .min(0),
            )
            .push(
                NumberInput::new(
                    margin_vertical_state,
                    margin_vertical,
                    999,
                    Message::MarginVerticalPicked,
                )
                .step(1)
                .min(0),
            )
            .push(
                NumberInput::new(
                    margin_right_state,
                    margin_right,
                    999,
                    Message::MarginRightPicked,
                )
                .step(1)
                .min(0),
            )
            .into()
    }

    #[inline(always)]
    fn tooltip<'a>(
        label: &str,
        element: Element<'a, Message>,
        position: tooltip::Position,
    ) -> Element<'a, Message> {
        Tooltip::new(element, label, position)
            .gap(5)
            .padding(10)
            .size(16)
            .style(style::Tooltip)
            .into()
    }

    #[inline(always)]
    fn picklist<'a, T>(
        state: &'a mut pick_list::State<T>,
        all: &'a [T],
        value: T,
        message: impl Fn(T) -> Message + 'static,
    ) -> PickList<'a, T, Message>
    where
        T: ToString + Eq + Clone,
        [T]: ToOwned<Owned = Vec<T>>,
    {
        PickList::new(state, all, Some(value), message)
            .width(Length::Fill)
            .text_size(16)
    }
}

mod style {
    use iced::container;
    use iced::Color;

    const TOOLTIP_BG: Color = Color::from_rgba(
        223 as f32 / 255.0,
        223 as f32 / 255.0,
        196 as f32 / 255.0,
        1.0,
    );

    pub struct Tooltip;

    impl container::StyleSheet for Tooltip {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::BLACK),
                background: Some(TOOLTIP_BG.into()),
                border_width: 1.0,
                border_color: Color::BLACK,
                ..container::Style::default()
            }
        }
    }
}
