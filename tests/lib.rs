use bdf::Font;

#[test]
fn open_gohufont_font() {
    let font = Font::open("tests/gohufont.bdf").unwrap();
    assert_eq!(font.format(), "2.1");
}
#[test]
fn open_cozette_font() {
    let font = Font::open("tests/cozette.bdf").unwrap();
    assert_eq!(font.format(), "2.1");
}

#[test]
#[should_panic]
fn open_fail() {
    Font::open("hue").unwrap();
}
