#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::pwm::SetDutyCycle;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use panic_halt as _;

enum Command {
    Move { axis: char, position: f32 },
    SetServo { pin: u8, angle: u16 },
}

enum Status {
    Ok,
    Position { axis: char, pos: f32 },
}

static COMMAND_CHANNEL: Channel<ThreadModeRawMutex, Command, 10> = Channel::new();

#[cfg(feature = "rp2040")]
use rp2040_hal::{entry as _, pac, pwm::{Channel as PwmChannel, Slices}, gpio::{Pins, Pin, FunctionPwm}};

#[entry]
fn main() -> ! {
    #[cfg(feature = "rp2040")]
    {
        let p = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut pwm_slices = Slices::new(p.PWM, &mut p.RESETS);

        let mut pwm = pwm_slices.pwm0;
        pwm.set_ph_correct();
        pwm.enable();
        let mut channel: PwmChannel<_, _> = pwm.channel_a;
        let pins = Pins::new(p.IO_BANK0, &mut p.PADS_BANK0, p.SPIO0, &mut p.RESETS);
        let pwm_pin: Pin<_, FunctionPwm, _> = pins.gpio0.into_function();

        loop {
            if let Some(cmd) = COMMAND_CHANNEL.try_receive() {
                match cmd {
                    Command::SetServo { pin: _, angle } => {
                        let duty = (angle as u32 * 1000 / 180 + 1000) as u16;
                        channel.set_duty_cycle(duty).ok();
                    }
                    _ => {}
                }
            }
        }
    }

    #[cfg(not(feature = "rp2040"))]
    loop {}
}