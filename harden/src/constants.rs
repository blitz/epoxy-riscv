/// The virtual address in processes where mappings of resources start.
///
/// TODO This should be configurable, because it might conflict with the addresses at which the
/// binaries are linked.
pub const VIRT_RESOURCE_START: u64 = 0x40000000;

/// The end of the resource area in processes.
pub const VIRT_RESOURCE_END: u64 = 0x50000000;

/// The default stack size for user programs.
pub const USER_STACK_SIZE: u64 = 0x4000;

/// The default page size.
pub const PAGE_SIZE: u64 = 0x1000;
