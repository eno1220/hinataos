use crate::memory::alloc;
use crate::println;
use x86::bits32::paging::Page;
use x86_64::structures::paging::{PageTableFlags, PhysFrame};
use x86_64::{instructions::tables, structures::paging::*, PhysAddr, VirtAddr};

const EMPTY_ENTRY: PageTable = PageTable::new();

static mut PML4_TABLE: PageTable = PageTable::new();
static mut KERNEL_PDP_TABLE: PageTable = PageTable::new();
static mut KERNEL_PAGE_DIR: [PageTable; 64] = [EMPTY_ENTRY; 64];
static mut KERNEL_PAGE_TABLES: [PageTable; 512] = [EMPTY_ENTRY; 512];

fn phys_frame_from_page_table(page_table: &PageTable) -> PhysFrame {
    PhysFrame::from_start_address(PhysAddr::new(page_table as *const _ as u64)).unwrap()
}

// メモリ領域を開け渡したい→メモリマップは見えるのでその中で適当な連続したアドレスをもらってくる
// その部分をユーザ用に使ってしまう
// structの中に入れてstructのrefを適当な（user）の戦闘領域
// 対応するところのuserbitを立てる

// カーネル空間は適当な範囲まで identity mapping する
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
                if i == 63 {
                    // 64番目のページディレクトリはユーザ空間に割り当てる
                    entry.set_addr(
                        PhysAddr::new(addr),
                        kernel_flags | PageTableFlags::HUGE_PAGE,
                    );
                    //println!("addr: {:x}", addr);
                } else {
                    entry.set_addr(PhysAddr::new(addr), user_flags | PageTableFlags::HUGE_PAGE);
                }
                //entry.set_addr(PhysAddr::new(addr), user_flags | PageTableFlags::HUGE_PAGE);
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

/*
pub fn dump_page_table() {
    use x86_64::registers::control::Cr3;
    let (level_4_table, _) = Cr3::read();

    let virtual_addt = level_4_table.start_address().as_u64();
    let page_table_ptr = virtual_addt as *const PageTable;

    unsafe {
        let page_table = &*page_table_ptr;
        for (i, entry) in page_table.iter().enumerate() {
            if !entry.is_unused() {
                println!("L4 Entry {}: {:?}", i, entry);
                let page_table_ptr = entry.addr().as_u64() as *const PageTable;
                let page_table = &*page_table_ptr;
                for (i, entry) in page_table.iter().enumerate() {
                    if !entry.is_unused() {
                        println!("  L3 Entry {}: {:?}", i, entry);
                        if entry
                            .flags()
                            .contains(x86_64::structures::paging::PageTableFlags::HUGE_PAGE)
                        {
                            continue;
                        }
                        let page_table_ptr = entry.addr().as_u64() as *const PageTable;
                        let page_table = &*page_table_ptr;
                        for (i, entry) in page_table.iter().enumerate() {
                            if !entry.is_unused() {
                                println!("    L2 Entry {}: {:?}", i, entry);
                                if entry
                                    .flags()
                                    .contains(x86_64::structures::paging::PageTableFlags::HUGE_PAGE)
                                {
                                    continue;
                                }
                                let page_table_ptr = entry.addr().as_u64() as *const PageTable;
                                let page_table = &*page_table_ptr;
                                for (i, entry) in page_table.iter().enumerate() {
                                    if !entry.is_unused() {
                                        println!("      L1 Entry {}: {:?}", i, entry);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}*/

// rustはコピー前提で動くので
// ページテーブルの適当な場所のユーザビットを立てる
// CR3にもう一度代入する（TLBをクリアする）
