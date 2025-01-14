#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::RgbColor,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder,
    },
    prelude::Point,
    text::{Alignment, Text},
    Drawable,
};

use esp32s3_hal::{
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    spi,
    timer::TimerGroup,
    Rtc,
    IO,
    Delay,
};

use mipidsi::DisplayOptions;

use core::f32::consts::PI;
use libm::{sin, cos};

#[allow(unused_imports)]
use esp_backtrace as _;

use xtensa_lx_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();

    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;

    let spi = spi::Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        4u32.MHz(),
        spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let mut backlight = io.pins.gpio45.into_push_pull_output();
    backlight.set_high().unwrap();

    let reset = io.pins.gpio48.into_push_pull_output();

    let di = SPIInterfaceNoCS::new(spi, io.pins.gpio4.into_push_pull_output());

    let display_options = DisplayOptions {
        orientation: mipidsi::Orientation::PortraitInverted(false),
        ..Default::default()
    };

    let mut display = mipidsi::Display::ili9342c_rgb565(di, core::prelude::v1::Some(reset), display_options);
    display.init(&mut delay).unwrap();

    let default_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::BLACK)
        .build();

    let mut vt;
    let mut x;
    let mut y;

    //body
    for i in 0..7000 {
        vt = i as f64 / (40.0 * PI as f64);
        x = (vt - 50.0) * sin(vt);
        y = (vt + 50.0) *  cos(vt);
        if i < 6500 || i > 6900 {
            Text::with_alignment("'", Point::new((x + 220.0) as i32, (y + 200.0) as i32), default_style,  Alignment::Center)
                .draw(&mut display)
                .unwrap();
        }
        
    }

    //head
    for i in 0..7000 {
        vt = i as f64 / (60.0 * PI as f64);
        x = (vt + 50.0) * cos(vt);
        y = (vt -  50.0) * sin(vt);
        
        Text::with_alignment("'", Point::new((x + 220.0) as i32, (y + 60.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();

    }

    //eyes
    for i in 0..1300 {
        vt = i as f64 / (20.0 * PI as f64);
        x = (vt - 15.0) * sin(vt);
        y = (vt -  15.0) * cos(vt);
        
        Text::with_alignment("'", Point::new((x + 200.0) as i32, (y + 60.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    for i in 0..1300 {
        vt = i as f64 / (20.0 * PI as f64);
        x = (vt - 15.0) * sin(vt);
        y = (vt -  15.0) * cos(vt);
        
        Text::with_alignment("'", Point::new((x + 240.0) as i32, (y + 60.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    //hand

    let mut b;
    for a in (125..175).rev() {
        b = a;
        Text::with_alignment("-", Point::new(a, b), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    //lollipop

    for i in 0..3300 {
        vt = i as f64 / (30.0 * PI as f64);
        x = (vt - 30.0) * sin(vt);
        y = (vt -  30.0) * cos(vt);
        
        Text::with_alignment("'", Point::new((x + 110.0) as i32, (y + 110.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }
    
    
    loop {}
}