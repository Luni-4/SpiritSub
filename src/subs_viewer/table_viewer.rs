use iced::{
    scrollable, Align, Column, Container, Element, HorizontalAlignment, Length, Row, Scrollable,
};

use super::{cell::Cell, SubsControlsValues};

#[derive(Debug, Default, Clone)]
pub struct TableViewer {
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    CellClicked(usize),
}

// Container properties
const CONTAINER_PADDING: u16 = 1;

// Scrollbar properties
const SCROLLBAR_MARGIN: u16 = 10;
const SCROLLBAR_WIDTH: u16 = 10;
const SCROLLER_WIDTH: u16 = 10;

// Cell properties
const CELL_PADDING: u16 = 4;
const HEADER_TEXT_SIZE: u16 = 20;
const CELL_TEXT_SIZE: u16 = 15;

// #, Start, End, Style, Actor, Text, Note, Duration, CPS
impl TableViewer {
    pub fn view<'a>(
        &'a mut self,
        subs_data: &'a [SubsControlsValues],
        focused_row: usize,
    ) -> Element<Message> {
        let center_align = HorizontalAlignment::Center;
        let rows = subs_data
            .into_iter()
            .enumerate()
            .map(|(row, sub)| {
                if focused_row == row {
                    Row::with_children(vec![
                        Self::first_cell(row, true),
                        Self::cell_row(row, &sub.start_time, center_align, true),
                        Self::cell_row(row, &sub.end_time, center_align, true),
                        Self::cell_row(row, sub.style_list.into(), center_align, true),
                        Self::cell_row(row, sub.actor_list.into(), center_align, true),
                        Self::cell_row(row, &sub.text, HorizontalAlignment::Left, true),
                        Self::cell_row(row, &sub.notes, HorizontalAlignment::Left, true),
                        Self::cell_row(row, &sub.duration, center_align, true),
                        Self::cell_row(row, "CPS", center_align, true),
                    ])
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .into()
                } else {
                    Row::with_children(vec![
                        Self::first_cell(row, false),
                        Self::cell_row(row, &sub.start_time, center_align, false),
                        Self::cell_row(row, &sub.end_time, center_align, false),
                        Self::cell_row(row, sub.style_list.into(), center_align, false),
                        Self::cell_row(row, sub.actor_list.into(), center_align, false),
                        Self::cell_row(row, &sub.text, HorizontalAlignment::Left, false),
                        Self::cell_row(row, &sub.notes, HorizontalAlignment::Left, false),
                        Self::cell_row(row, &sub.duration, center_align, false),
                        Self::cell_row(row, "CPS", center_align, false),
                    ])
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .into()
                }
            })
            .collect();

        let columns = Column::with_children(rows)
            .align_items(Align::Center)
            .width(Length::Fill)
            .height(Length::Fill);

        let scrollable = Scrollable::new(&mut self.scroll)
            .scrollbar_margin(SCROLLBAR_MARGIN)
            .scrollbar_width(SCROLLBAR_WIDTH)
            .scroller_width(SCROLLER_WIDTH)
            .push(columns);

        let final_column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Self::create_header())
            .push(scrollable);

        Container::new(final_column)
            .padding(CONTAINER_PADDING)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn create_header<'a>() -> Element<'a, Message> {
        Row::with_children(vec![
            Self::first_header(),
            Self::header_center("Start"),
            Self::header_center("End"),
            Self::header_center("Style"),
            Self::header_center("Actor"),
            Self::header_left("Text"),
            Self::header_left("Note"),
            Self::header_center("Duration"),
            Self::header_center("CPS"),
        ])
        .align_items(Align::Center)
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
    }

    #[inline(always)]
    fn first_header<'a>() -> Element<'a, Message> {
        Cell::no_interactive("")
            .padding(CELL_PADDING)
            .size(HEADER_TEXT_SIZE)
            .width(Length::Units(10))
            .style(style::Header)
            .horizontal_alignment(HorizontalAlignment::Center)
            .into()
    }

    #[inline(always)]
    fn header_center(value: &str) -> Element<Message> {
        Cell::no_interactive(value)
            .padding(CELL_PADDING)
            .size(HEADER_TEXT_SIZE)
            .style(style::Header)
            .horizontal_alignment(HorizontalAlignment::Center)
            .into()
    }

    #[inline(always)]
    fn header_left(value: &str) -> Element<Message> {
        Cell::no_interactive(value)
            .padding(CELL_PADDING)
            .size(HEADER_TEXT_SIZE)
            .style(style::Header)
            .into()
    }

    #[inline(always)]
    fn first_cell<'a>(row: usize, is_focused: bool) -> Element<'a, Message> {
        let value = (row + 1).to_string();
        Cell::interactive(row, &value)
            .padding(CELL_PADDING)
            .size(CELL_TEXT_SIZE)
            .width(Length::Units(10))
            .horizontal_alignment(HorizontalAlignment::Center)
            .style(style::FirstCell)
            .on_click(Message::CellClicked)
            .focus(is_focused)
            .into()
    }

    #[inline(always)]
    fn cell_row<'a>(
        row: usize,
        value: &str,
        alignment: HorizontalAlignment,
        is_focused: bool,
    ) -> Element<'a, Message> {
        Cell::interactive(row, value)
            .padding(CELL_PADDING)
            .size(CELL_TEXT_SIZE)
            .horizontal_alignment(alignment)
            .style(style::Cell)
            .on_click(Message::CellClicked)
            .focus(is_focused)
            .into()
    }
}

