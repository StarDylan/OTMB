#![no_std]
#![no_main]

use cortex_m::Peripherals;
// use defmt::*;
// use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, gpio::{Level, Output, Speed}, peripherals, rng::{self, Rng}, time::Hertz, Config};
use rand_core::RngCore;
// use embassy_time::Timer;
// use {defmt_rtt as _, panic_probe as _};


// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     let config = Config::default();

//     let p = embassy_stm32::init(Default::default());
//     info!("Hello World!");

//     let mut rng = Rng::new(p.RNG, Irqs);

//     let mut led = Output::new(p.PB7, Level::High, Speed::Low);

//     let data = rng.next_u64() % 10;
    
//     loop {
//         info!("high");
//         led.set_high();
//         Timer::after_secs(data).await;

//         info!("low");
//         led.set_low();
//         Timer::after_millis(300).await;
//     }
// }

bind_interrupts!(struct Irqs {
    HASH_RNG => rng::InterruptHandler<peripherals::RNG>;
});

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {



    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Bypass,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL180,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 180 / 2 = 180Mhz.
            divq: None,
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }

    let p = embassy_stm32::init(config);

    info!("Hello World!");
    
    let mut rng = Rng::new(p.RNG, Irqs);

    let mut led = Output::new(p.PB7, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_secs((rng.next_u64() % 4)  + 1).await;

        info!("low");
        led.set_low();
        Timer::after_secs((rng.next_u64() % 4) + 1).await;
    }
}