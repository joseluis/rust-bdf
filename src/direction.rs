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