mod style {
    use iced::Color;

    use super::super::cell::{Directions, Style, StyleSheet};

    const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
    );

    const HEADER_BG: Color =
        Color::from_rgb(85 as f32 / 255.0, 86 as f32 / 255.0, 89 as f32 / 255.0);

    const HEADER_SEPARATOR: Color = Color::from_rgba(
        153 as f32 / 255.0,
        153 as f32 / 255.0,
        153 as f32 / 255.0,
        0.51,
    );

    const CELL_HIGHLIGHTED: Color = Color::from_rgba(
        169 as f32 / 255.0,
        169 as f32 / 255.0,
        226 as f32 / 255.0,
        0.42,
    );

    pub struct Header;

    impl StyleSheet for Header {
        fn active(&self) -> Style {
            Style {
                background: HEADER_BG.into(),
                border_width: 0.,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
                separator: Some((HEADER_SEPARATOR, Directions::new().right())),
            }
        }

        fn hovered(&self) -> Style {
            Style {
                background: HEADER_BG.into(),
                border_width: 0.,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
                separator: Some((HEADER_SEPARATOR, Directions::new().right())),
            }
        }

        fn highlight(&self) -> Style {
            Style {
                background: HEADER_BG.into(),
                border_width: 0.,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
                separator: Some((HEADER_SEPARATOR, Directions::new().right())),
            }
        }

        fn hover_highlight(&self) -> Style {
            Style {
                background: HEADER_BG.into(),
                border_width: 0.,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
                separator: Some((HEADER_SEPARATOR, Directions::new().right())),
            }
        }
    }

    pub struct FirstCell;

    impl StyleSheet for FirstCell {
        fn active(&self) -> Style {
            Style {
                background: HEADER_BG.into(),
                border_width: 0.,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
                separator: Some((HEADER_SEPARATOR, Directions::new().right())),
            }
        }

        fn hovered(&self) -> Style {
            Style {
                background: HEADER_BG.into(),
                border_width: 0.,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
                separator: Some((HEADER_SEPARATOR, Directions::new().right())),
            }
        }

        fn highlight(&self) -> Style {
            Style {
                background: CELL_HIGHLIGHTED.into(),
                border_width: 0.,
                border_color: HEADER_SEPARATOR,
                text_color: Color::BLACK,
                separator: Some((HEADER_BG, Directions::new().right())),
            }
        }

        fn hover_highlight(&self) -> Style {
            Style {
                background: CELL_HIGHLIGHTED.into(),
                border_width: 0.,
                border_color: HEADER_SEPARATOR,
                text_color: Color::WHITE,
                separator: Some((HEADER_BG, Directions::new().right())),
            }
        }
    }

    pub struct Cell;

    impl StyleSheet for Cell {
        fn active(&self) -> Style {
            Style {
                background: SURFACE.into(),
                border_width: 0.,
                border_color: HEADER_BG,
                text_color: Color::BLACK,
                separator: Some((HEADER_BG, Directions::new().right().bottom())),
            }
        }

        fn hovered(&self) -> Style {
            Style {
                background: HEADER_BG.into(),
                border_width: 0.,
                border_color: HEADER_BG,
                text_color: Color::WHITE,
                separator: Some((HEADER_BG, Directions::new().right().bottom())),
            }
        }

        fn highlight(&self) -> Style {
            Style {
                background: CELL_HIGHLIGHTED.into(),
                border_width: 0.,
                border_color: HEADER_SEPARATOR,
                text_color: Color::BLACK,
                separator: Some((HEADER_BG, Directions::new().right().bottom())),
            }
        }

        fn hover_highlight(&self) -> Style {
            Style {
                background: CELL_HIGHLIGHTED.into(),
                border_width: 0.,
                border_color: HEADER_SEPARATOR,
                text_color: Color::WHITE,
                separator: Some((HEADER_BG, Directions::new().right().bottom())),
            }
        }
    }
}
