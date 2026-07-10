#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod keymap;

// use defmt::{debug, error, info, warn};
// use defmt_rtt as _;
#[defmt::global_logger]
struct Logger;

unsafe impl defmt::Logger for Logger {
    fn acquire() {}
    unsafe fn flush() {}
    unsafe fn release() {}
    unsafe fn write(_bytes: &[u8]) {}
}

// use embassy_stm32::gpio::{Input, Level, Output, Speed};
// use embassy_stm32::peripherals::USB;
// use embassy_stm32::usb::{Driver, InterruptHandler};
// use embassy_stm32::{Config, bind_interrupts};

use ch32_hal as hal;
use hal::gpio::{Input, Level, Output, Speed};
use hal::usbd::Driver;
use hal::{Config, bind_interrupts};

use embassy_executor::Spawner;
use embassy_time::Timer;
use keymap::{COL, ROW};
use panic_halt as _;
use rmk::config::{BehaviorConfig, PositionalConfig, RmkConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::processor::builtin::wpm::WpmProcessor;
use rmk::usb::UsbTransport;
use rmk::{KeymapData, initialize_keymap, run_all};

// pinmaps from edg
// [
// i2c=I2C1,
// i2c.scl=PB6, 29,
// i2c.sda=PB7, 30,
// led=PB4, 27,
// enc_a=PA3, 9,
// enc_b=PA2, 8,
// enc_sw=PA1, 7,
// oled_rst=PB1, 15,
// npx=PB5, 28,
// sw_col_0=PA8, 18,
// sw_col_1=PA6, 12,
// sw_col_2=PA4, 10,
// sw_row_0=PA10, 20,
// sw_row_1=PA9, 19,
// sw_row_2=PA7, 13,
// sw_row_3=PA5, 11,
// 0=USB,
// 0.dp=PA12, 22,
// 0.dm=PA11, 21
// ]

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => hal::usbd::InterruptHandler<hal::peripherals::USBD>;
});

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) {
    // info!("main start");

    // Initialize peripherals
    let p = hal::init(hal::Config {
        rcc: hal::rcc::Config::SYSCLK_FREQ_144MHZ_HSI,
        ..Default::default()
    });

    // Usb driver
    let driver = Driver::new(p.USBD, Irqs, p.PA12, p.PA11);
    // info!("usb driver created");

    // Pin config
    let (row_pins, col_pins) =
        config_matrix_pins_ch32v!(peripherals: p, input: [PA10, PA9, PA7, PA5], output: [PA8, PA6, PA4]);
    // info!("matrix created");

    // Keyboard config
    let rmk_config = RmkConfig { ..Default::default() };

    // Initialize the storage and keymap
    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let mut behavior_config = BehaviorConfig::default();
    let per_key_config = PositionalConfig::default();
    let keymap = initialize_keymap(&mut keymap_data, &mut behavior_config, &per_key_config).await;
    // info!("keymap init'd");

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::new();
    let mut matrix = Matrix::<_, _, _, ROW, COL, true>::new(row_pins, col_pins, debouncer);
    let mut keyboard = Keyboard::new(&keymap);

    let mut usb_transport = UsbTransport::new(driver, rmk_config.device_config);
    let mut wpm_processor = WpmProcessor::new();

    // info!("init done");

    // Start
    run_all!(matrix, usb_transport, wpm_processor, keyboard).await;
}
