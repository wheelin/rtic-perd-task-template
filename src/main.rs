#![no_std]
#![no_main]

use panic_halt as _;
use rtic::{
    app,
    cyccnt::U32Ext,
};

use cortex_m::asm;

mod tasks_def;
mod periph;
mod sys;

const SYSFREQ : u32 = 72_000_000;

const fn time_to_tick(time_us : u32) -> u32 {
    (time_us as u64 * SYSFREQ as u64 / 1_000_000) as u32
}

static PERD_5MS : u32 = time_to_tick(5_000);
static PERD_10MS : u32 = PERD_5MS * 2;
static PERD_20MS : u32 = PERD_10MS * 2;
static PERD_50MS : u32 = PERD_10MS * 5;
static PERD_100MS : u32 = PERD_50MS * 2;
static PERD_1000MS : u32 = PERD_100MS * 10;

#[app(device = stm32f1::stm32f103, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const PERD_TASKS: () = {
    #[init(schedule = [proc_5ms, proc_10ms, proc_20ms, proc_50ms, proc_100ms, proc_1000ms])]
    fn init(mut cx : init::Context) {
        sys::init_clks();
        tasks_def::init_hw();
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        cx.schedule.proc_5ms(cx.start).unwrap();
        cx.schedule.proc_10ms(cx.start).unwrap();
        cx.schedule.proc_20ms(cx.start).unwrap();
        cx.schedule.proc_50ms(cx.start).unwrap();
        cx.schedule.proc_100ms(cx.start).unwrap();
        cx.schedule.proc_1000ms(cx.start).unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(schedule = [proc_5ms])]
    fn proc_5ms(cx : proc_5ms::Context) {
        tasks_def::task_5ms();
        cx.schedule.proc_5ms(cx.scheduled + PERD_5MS.cycles()).unwrap();
    }

    #[task(schedule = [proc_10ms])]
    fn proc_10ms(cx : proc_10ms::Context) {
        tasks_def::task_10ms();
        cx.schedule.proc_10ms(cx.scheduled + PERD_10MS.cycles()).unwrap();
    }

    #[task(schedule = [proc_20ms])]
    fn proc_20ms(cx : proc_20ms::Context) {
        tasks_def::task_20ms();
        cx.schedule.proc_20ms(cx.scheduled + PERD_20MS.cycles()).unwrap();
    }

    #[task(schedule = [proc_50ms])]
    fn proc_50ms(cx : proc_50ms::Context) {
        tasks_def::task_50ms();
        cx.schedule.proc_50ms(cx.scheduled + PERD_50MS.cycles()).unwrap();
    }

    #[task(schedule = [proc_100ms])]
    fn proc_100ms(cx : proc_100ms::Context) {
        tasks_def::task_100ms();
        cx.schedule.proc_100ms(cx.scheduled + PERD_100MS.cycles()).unwrap();
    }

    #[task(schedule = [proc_1000ms])]
    fn proc_1000ms(cx : proc_1000ms::Context) {
        tasks_def::task_1000ms();
        cx.schedule.proc_1000ms(cx.scheduled + PERD_1000MS.cycles()).unwrap();
    }

    extern "C" {
        fn TIM2();
    }
};
