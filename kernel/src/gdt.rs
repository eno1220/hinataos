use x86_64::{
    registers::segmentation::*,
    structures::gdt::{Descriptor, GlobalDescriptorTable},
};

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

pub fn init() {
    unsafe {
        // ref: mikan本 8.5-8.6
        GDT.add_entry(Descriptor::kernel_code_segment());
        GDT.add_entry(Descriptor::kernel_data_segment());
        GDT.add_entry(Descriptor::user_code_segment());
        GDT.add_entry(Descriptor::user_data_segment());
        GDT.load();

        DS::set_reg(SegmentSelector(0));
        ES::set_reg(SegmentSelector(0));
        FS::set_reg(SegmentSelector(0));
        GS::set_reg(SegmentSelector(0));

        CS::set_reg(SegmentSelector(1 << 3));
        SS::set_reg(SegmentSelector(2 << 3));
        // 代わりに3,4にして
        // ユーザ
        // GDTに書き込んだ段階でCPUの状態を書き換えることはない
        // レジスタに書き込んだ時にCPUがメモリを呼びにいく
        // code segment
    }
}

pub fn get_user_segment() -> (u16,u16){
        (SegmentSelector(3 << 3).0, SegmentSelector(4 << 3).0)
}