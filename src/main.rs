#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_semihosting;

use rtic::app;

// Check why I can't
#[cfg(feature = "52840")]
use nrf52840_hal as hal;

// This works but this is the pac and not hal
// use nrf52840_pac as pac;

#[app(device = crate::hal::pac)]
mod app {

    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("idle");

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }
}
