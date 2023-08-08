use crate::println;
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

pub fn print_bitmap_info() {
    let mut flag = MEMORY_MANAGER.lock().get_bit(0);
    let mut count = 0;
    let mut available_count = 0;
    let mut not_available_count = 0;
    for i in 0..FRAME_COUNT {
        if flag != MEMORY_MANAGER.lock().get_bit(i) {
            if flag {
                println!("not available: {}pages", count);
                not_available_count += count;
            } else {
                println!("available: {}pages", count);
                available_count += count;
            }
            flag = MEMORY_MANAGER.lock().get_bit(i);
            count = 1;
        } else {
            count += 1;
        }
    }
    if flag {
        println!("not available: {}pages", count);
        not_available_count += count;
    } else {
        println!("available: {}pages", count);
        available_count += count;
    }
    println!("available: {}pages", available_count);
    println!("not available: {}pages", not_available_count);
}

pub fn dump_memory_map() {
    let memory_manager = MEMORY_MANAGER.lock();
    for i in 0..FRAME_SIZE {
        println!("{:064b}", memory_manager.frame_bitmap[i]);
    }
    println!("range_begin: {}", memory_manager.range_begin.0);
    println!("range_end: {}", memory_manager.range_end.0);
}

pub fn dump_memory_map_by_range(start: usize, end: usize) {
    let memory_manager = MEMORY_MANAGER.lock();
    for i in start / BITS_PER_MAP_LINE..end / BITS_PER_MAP_LINE {
        println!("{:064b}", memory_manager.frame_bitmap[i]);
    }
    println!("range_begin: {}", memory_manager.range_begin.0);
    println!("range_end: {}", memory_manager.range_end.0);
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

// ページング（有効化されているはず、long mode）
// 権限の設定（GDT）
// CR3から逆算（writing os in rust）
// UEFIの仕様的に全メモリ（読み書きできるメモリ）がリニア（アイデンティティマップ）されているので、（仕様）
// 仮想アドレスと物理アドレスは同じ（仕様）
// ページテーブルの1bit目が有効無効を切り替える
// CR3をdumpしてその先をdumpしてみよう（実装するとむずいので大抵の場合内容を表示する関数を作っておくと便利）
// そのあと自分のやつに切り替えていく→うまくいかないときはprintしてみる
// ページングについて調べてdumpしてみる
// ページフォルトのハンドラ→アクセスできないことを
// アプリケーションをバイナリとして埋め込んで実行するという形であればユーザアプリケーションを動かせる
// カーネルのアドレスを直指定すれば実現可能
// pagingのセットアップってどうやるんだ
