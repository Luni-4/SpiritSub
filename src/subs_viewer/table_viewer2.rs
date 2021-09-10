use std::ops::Range;

use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout, mouse, scrollable, text, Align, Background, Color, Element, Hasher,
    HorizontalAlignment, Layout, Length, Point, Rectangle, Row, Size, Widget, Container, Column, Scrollable
};

use super::{cell::Cell, SubsControlsValues};

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

// Units length step (counted as the number of digits, 1 digit = 10 units)
// for the number contained in the first cell of a row
const FIRST_CELL_STEP: u16 = 10;

pub struct TableViewer<'a, Message, B>
where
    B: 'a + Backend + iced_graphics::backend::Text,
{
    state: &'a mut State,
    header: Element<'a, Message, Renderer<B>>,
    rows: Vec<Element<'a, Message, Renderer<B>>>,
    width: Length,
    height: Length,
    on_click: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    on_drag: Option<Box<dyn Fn(&'a [usize]) -> Message + 'a>>,
}

// , Start, End, Style, Actor, Text, Note, Duration, CPS
impl<'a, Message, B> TableViewer<'a, Message, B>
where
    B: 'a + Backend + iced_graphics::backend::Text,
    Message: 'a + Clone,
{
    pub fn new(
        state: &'a mut State,
        subs_data: &'a [SubsControlsValues],
        focused_rows: &Range<usize>,
    ) -> Self {
        let max_first_cell_units =
            subs_data.len().to_string().chars().count() as u16 * FIRST_CELL_STEP;

        let header = Self::create_header(max_first_cell_units).into();

        let center_align = HorizontalAlignment::Center;
        let max_first_cell_units =
            subs_data.len().to_string().chars().count() as u16 * FIRST_CELL_STEP;
        let rows = subs_data
            .into_iter()
            .enumerate()
            .map(|(row, sub)| {
                if focused_rows.contains(&row) {
                    Row::with_children(vec![
                        Self::first_cell(row, max_first_cell_units, true),
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
                    .height(Length::Shrink).into()
                } else {
                    Row::with_children(vec![
                        Self::first_cell(row, max_first_cell_units, false),
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
                    .height(Length::Shrink).into()
                }
            })
            .collect();

        Self {
            state,
            header,
            rows,
            width: Length::Fill,
            height: Length::Fill,
            on_click: None,
            on_drag: None,
        }

    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(usize) -> Message,
    {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn on_drag<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&'a [usize]) -> Message,
    {
        self.on_drag = Some(Box::new(f));
        self
    }

    fn create_header(length_units: u16) -> Row<'a, Message, Renderer<B>> {
        Row::with_children(vec![
            Self::first_header(length_units),
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
    }

    #[inline(always)]
    fn first_header(length_units: u16) -> Element<'a, Message, Renderer<B>> {
        Cell::no_interactive("")
            .padding(CELL_PADDING)
            .size(HEADER_TEXT_SIZE)
            .width(Length::Units(length_units))
            .style(style::Header)
            .horizontal_alignment(HorizontalAlignment::Center)
            .into()
    }

    #[inline(always)]
    fn header_center(value: &'a str) -> Element<Message, Renderer<B>> {
        Cell::no_interactive(value)
            .padding(CELL_PADDING)
            .size(HEADER_TEXT_SIZE)
            .style(style::Header)
            .horizontal_alignment(HorizontalAlignment::Center)
            .into()
    }

    #[inline(always)]
    fn header_left(value: &'a str) -> Element<Message, Renderer<B>> {
        Cell::no_interactive(value)
            .padding(CELL_PADDING)
            .size(HEADER_TEXT_SIZE)
            .style(style::Header)
            .into()
    }

    #[inline(always)]
    fn first_cell(
        row: usize,
        length_units: u16,
        is_focused: bool,
    ) -> Element<'a, Message, Renderer<B>> {
        let value = (row + 1).to_string();
        Cell::interactive(row, &value)
            .padding(CELL_PADDING)
            .size(CELL_TEXT_SIZE)
            .width(Length::Units(length_units))
            .horizontal_alignment(HorizontalAlignment::Center)
            .style(style::FirstCell)
            .focus(is_focused)
            .into()
    }

    #[inline(always)]
    fn cell_row(
        row: usize,
        value: &str,
        alignment: HorizontalAlignment,
        is_focused: bool,
    ) -> Element<'a, Message, Renderer<B>> {
        Cell::interactive(row, value)
            .padding(CELL_PADDING)
            .size(CELL_TEXT_SIZE)
            .horizontal_alignment(alignment)
            .style(style::Cell)
            .focus(is_focused)
            .into()
    }
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for TableViewer<'a, Message, B>
where
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(Size::ZERO);

        let columns = Column::<Message, Renderer<B>>::with_children(self.rows)
            .align_items(Align::Center)
            .width(self.width)
            .height(self.height);

        let scrollable = Scrollable::<Message, Renderer<B>>::new(&mut self.state.scroll)
            .scrollbar_margin(SCROLLBAR_MARGIN)
            .scrollbar_width(SCROLLBAR_WIDTH)
            .scroller_width(SCROLLER_WIDTH)
            .push(columns);

        let final_column = Column::<Message, Renderer<B>>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.header)
            .push(scrollable);

        let container = Container::<Message, Renderer<B>>::new(final_column)
            .padding(CONTAINER_PADDING)
            .width(self.width)
            .height(self.height).layout(renderer, &limits.loose());

        /*let children: Vec<layout::Node> = self
            .rows
            .iter()
            .filter_map(|row| {
                let mut node = row.layout(renderer, &limits.loose());

                Some(node)
            })
            .collect();*/

        layout::Node::with_children(size, container)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;

        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);

        for row in &self.rows {
            row.hash_layout(state);
        }
    }

    fn draw(
        &self,
        _renderer: &mut Renderer<B>,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {

        let
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::BLACK),
                border_radius: self.radius,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            mouse::Interaction::default(),
        )
    }
}

impl<'a, Message, B> From<TableViewer<'a, Message, B>> for Element<'a, Message, Renderer<B>>
where
    Message: 'a + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn from(table_viewer: TableViewer<'a, Message, B>) -> Self {
        Element::new(table_viewer)
    }
}

#[derive(Default, Clone, Debug)]
pub struct State {
    scroll: scrollable::State,
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
