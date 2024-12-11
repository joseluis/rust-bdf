mod property {
    use crate::Property;

    #[test]
    fn parse_property() {
        assert_eq!(
            Property::String("Hello World".into()),
            Property::parse(r#""Hello World""#)
        );
        assert_eq!(
            Property::String("Hello World".into()),
            Property::parse(r#"Hello World"#)
        );
        assert_eq!(Property::Integer(41), Property::parse(r#"41"#));
        assert_eq!(Property::String("41".into()), Property::parse(r#""41""#));
    }
}

mod reader {
    use crate::{reader, Bitmap, BoundingBox, Direction, Entry, Property};

    pub fn assert(string: &str, entry: Entry) {
        let input = reader::new(string.as_bytes()).last().unwrap();

        assert_eq!(input, entry);
    }

    #[test]
    fn start_font() {
        assert("STARTFONT 2.2\n", Entry::StartFont("2.2".to_owned()));
    }

    #[test]
    fn comment() {
        assert("COMMENT \"hue\"\n", Entry::Comment("hue".to_owned()));
    }

    #[test]
    fn content_version() {
        assert(
            "CONTENTVERSION 1.0.0\n",
            Entry::ContentVersion("1.0.0".to_owned()),
        );
    }

    #[test]
    fn font() {
        assert(
            "FONT -Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1\n",
            Entry::Font("-Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1".to_owned()),
        );
    }

    #[test]
    fn size() {
        assert("SIZE 16 100 100\n", Entry::Size(16, 100, 100));
    }

    #[test]
    fn chars() {
        assert("CHARS 42\n", Entry::Chars(42));
    }

    #[test]
    fn font_bounding_box() {
        assert(
            "FONTBOUNDINGBOX 6 11 0 -2\n",
            Entry::FontBoundingBox(BoundingBox {
                width: 6,
                height: 11,
                x: 0,
                y: -2,
            }),
        );
    }

    #[test]
    fn end_font() {
        assert("ENDFONT\n", Entry::EndFont);
    }

    #[test]
    fn start_properties() {
        assert("STARTPROPERTIES 23\n", Entry::StartProperties(23));
    }

    #[test]
    fn property() {
        assert(
            "FOUNDRY \"GohuFont\"\n",
            Entry::Property(
                "FOUNDRY".to_owned(),
                Property::String("GohuFont".to_owned()),
            ),
        );

        assert(
            "X_HEIGHT 4\n",
            Entry::Property("X_HEIGHT".to_owned(), Property::Integer(4)),
        );
    }

    #[test]
    fn end_properties() {
        assert("ENDPROPERTIES\n", Entry::EndProperties);
    }

    #[test]
    fn start_char() {
        assert(
            "STARTCHAR <control>\n",
            Entry::StartChar("<control>".to_owned()),
        );
    }

    #[test]
    fn encoding() {
        assert("ENCODING 0\n", Entry::Encoding('\u{0}'));
    }

    #[test]
    fn direction() {
        assert("METRICSSET 0\n", Entry::Direction(Direction::Default));
        assert("METRICSSET 1\n", Entry::Direction(Direction::Alternate));
        assert("METRICSSET 2\n", Entry::Direction(Direction::Both));
    }

    #[test]
    fn scalable_width() {
        assert("SWIDTH 392 0\n", Entry::ScalableWidth(392, 0));
    }

    #[test]
    fn device_width() {
        assert("DWIDTH 6 0\n", Entry::DeviceWidth(6, 0));
    }

    #[test]
    fn alternate_scalable_width() {
        assert("SWIDTH1 392 0\n", Entry::AlternateScalableWidth(392, 0));
    }

    #[test]
    fn alternate_device_width() {
        assert("DWIDTH1 6 0\n", Entry::AlternateDeviceWidth(6, 0));
    }

    #[test]
    fn vector() {
        assert("VVECTOR 6 0\n", Entry::Vector(6, 0));
    }

    #[test]
    fn bounding_box() {
        assert(
            "BBX 6 11 0 -2\n",
            Entry::BoundingBox(BoundingBox {
                width: 6,
                height: 11,
                x: 0,
                y: -2,
            }),
        );
    }

    #[test]
    fn bitmap() {
        let mut bitmap = Bitmap::new(6, 11);

        // 00

        // 70
        bitmap.set(1, 1, true);
        bitmap.set(2, 1, true);
        bitmap.set(3, 1, true);

        // D8
        bitmap.set(0, 2, true);
        bitmap.set(1, 2, true);
        bitmap.set(3, 2, true);
        bitmap.set(4, 2, true);

        // D8
        bitmap.set(0, 3, true);
        bitmap.set(1, 3, true);
        bitmap.set(3, 3, true);
        bitmap.set(4, 3, true);

        // F8
        bitmap.set(0, 4, true);
        bitmap.set(1, 4, true);
        bitmap.set(2, 4, true);
        bitmap.set(3, 4, true);
        bitmap.set(4, 4, true);

        // D8
        bitmap.set(0, 5, true);
        bitmap.set(1, 5, true);
        bitmap.set(3, 5, true);
        bitmap.set(4, 5, true);

        // D8
        bitmap.set(0, 6, true);
        bitmap.set(1, 6, true);
        bitmap.set(3, 6, true);
        bitmap.set(4, 6, true);

        // D8
        bitmap.set(0, 7, true);
        bitmap.set(1, 7, true);
        bitmap.set(3, 7, true);
        bitmap.set(4, 7, true);

        // D8
        bitmap.set(0, 8, true);
        bitmap.set(1, 8, true);
        bitmap.set(3, 8, true);
        bitmap.set(4, 8, true);

        // 00

        // 00

        assert(
            "BBX 6 11 0 -2\n\
             BITMAP\n\
             00\n\
             70\n\
             D8\n\
             D8\n\
             F8\n\
             D8\n\
             D8\n\
             D8\n\
             D8\n\
             00\n\
             00\n",
            Entry::Bitmap(bitmap),
        );
    }

    #[test]
    fn end_char() {
        assert("ENDCHAR\n", Entry::EndChar);
    }

    #[test]
    fn unknown() {
        assert("HUE", Entry::Unknown("HUE".to_owned()));
    }
}

mod writer {
    use crate::{writer, Bitmap, BoundingBox, Direction, Entry, Property};
    use core::str::from_utf8;

    pub fn assert(entry: Entry, string: &str) {
        let mut output = Vec::new();

        {
            let mut writer = writer::new(&mut output);
            writer.entry(&entry).unwrap();
        }

        println!("in: {}", from_utf8(&output).unwrap());
        println!("out: {}", &string);

        assert_eq!(from_utf8(&output).unwrap(), string);
    }

    #[test]
    fn start_font() {
        assert(Entry::StartFont("2.2".to_owned()), "STARTFONT 2.2\n");
    }

    #[test]
    fn comment() {
        assert(Entry::Comment("test".to_owned()), "COMMENT \"test\"\n");
    }

    #[test]
    fn content_version() {
        assert(
            Entry::ContentVersion("1.0.0".to_owned()),
            "CONTENTVERSION 1.0.0\n",
        );
    }

    #[test]
    fn font() {
        assert(
            Entry::Font("-Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1".to_owned()),
            "FONT -Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1\n",
        );
    }

    #[test]
    fn size() {
        assert(Entry::Size(16, 100, 100), "SIZE 16 100 100\n");
    }

    #[test]
    fn chars() {
        assert(Entry::Chars(42), "CHARS 42\n");
    }

    #[test]
    fn font_bounding_box() {
        assert(
            Entry::FontBoundingBox(BoundingBox {
                width: 6,
                height: 11,
                x: 0,
                y: -2,
            }),
            "FONTBOUNDINGBOX 6 11 0 -2\n",
        );
    }

    #[test]
    fn end_font() {
        assert(Entry::EndFont, "ENDFONT\n");
    }

    #[test]
    fn start_properties() {
        assert(Entry::StartProperties(23), "STARTPROPERTIES 23\n");
    }

    #[test]
    fn property() {
        assert(
            Entry::Property(
                "FOUNDRY".to_owned(),
                Property::String("GohuFont".to_owned()),
            ),
            "FOUNDRY \"GohuFont\"\n",
        );

        assert(
            Entry::Property("X_HEIGHT".to_owned(), Property::Integer(4)),
            "X_HEIGHT 4\n",
        );
    }

    #[test]
    fn end_properties() {
        assert(Entry::EndProperties, "ENDPROPERTIES\n");
    }

    #[test]
    fn start_char() {
        assert(
            Entry::StartChar("<control>".to_owned()),
            "STARTCHAR <control>\n",
        );
    }

    #[test]
    fn encoding() {
        assert(Entry::Encoding('\u{0}'), "ENCODING 0\n");
    }

    #[test]
    fn direction() {
        assert(Entry::Direction(Direction::Default), "METRICSSET 0\n");
        assert(Entry::Direction(Direction::Alternate), "METRICSSET 1\n");
        assert(Entry::Direction(Direction::Both), "METRICSSET 2\n");
    }

    #[test]
    fn scalable_width() {
        assert(Entry::ScalableWidth(392, 0), "SWIDTH 392 0\n");
    }

    #[test]
    fn device_width() {
        assert(Entry::DeviceWidth(6, 0), "DWIDTH 6 0\n");
    }

    #[test]
    fn alternate_scalable_width() {
        assert(Entry::AlternateScalableWidth(392, 0), "SWIDTH1 392 0\n");
    }

    #[test]
    fn alternate_device_width() {
        assert(Entry::AlternateDeviceWidth(6, 0), "DWIDTH1 6 0\n");
    }

    #[test]
    fn vector() {
        assert(Entry::Vector(6, 0), "VVECTOR 6 0\n");
    }

    #[test]
    fn bounding_box() {
        assert(
            Entry::BoundingBox(BoundingBox {
                width: 6,
                height: 11,
                x: 0,
                y: -2,
            }),
            "BBX 6 11 0 -2\n",
        );
    }

    #[test]
    fn bitmap() {
        let mut bitmap = Bitmap::new(6, 11);

        // 00

        // 70
        bitmap.set(1, 1, true);
        bitmap.set(2, 1, true);
        bitmap.set(3, 1, true);

        // D8
        bitmap.set(0, 2, true);
        bitmap.set(1, 2, true);
        bitmap.set(3, 2, true);
        bitmap.set(4, 2, true);

        // D8
        bitmap.set(0, 3, true);
        bitmap.set(1, 3, true);
        bitmap.set(3, 3, true);
        bitmap.set(4, 3, true);

        // F8
        bitmap.set(0, 4, true);
        bitmap.set(1, 4, true);
        bitmap.set(2, 4, true);
        bitmap.set(3, 4, true);
        bitmap.set(4, 4, true);

        // D8
        bitmap.set(0, 5, true);
        bitmap.set(1, 5, true);
        bitmap.set(3, 5, true);
        bitmap.set(4, 5, true);

        // D8
        bitmap.set(0, 6, true);
        bitmap.set(1, 6, true);
        bitmap.set(3, 6, true);
        bitmap.set(4, 6, true);

        // D8
        bitmap.set(0, 7, true);
        bitmap.set(1, 7, true);
        bitmap.set(3, 7, true);
        bitmap.set(4, 7, true);

        // D8
        bitmap.set(0, 8, true);
        bitmap.set(1, 8, true);
        bitmap.set(3, 8, true);
        bitmap.set(4, 8, true);

        // 00

        // 00

        assert(
            Entry::Bitmap(bitmap),
            "BITMAP\n\
             00\n\
             70\n\
             D8\n\
             D8\n\
             F8\n\
             D8\n\
             D8\n\
             D8\n\
             D8\n\
             00\n\
             00\n",
        );
    }

    #[test]
    fn end_char() {
        assert(Entry::EndChar, "ENDCHAR\n");
    }

    #[test]
    #[should_panic]
    fn unknown() {
        assert(Entry::Unknown("HUE".to_owned()), "");
    }
}
