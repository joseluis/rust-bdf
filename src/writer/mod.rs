use crate::{Direction, Entry, Error, Font, Property};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[cfg(test)]
mod tests;

// helper
macro_rules! write { ($dst:expr, $($arg:tt)*) => ( $dst.write_all(format!($($arg)*).as_bytes())?) }

/// The font writer.
pub struct Writer<T: Write> {
    stream: BufWriter<T>,
}

#[rustfmt::skip]
impl<T: Write> From<T> for Writer<T> {
    fn from(stream: T) -> Writer<T> { Writer { stream: BufWriter::new(stream) } }
}

impl<T: Write> Writer<T> {
    /// Write an entry.
    pub fn entry(&mut self, entry: &Entry) -> Result<(), Error> {
        match *entry {
            Entry::StartFont(ref string) => write!(self.stream, "STARTFONT {}\n", string),
            Entry::Comment(ref string) => write!(
                self.stream,
                "COMMENT \"{}\"\n",
                string.replace("\"", "\"\"")
            ),
            Entry::ContentVersion(ref string) => {
                write!(self.stream, "CONTENTVERSION {}\n", string)
            }
            Entry::Font(ref string) => write!(self.stream, "FONT {}\n", string),
            Entry::Size(pt, x, y) => write!(self.stream, "SIZE {} {} {}\n", pt, x, y),
            Entry::Chars(chars) => write!(self.stream, "CHARS {}\n", chars),
            Entry::FontBoundingBox(ref bbx) => write!(
                self.stream,
                "FONTBOUNDINGBOX {} {} {} {}\n",
                bbx.width, bbx.height, bbx.x, bbx.y
            ),
            Entry::EndFont => write!(self.stream, "ENDFONT\n"),
            Entry::StartProperties(len) => write!(self.stream, "STARTPROPERTIES {}\n", len),
            Entry::Property(ref name, ref value) => match *value {
                Property::String(ref string) => write!(
                    self.stream,
                    "{} \"{}\"\n",
                    name,
                    string.replace("\"", "\"\"")
                ),
                Property::Integer(value) => write!(self.stream, "{} {}\n", name, value),
            },
            Entry::EndProperties => write!(self.stream, "ENDPROPERTIES\n"),
            Entry::StartChar(ref name) => write!(self.stream, "STARTCHAR {}\n", name),
            Entry::Encoding(value) => write!(self.stream, "ENCODING {}\n", value as u32),
            Entry::Direction(direction) => match direction {
                Direction::Default => write!(self.stream, "METRICSSET 0\n"),
                Direction::Alternate => write!(self.stream, "METRICSSET 1\n"),
                Direction::Both => write!(self.stream, "METRICSSET 2\n"),
            },
            Entry::ScalableWidth(x, y) => write!(self.stream, "SWIDTH {} {}\n", x, y),
            Entry::DeviceWidth(x, y) => write!(self.stream, "DWIDTH {} {}\n", x, y),
            Entry::AlternateScalableWidth(x, y) => write!(self.stream, "SWIDTH1 {} {}\n", x, y),
            Entry::AlternateDeviceWidth(x, y) => write!(self.stream, "DWIDTH1 {} {}\n", x, y),
            Entry::Vector(x, y) => write!(self.stream, "VVECTOR {} {}\n", x, y),
            Entry::BoundingBox(ref bbx) => write!(
                self.stream,
                "BBX {} {} {} {}\n",
                bbx.width, bbx.height, bbx.x, bbx.y
            ),

            Entry::Bitmap(ref map) => {
                write!(self.stream, "BITMAP\n");

                for y in 0..map.height() {
                    let mut value: u64 = 0;
                    for x in 0..map.width() {
                        value <<= 1;
                        value |= map.get(x, y) as u64;
                    }
                    value <<= (-(map.width() as i32)).rem_euclid(8);
                    let hex_width = ((map.width() + 7) >> 3 << 1) as usize;
                    write!(self.stream, "{:0>1$X}\n", value, hex_width);
                }
            }
            Entry::EndChar => write!(self.stream, "ENDCHAR\n"),
            Entry::Unknown(..) => unreachable!(),
        }
        Ok(())
    }
}

