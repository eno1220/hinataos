use crate::println;

use x86_64::{
    instructions::interrupts,
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
    interrupts::disable();
    unsafe {
        IDT.page_fault
            .set_handler_fn(page_fault_handler)
            .set_stack_index(0);
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(0);
        IDT.general_protection_fault
            .set_handler_fn(general_protection_fault_handler)
            .set_stack_index(0);
        IDT.load();
    }
    interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("[EXCEPTION] BREAKPOINT\nStack Frame: {:?}", stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "[EXCEPTION] GENERAL PROTECTION FAULT\nError Code: {:?}\nStack Frame: {:?}",
        error_code, stack_frame
    );
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!(
        "[EXCEPTION] PAGE FAULT\nAccessed Address: {:?}\nError Code: {:?}\nStack Frame: {:?}",
        Cr2::read(),
        error_code,
        stack_frame
    );

    /*let rip = &mut unsafe { stack_frame.as_mut() }
        .extract_inner()
        .instruction_pointer;

    *rip = VirtAddr::new(rip.as_u64() + 3);*/
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("[EXCEPTION] DOUBLE FAULT\nStack Frame: {:?}", stack_frame);
}
