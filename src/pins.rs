use ariel_os::hal::peripherals;

#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_13 });

#[cfg(context = "nrf52dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_17 });
