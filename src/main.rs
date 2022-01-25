use embedded_graphics::mono_font::iso_8859_13::FONT_10X20;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_hal::spi::SPI2;

use std::{thread, time::Duration};

use embedded_graphics::{
    pixelcolor::BinaryColor::On as Black, prelude::*, primitives::{Line, PrimitiveStyle},
    mono_font::{MonoTextStyle},
    text::Text,
};
use epd_waveshare::{epd4in2::*, prelude::*,graphics::VarDisplay};



fn init_screen()-> (spi::Master<SPI2, gpio::Gpio13<gpio::Unknown>, gpio::Gpio14<gpio::Unknown>, gpio::Gpio12<gpio::Unknown>, gpio::Gpio15<gpio::Unknown>>, 
    Epd4in2<spi::Master<SPI2, gpio::Gpio13<gpio::Unknown>, gpio::Gpio14<gpio::Unknown>, gpio::Gpio12<gpio::Unknown>, gpio::Gpio15<gpio::Unknown>>, gpio::Gpio15<gpio::Output>, gpio::Gpio25<gpio::Input>, gpio::Gpio27<gpio::Output>, gpio::Gpio26<gpio::Output>, delay::Ets>){
        let peripherals = Peripherals::take().unwrap();
        let pins = peripherals.pins;
        
        let spi  = peripherals.spi2;
        let sclk = pins.gpio13;
        let sdo = pins.gpio14;
        let cs = pins.gpio15.into_output().unwrap();
        let busy_in = pins.gpio25.into_input().unwrap();
        let dc = pins.gpio27.into_output().unwrap();
        let rst = pins.gpio26.into_output().unwrap();

        let config = <spi::config::Config as Default>::default().baudrate(26.MHz().into());
        let mut my_spi = spi::Master::<spi::SPI2, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk: sclk,
                sdo: sdo ,
                sdi: Option::<gpio::Gpio12<gpio::Unknown>>::None,
                cs: Option::<gpio::Gpio15<gpio::Unknown>>::None,
            },
            config,
        ).unwrap();
        // Setup EPD
        let epd = Epd4in2::new(&mut my_spi, cs, busy_in, dc, rst, &mut delay::Ets).unwrap();
        (my_spi,epd)
}
fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let (mut my_spi, mut epd) = init_screen();


    // Use display graphics from embedded-graphics
    let mut buffer = vec![DEFAULT_BACKGROUND_COLOR.get_byte_value(); WIDTH as usize / 8 * HEIGHT as usize];
    let mut display = VarDisplay::new(WIDTH, HEIGHT, &mut buffer);


    let style = MonoTextStyle::new(&FONT_10X20, Black);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::new("Hello Rust!", Point::new(20, 30), style).draw(&mut display)?;
    
    // Use embedded graphics for drawing a line
    let _ = Line::new(Point::new(0, 120), Point::new(0, 295))
        .into_styled(PrimitiveStyle::with_stroke(Black, 1))
        .draw(&mut display);
    // Display updated frame

    epd.update_frame(&mut my_spi, &display.buffer(), &mut delay::Ets)?;
    epd.display_frame(&mut my_spi, &mut delay::Ets)?;

    // Set the EPD to sleep
    epd.sleep(&mut my_spi, &mut delay::Ets)?;
    loop {
        println!("Hello world");
        thread::sleep(Duration::from_secs(1));
    }
}
