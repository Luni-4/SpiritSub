mod cell;
mod subs_controls;
//mod table_viewer;
mod table_viewer2;

use std::ops::Range;

use iced::{Align, Column, Container, Element, Length};

use subs_controls::{ActorList, StyleList, SubsControls};
use table_viewer2::TableViewer;

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
    TableViewerClicked(usize),
}

pub struct SubsViewer {
    subs_controls: SubsControls,
    table_viewer: table_viewer2::State,
    controls_values: Vec<SubsControlsValues>,
    focused_subs: Range<usize>,
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
            table_viewer: table_viewer2::State::new(),
            controls_values,
            focused_subs: Range { start: 0, end: 0 },
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SubsControls(message) => {
                let focus_sub = if self.focused_subs.start == self.focused_subs.end {
                    self.focused_subs.start
                } else {
                    self.focused_subs.end
                };

                self.subs_controls
                    .update(message, &mut self.controls_values[focus_sub])
            }
            Message::TableViewerClicked(row) => {
                self.focused_subs = Range {
                    start: row,
                    end: row,
                };
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let focus_sub = if self.focused_subs.start == self.focused_subs.end {
            self.focused_subs.start
        } else {
            self.focused_subs.end
        };
        let content = Column::new()
            .padding(PADDING)
            .spacing(COLUMN_SPACING)
            .align_items(Align::Center)
            .push(
                self.subs_controls
                    .view(&self.controls_values[focus_sub])
                    .map(move |message| Message::SubsControls(message)),
            )
            .push(
                TableViewer::new(
                    &mut self.table_viewer,
                    &self.controls_values,
                    &self.focused_subs,
                )
                .on_click(Message::TableViewerClicked),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}
