// Embedded Graphics Demo for SMEC - CTCL 2021-2024
// Created: September 15, 2024
// Modified: September 26, 2024

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::Rgb666,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use u8g2_fonts::{fonts, types::{FontColor, HorizontalAlignment, VerticalPosition}, FontRenderer};

const FRAMERATE: u8 = 24;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb666> = SimulatorDisplay::new(Size::new(820, 320));

    display.fill_solid(&display.bounding_box(), Rgb666::BLACK)?;

    let font = FontRenderer::new::<fonts::u8g2_font_VCR_OSD_tf>();

    font.render_aligned(
        "INSUFFICIENT FIREPOWER",
        display.bounding_box().center() + Point::new(0, 16),
        VerticalPosition::Baseline,
        HorizontalAlignment::Center,
        FontColor::Transparent(Rgb666::RED),
        &mut display,
    ).unwrap();


    let output_settings = OutputSettingsBuilder::new()
        //.theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Demo", &output_settings).show_static(&display);

    Ok(())
}
