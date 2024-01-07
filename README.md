Current issues that need fixing:
        - currently needs bits = LED_COUNT + 1
        - reset is now at 16 duty cycles


Reminders:
    dma:
        - arr - auto reload register - these are the TIMs which have channels; responsible for interrupts - https://ziblog.ru/2011/01/15/stm32-chast-8-ndash-taymeryi-obshhego-naznacheniya-preryivaniya.html
        - pre-scalar - cuts the frequency of timer ticks - TIM1 & TIM8, while 2-5 are general-purpose timers; others are basic
        - stm32 metapac - crate for working with dma stream mappings, interrupts, etc - https://crates.io/crates/stm32-metapac

    pwm:
        - cce??

    ws2812-pwm-dma crate which this was initially based of doesnt currently work because of crate conflicts and bugs (bivtec 1.0.1 -> funty -1.1.0 but  funy is 2.0.0 and doesnt let earlier version get added (still doesnt fix the issue))

    stm32f3xx-hal has a ::pwm, can study further what theyve done (does seem to use sys clock thou)