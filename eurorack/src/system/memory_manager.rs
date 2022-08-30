use core::mem::MaybeUninit;

use daisy::sdram::SDRAM;
pub use sirena::memory_manager::MemoryManager;

pub fn new(sdram: SDRAM) -> MemoryManager {
    let ram_slice = unsafe {
        let ram_items = sdram.size() / core::mem::size_of::<MaybeUninit<u32>>();
        let ram_ptr = sdram.base_address as *mut MaybeUninit<u32>;
        core::slice::from_raw_parts_mut(ram_ptr, ram_items)
    };
    MemoryManager::from(ram_slice)
}
