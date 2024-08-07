use termint::{
    buffer::Buffer, enums::Color, geometry::Coords, style::Style,
    widgets::Widget,
};

/// Widget that prints text on given coordinates
///
/// It doesn't implement any wrapping or anything else, it is used only for
/// raw printing - for example using len() on emojis returns 4 and Span widget
/// adds ellipsis and doesn't print the emoji, when the width is less then 4
pub struct RawSpan {
    text: String,
    style: Style,
}

impl RawSpan {
    /// Creates new raw span
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
            style: Default::default(),
        }
    }

    /// Sets style of the [`RawSpan`]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets foreground color of [`RawSpan`]
    pub fn fg(mut self, fg: Color) -> Self {
        self.style = self.style.fg(fg);
        self
    }

    /// Sets background color of [`RawSpan`]
    pub fn bg(mut self, bg: Color) -> Self {
        self.style = self.style.bg(bg);
        self
    }

    /// Sets [`RawSpan`] modifier
    pub fn modifier(mut self, modifier: u8) -> Self {
        self.style = self.style.modifier(modifier);
        self
    }
}

impl Widget for RawSpan {
    fn render(&self, buffer: &mut Buffer) {
        let mut offset = 0;
        for line in self.text.lines() {
            offset = self.render_line(buffer, line, offset);
        }
        let stext: String = self.text.chars().take(buffer.area()).collect();
        buffer.set_str_styled(&stext, &buffer.pos(), self.style);
    }

    fn height(&self, size: &Coords) -> usize {
        self.size(size.x)
    }

    fn width(&self, size: &Coords) -> usize {
        self.size(size.y)
    }
}

impl RawSpan {
    /// Renders single line of the [`RawSpan`]
    fn render_line(
        &self,
        buffer: &mut Buffer,
        text: &str,
        offset: usize,
    ) -> usize {
        let stext: String = text.chars().take(buffer.area()).collect();
        buffer.set_str_styled(
            &stext,
            &Coords::new(buffer.x(), buffer.y() + offset),
            self.style,
        );
        (stext.chars().count() as f32 / buffer.width() as f32).ceil() as usize
    }

    /// Gets size of the [`RawSpan`]
    fn size(&self, size: usize) -> usize {
        (self.text.chars().count() as f32 / size as f32).ceil() as usize
    }
}

impl From<RawSpan> for Box<dyn Widget> {
    fn from(value: RawSpan) -> Self {
        Box::new(value)
    }
}
