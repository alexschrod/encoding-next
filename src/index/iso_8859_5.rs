// AUTOGENERATED FROM index-iso-8859-5.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-5.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034,
    1035, 1036, 173, 1038, 1039, 1040, 1041, 1042, 1043, 1044, 1045, 1046,
    1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058,
    1059, 1060, 1061, 1062, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070,
    1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1079, 1080, 1081, 1082,
    1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094,
    1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102, 1103, 8470, 1105, 1106,
    1107, 1108, 1109, 1110, 1111, 1112, 1113, 1114, 1115, 1116, 167, 1118,
    1119,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        128 => 128, 129 => 129, 130 => 130, 131 => 131, 132 => 132, 133 => 133,
        134 => 134, 135 => 135, 136 => 136, 137 => 137, 138 => 138, 139 => 139,
        140 => 140, 141 => 141, 142 => 142, 143 => 143, 144 => 144, 145 => 145,
        146 => 146, 147 => 147, 148 => 148, 149 => 149, 150 => 150, 151 => 151,
        152 => 152, 153 => 153, 154 => 154, 155 => 155, 156 => 156, 157 => 157,
        158 => 158, 159 => 159, 160 => 160, 1025 => 161, 1026 => 162,
        1027 => 163, 1028 => 164, 1029 => 165, 1030 => 166, 1031 => 167,
        1032 => 168, 1033 => 169, 1034 => 170, 1035 => 171, 1036 => 172,
        173 => 173, 1038 => 174, 1039 => 175, 1040 => 176, 1041 => 177,
        1042 => 178, 1043 => 179, 1044 => 180, 1045 => 181, 1046 => 182,
        1047 => 183, 1048 => 184, 1049 => 185, 1050 => 186, 1051 => 187,
        1052 => 188, 1053 => 189, 1054 => 190, 1055 => 191, 1056 => 192,
        1057 => 193, 1058 => 194, 1059 => 195, 1060 => 196, 1061 => 197,
        1062 => 198, 1063 => 199, 1064 => 200, 1065 => 201, 1066 => 202,
        1067 => 203, 1068 => 204, 1069 => 205, 1070 => 206, 1071 => 207,
        1072 => 208, 1073 => 209, 1074 => 210, 1075 => 211, 1076 => 212,
        1077 => 213, 1078 => 214, 1079 => 215, 1080 => 216, 1081 => 217,
        1082 => 218, 1083 => 219, 1084 => 220, 1085 => 221, 1086 => 222,
        1087 => 223, 1088 => 224, 1089 => 225, 1090 => 226, 1091 => 227,
        1092 => 228, 1093 => 229, 1094 => 230, 1095 => 231, 1096 => 232,
        1097 => 233, 1098 => 234, 1099 => 235, 1100 => 236, 1101 => 237,
        1102 => 238, 1103 => 239, 8470 => 240, 1105 => 241, 1106 => 242,
        1107 => 243, 1108 => 244, 1109 => 245, 1110 => 246, 1111 => 247,
        1112 => 248, 1113 => 249, 1114 => 250, 1115 => 251, 1116 => 252,
        167 => 253, 1118 => 254, 1119 => 255, _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(128, 256) {
            let i = i as u8;
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}
