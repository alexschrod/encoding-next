// AUTOGENERATED FROM index-iso-8859-5.txt, ORIGINAL COMMENT FOLLOWS:
//
// For details on index index-iso-8859-5.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: fa9b1f3f5242df43e2e7bca80e9b6997c67944f20a4af91ee06bacc4e132d9c9
// Date: 2018-01-06

#[allow(dead_code)]
const X: u16 = 0xffff;

const FORWARD_TABLE: &[u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146,
    147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 1025, 1026, 1027, 1028,
    1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 173, 1038, 1039, 1040, 1041, 1042, 1043, 1044,
    1045, 1046, 1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060,
    1061, 1062, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076,
    1077, 1078, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092,
    1093, 1094, 1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102, 1103, 8470, 1105, 1106, 1107, 1108,
    1109, 1110, 1111, 1112, 1113, 1114, 1115, 1116, 167, 1118, 1119,
]; // 128 entries

/// Returns the index code point for pointer `code` in this index.
#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as usize]
}

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_LOWER: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146,
    147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 0, 0, 0, 0, 0, 0, 253, 0,
    0, 0, 0, 0, 173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 225, 226, 227, 228,
    229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 0, 241, 242, 243, 244, 245, 246, 247,
    248, 249, 250, 251, 252, 0, 254, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 161, 162, 163, 164,
    165, 166, 167, 168, 169, 170, 171, 172, 0, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183,
    184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
    203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221,
    222, 223,
]; // 297 entries

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_UPPER: &[u16] = &[
    0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 233, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 170,
]; // 133 entries

/// Returns the index pointer for code point `code` in this index.
#[inline]
#[cfg(not(feature = "no-optimized-legacy-encoding"))]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 6) as usize;
    let offset = if offset < 133 {
        BACKWARD_TABLE_UPPER[offset] as usize
    } else {
        0
    };
    BACKWARD_TABLE_LOWER[offset + ((code & 63) as usize)]
}

/// Returns the index pointer for code point `code` in this index.
#[cfg(feature = "no-optimized-legacy-encoding")]
pub fn backward(code: u32) -> u8 {
    if code > 8470 || ((0x10005u32 >> (code >> 9)) & 1) == 0 {
        return 0;
    }
    let code = code as u16;
    for i in 0..0x80 {
        if FORWARD_TABLE[i as usize] == code {
            return 0x80 + i;
        }
    }
    0
}

#[cfg(test)]
single_byte_tests! {}
