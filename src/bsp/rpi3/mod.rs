/// Used by `arch` code to find the early boot core.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start_arguments")]
pub static BOOT_CORE_ID: u64 = 0;
