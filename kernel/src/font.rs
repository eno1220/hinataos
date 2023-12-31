pub static FONT: [[u8; 16]; 256] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [
        0, 0, 56, 68, 130, 170, 170, 130, 130, 170, 146, 68, 56, 0, 0, 0,
    ],
    [
        0, 0, 56, 124, 254, 214, 214, 254, 254, 214, 238, 124, 56, 0, 0, 0,
    ],
    [0, 0, 0, 0, 108, 254, 254, 254, 124, 56, 16, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 16, 56, 124, 254, 124, 56, 16, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 16, 56, 84, 254, 84, 16, 56, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 16, 56, 124, 254, 214, 16, 56, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 24, 60, 60, 24, 0, 0, 0, 0, 0, 0],
    [
        255, 255, 255, 255, 255, 255, 231, 195, 195, 231, 255, 255, 255, 255, 255, 255,
    ],
    [0, 0, 0, 0, 0, 60, 102, 66, 66, 102, 60, 0, 0, 0, 0, 0],
    [
        255, 255, 255, 255, 255, 195, 153, 189, 189, 153, 195, 255, 255, 255, 255, 255,
    ],
    [
        0, 16, 56, 84, 146, 16, 16, 56, 68, 130, 130, 130, 68, 56, 0, 0,
    ],
    [
        0, 56, 68, 130, 130, 130, 68, 56, 16, 16, 254, 16, 16, 16, 0, 0,
    ],
    [0, 0, 12, 14, 11, 11, 10, 8, 8, 24, 120, 248, 112, 0, 0, 0],
    [0, 0, 31, 31, 17, 17, 17, 17, 17, 17, 119, 255, 102, 0, 0, 0],
    [0, 0, 0, 0, 16, 84, 56, 40, 56, 84, 16, 0, 0, 0, 0, 0],
    [
        0, 128, 192, 224, 240, 248, 252, 254, 252, 248, 240, 224, 192, 128, 0, 0,
    ],
    [0, 2, 6, 14, 30, 62, 126, 254, 126, 62, 30, 14, 6, 2, 0, 0],
    [0, 0, 16, 56, 84, 146, 16, 16, 16, 146, 84, 56, 16, 0, 0, 0],
    [0, 0, 68, 68, 68, 68, 68, 68, 68, 68, 0, 0, 68, 68, 0, 0],
    [
        0, 62, 74, 138, 138, 138, 138, 74, 58, 10, 10, 10, 10, 10, 0, 0,
    ],
    [
        124, 130, 64, 32, 56, 68, 130, 130, 130, 68, 56, 8, 4, 130, 124, 0,
    ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 254, 254, 0, 0],
    [
        0, 0, 16, 56, 84, 146, 16, 16, 16, 146, 84, 56, 16, 124, 0, 0,
    ],
    [0, 16, 56, 84, 146, 16, 16, 16, 16, 16, 16, 16, 16, 16, 0, 0],
    [0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 146, 84, 56, 16, 0, 0],
    [0, 0, 0, 0, 16, 8, 4, 254, 4, 8, 16, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 16, 32, 64, 254, 64, 32, 16, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 254, 0, 0],
    [0, 0, 0, 0, 0, 40, 68, 254, 68, 40, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 16, 16, 56, 56, 124, 124, 254, 254, 0, 0, 0, 0],
    [0, 0, 0, 0, 254, 254, 124, 124, 56, 56, 16, 16, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 0, 0, 16, 16, 0, 0],
    [40, 40, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [
        0, 68, 68, 68, 254, 68, 68, 68, 68, 68, 254, 68, 68, 68, 0, 0,
    ],
    [
        16, 58, 86, 146, 146, 144, 80, 56, 20, 18, 146, 146, 212, 184, 16, 16,
    ],
    [
        98, 146, 148, 148, 104, 8, 16, 16, 32, 44, 82, 82, 146, 140, 0, 0,
    ],
    [
        0, 112, 136, 136, 136, 144, 96, 71, 162, 146, 138, 132, 70, 57, 0, 0,
    ],
    [4, 8, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 4, 8, 8, 16, 16, 16, 16, 16, 16, 16, 8, 8, 4, 2, 0],
    [
        128, 64, 32, 32, 16, 16, 16, 16, 16, 16, 16, 32, 32, 64, 128, 0,
    ],
    [0, 0, 0, 0, 0, 16, 146, 84, 56, 84, 146, 16, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 16, 16, 16, 254, 16, 16, 16, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 24, 8, 8, 16],
    [0, 0, 0, 0, 0, 0, 0, 0, 254, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 24, 0, 0],
    [2, 2, 4, 4, 8, 8, 8, 16, 16, 32, 32, 64, 64, 64, 128, 128],
    [0, 24, 36, 36, 66, 66, 66, 66, 66, 66, 66, 36, 36, 24, 0, 0],
    [0, 8, 24, 40, 8, 8, 8, 8, 8, 8, 8, 8, 8, 62, 0, 0],
    [0, 24, 36, 66, 66, 2, 4, 8, 16, 32, 32, 64, 64, 126, 0, 0],
    [0, 24, 36, 66, 2, 2, 4, 24, 4, 2, 2, 66, 36, 24, 0, 0],
    [0, 12, 12, 12, 20, 20, 20, 36, 36, 68, 126, 4, 4, 30, 0, 0],
    [0, 124, 64, 64, 64, 88, 100, 2, 2, 2, 2, 66, 36, 24, 0, 0],
    [0, 24, 36, 66, 64, 88, 100, 66, 66, 66, 66, 66, 36, 24, 0, 0],
    [0, 126, 66, 66, 4, 4, 8, 8, 8, 16, 16, 16, 16, 56, 0, 0],
    [0, 24, 36, 66, 66, 66, 36, 24, 36, 66, 66, 66, 36, 24, 0, 0],
    [0, 24, 36, 66, 66, 66, 66, 66, 38, 26, 2, 66, 36, 24, 0, 0],
    [0, 0, 0, 0, 0, 24, 24, 0, 0, 0, 0, 0, 24, 24, 0, 0],
    [0, 0, 0, 0, 0, 24, 24, 0, 0, 0, 0, 24, 24, 8, 8, 16],
    [0, 2, 4, 8, 16, 32, 64, 128, 128, 64, 32, 16, 8, 4, 2, 0],
    [0, 0, 0, 0, 0, 0, 254, 0, 0, 254, 0, 0, 0, 0, 0, 0],
    [0, 128, 64, 32, 16, 8, 4, 2, 2, 4, 8, 16, 32, 64, 128, 0],
    [0, 56, 68, 130, 130, 130, 4, 8, 16, 16, 0, 0, 24, 24, 0, 0],
    [
        0, 56, 68, 130, 154, 170, 170, 170, 170, 170, 156, 128, 70, 56, 0, 0,
    ],
    [
        0, 24, 24, 24, 24, 36, 36, 36, 36, 126, 66, 66, 66, 231, 0, 0,
    ],
    [
        0, 240, 72, 68, 68, 68, 72, 120, 68, 66, 66, 66, 68, 248, 0, 0,
    ],
    [
        0, 58, 70, 66, 130, 128, 128, 128, 128, 128, 130, 66, 68, 56, 0, 0,
    ],
    [
        0, 248, 68, 68, 66, 66, 66, 66, 66, 66, 66, 68, 68, 248, 0, 0,
    ],
    [
        0, 254, 66, 66, 64, 64, 68, 124, 68, 64, 64, 66, 66, 254, 0, 0,
    ],
    [
        0, 254, 66, 66, 64, 64, 68, 124, 68, 68, 64, 64, 64, 240, 0, 0,
    ],
    [
        0, 58, 70, 66, 130, 128, 128, 158, 130, 130, 130, 66, 70, 56, 0, 0,
    ],
    [
        0, 231, 66, 66, 66, 66, 66, 126, 66, 66, 66, 66, 66, 231, 0, 0,
    ],
    [
        0, 124, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 124, 0, 0,
    ],
    [0, 31, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 132, 72, 48, 0],
    [
        0, 231, 66, 68, 72, 80, 80, 96, 80, 80, 72, 68, 66, 231, 0, 0,
    ],
    [
        0, 240, 64, 64, 64, 64, 64, 64, 64, 64, 64, 66, 66, 254, 0, 0,
    ],
    [
        0, 195, 66, 102, 102, 102, 90, 90, 90, 66, 66, 66, 66, 231, 0, 0,
    ],
    [
        0, 199, 66, 98, 98, 82, 82, 82, 74, 74, 74, 70, 70, 226, 0, 0,
    ],
    [
        0, 56, 68, 130, 130, 130, 130, 130, 130, 130, 130, 130, 68, 56, 0, 0,
    ],
    [
        0, 248, 68, 66, 66, 66, 68, 120, 64, 64, 64, 64, 64, 240, 0, 0,
    ],
    [
        0, 56, 68, 130, 130, 130, 130, 130, 130, 130, 146, 138, 68, 58, 0, 0,
    ],
    [
        0, 252, 66, 66, 66, 66, 124, 68, 66, 66, 66, 66, 66, 231, 0, 0,
    ],
    [
        0, 58, 70, 130, 130, 128, 64, 56, 4, 2, 130, 130, 196, 184, 0, 0,
    ],
    [
        0, 254, 146, 146, 16, 16, 16, 16, 16, 16, 16, 16, 16, 124, 0, 0,
    ],
    [0, 231, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 36, 60, 0, 0],
    [0, 231, 66, 66, 66, 66, 36, 36, 36, 36, 24, 24, 24, 24, 0, 0],
    [0, 231, 66, 66, 66, 90, 90, 90, 90, 36, 36, 36, 36, 36, 0, 0],
    [
        0, 231, 66, 66, 36, 36, 36, 24, 36, 36, 36, 66, 66, 231, 0, 0,
    ],
    [
        0, 238, 68, 68, 68, 40, 40, 40, 16, 16, 16, 16, 16, 124, 0, 0,
    ],
    [
        0, 254, 132, 132, 8, 8, 16, 16, 32, 32, 64, 66, 130, 254, 0, 0,
    ],
    [0, 62, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 62, 0],
    [128, 128, 64, 64, 32, 32, 32, 16, 16, 8, 8, 4, 4, 4, 2, 2],
    [0, 124, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 124, 0],
    [0, 16, 40, 68, 130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 0],
    [16, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 112, 8, 4, 60, 68, 132, 132, 140, 118, 0, 0],
    [
        192, 64, 64, 64, 64, 88, 100, 66, 66, 66, 66, 66, 100, 88, 0, 0,
    ],
    [0, 0, 0, 0, 0, 48, 76, 132, 132, 128, 128, 130, 68, 56, 0, 0],
    [
        12, 4, 4, 4, 4, 52, 76, 132, 132, 132, 132, 132, 76, 54, 0, 0,
    ],
    [0, 0, 0, 0, 0, 56, 68, 130, 130, 252, 128, 130, 66, 60, 0, 0],
    [
        14, 16, 16, 16, 16, 124, 16, 16, 16, 16, 16, 16, 16, 16, 0, 0,
    ],
    [0, 0, 0, 0, 0, 54, 76, 132, 132, 132, 132, 76, 52, 4, 4, 120],
    [
        192, 64, 64, 64, 64, 88, 100, 66, 66, 66, 66, 66, 66, 227, 0, 0,
    ],
    [0, 16, 16, 0, 0, 48, 16, 16, 16, 16, 16, 16, 16, 56, 0, 0],
    [0, 4, 4, 0, 0, 12, 4, 4, 4, 4, 4, 4, 4, 8, 8, 48],
    [
        192, 64, 64, 64, 64, 78, 68, 72, 80, 96, 80, 72, 68, 230, 0, 0,
    ],
    [
        112, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 14, 0, 0,
    ],
    [0, 0, 0, 0, 0, 246, 73, 73, 73, 73, 73, 73, 73, 219, 0, 0],
    [0, 0, 0, 0, 0, 216, 100, 66, 66, 66, 66, 66, 66, 227, 0, 0],
    [0, 0, 0, 0, 0, 56, 68, 130, 130, 130, 130, 130, 68, 56, 0, 0],
    [
        0, 0, 0, 0, 0, 216, 100, 66, 66, 66, 66, 66, 100, 88, 64, 224,
    ],
    [
        0, 0, 0, 0, 0, 52, 76, 132, 132, 132, 132, 132, 76, 52, 4, 14,
    ],
    [0, 0, 0, 0, 0, 220, 98, 66, 64, 64, 64, 64, 64, 224, 0, 0],
    [
        0, 0, 0, 0, 0, 122, 134, 130, 192, 56, 6, 130, 194, 188, 0, 0,
    ],
    [0, 0, 16, 16, 16, 124, 16, 16, 16, 16, 16, 16, 16, 14, 0, 0],
    [0, 0, 0, 0, 0, 198, 66, 66, 66, 66, 66, 66, 70, 59, 0, 0],
    [0, 0, 0, 0, 0, 231, 66, 66, 66, 36, 36, 36, 24, 24, 0, 0],
    [0, 0, 0, 0, 0, 231, 66, 66, 90, 90, 90, 36, 36, 36, 0, 0],
    [0, 0, 0, 0, 0, 198, 68, 40, 40, 16, 40, 40, 68, 198, 0, 0],
    [0, 0, 0, 0, 0, 66, 66, 66, 36, 36, 36, 24, 24, 16, 16, 96],
    [0, 0, 0, 0, 0, 254, 2, 4, 8, 16, 32, 64, 128, 254, 0, 0],
    [0, 6, 8, 16, 16, 16, 16, 96, 16, 16, 16, 16, 8, 6, 0, 0],
    [
        16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    ],
    [0, 96, 16, 8, 8, 8, 8, 6, 8, 8, 8, 8, 16, 96, 0, 0],
    [0, 114, 140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 16, 40, 68, 130, 254, 130, 254, 0, 0, 0, 0, 0],
    [
        0, 56, 68, 130, 128, 128, 128, 128, 128, 128, 128, 130, 68, 56, 16, 32,
    ],
    [
        0, 0, 36, 36, 0, 130, 130, 130, 130, 130, 130, 130, 66, 62, 0, 0,
    ],
    [
        12, 8, 16, 0, 0, 56, 68, 130, 130, 254, 128, 130, 68, 56, 0, 0,
    ],
    [0, 16, 40, 68, 0, 120, 4, 4, 60, 68, 132, 132, 68, 62, 0, 0],
    [0, 0, 36, 36, 0, 120, 4, 4, 60, 68, 132, 132, 68, 62, 0, 0],
    [16, 8, 4, 0, 0, 120, 4, 4, 60, 68, 132, 132, 68, 62, 0, 0],
    [0, 24, 36, 24, 0, 120, 4, 4, 60, 68, 132, 132, 68, 62, 0, 0],
    [
        0, 0, 0, 0, 0, 60, 66, 128, 128, 128, 128, 128, 66, 60, 8, 16,
    ],
    [
        0, 16, 40, 68, 0, 56, 68, 130, 130, 254, 128, 130, 68, 56, 0, 0,
    ],
    [
        0, 0, 36, 36, 0, 56, 68, 130, 130, 254, 128, 130, 68, 56, 0, 0,
    ],
    [
        16, 8, 4, 0, 0, 56, 68, 130, 130, 254, 128, 130, 68, 56, 0, 0,
    ],
    [0, 0, 36, 36, 0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 0, 0],
    [0, 16, 40, 68, 0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 0, 0],
    [16, 8, 4, 0, 0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 0, 0],
    [
        36, 36, 0, 56, 68, 130, 130, 130, 130, 254, 130, 130, 130, 130, 0, 0,
    ],
    [
        0, 56, 68, 56, 68, 130, 130, 130, 130, 254, 130, 130, 130, 130, 0, 0,
    ],
    [
        12, 8, 16, 254, 128, 128, 128, 128, 248, 128, 128, 128, 128, 254, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 96, 28, 18, 114, 158, 144, 144, 146, 108, 0, 0,
    ],
    [
        12, 16, 32, 40, 40, 40, 254, 40, 40, 40, 40, 40, 40, 40, 0, 0,
    ],
    [
        0, 16, 40, 68, 0, 56, 68, 130, 130, 130, 130, 130, 68, 56, 0, 0,
    ],
    [
        0, 0, 36, 36, 0, 56, 68, 130, 130, 130, 130, 130, 68, 56, 0, 0,
    ],
    [
        16, 8, 4, 0, 0, 56, 68, 130, 130, 130, 130, 130, 68, 56, 0, 0,
    ],
    [
        0, 16, 40, 68, 0, 130, 130, 130, 130, 130, 130, 130, 66, 62, 0, 0,
    ],
    [
        16, 8, 4, 0, 0, 130, 130, 130, 130, 130, 130, 130, 66, 62, 0, 0,
    ],
    [
        0, 0, 36, 36, 0, 130, 130, 68, 68, 40, 40, 16, 16, 32, 32, 64,
    ],
    [
        36, 36, 0, 56, 68, 130, 130, 130, 130, 130, 130, 130, 68, 56, 0, 0,
    ],
    [
        36, 36, 0, 130, 130, 130, 130, 130, 130, 130, 130, 130, 68, 56, 0, 0,
    ],
    [
        0, 40, 40, 40, 60, 106, 168, 168, 168, 168, 168, 106, 60, 40, 40, 40,
    ],
    [
        0, 12, 18, 32, 32, 32, 252, 32, 32, 32, 96, 160, 178, 76, 0, 0,
    ],
    [
        0, 130, 130, 68, 40, 16, 254, 16, 16, 254, 16, 16, 16, 16, 0, 0,
    ],
    [
        0, 224, 144, 136, 136, 136, 148, 228, 159, 132, 132, 132, 132, 132, 0, 0,
    ],
    [
        0, 12, 18, 16, 16, 16, 254, 16, 16, 16, 16, 16, 144, 96, 0, 0,
    ],
    [12, 8, 16, 0, 0, 120, 4, 4, 60, 68, 132, 132, 68, 62, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 144, 144, 96],
    [
        240, 128, 128, 128, 128, 128, 128, 128, 128, 128, 0, 0, 0, 0, 0, 0,
    ],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 15],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 64, 32, 16],
    [0, 0, 0, 0, 0, 0, 0, 0, 126, 0, 0, 0, 0, 0, 0, 0],
    [0, 255, 1, 1, 1, 1, 1, 254, 2, 4, 4, 8, 8, 16, 96, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 1, 10, 8, 16, 32, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 12, 48, 200, 8, 8, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 255, 129, 2, 4, 8, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 124, 16, 16, 16, 254, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 254, 24, 40, 72, 136, 8],
    [0, 0, 0, 0, 0, 0, 0, 0, 32, 47, 242, 36, 16, 16, 16, 16],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 8, 8, 8, 8, 255, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 1, 1, 255, 1, 1, 255],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 73, 1, 2, 4, 8, 16],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 254, 0, 0, 0, 0, 0, 0],
    [0, 0, 255, 1, 18, 20, 16, 16, 16, 16, 32, 32, 64, 64, 128, 0],
    [0, 0, 1, 3, 12, 48, 200, 136, 8, 8, 8, 8, 8, 8, 0, 0],
    [0, 0, 16, 16, 255, 129, 129, 2, 2, 4, 4, 8, 8, 16, 32, 0],
    [0, 0, 124, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 254, 0, 0],
    [0, 0, 0, 8, 8, 255, 24, 24, 40, 40, 72, 72, 136, 24, 8, 0],
    [0, 0, 16, 16, 255, 17, 33, 33, 33, 33, 65, 66, 146, 12, 4, 0],
    [0, 0, 16, 16, 23, 56, 208, 11, 28, 232, 8, 8, 8, 4, 4, 0],
    [0, 0, 0, 31, 17, 17, 17, 17, 33, 33, 66, 2, 4, 4, 8, 0],
    [0, 0, 32, 32, 63, 34, 66, 66, 132, 4, 4, 8, 8, 16, 32, 0],
    [0, 0, 0, 0, 127, 1, 1, 1, 1, 1, 1, 127, 0, 0, 0, 0],
    [0, 0, 68, 68, 255, 68, 132, 132, 8, 8, 8, 16, 16, 32, 32, 0],
    [0, 0, 0, 96, 24, 1, 97, 25, 2, 2, 4, 8, 16, 32, 64, 0],
    [0, 0, 0, 252, 4, 4, 8, 8, 16, 16, 40, 40, 68, 68, 130, 0],
    [0, 0, 0, 32, 32, 32, 63, 225, 34, 32, 32, 32, 32, 31, 0, 0],
    [0, 0, 0, 65, 33, 33, 17, 2, 2, 2, 2, 4, 4, 8, 16, 0],
    [0, 0, 63, 33, 33, 65, 89, 134, 2, 2, 4, 4, 8, 8, 16, 0],
    [0, 0, 0, 12, 56, 8, 8, 255, 8, 16, 16, 16, 32, 32, 64, 0],
    [0, 0, 0, 73, 73, 73, 1, 2, 2, 2, 2, 4, 4, 8, 16, 0],
    [0, 0, 0, 126, 0, 0, 255, 8, 8, 8, 16, 16, 32, 32, 64, 0],
    [0, 0, 16, 16, 16, 16, 16, 24, 20, 20, 18, 18, 16, 16, 16, 0],
    [0, 0, 0, 8, 8, 8, 255, 8, 16, 16, 16, 32, 32, 32, 64, 0],
    [0, 0, 0, 0, 126, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0],
    [0, 0, 0, 255, 1, 2, 2, 36, 20, 8, 12, 18, 16, 32, 32, 0],
    [0, 0, 16, 16, 126, 2, 4, 8, 28, 42, 73, 136, 8, 8, 8, 0],
    [0, 0, 2, 2, 2, 2, 2, 4, 4, 8, 8, 16, 16, 32, 32, 0],
    [
        0, 0, 0, 0, 36, 66, 66, 66, 129, 129, 129, 129, 129, 129, 0, 0,
    ],
    [0, 0, 0, 32, 32, 39, 56, 32, 32, 32, 32, 32, 32, 16, 15, 0],
    [0, 0, 0, 255, 1, 1, 1, 1, 1, 1, 2, 2, 4, 8, 16, 0],
    [0, 0, 0, 0, 16, 40, 68, 132, 2, 2, 1, 1, 0, 0, 0, 0],
    [
        0, 0, 16, 16, 16, 255, 16, 16, 84, 84, 146, 146, 16, 16, 0, 0,
    ],
    [0, 0, 0, 255, 1, 2, 2, 4, 68, 40, 24, 8, 4, 2, 1, 0],
    [0, 0, 96, 24, 6, 0, 0, 96, 24, 6, 0, 0, 96, 24, 6, 0],
    [0, 0, 0, 8, 8, 16, 16, 32, 32, 64, 68, 66, 77, 241, 0, 0],
    [0, 0, 4, 4, 72, 40, 16, 24, 20, 34, 32, 64, 64, 128, 128, 0],
    [0, 0, 0, 0, 255, 16, 16, 16, 255, 16, 16, 16, 16, 8, 7, 0],
    [0, 0, 64, 67, 77, 50, 224, 32, 16, 16, 16, 8, 8, 8, 4, 0],
    [0, 0, 0, 0, 126, 2, 2, 4, 4, 8, 8, 255, 0, 0, 0, 0],
    [0, 0, 126, 2, 2, 2, 2, 126, 2, 2, 2, 2, 126, 0, 0, 0],
    [0, 0, 126, 0, 0, 255, 1, 1, 1, 1, 2, 2, 4, 4, 8, 0],
    [0, 0, 66, 66, 66, 66, 66, 2, 2, 4, 4, 8, 8, 16, 16, 0],
    [0, 0, 8, 40, 40, 40, 72, 72, 72, 137, 138, 140, 8, 0, 0, 0],
    [0, 0, 64, 64, 64, 64, 64, 65, 66, 68, 72, 80, 96, 0, 0, 0],
    [0, 0, 0, 126, 66, 66, 66, 66, 66, 66, 66, 126, 66, 0, 0, 0],
    [0, 0, 0, 127, 65, 65, 1, 1, 1, 1, 1, 2, 2, 4, 8, 0],
    [0, 0, 0, 225, 25, 1, 1, 2, 2, 4, 4, 8, 8, 16, 96, 0],
    [0, 32, 144, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 64, 160, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];
