#[test]
fn open_gohufont_font() {
    let font = bdf::open("tests/gohufont.bdf").unwrap();
    assert_eq!(font.format(), "2.1");
}
#[test]
fn open_cozette_font() {
    let font = bdf::open("tests/cozette.bdf").unwrap();
    assert_eq!(font.format(), "2.1");
}

#[test]
#[should_panic]
fn open_fail() {
    bdf::open("hue").unwrap();
}
