//! Draw a demo screen for the 128x160 SH1108 display.
//!
//! This example is for the STM32F103 "Blue Pill" board using SPI.
//!
//! Wiring connections are as follows:
//!
//! ```
//! Display -> Blue Pill
//!     GND -> GND
//!     VCC -> 3.3V or 5V (check your module's input voltage)
//!     SCK -> PA5
//!    MOSI -> PA7
//!      DC -> PA2
//!      CS -> PA1 (optional, connect to ground on module if unused)
//! ```
//!
//! Run on a Blue Pill with `cargo run --example demo`.

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use display_interface_spi::SPIInterface;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_hal::spi;
use panic_semihosting as _;
use sh1108::{prelude::*, Builder};
use stm32f1xx_hal::{prelude::*, spi::Spi, stm32};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpioa = dp.GPIOA.split();

    let mut res = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let dc = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let cs = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut delay = cp.SYST.delay(&clocks);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        spi::MODE_0,
        400u32.kHz(),
        clocks,
    );

    let spi_interface = SPIInterface::new(spi, dc);

    let mut disp: GraphicsMode<_> = Builder::new().connect(spi_interface).into();

    disp.reset(&mut res, &mut delay).unwrap();

    disp.init().unwrap();
    disp.flush().unwrap();

    draw_demo(&mut disp).unwrap();

    disp.flush().unwrap();

    loop {}
}

fn draw_demo<D>(disp: &mut D) -> Result<(), D::Error>
where
    D: embedded_graphics::draw_target::DrawTarget<Color = BinaryColor>,
{
    use embedded_graphics::{
        image::{Image, ImageRawLE},
        mono_font::MonoTextStyle,
        prelude::*,
        primitives::{Circle, Line, PrimitiveStyleBuilder, Rectangle, Triangle},
        text::Text,
    };
    use profont::{PROFONT_10_POINT, PROFONT_12_POINT, PROFONT_24_POINT, PROFONT_7_POINT};

    let rust_im: ImageRawLE<BinaryColor> = ImageRawLE::new(include_bytes!("./rust.raw"), 64);
    let qr_im: ImageRawLE<BinaryColor> = ImageRawLE::new(include_bytes!("./qr.raw"), 33);
    let dither_im: ImageRawLE<BinaryColor> = ImageRawLE::new(include_bytes!("./dither.raw"), 36);

    let filled_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::Off)
        .fill_color(BinaryColor::On)
        .stroke_width(1)
        .build();
    let stroked_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    let title_textstyle = MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::On);
    let subtitle_textstyle = MonoTextStyle::new(&PROFONT_12_POINT, BinaryColor::On);
    let normal_textstyle = MonoTextStyle::new(&PROFONT_10_POINT, BinaryColor::On);
    let inverted_textstyle = MonoTextStyle::new(&PROFONT_10_POINT, BinaryColor::Off);
    let tiny_textstyle = MonoTextStyle::new(&PROFONT_7_POINT, BinaryColor::On);

    // Title
    Text::new("SH1108", Point::new(0, 20), title_textstyle).draw(disp)?;
    // Next to title
    Text::new("RUST", Point::new(100, 7), tiny_textstyle).draw(disp)?;
    Text::new("CRATE", Point::new(100, 18), tiny_textstyle).draw(disp)?;
    // Subtitle
    Text::new("Mono OLED Driver", Point::new(0, 32), subtitle_textstyle).draw(disp)?;
    // Divider
    Line::new(Point::new(0, 38), Point::new(128, 38))
        .into_styled(stroked_style)
        .draw(disp)?;

    // Features
    Text::new("-SPI/I2C/6800/8080", Point::new(0, 50), normal_textstyle).draw(disp)?;
    Text::new("-embedded_graphics", Point::new(0, 62), normal_textstyle).draw(disp)?;
    Text::new("-embedded_hal", Point::new(0, 74), normal_textstyle).draw(disp)?;

    // Dithered gradient
    Image::new(&dither_im, Point::new(92, 68)).draw(disp)?;

    // Rust logo
    Image::new(&rust_im, Point::new(64, 96)).draw(disp)?;

    // Display size
    Text::new("CURRENT CONFIG", Point::new(0, 90), tiny_textstyle).draw(disp)?;
    Rectangle::new(Point::new(-1, 92), Size::new(98, 16))
        .into_styled(filled_style)
        .draw(disp)?;
    Text::new("1.92\" 128x160", Point::new(3, 103), inverted_textstyle).draw(disp)?;

    // Primitive shapes
    Rectangle::new(Point::new(40, 112), Size::new(16, 16))
        .into_styled(stroked_style)
        .draw(disp)?;
    Circle::new(Point::new(43, 126), 16)
        .into_styled(stroked_style)
        .draw(disp)?;
    Triangle::new(
        Point::new(54, 140),
        Point::new(46, 155),
        Point::new(62, 155),
    )
    .into_styled(stroked_style)
    .draw(disp)?;

    // QR code
    Text::new("DOCS.RS", Point::new(0, 124), tiny_textstyle).draw(disp)?;
    Image::new(&qr_im, Point::new(0, 127)).draw(disp)?;

    Ok(())
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
