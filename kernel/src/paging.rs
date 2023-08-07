use x86_64::structures::paging::PageTable;

use crate::println;

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
            }
        }
    }
}
