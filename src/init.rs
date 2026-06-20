extern crate alloc;

use alloc::boxed::Box;
use core::cmp::max;

use crate::{
    acpi::AcpiRsdpStruct,
    allocator::ALLOCATOR,
    hpet::{set_global_hpet, Hpet},
    info,
    uefi::{
        exit_from_efi_boot_services, EfiHandle, EfiMemoryType::*, EfiSystemTable, MemoryMapHolder,
    },
    x86::{write_cr3, PageAttr, PAGE_SIZE, PML4},
};

pub fn init_basic_runtime(
    image_handle: EfiHandle,
    efi_system_table: &EfiSystemTable,
) -> MemoryMapHolder {
    let mut memory_map = MemoryMapHolder::new();
    exit_from_efi_boot_services(image_handle, efi_system_table, &mut memory_map);
    ALLOCATOR.init_with_mmap(&memory_map);
    memory_map
}

pub fn init_paging(memory_map: &MemoryMapHolder) {
    let mut table = PML4::new();
    let mut end_of_mem = 0x1_0000_0000u64;
    for e in memory_map.iter() {
        match e.memory_type() {
            CONVENTIONAL_MEMORY | LOADER_CODE | LOADER_DATA => {
                end_of_mem = max(
                    end_of_mem,
                    e.physical_start() + e.number_of_pages() * (PAGE_SIZE as u64),
                )
            }
            _ => (),
        }
    }
    table
        .create_mapping(0, end_of_mem, 0, PageAttr::ReadWriteKernel)
        .expect("Failed to create initial page mapping");
    unsafe {
        write_cr3(Box::into_raw(table));
    }
}

pub fn init_hpet(acpi: &AcpiRsdpStruct) {
    let hpet = acpi.hpet().expect("Failed to get HPET from ACPI");
    let hpet = hpet
        .base_address()
        .expect("Failed to get HPET base address");
    info!("HPET found at {:#p}", hpet);
    let hpet = Hpet::new(hpet);
    set_global_hpet(hpet);
}
