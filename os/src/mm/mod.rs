mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::PhysPageNum;
pub use address::VirtAddr;
pub use memory_set::remap_test;
pub use memory_set::MapPermission;
pub use memory_set::MemorySet;
pub use memory_set::KERNEL_SPACE;
pub use page_table::translated_byte_buffer;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
