use defmt::{error, info};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

pub struct Oled<I2C> {
    pub display: Ssd1306<
        I2CInterface<I2C>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl<I2C> Oled<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub fn new(i2c: I2C) -> Self {
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        match display.init() {
            Ok(_) => {
                info!("OLED init OK ");
            }
            Err(_) => {
                error!("OLED init FAILED at 0x3C — check wiring and pull-up resistors");
                error!("SDA=GPIO21, SCL=GPIO20. If your module uses 0x3D, update the address.");
                panic!("OLED init failed");
            }
        }

        Self { display }
    }

    pub fn print_line(&mut self, text: &str, y: i32) {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();
        Text::with_baseline(text, Point::new(0, y), style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn draw_lines(&mut self, lines: &[&str]) {
        self.display.clear(BinaryColor::Off).unwrap();
        for (i, line) in lines.iter().enumerate() {
            self.print_line(line, i as i32 * 12);
        }
        self.display.flush().unwrap();
    }
}
