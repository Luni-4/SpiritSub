use iced_graphics::{
    backend::{self, Backend},
    Primitive, Renderer,
};
use iced_native::{mouse, HorizontalAlignment, Point, Rectangle, VerticalAlignment};
use iced_native::{Background, Color};

pub type Cell<'a, Message, Backend> = cell::Cell<'a, Message, Renderer<Backend>>;

impl<B> cell::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        text_bounds: Rectangle,
        cursor_position: Point,
        font: Self::Font,
        size: u16,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
        is_header: bool,
        is_highlighted: bool,
        value: &str,
        style: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let styling = if is_header {
            style.active()
        } else if is_mouse_over && is_highlighted {
            style.hover_highlight()
        } else if is_highlighted {
            style.highlight()
        } else if is_mouse_over {
            style.hovered()
        } else {
            style.active()
        };

        let background = Primitive::Quad {
            bounds,
            background: styling.background,
            border_radius: 0.,
            border_width: styling.border_width,
            border_color: styling.border_color,
        };

        let text_x = match horizontal_alignment {
            HorizontalAlignment::Left => text_bounds.x,
            HorizontalAlignment::Center => text_bounds.center_x(),
            HorizontalAlignment::Right => text_bounds.x + text_bounds.width,
        };

        let text_y = match vertical_alignment {
            VerticalAlignment::Top => text_bounds.y,
            VerticalAlignment::Center => text_bounds.center_y(),
            VerticalAlignment::Bottom => text_bounds.y + text_bounds.height,
        };

        let text_value = Primitive::Text {
            content: value.to_owned(),
            color: styling.text_color,
            font,
            bounds: Rectangle {
                x: text_x,
                y: text_y,
                width: f32::INFINITY,
                ..text_bounds
            },
            size: f32::from(size),
            horizontal_alignment,
            vertical_alignment,
        };

        let mut primitives = vec![background, text_value];

        if let Some((separator_style, direction)) = styling.separator {
            if direction.is_right() {
                let right_separator = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds.x + bounds.width - 1.0,
                        y: bounds.y,
                        width: 1.0,
                        height: bounds.height,
                    },
                    background: separator_style.into(),
                    border_radius: 0.,
                    border_width: 0.,
                    border_color: separator_style,
                };
                primitives.push(right_separator);
            }

            if direction.is_bottom() {
                let bottom_separator = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bounds.y + bounds.height - 1.0,
                        width: bounds.width,
                        height: 1.0,
                    },
                    background: separator_style.into(),
                    border_radius: 0.,
                    border_width: 0.,
                    border_color: separator_style,
                };
                primitives.push(bottom_separator);
            }
        }

        (
            Primitive::Group { primitives },
            mouse::Interaction::default(),
        )
    }
}

mod cell {
    use iced_native::{
        event,
        layout::{Limits, Node},
        mouse, text, touch, Clipboard, Element, Event, Hasher, HorizontalAlignment, Layout, Length,
        Point, Rectangle, Size, VerticalAlignment, Widget,
    };

