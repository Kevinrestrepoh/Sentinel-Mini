#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_hal::rtc_cntl::{Rtc, SocResetReason, reset_reason};
use esp_hal::system::Cpu;
use esp_hal::time::Rate;
use esp_println as _;
use esp_storage::FlashStorage;
use sentinel_mini::app::App;
use sentinel_mini::drivers::battery::Battery;
use sentinel_mini::drivers::buzzer::Buzzer;
use sentinel_mini::drivers::oled::Oled;
use sentinel_mini::drivers::touch::TouchSensor;

esp_bootloader_esp_idf::esp_app_desc!();

fn init_heap() {
    const HEAP_SIZE: usize = 8 * 1024;
    static mut HEAP: core::mem::MaybeUninit<[u8; HEAP_SIZE]> = core::mem::MaybeUninit::uninit();
    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            core::ptr::addr_of_mut!(HEAP) as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    init_heap();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let woke_from_deep_sleep = reset_reason(Cpu::ProCpu) == Some(SocResetReason::CoreDeepSleep);

    let i2c = I2c::new(
        peripherals.I2C0,
        Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO20)
    .with_scl(peripherals.GPIO21);

    let oled = Oled::new(i2c);
    let touch = TouchSensor::new(peripherals.GPIO1);
    let buzzer = Buzzer::new(peripherals.GPIO2);
    let battery = Battery::new();
    let flash = FlashStorage::new(peripherals.FLASH);

    let rtc = Rtc::new(peripherals.LPWR);

    let mut app = App::new(
        oled,
        touch,
        buzzer,
        battery,
        flash,
        rtc,
        woke_from_deep_sleep,
    );
    app.run();
}
