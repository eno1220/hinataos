use x86_64::structures::paging::{PageTableFlags, PhysFrame};
use x86_64::{structures::paging::*, PhysAddr, VirtAddr};

const EMPTY_ENTRY: PageTable = PageTable::new();

static mut PML4_TABLE: PageTable = PageTable::new();
static mut KERNEL_PDP_TABLE: PageTable = PageTable::new();
static mut KERNEL_PAGE_DIR: [PageTable; 64] = [EMPTY_ENTRY; 64];
// static mut KERNEL_PAGE_TABLES: [PageTable; 512] = [EMPTY_ENTRY; 512];

fn phys_frame_from_page_table(page_table: &PageTable) -> PhysFrame {
    PhysFrame::from_start_address(PhysAddr::new(page_table as *const _ as u64)).unwrap()
}

pub fn init() {
    let user_flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
    let kernel_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
        PML4_TABLE[0].set_frame(phys_frame_from_page_table(&KERNEL_PDP_TABLE), user_flags);

        for (i, table) in KERNEL_PAGE_DIR.iter_mut().enumerate() {
            KERNEL_PDP_TABLE[i].set_frame(phys_frame_from_page_table(table), user_flags);
            for (j, entry) in KERNEL_PAGE_DIR[i].iter_mut().enumerate() {
                let addr = i as u64 * Size1GiB::SIZE + j as u64 * Size2MiB::SIZE;
                entry.set_addr(PhysAddr::new(addr), kernel_flags | PageTableFlags::HUGE_PAGE);
            }
        }
        use x86_64::registers::control::{Cr3, Cr3Flags};
        Cr3::write(phys_frame_from_page_table(&PML4_TABLE), Cr3Flags::empty());
    }
}

fn translate_addr_inner(addr: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (level_4_table, _) = Cr3::read();
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table;

    for &index in &table_indexes {
        let virt = frame.start_address().as_u64();
        let tables_ptr = virt as *const PageTable;
        let table = unsafe { &*tables_ptr };

        let entry: &page_table::PageTableEntry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub unsafe fn translate_addr(addr: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr)
}

// todo(eno1220): dump page table