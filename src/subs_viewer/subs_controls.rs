use iced::pick_list;
use iced::{text_input, Align, Checkbox, Column, Element, Length, PickList, Row, TextInput};

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
    StyleListPicked(StyleList),
    ActorListPicked(ActorList),
    LayerPicked(usize),
    MarginLeftPicked(usize),
    MarginVerticalPicked(usize),
    MarginRightPicked(usize),
    StartTimeChanged(String),
    EndTimeChanged(String),
    DurationChanged(String),
    CommentToggled(bool),
    TextChanged(String),
    NotesChanged(String),
}

impl SubsControls {
    pub fn update(&mut self, message: Message, subs_controls: &mut SubsControlsValues) {
        match message {
            Message::StyleListPicked(style_list) => subs_controls.style_list = style_list,
            Message::ActorListPicked(actor_list) => subs_controls.actor_list = actor_list,
            Message::LayerPicked(layer) => subs_controls.layer = layer,
            Message::MarginLeftPicked(margin_left) => subs_controls.margin_left = margin_left,
            Message::MarginVerticalPicked(margin_vertical) => {
                subs_controls.margin_vertical = margin_vertical
            }
            Message::MarginRightPicked(margin_right) => subs_controls.margin_right = margin_right,
            Message::StartTimeChanged(start_time) => subs_controls.start_time = start_time,
            Message::EndTimeChanged(end_time) => subs_controls.end_time = end_time,
            Message::DurationChanged(duration) => subs_controls.duration = duration,
            Message::CommentToggled(is_comment) => subs_controls.is_comment = is_comment,
            Message::TextChanged(text) => subs_controls.text = text,
            Message::NotesChanged(notes) => subs_controls.notes = notes,
        }
    }

    pub fn view(&mut self, values: &SubsControlsValues) -> Element<Message> {
        let first_row = Row::new()
            .spacing(ROW_SPACING)
            .align_items(Align::Center)
            .push(Self::picklist(
                &mut self.style_list,
                ALL_STYLES,
                values.style_list,
                Message::StyleListPicked,
            ))
            .push(Self::picklist(
                &mut self.actor_list,
                ALL_ACTORS,
                values.actor_list,
                Message::ActorListPicked,
            ))
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
            ));

        let third_row = Row::new()
            .spacing(ROW_SPACING)
            .align_items(Align::Center)
            .push(Self::text_margins(
                &mut self.margin_left,
                &mut self.margin_vertical,
                &mut self.margin_right,
                values.margin_left,
                values.margin_vertical,
                values.margin_right,
            ))
            .push(Checkbox::new(
                values.is_comment,
                "Comment",
                Message::CommentToggled,
            ));

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
        PickList::new(state, all, Some(value), message).text_size(16)
    }
}
