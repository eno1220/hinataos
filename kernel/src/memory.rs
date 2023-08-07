use core::cell::OnceCell;

use crate::println;
use common::types::{is_available, MemoryMap};

pub fn init(memory_map: &MemoryMap) {
    for descriptor in &memory_map.buffer {
        if is_available(descriptor.ty) {
            println!(
                "addr: [{:#010x} - {:#010x}], len: {:#06} KiB, type: {:?}",
                descriptor.phys_start,
                descriptor.phys_start + descriptor.page_count * 4 * 1024 - 1,
                descriptor.page_count * 4,
                descriptor.ty
            );
        }
    }
}

pub struct FrameID {
    pub frame_id: usize,
}

const MAX_PHYSICAL_MEMORY_BYTES: usize = 0x100000000; // 4GiB
const FRAME_SIZE: usize = MAX_PHYSICAL_MEMORY_BYTES / 0x1000; // 4MiB
const BITS_PER_MAP_LINE: usize = 64;

pub struct BitmapMemoryManager {
    frame_bitmap: [u64; FRAME_SIZE / BITS_PER_MAP_LINE],
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
