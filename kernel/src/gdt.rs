use x86_64::{
    registers::segmentation::*,
    structures::gdt::{Descriptor, GlobalDescriptorTable},
    structures::tss::TaskStateSegment,

    instructions::tables::load_tss,
    VirtAddr
};

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
pub static mut USER_CODE_SEGEMNT: SegmentSelector = SegmentSelector(0);

static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut KERNEL_CODE_SELECTOR: SegmentSelector = SegmentSelector(0);
static mut TSS_SELECTOR: SegmentSelector = SegmentSelector(0);
static mut TSS_STACK: [u8; 4096 * 5] = [0; 4096 * 5];



pub fn init() {
    unsafe {
        TSS.interrupt_stack_table[0] = VirtAddr::new(TSS_STACK.as_ptr() as u64 + TSS_STACK.len() as u64);
        // ref: mikan本 8.5-8.6
        KERNEL_CODE_SELECTOR = GDT.add_entry(Descriptor::kernel_code_segment());
        GDT.add_entry(Descriptor::kernel_data_segment());
        GDT.add_entry(Descriptor::user_code_segment());
        GDT.add_entry(Descriptor::user_data_segment());
        TSS_SELECTOR=GDT.add_entry(Descriptor::tss_segment(&TSS));
        GDT.load();

        DS::set_reg(SegmentSelector(0));
        ES::set_reg(SegmentSelector(0));
        FS::set_reg(SegmentSelector(0));
        GS::set_reg(SegmentSelector(0));

        CS::set_reg(SegmentSelector(1 << 3));
        SS::set_reg(SegmentSelector(2 << 3));
        load_tss(TSS_SELECTOR);
        
        // 代わりに3,4にして
        // ユーザ
        // GDTに書き込んだ段階でCPUの状態を書き換えることはない
        // レジスタに書き込んだ時にCPUがメモリを呼びにいく
        // code segment
    }
}

pub fn get_user_segment() -> (u16, u16) {
    let mut user_code_segment = SegmentSelector(3 << 3);
    user_code_segment.set_rpl(x86_64::PrivilegeLevel::Ring3);
    let mut user_data_segment = SegmentSelector(4 << 3);
    user_data_segment.set_rpl(x86_64::PrivilegeLevel::Ring3);
    (user_code_segment.0, user_data_segment.0)
}


pub fn set_user_segment() {
    unsafe {
        CS::set_reg(SegmentSelector(3 << 3));
        SS::set_reg(SegmentSelector(4 << 3));
    }
}
