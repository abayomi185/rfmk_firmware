#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_semihosting;

use rtic::app;

// Check why I can't
#[cfg(feature = "52840")]
use nrf52840_hal as hal;

#[app(device = hal::pac)]
mod app {

    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init").unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("idle").unwrap();

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }
}