/// Create a `Writer` from a `Write`.
pub fn new<T: Write>(stream: T) -> Writer<T> {
    Writer::from(stream)
}

/// Save the font into a BDF file.
pub fn save<T: AsRef<Path>>(path: T, font: &Font) -> Result<(), Error> {
    write(File::create(path)?, font)
}

/// Write the font to the writer.
pub fn write<T: Write>(stream: T, font: &Font) -> Result<(), Error> {
    if !font.validate() {
        return Err(Error::MalformedFont);
    }
    if font.glyphs().iter().any(|(_, g)| !g.validate()) {
        return Err(Error::MalformedChar);
    }
    let mut writer = new(stream);
    writer.entry(&Entry::StartFont(font.format().to_owned()))?;
    writer.entry(&Entry::Font(font.name().to_owned()))?;
    writer.entry(&Entry::Size(font.size().pt, font.size().x, font.size().y))?;

    if let Some(version) = font.version() {
        writer.entry(&Entry::ContentVersion(version.to_owned()))?;
    }
    writer.entry(&Entry::FontBoundingBox(*font.bounds()))?;

    if font.direction() != Direction::Default {
        writer.entry(&Entry::Direction(font.direction()))?;
    }
    if let Some(&(x, y)) = font.scalable_width() {
        writer.entry(&Entry::ScalableWidth(x, y))?;
    }
    if let Some(&(x, y)) = font.device_width() {
        writer.entry(&Entry::DeviceWidth(x, y))?;
    }
    if let Some(&(x, y)) = font.alternate_scalable_width() {
        writer.entry(&Entry::AlternateScalableWidth(x, y))?;
    }
    if let Some(&(x, y)) = font.alternate_device_width() {
        writer.entry(&Entry::AlternateDeviceWidth(x, y))?;
    }
    if let Some(&(x, y)) = font.vector() {
        writer.entry(&Entry::Vector(x, y))?;
    }
    if !font.properties().is_empty() {
        writer.entry(&Entry::StartProperties(font.properties().len()))?;

        for (name, value) in font.properties() {
            writer.entry(&Entry::Property(name.clone(), value.clone()))?;
        }
        writer.entry(&Entry::EndProperties)?;
    }
    writer.entry(&Entry::Chars(font.glyphs().len()))?;

    for (codepoint, glyph) in font.glyphs() {
        writer.entry(&Entry::StartChar(glyph.name().to_owned()))?;
        writer.entry(&Entry::Encoding(*codepoint))?;

        if glyph.direction() != Direction::Default {
            writer.entry(&Entry::Direction(glyph.direction()))?;
        }
        if let Some(&(x, y)) = glyph.scalable_width() {
            writer.entry(&Entry::ScalableWidth(x, y))?;
        }
        if let Some(&(x, y)) = glyph.device_width() {
            writer.entry(&Entry::DeviceWidth(x, y))?;
        }
        if let Some(&(x, y)) = glyph.alternate_scalable_width() {
            writer.entry(&Entry::AlternateScalableWidth(x, y))?;
        }
        if let Some(&(x, y)) = glyph.alternate_device_width() {
            writer.entry(&Entry::AlternateDeviceWidth(x, y))?;
        }
        if let Some(&(x, y)) = glyph.vector() {
            writer.entry(&Entry::Vector(x, y))?;
        }
        writer.entry(&Entry::BoundingBox(*glyph.bounds()))?;
        writer.entry(&Entry::Bitmap(glyph.map().clone()))?;
        writer.entry(&Entry::EndChar)?;
    }
    writer.entry(&Entry::EndFont)?;
    Ok(())
}
