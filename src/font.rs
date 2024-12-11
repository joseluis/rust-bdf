use crate::{BoundingBox, Glyph};
use bit_set::BitSet;
use core::ops::{Deref, DerefMut};
use std::collections::HashMap;

/// The bitmap of a glyph.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bitmap {
    width: u32,
    height: u32,
    bits: BitSet,
}

#[rustfmt::skip]
impl Bitmap {
    /// Creates a bitmap of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        Bitmap { width, height, bits: BitSet::new() }
    }

    /// Gets the width.
    pub fn width(&self) -> u32 { self.width }

    /// Gets the height.
    pub fn height(&self) -> u32 { self.height }

    /// Gets a bit from the map.
    pub fn get(&self, x: u32, y: u32) -> bool {
        if y >= self.height || x >= self.width { panic!("out of bounds"); }
        self.bits.contains((y * self.width + x) as usize)
    }

    /// Sets a bit of the map.
    pub fn set(&mut self, x: u32, y: u32, value: bool) {
        if y >= self.height || x >= self.width { panic!("out of bounds"); }
        if value {
            self.bits.insert((y * self.width + x) as usize);
        } else {
            self.bits.remove((y * self.width + x) as usize);
        }
    }
}
impl Deref for Bitmap {
    type Target = BitSet;
    #[rustfmt::skip]    fn deref(&self) -> &BitSet { &self.bits }
}
impl DerefMut for Bitmap {
    #[rustfmt::skip]    fn deref_mut(&mut self) -> &mut BitSet { &mut self.bits }
}

/// The possible entries in BDF.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Entry {
    /// `STARTFONT` marks the beginning of the font declaration and contains
    /// the BDF version.
    StartFont(String),

    /// `COMMENT` contains the comment body.
    Comment(String),

    /// `CONTENTVERSION` contains the font version.
    ContentVersion(String),

    /// `FONT` contains the font name.
    Font(String),

    /// `SIZE` contains the pt size, X-axis DPI and Y-axis DPI.
    Size(u16, u16, u16),

    /// `CHARS` contains the number of characters stored.
    Chars(usize),

    /// `FONTBOUNDINGBOX` contains the default bounding box.
    FontBoundingBox(BoundingBox),

    /// `ENDFONT` marks the end of the font declaration.
    EndFont,

    /// `STARTPROPERTIES` marks the beginning of the property declarations and
    /// contains the number of properties.
    StartProperties(usize),

    /// Contains the name and value of a property.
    Property(String, Property),

    /// `ENDPROPERTIES` marks the end of the property declarations.
    EndProperties,

    /// `STARTCHAR` marks the beginning of the character declaration and contains
    /// the name of the character.
    StartChar(String),

    /// `ENCODING` contains the codepoint for the glyph.
    Encoding(char),

    /// `METRICSSET` contains the direction for the glyph.
    Direction(Direction),

    /// `SWIDTH` contains the scalable width (x, y) of the glyph.
    ScalableWidth(u32, u32),

    /// `DWIDTH` contains the device width (x, y) of the glyph.
    DeviceWidth(u32, u32),

    /// `SWIDTH1` contains the alternate scalable width (x, y) of the glyph.
    AlternateScalableWidth(u32, u32),

    /// `DWIDTH1` contains the alternate device width (x, y) of the glyph.
    AlternateDeviceWidth(u32, u32),

    /// `VVECTOR` contains the vector offset for the glyph.
    Vector(u32, u32),

    /// `BBX` contains the bounds for the glyph.
    BoundingBox(BoundingBox),

    /// `BITMAP` contains the bits of the glyph.
    Bitmap(Bitmap),

    /// `ENDCHAR` marks the end of the character declaration.
    EndChar,

    /// Contains the unknown id.
    Unknown(String),
}
/// The direction of the glyph.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    /// Default direction, typically lef-to-right.
    #[default]
    Default,
    /// Alternate direction, typically right-to-left.
    Alternate,
    /// Both directions.
    Both,
}

/// A `Font` property.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Property {
    ///
    String(String),
    ///
    Integer(i64),
}

impl Property {
    /// Parse a property string.
    pub fn parse(string: &str) -> Property {
        if string.starts_with('"') {
            Property::String(Property::extract(string))
        } else if let Ok(int) = string.parse() {
            Property::Integer(int)
        } else {
            Property::String(string.into())
        }
    }

    ///
    pub(crate) fn extract(string: &str) -> String {
        string[1..string.len() - 1].replace("\"\"", "\"")
    }
}

/// The size of a font.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Size {
    /// Point size of the font.
    pub pt: u16,
    /// X-axis DPI.
    pub x: u16,
    /// Y-axis DPI.
    pub y: u16,
}

/// A BDF font.
#[derive(Clone, Debug)]
pub struct Font {
    format: String,

    name: Option<String>,
    version: Option<String>,

