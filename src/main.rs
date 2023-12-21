use std::error::Error;

use rppal::spi::{Bus, SlaveSelect, Mode, Spi};
use rppal::system::DeviceInfo;

const CLOCK_FREQUENCY: u32 = 2_400_000;
const START: u8 = 0b0000_0001;
const CONFIG: u8 = 0b1110_0000;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Device model: {}.", DeviceInfo::new()?.model());

    let spi0 = Spi::new(Bus::Spi0, SlaveSelect::Ss0, CLOCK_FREQUENCY, Mode::Mode0)?;
    let mut buffer = [0u8; 3];
    let mut signal = [0u16; 100];
    for i in 0usize.. {
        spi0.transfer(&mut buffer, &[START, CONFIG, 0])?;
        if i == signal.len() {
            break;
        }
        signal[i % signal.len()] = (buffer[1] as u16) << 8 | buffer[2] as u16;
    }
    for value in signal {
        println!("{}", value);
    }

    Ok(())
}
