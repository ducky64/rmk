macro_rules! config_matrix_pins_ch32v {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new($p.$out_pin, ch32_hal::gpio::Level::Low, ch32_hal::gpio::Speed::High)), +];
            let input_pins = [$(Input::new($p.$in_pin, ch32_hal::gpio::Pull::Down)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