    size: Option<Size>,
    bounds: Option<BoundingBox>,

    direction: Direction,

    scalable_width: Option<(u32, u32)>,
    device_width: Option<(u32, u32)>,

    alternate_scalable_width: Option<(u32, u32)>,
    alternate_device_width: Option<(u32, u32)>,

    vector: Option<(u32, u32)>,

    properties: HashMap<String, Property>,
    glyphs: HashMap<char, Glyph>,
}

impl Default for Font {
    fn default() -> Self {
        Font {
            format: "2.2".to_owned(),
            name: None,
            version: None,
            size: None,
            bounds: None,
            direction: Default::default(),
            scalable_width: None,
            device_width: None,
            alternate_scalable_width: None,
            alternate_device_width: None,
            vector: None,
            properties: HashMap::new(),
            glyphs: HashMap::new(),
        }
    }
}

#[rustfmt::skip]
impl Font {
    /// Create a new font with the given name and content-version.
    pub fn new<T: Into<String>>(name: T, version: Option<T>) -> Self {
        Font {
            name: Some(name.into()),
            version: version.map(|v| v.into()),
            ..Default::default()
        }
    }

    /// Validates the definition.
    pub fn validate(&self) -> bool {
        if self.name.is_none() { return false; }
        if self.size.is_none() { return false; }
        if self.bounds.is_none() { return false; }
        true
    }

    /// Gets BDF format version.
    pub fn format(&self) -> &str { &self.format }

    /// Sets the BDF format version.
    pub fn set_format<T: Into<String>>(&mut self, format: T) { self.format = format.into(); }

    /// Gets the name.
    pub fn name(&self) -> &str { self.name.as_ref().unwrap().as_ref() }

    /// Sets the name.
    pub fn set_name<T: Into<String>>(&mut self, name: T) { self.name = Some(name.into()); }

    /// Gets the content-version.
    pub fn version(&self) -> Option<&str> { self.version.as_ref().map(|v| v.as_ref()) }

    /// Sets the content-version.
    pub fn set_version<T: Into<String>>(&mut self, version: Option<T>) {
        self.version = version.map(|v| v.into());
    }
    /// Gets the size.
    pub fn size(&self) -> &Size { self.size.as_ref().unwrap() }

    /// Sets the size.
    pub fn set_size(&mut self, size: Size) { self.size = Some(size); }

    /// Gets the default bounding box.
    pub fn bounds(&self) -> &BoundingBox { self.bounds.as_ref().unwrap() }

    /// Sets the default bounding box.
    pub fn set_bounds(&mut self, bounds: BoundingBox) { self.bounds = Some(bounds); }

    /// Gets the default direction.
    pub fn direction(&self) -> Direction { self.direction }

    /// Sets the default direction.
    pub fn set_direction(&mut self, direction: Direction) { self.direction = direction; }

    /// Gets the default scalable width.
    pub fn scalable_width(&self) -> Option<&(u32, u32)> { self.scalable_width.as_ref() }

    /// Sets the default scalable width.
    pub fn set_scalable_width(&mut self, value: Option<(u32, u32)>) { self.scalable_width = value; }

    /// Gets the default device width.
    pub fn device_width(&self) -> Option<&(u32, u32)> { self.device_width.as_ref() }

    /// Sets the default device width.
    pub fn set_device_width(&mut self, value: Option<(u32, u32)>) { self.device_width = value; }

    /// Gets the default alternate scalable width.
    pub fn alternate_scalable_width(&self) -> Option<&(u32, u32)> {
        self.alternate_scalable_width.as_ref()
    }
    /// Sets the default alternate scalable width.
    pub fn set_alternate_scalable_width(&mut self, value: Option<(u32, u32)>) {
        self.alternate_scalable_width = value;
    }
    /// Gets the default alternate device width.
    pub fn alternate_device_width(&self) -> Option<&(u32, u32)> {
        self.alternate_device_width.as_ref()
    }
    /// Sets the default alternate device width.
    pub fn set_alternate_device_width(&mut self, value: Option<(u32, u32)>) {
        self.alternate_device_width = value;
    }
    /// Gets the default offset vector.
    pub fn vector(&self) -> Option<&(u32, u32)> { self.vector.as_ref() }

    /// Sets the default offset vector.
    pub fn set_vector(&mut self, value: Option<(u32, u32)>) { self.vector = value; }

    /// Gets the properties.
    pub fn properties(&self) -> &HashMap<String, Property> { &self.properties }

    /// Gets a mutable reference to the properties.
    pub fn properties_mut(&mut self) -> &mut HashMap<String, Property> { &mut self.properties }

    /// Gets the glyphs.
    pub fn glyphs(&self) -> &HashMap<char, Glyph> { &self.glyphs }

    /// Gets a mutable reference to the glyphs.
    pub fn glyphs_mut(&mut self) -> &mut HashMap<char, Glyph> { &mut self.glyphs }
}
