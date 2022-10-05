use notan_egui::*;

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn configure_text_styles(ctx: &notan_egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(14.0, Proportional)),
        (heading2(), FontId::new(13.0, Proportional)),
        (heading3(), FontId::new(12.0, Proportional)),
        (TextStyle::Body, FontId::new(10.0, Proportional)),
        (TextStyle::Monospace, FontId::new(10.0, Proportional)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ].into();

    ctx.set_style(style);
}