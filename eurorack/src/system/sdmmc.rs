use hal::device::SDMMC1;
use hal::sdmmc::{SdCard, Sdmmc};
use stm32h7xx_hal as hal;

pub type SDMMC = Sdmmc<SDMMC1, SdCard>;
