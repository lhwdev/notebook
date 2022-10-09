use utils::note_node;

pub type StyleId = u32;

#[note_node]
#[repr(transparent)]
pub struct Color(StyleId);

#[note_node]
pub enum Fill {
    Empty,
    Color(Color),
    Gradient { start: Color, end: Color },
}

#[note_node]
pub struct Stroke {
    width: u32, // or f32?
    line_type: StrokeType,
}

#[note_node]
pub enum StrokeType {
    Solid,
    Dotted,
}

#[note_node]
pub struct DropShadow {
    // I don't want notebook to be full-fledged document editor in regard of style & layout.
    // So parameter is rather simplified; but DropShadow in theming / custom component should be
    // complex, like 'size, opacity, distance, direction, softness, color, ...'
    size: f32,
    style_type: u32,
    // style_parameter:
}

pub type FontFamilyId = u32;

#[note_node]
pub struct Font {
    font_family: FontFamilyId,
}

#[note_node]
#[derive(Default)]
pub struct BoxStyle {
    pub fill: Option<Fill>,
    pub font: Option<Font>,

    pub background_fill: Option<Fill>,

    pub box_stroke: Option<Stroke>,
    pub box_shadow: Option<DropShadow>,
}

pub type IconId = u32;

#[note_node]
pub enum Icon {
    Unicode(char),
    Custom(IconId),
}
