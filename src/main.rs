#![no_std]
#![no_main]

use cortex_m_rt::entry;
use smart_leds_trait::{SmartLedsWrite, RGB8};
use stm32f4xx_hal::{
    dma::{Stream5, StreamsTuple},
    gpio::GpioExt,
    pac::{
        self, DMA1, TIM2, TIM3,
    },
    prelude::*,
    rcc::RccExt,
};
use {defmt_rtt as _, panic_probe as _};

use pwm_dma::Ws2812Pwm;

pub const LED_COUNT: usize = 8;
pub const RESET: usize = 40;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(26.MHz())
        .sysclk(100.MHz())
        .hclk(100.MHz())
        .pclk1(50.MHz())
        .pclk2(100.MHz())
        .freeze();
    let gpiob = dp.GPIOB.split();

    let mut delay = TIM2::delay_us(dp.TIM2, &clocks);

    let led_buf = {
        static mut LED_BUF: [u16; 24 * (LED_COUNT) + RESET] = [0; 24 * (LED_COUNT) + RESET];
        unsafe { &mut LED_BUF }
    };

    let dma1 = StreamsTuple::new(dp.DMA1);

    let ws_pin = gpiob.pb5.into_alternate();

    let mut ws: Ws2812Pwm<TIM3, Stream5<DMA1>, _, 5, 1, 800_000u32> =
        Ws2812Pwm::new(dp.TIM3, ws_pin, dma1.5, led_buf, &clocks);

    let buf = [
        RGB8::new(255, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(255, 30, 0),
        RGB8::new(255, 180, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 255, 0),
    ];

    loop {
        ws.write((0..LED_COUNT).enumerate().map(|(ix, _)| buf[ix]))
            .unwrap();
        delay.delay_ms(44_u32);
    }
}
