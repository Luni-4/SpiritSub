mod cell;
mod subs_controls;
mod table_viewer;

use iced::{Align, Column, Container, Element, Length};

use subs_controls::{ActorList, StyleList, SubsControls};
use table_viewer::TableViewer;

const PADDING: u16 = 10;
const COLUMN_SPACING: u16 = 40;

#[derive(Debug, Default)]
pub struct SubsControlsValues {
    pub style_list: StyleList,
    pub actor_list: ActorList,
    pub layer: usize,
    pub margin_left: usize,
    pub margin_vertical: usize,
    pub margin_right: usize,
    pub start_time: String,
    pub end_time: String,
    pub duration: String,
    pub is_comment: bool,
    pub text: String,
    pub notes: String,
}

// FIXME Create a LineEdit widget
impl SubsControlsValues {
    pub fn new() -> Self {
        Self {
            start_time: "00:00:00:00".to_owned(),
            end_time: "00:00:00:00".to_owned(),
            duration: "00:00:00:00".to_owned(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // Subs Controller events
    SubsControls(subs_controls::Message),
    // Table viewer events
    TableViewer(table_viewer::Message),
}

pub struct SubsViewer {
    subs_controls: SubsControls,
    table_viewer: TableViewer,
    controls_values: Vec<SubsControlsValues>,
    focused_sub: usize,
}

impl SubsViewer {
    pub fn new() -> Self {
        let controls_values = vec![
            SubsControlsValues::new(),
            SubsControlsValues::new(),
            SubsControlsValues::new(),
        ];
        Self {
            subs_controls: SubsControls::default(),
            table_viewer: TableViewer::default(),
            controls_values,
            focused_sub: 0,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SubsControls(message) => self
                .subs_controls
                .update(message, &mut self.controls_values[self.focused_sub]),
            Message::TableViewer(message) => match message {
                table_viewer::Message::CellClicked(row) => {
                    self.focused_sub = row;
                }
            },
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(PADDING)
            .spacing(COLUMN_SPACING)
            .align_items(Align::Center)
            .push(
                self.subs_controls
                    .view(&self.controls_values[self.focused_sub])
                    .map(move |message| Message::SubsControls(message)),
            )
            .push(
                self.table_viewer
                    .view(&self.controls_values, self.focused_sub)
                    .map(move |message| Message::TableViewer(message)),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}
