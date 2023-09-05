use common::types::{is_available, MemoryMap};
use spin::Mutex;

pub static MEMORY_MANAGER: Mutex<BitmapMemoryManager> = Mutex::new(BitmapMemoryManager::new());

pub fn init(memory_map: &MemoryMap) {
    let buffer = memory_map.buffer;
    let mut available_end = 0;
    for i in 0..memory_map.length {
        let memory_descriptor = buffer[i];
        let physical_start = memory_descriptor.phys_start as usize;
        let number_of_pages = memory_descriptor.page_count as usize;
        if available_end < physical_start {
            MEMORY_MANAGER.lock().mark_allocated(
                available_end / FRAME_SIZE,
                (physical_start - available_end) / FRAME_SIZE,
            );
        }
        if is_available(memory_descriptor.ty) {
            available_end = physical_start + number_of_pages * FRAME_SIZE;
        } else {
            MEMORY_MANAGER
                .lock()
                .mark_allocated(physical_start / FRAME_SIZE, number_of_pages);
        }
    }
    MEMORY_MANAGER
        .lock()
        .set_memory_range(FrameID(0), FrameID(available_end / FRAME_SIZE));
}

pub fn alloc(num_frames: usize) -> usize {
    let frame_id = MEMORY_MANAGER.lock().allocate(num_frames);
    frame_id.0 * FRAME_SIZE
}

// ptrじゃなくてframe_idを渡すようにした方が良い?
// エラー処理も必要
pub fn free(ptr: *mut u8, num_frames: usize) {
    MEMORY_MANAGER
        .lock()
        .free(ptr as usize / FRAME_SIZE, num_frames);
}

pub struct FrameID(usize);

impl FrameID {
    pub fn new(frame_id: usize) -> Self {
        Self(frame_id)
    }

    pub fn to_physical_address(&self) -> usize {
        self.0 * FRAME_SIZE
    }
}

const FRAME_SIZE: usize = 0x1000; // 4KiB

const MAX_PHYSICAL_MEMORY_BYTES: usize = 0x100000000; // 4GiB
const FRAME_COUNT: usize = MAX_PHYSICAL_MEMORY_BYTES / FRAME_SIZE;
const BITS_PER_MAP_LINE: usize = 64;

pub struct BitmapMemoryManager {
    frame_bitmap: [u64; FRAME_COUNT / BITS_PER_MAP_LINE],
    range_begin: FrameID,
    range_end: FrameID,
}

impl BitmapMemoryManager {
    pub const fn new() -> Self {
        Self {
            frame_bitmap: [0; FRAME_COUNT / BITS_PER_MAP_LINE],
            range_begin: FrameID(0),
            range_end: FrameID(FRAME_COUNT),
        }
    }

    pub fn allocate(&mut self, num_frames: usize) -> FrameID {
        let mut start_frame_id = self.range_begin.0;
        loop {
            let mut i: usize = 0;
            while i < num_frames {
                if start_frame_id + i >= self.range_end.0 {
                    panic!("no enough memory");
                }
                if self.get_bit(start_frame_id + i) {
                    break;
                }
                i += 1;
            }

            if i == num_frames {
                for j in 0..num_frames {
                    self.set_bit(start_frame_id + j, true);
                }
                return FrameID::new(start_frame_id);
            } else {
                start_frame_id += i + 1;
            }
        }
    }

    pub fn free(&mut self, start_frame_id: usize, num_frames: usize) {
        for i in 0..num_frames {
            self.set_bit(start_frame_id + i, false);
        }
    }

    fn get_bit(&self, frame_id: usize) -> bool {
        let line = frame_id / BITS_PER_MAP_LINE;
        let bit = frame_id % BITS_PER_MAP_LINE;
        (self.frame_bitmap[line] >> bit) & 1 == 1
    }

    fn set_bit(&mut self, frame_id: usize, value: bool) {
        let line = frame_id / BITS_PER_MAP_LINE;
        let bit = frame_id % BITS_PER_MAP_LINE;
        if value {
            self.frame_bitmap[line] |= 1 << bit;
        } else {
            self.frame_bitmap[line] &= !(1 << bit);
        }
    }

    pub fn mark_allocated(&mut self, start_frame_id: usize, num_frames: usize) {
        for i in 0..num_frames {
            self.set_bit(start_frame_id + i, true);
        }
    }

    pub fn set_memory_range(&mut self, range_begin: FrameID, range_end: FrameID) {
        self.range_begin = range_begin;
        self.range_end = range_end;
    }
}


// todo(eno1220): メモリの情報を取得する→メモリマップ用の領域をallocする→allocしたメモリを使ってメモリマップを作成する

/* 
use common::types::{is_available, MemoryMap};
use spin::Mutex;

pub struct BitmapMemoryManager {
    // todo(eno1220): 動的に決定する
    bitmap_address: usize,
    length: usize,
}

const BITS_PER_MAP_LINE: usize = 64;

pub static mut MEMORY_MANAGER: Mutex<BitmapMemoryManager> = Mutex::new(BitmapMemoryManager::new());

impl BitmapMemoryManager{
    pub const fn new() -> Self{
        Self{
            bitmap_address: 0,
            length: 0,
        }
    }

    pub fn init(&mut self, memory_map: &MemoryMap){
        let mut total_pages = 0;
        // todo: fix
        for i in 0..memory_map.length{
            total_pages += memory_map.buffer[i].page_count as usize;
        }
        let bitmap_length = total_pages / BITS_PER_MAP_LINE;

        let mut bitmap_address = 0;
        for i in 0..memory_map.length{
            let memory_descriptor = memory_map.buffer[i];
            // todo: fix
            if is_available(memory_descriptor.ty) && (memory_descriptor.page_count * 4096) as usize >= bitmap_length{
                bitmap_address = memory_descriptor.phys_start as usize;
                log::info!("bitmap_address: {}",bitmap_address);
                log::info!("{:?}" ,memory_descriptor);
                break;
            }
        }

        self.bitmap_address = bitmap_address;
        self.length = bitmap_length;

        for i in 0..memory_map.length{
            let memory_discriptor = memory_map.buffer[i];
            if !is_available(memory_discriptor.ty){
                self.mark_allocated(memory_discriptor.phys_start as usize / 4096, memory_discriptor.page_count as usize);
                log::info!("phys_start: {}",memory_discriptor.phys_start as usize / 4096);
                log::info!("page_count: {}",memory_discriptor.page_count as usize);
            }
        }

        // 考える

        self.mark_allocated(self.bitmap_address / 4096, bitmap_length / 64 / 4096);
        log::info!("bitmap_addr: {}",self.bitmap_address);
        log::info!("length: {}",self.length);
    }

    fn mark_allocated(&mut self, start_frame: usize, num: usize) {
        for i in 0..num {
            self.set_bit_allocated(start_frame + i);
        }
    }

    fn set_bit_allocated(&mut self, frame_id: usize) {
        let line = frame_id / BITS_PER_MAP_LINE;
        let bit = frame_id % BITS_PER_MAP_LINE;
            unsafe{
                *((self.bitmap_address + line * 8) as *mut u64) |= 1 << bit;
            }
    }

    fn set_bit_deallocated(&mut self, frame_id: usize) {
        let line = frame_id / BITS_PER_MAP_LINE;
        let bit = frame_id % BITS_PER_MAP_LINE;
            unsafe{
                *((self.bitmap_address + line * 8) as *mut u64) &= !(1 << bit);
            }
    }
}*/