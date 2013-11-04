// AUTOGENERATED FROM index-iso-8859-14.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-14.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 7682, 7683, 163, 266, 267, 7690, 167, 7808, 169, 7810, 7691,
    7922, 173, 174, 376, 7710, 7711, 288, 289, 7744, 7745, 182, 7766, 7809,
    7767, 7811, 7776, 7923, 7812, 7813, 7777, 192, 193, 194, 195, 196, 197,
    198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 372, 209, 210, 211, 212,
    213, 214, 7786, 216, 217, 218, 219, 220, 221, 374, 223, 224, 225, 226, 227,
    228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 373, 241, 242,
    243, 244, 245, 246, 7787, 248, 249, 250, 251, 252, 253, 375, 255,
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
        158 => 158, 159 => 159, 160 => 160, 7682 => 161, 7683 => 162,
        163 => 163, 266 => 164, 267 => 165, 7690 => 166, 167 => 167,
        7808 => 168, 169 => 169, 7810 => 170, 7691 => 171, 7922 => 172,
        173 => 173, 174 => 174, 376 => 175, 7710 => 176, 7711 => 177,
        288 => 178, 289 => 179, 7744 => 180, 7745 => 181, 182 => 182,
        7766 => 183, 7809 => 184, 7767 => 185, 7811 => 186, 7776 => 187,
        7923 => 188, 7812 => 189, 7813 => 190, 7777 => 191, 192 => 192,
        193 => 193, 194 => 194, 195 => 195, 196 => 196, 197 => 197, 198 => 198,
        199 => 199, 200 => 200, 201 => 201, 202 => 202, 203 => 203, 204 => 204,
        205 => 205, 206 => 206, 207 => 207, 372 => 208, 209 => 209, 210 => 210,
        211 => 211, 212 => 212, 213 => 213, 214 => 214, 7786 => 215,
        216 => 216, 217 => 217, 218 => 218, 219 => 219, 220 => 220, 221 => 221,
        374 => 222, 223 => 223, 224 => 224, 225 => 225, 226 => 226, 227 => 227,
        228 => 228, 229 => 229, 230 => 230, 231 => 231, 232 => 232, 233 => 233,
        234 => 234, 235 => 235, 236 => 236, 237 => 237, 238 => 238, 239 => 239,
        373 => 240, 241 => 241, 242 => 242, 243 => 243, 244 => 244, 245 => 245,
        246 => 246, 7787 => 247, 248 => 248, 249 => 249, 250 => 250,
        251 => 251, 252 => 252, 253 => 253, 375 => 254, 255 => 255, _ => 0
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