    pub struct Cell<'a, Message, Renderer: self::Renderer> {
        row: isize,
        value: String,
        is_highlighted: bool,
        is_header: bool,
        font: Renderer::Font,
        width: Length,
        max_width: u32,
        padding: u16,
        size: Option<u16>,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
        on_click: Option<Box<dyn Fn(usize) -> Message + 'a>>,
        style: Renderer::Style,
    }

    impl<'a, Message, Renderer> Cell<'a, Message, Renderer>
    where
        Renderer: self::Renderer,
    {
        pub fn interactive(row: usize, value: &str) -> Self {
            Self {
                row: row as isize,
                value: String::from(value),
                is_header: false,
                is_highlighted: false,
                font: Default::default(),
                width: Length::Fill,
                max_width: u32::MAX,
                padding: 0,
                size: None,
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Center,
                on_click: None,
                style: Renderer::Style::default(),
            }
        }

        pub fn no_interactive(value: &str) -> Self {
            Self {
                row: -1,
                value: String::from(value),
                is_header: true,
                is_highlighted: false,
                font: Default::default(),
                width: Length::Fill,
                max_width: u32::MAX,
                padding: 0,
                size: None,
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Center,
                on_click: None,
                style: Renderer::Style::default(),
            }
        }

        pub fn font(mut self, font: Renderer::Font) -> Self {
            self.font = font;
            self
        }

        pub fn width(mut self, width: Length) -> Self {
            self.width = width;
            self
        }

        pub fn max_width(mut self, max_width: u32) -> Self {
            self.max_width = max_width;
            self
        }

        pub fn padding(mut self, units: u16) -> Self {
            self.padding = units;
            self
        }

        pub fn size(mut self, size: u16) -> Self {
            self.size = Some(size);
            self
        }

        pub fn horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
            self.horizontal_alignment = alignment;
            self
        }

        pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
            self.vertical_alignment = alignment;
            self
        }

        pub fn on_click<F>(mut self, f: F) -> Self
        where
            F: 'a + Fn(usize) -> Message,
        {
            self.on_click = Some(Box::new(f));
            self
        }

        pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
            self.style = style.into();
            self
        }

        pub fn focus(mut self, is_focused: bool) -> Self {
            self.is_highlighted = is_focused;
            self
        }
    }

    impl<'a, Message, Renderer> Widget<Message, Renderer> for Cell<'a, Message, Renderer>
    where
        Renderer: self::Renderer,
    {
        fn width(&self) -> Length {
            self.width
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
            let padding = f32::from(self.padding);
            let text_size = self.size.unwrap_or(renderer.default_size());

            let limits = limits
                .pad(padding)
                .width(self.width)
                .max_width(self.max_width)
                .height(Length::Units(text_size));

            let mut text = Node::new(limits.resolve(Size::ZERO));
            text.move_to(Point::new(padding, padding));

            Node::with_children(text.size().pad(padding), vec![text])
        }

        fn on_event(
            &mut self,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            _renderer: &Renderer,
            _clipboard: &mut dyn Clipboard,
            messages: &mut Vec<Message>,
        ) -> event::Status {
            let bounds = layout.bounds();

            if !self.is_header && bounds.contains(cursor_position) {
                let mut event_status = event::Status::Captured;
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerPressed { .. }) => {
                        self.is_highlighted = true;
                    }
                    Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerLifted { .. }) => {
                        if let Some(on_click) = &self.on_click {
                            messages.push((on_click)(self.row as usize));
                        } else {
                            event_status = event::Status::Ignored;
                        }
                    }
                    _ => event_status = event::Status::Ignored,
                }
                event_status
            } else {
                event::Status::Ignored
            }
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            _defaults: &Renderer::Defaults,
            layout: Layout<'_>,
            cursor_position: Point,
            _viewport: &Rectangle,
        ) -> Renderer::Output {
            let bounds = layout.bounds();
            let text_layout = layout.children().next().expect("fail to get text layout");
            let text_bounds = text_layout.bounds();

            self::Renderer::draw(
                renderer,
                bounds,
                text_bounds,
                cursor_position,
                self.font,
                self.size.unwrap_or(renderer.default_size()),
                self.horizontal_alignment,
                self.vertical_alignment,
                self.is_header,
                self.is_highlighted,
                &self.value,
                &self.style,
            )
        }

        fn hash_layout(&self, state: &mut Hasher) {
            use std::hash::Hash;
            struct Marker;
            std::any::TypeId::of::<Marker>().hash(state);

            self.width.hash(state);
            self.max_width.hash(state);
            self.padding.hash(state);
            self.size.hash(state);
        }
    }

    pub trait Renderer: text::Renderer + Sized {
        type Style: Default;

        fn draw(
            &mut self,
            bounds: Rectangle,
            text_bounds: Rectangle,
            cursor_position: Point,
            font: Self::Font,
            size: u16,
            horizontal_alignment: HorizontalAlignment,
            vertical_alignment: VerticalAlignment,
            is_header: bool,
            is_highlighted: bool,
            value: &str,
            style: &Self::Style,
        ) -> Self::Output;
    }

    impl<'a, Message, Renderer> From<Cell<'a, Message, Renderer>> for Element<'a, Message, Renderer>
    where
        Message: 'a + Clone,
        Renderer: 'a + self::Renderer,
    {
        fn from(cell: Cell<'a, Message, Renderer>) -> Self {
            Element::new(cell)
        }
    }

    impl Renderer for iced_native::renderer::Null {
        type Style = ();

        fn draw(
            &mut self,
            _: Rectangle,
            _: Rectangle,
            _: Point,
            _: Self::Font,
            _: u16,
            _: HorizontalAlignment,
            _: VerticalAlignment,
            _: bool,
            _: bool,
            _: &str,
            _: &Self::Style,
        ) -> Self::Output {
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub background: Background,

    pub border_width: f32,

    pub border_color: Color,

    pub text_color: Color,

    pub separator: Option<(Color, Directions)>,
}

pub trait StyleSheet {
    fn active(&self) -> Style;

    fn hovered(&self) -> Style;

    fn highlight(&self) -> Style;

    fn hover_highlight(&self) -> Style;
}

#[derive(Clone, Copy, Debug)]
pub struct Directions {
    right: bool,
    bottom: bool,
}

impl Directions {
    pub fn new() -> Self {
        Self {
            right: false,
            bottom: false,
        }
    }

    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    pub fn bottom(mut self) -> Self {
        self.bottom = true;
        self
    }

    pub fn is_right(&self) -> bool {
        self.right
    }

    pub fn is_bottom(&self) -> bool {
        self.bottom
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Default;

const SURFACE: Color = Color::from_rgb(
    0xF2 as f32 / 255.0,
    0xF3 as f32 / 255.0,
    0xF5 as f32 / 255.0,
);

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: SURFACE.into(),
            border_width: 0.,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            separator: None,
        }
    }

    fn hovered(&self) -> Style {
        Style {
            background: SURFACE.into(),
            border_width: 0.,
            border_color: Color::BLACK,
            text_color: Color::WHITE,
            separator: None,
        }
    }

    fn highlight(&self) -> Style {
        Style {
            background: SURFACE.into(),
            border_width: 0.,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            separator: None,
        }
    }

    fn hover_highlight(&self) -> Style {
        Style {
            background: SURFACE.into(),
            border_width: 0.,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            separator: None,
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
