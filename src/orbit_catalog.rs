use crate::checksum;

#[derive(Clone, Copy, Debug)]
pub struct OrbitRecord {
    pub id: u16,
    pub norad_hint: u32,
    pub min_elevation_deg: u8,
    pub max_pass_minutes: u16,
    pub flags: u32,
}

pub fn record_0001() -> OrbitRecord {
    OrbitRecord {
        id: 1,
        norad_hint: 40037,
        min_elevation_deg: 12,
        max_pass_minutes: 14,
        flags: 0x4eb52bb5,
    }
}

pub fn record_0002() -> OrbitRecord {
    OrbitRecord {
        id: 2,
        norad_hint: 40074,
        min_elevation_deg: 19,
        max_pass_minutes: 25,
        flags: 0x4ca6576a,
    }
}

pub fn record_0003() -> OrbitRecord {
    OrbitRecord {
        id: 3,
        norad_hint: 40111,
        min_elevation_deg: 26,
        max_pass_minutes: 36,
        flags: 0x4a97831f,
    }
}

pub fn record_0004() -> OrbitRecord {
    OrbitRecord {
        id: 4,
        norad_hint: 40148,
        min_elevation_deg: 33,
        max_pass_minutes: 47,
        flags: 0x4880aed4,
    }
}

pub fn record_0005() -> OrbitRecord {
    OrbitRecord {
        id: 5,
        norad_hint: 40185,
        min_elevation_deg: 40,
        max_pass_minutes: 58,
        flags: 0x46f1da89,
    }
}

pub fn record_0006() -> OrbitRecord {
    OrbitRecord {
        id: 6,
        norad_hint: 40222,
        min_elevation_deg: 47,
        max_pass_minutes: 69,
        flags: 0x44e3063e,
    }
}

pub fn record_0007() -> OrbitRecord {
    OrbitRecord {
        id: 7,
        norad_hint: 40259,
        min_elevation_deg: 54,
        max_pass_minutes: 80,
        flags: 0x42dc31f3,
    }
}

pub fn record_0008() -> OrbitRecord {
    OrbitRecord {
        id: 8,
        norad_hint: 40296,
        min_elevation_deg: 61,
        max_pass_minutes: 91,
        flags: 0x40cd5da8,
    }
}

pub fn record_0009() -> OrbitRecord {
    OrbitRecord {
        id: 9,
        norad_hint: 40333,
        min_elevation_deg: 68,
        max_pass_minutes: 102,
        flags: 0x5e3e895d,
    }
}

pub fn record_0010() -> OrbitRecord {
    OrbitRecord {
        id: 10,
        norad_hint: 40370,
        min_elevation_deg: 5,
        max_pass_minutes: 113,
        flags: 0x5c2fb512,
    }
}

pub fn record_0011() -> OrbitRecord {
    OrbitRecord {
        id: 11,
        norad_hint: 40407,
        min_elevation_deg: 12,
        max_pass_minutes: 124,
        flags: 0x5a18e0c7,
    }
}

pub fn record_0012() -> OrbitRecord {
    OrbitRecord {
        id: 12,
        norad_hint: 40444,
        min_elevation_deg: 19,
        max_pass_minutes: 135,
        flags: 0x580a0c7c,
    }
}

pub fn record_0013() -> OrbitRecord {
    OrbitRecord {
        id: 13,
        norad_hint: 40481,
        min_elevation_deg: 26,
        max_pass_minutes: 146,
        flags: 0x567b3831,
    }
}

pub fn record_0014() -> OrbitRecord {
    OrbitRecord {
        id: 14,
        norad_hint: 40518,
        min_elevation_deg: 33,
        max_pass_minutes: 157,
        flags: 0x547463e6,
    }
}

pub fn record_0015() -> OrbitRecord {
    OrbitRecord {
        id: 15,
        norad_hint: 40555,
        min_elevation_deg: 40,
        max_pass_minutes: 168,
        flags: 0x52658f9b,
    }
}

pub fn record_0016() -> OrbitRecord {
    OrbitRecord {
        id: 16,
        norad_hint: 40592,
        min_elevation_deg: 47,
        max_pass_minutes: 179,
        flags: 0x5056bb50,
    }
}

pub fn record_0017() -> OrbitRecord {
    OrbitRecord {
        id: 17,
        norad_hint: 40629,
        min_elevation_deg: 54,
        max_pass_minutes: 190,
        flags: 0x6e47e705,
    }
}

pub fn record_0018() -> OrbitRecord {
    OrbitRecord {
        id: 18,
        norad_hint: 40666,
        min_elevation_deg: 61,
        max_pass_minutes: 201,
        flags: 0x6db112ba,
    }
}

pub fn record_0019() -> OrbitRecord {
    OrbitRecord {
        id: 19,
        norad_hint: 40703,
        min_elevation_deg: 68,
        max_pass_minutes: 212,
        flags: 0x6ba23e6f,
    }
}

pub fn record_0020() -> OrbitRecord {
    OrbitRecord {
        id: 20,
        norad_hint: 40740,
        min_elevation_deg: 5,
        max_pass_minutes: 223,
        flags: 0x69936a24,
    }
}

pub fn record_0021() -> OrbitRecord {
    OrbitRecord {
        id: 21,
        norad_hint: 40777,
        min_elevation_deg: 12,
        max_pass_minutes: 234,
        flags: 0x678c95d9,
    }
}

pub fn record_0022() -> OrbitRecord {
    OrbitRecord {
        id: 22,
        norad_hint: 40814,
        min_elevation_deg: 19,
        max_pass_minutes: 5,
        flags: 0x65fdc18e,
    }
}

pub fn record_0023() -> OrbitRecord {
    OrbitRecord {
        id: 23,
        norad_hint: 40851,
        min_elevation_deg: 26,
        max_pass_minutes: 16,
        flags: 0x63eeed43,
    }
}

pub fn record_0024() -> OrbitRecord {
    OrbitRecord {
        id: 24,
        norad_hint: 40888,
        min_elevation_deg: 33,
        max_pass_minutes: 27,
        flags: 0x61d818f8,
    }
}

pub fn record_0025() -> OrbitRecord {
    OrbitRecord {
        id: 25,
        norad_hint: 40925,
        min_elevation_deg: 40,
        max_pass_minutes: 38,
        flags: 0x7fc944ad,
    }
}

pub fn record_0026() -> OrbitRecord {
    OrbitRecord {
        id: 26,
        norad_hint: 40962,
        min_elevation_deg: 47,
        max_pass_minutes: 49,
        flags: 0x7d3a7062,
    }
}

pub fn record_0027() -> OrbitRecord {
    OrbitRecord {
        id: 27,
        norad_hint: 40999,
        min_elevation_deg: 54,
        max_pass_minutes: 60,
        flags: 0x7b2b9c17,
    }
}

pub fn record_0028() -> OrbitRecord {
    OrbitRecord {
        id: 28,
        norad_hint: 41036,
        min_elevation_deg: 61,
        max_pass_minutes: 71,
        flags: 0x7924c7cc,
    }
}

pub fn record_0029() -> OrbitRecord {
    OrbitRecord {
        id: 29,
        norad_hint: 41073,
        min_elevation_deg: 68,
        max_pass_minutes: 82,
        flags: 0x7715f381,
    }
}

pub fn record_0030() -> OrbitRecord {
    OrbitRecord {
        id: 30,
        norad_hint: 41110,
        min_elevation_deg: 5,
        max_pass_minutes: 93,
        flags: 0x75071f36,
    }
}

pub fn record_0031() -> OrbitRecord {
    OrbitRecord {
        id: 31,
        norad_hint: 41147,
        min_elevation_deg: 12,
        max_pass_minutes: 104,
        flags: 0x73704aeb,
    }
}

pub fn record_0032() -> OrbitRecord {
    OrbitRecord {
        id: 32,
        norad_hint: 41184,
        min_elevation_deg: 19,
        max_pass_minutes: 115,
        flags: 0x716176a0,
    }
}

pub fn record_0033() -> OrbitRecord {
    OrbitRecord {
        id: 33,
        norad_hint: 41221,
        min_elevation_deg: 26,
        max_pass_minutes: 126,
        flags: 0x0f52a255,
    }
}

pub fn record_0034() -> OrbitRecord {
    OrbitRecord {
        id: 34,
        norad_hint: 41258,
        min_elevation_deg: 33,
        max_pass_minutes: 137,
        flags: 0x0d43ce0a,
    }
}

pub fn record_0035() -> OrbitRecord {
    OrbitRecord {
        id: 35,
        norad_hint: 41295,
        min_elevation_deg: 40,
        max_pass_minutes: 148,
        flags: 0x0cbcf9bf,
    }
}

pub fn record_0036() -> OrbitRecord {
    OrbitRecord {
        id: 36,
        norad_hint: 41332,
        min_elevation_deg: 47,
        max_pass_minutes: 159,
        flags: 0x0aae2574,
    }
}

pub fn record_0037() -> OrbitRecord {
    OrbitRecord {
        id: 37,
        norad_hint: 41369,
        min_elevation_deg: 54,
        max_pass_minutes: 170,
        flags: 0x089f5129,
    }
}

pub fn record_0038() -> OrbitRecord {
    OrbitRecord {
        id: 38,
        norad_hint: 41406,
        min_elevation_deg: 61,
        max_pass_minutes: 181,
        flags: 0x06887cde,
    }
}

pub fn record_0039() -> OrbitRecord {
    OrbitRecord {
        id: 39,
        norad_hint: 41443,
        min_elevation_deg: 68,
        max_pass_minutes: 192,
        flags: 0x04f9a893,
    }
}

pub fn record_0040() -> OrbitRecord {
    OrbitRecord {
        id: 40,
        norad_hint: 41480,
        min_elevation_deg: 5,
        max_pass_minutes: 203,
        flags: 0x02ead448,
    }
}

pub fn record_0041() -> OrbitRecord {
    OrbitRecord {
        id: 41,
        norad_hint: 41517,
        min_elevation_deg: 12,
        max_pass_minutes: 214,
        flags: 0x00dbfffd,
    }
}

pub fn record_0042() -> OrbitRecord {
    OrbitRecord {
        id: 42,
        norad_hint: 41554,
        min_elevation_deg: 19,
        max_pass_minutes: 225,
        flags: 0x1ed52bb2,
    }
}

pub fn record_0043() -> OrbitRecord {
    OrbitRecord {
        id: 43,
        norad_hint: 41591,
        min_elevation_deg: 26,
        max_pass_minutes: 236,
        flags: 0x1cc65767,
    }
}

pub fn record_0044() -> OrbitRecord {
    OrbitRecord {
        id: 44,
        norad_hint: 41628,
        min_elevation_deg: 33,
        max_pass_minutes: 7,
        flags: 0x1a37831c,
    }
}

pub fn record_0045() -> OrbitRecord {
    OrbitRecord {
        id: 45,
        norad_hint: 41665,
        min_elevation_deg: 40,
        max_pass_minutes: 18,
        flags: 0x1820aed1,
    }
}

pub fn record_0046() -> OrbitRecord {
    OrbitRecord {
        id: 46,
        norad_hint: 41702,
        min_elevation_deg: 47,
        max_pass_minutes: 29,
        flags: 0x1611da86,
    }
}

pub fn record_0047() -> OrbitRecord {
    OrbitRecord {
        id: 47,
        norad_hint: 41739,
        min_elevation_deg: 54,
        max_pass_minutes: 40,
        flags: 0x1403063b,
    }
}

pub fn record_0048() -> OrbitRecord {
    OrbitRecord {
        id: 48,
        norad_hint: 41776,
        min_elevation_deg: 61,
        max_pass_minutes: 51,
        flags: 0x127c31f0,
    }
}

pub fn record_0049() -> OrbitRecord {
    OrbitRecord {
        id: 49,
        norad_hint: 41813,
        min_elevation_deg: 68,
        max_pass_minutes: 62,
        flags: 0x106d5da5,
    }
}

pub fn record_0050() -> OrbitRecord {
    OrbitRecord {
        id: 50,
        norad_hint: 41850,
        min_elevation_deg: 5,
        max_pass_minutes: 73,
        flags: 0x2e5e895a,
    }
}

pub fn record_0051() -> OrbitRecord {
    OrbitRecord {
        id: 51,
        norad_hint: 41887,
        min_elevation_deg: 12,
        max_pass_minutes: 84,
        flags: 0x2c4fb50f,
    }
}

pub fn record_0052() -> OrbitRecord {
    OrbitRecord {
        id: 52,
        norad_hint: 41924,
        min_elevation_deg: 19,
        max_pass_minutes: 95,
        flags: 0x2bb8e0c4,
    }
}

pub fn record_0053() -> OrbitRecord {
    OrbitRecord {
        id: 53,
        norad_hint: 41961,
        min_elevation_deg: 26,
        max_pass_minutes: 106,
        flags: 0x29aa0c79,
    }
}

pub fn record_0054() -> OrbitRecord {
    OrbitRecord {
        id: 54,
        norad_hint: 41998,
        min_elevation_deg: 33,
        max_pass_minutes: 117,
        flags: 0x279b382e,
    }
}

pub fn record_0055() -> OrbitRecord {
    OrbitRecord {
        id: 55,
        norad_hint: 42035,
        min_elevation_deg: 40,
        max_pass_minutes: 128,
        flags: 0x259463e3,
    }
}

pub fn record_0056() -> OrbitRecord {
    OrbitRecord {
        id: 56,
        norad_hint: 42072,
        min_elevation_deg: 47,
        max_pass_minutes: 139,
        flags: 0x23858f98,
    }
}

pub fn record_0057() -> OrbitRecord {
    OrbitRecord {
        id: 57,
        norad_hint: 42109,
        min_elevation_deg: 54,
        max_pass_minutes: 150,
        flags: 0x21f6bb4d,
    }
}

pub fn record_0058() -> OrbitRecord {
    OrbitRecord {
        id: 58,
        norad_hint: 42146,
        min_elevation_deg: 61,
        max_pass_minutes: 161,
        flags: 0x3fe7e702,
    }
}

pub fn record_0059() -> OrbitRecord {
    OrbitRecord {
        id: 59,
        norad_hint: 42183,
        min_elevation_deg: 68,
        max_pass_minutes: 172,
        flags: 0x3dd112b7,
    }
}

pub fn record_0060() -> OrbitRecord {
    OrbitRecord {
        id: 60,
        norad_hint: 42220,
        min_elevation_deg: 5,
        max_pass_minutes: 183,
        flags: 0x3bc23e6c,
    }
}

pub fn record_0061() -> OrbitRecord {
    OrbitRecord {
        id: 61,
        norad_hint: 42257,
        min_elevation_deg: 12,
        max_pass_minutes: 194,
        flags: 0x39336a21,
    }
}

pub fn record_0062() -> OrbitRecord {
    OrbitRecord {
        id: 62,
        norad_hint: 42294,
        min_elevation_deg: 19,
        max_pass_minutes: 205,
        flags: 0x372c95d6,
    }
}

pub fn record_0063() -> OrbitRecord {
    OrbitRecord {
        id: 63,
        norad_hint: 42331,
        min_elevation_deg: 26,
        max_pass_minutes: 216,
        flags: 0x351dc18b,
    }
}

pub fn record_0064() -> OrbitRecord {
    OrbitRecord {
        id: 64,
        norad_hint: 42368,
        min_elevation_deg: 33,
        max_pass_minutes: 227,
        flags: 0x330eed40,
    }
}

pub fn record_0065() -> OrbitRecord {
    OrbitRecord {
        id: 65,
        norad_hint: 42405,
        min_elevation_deg: 40,
        max_pass_minutes: 238,
        flags: 0x317818f5,
    }
}

pub fn record_0066() -> OrbitRecord {
    OrbitRecord {
        id: 66,
        norad_hint: 42442,
        min_elevation_deg: 47,
        max_pass_minutes: 9,
        flags: 0xcf6944aa,
    }
}

pub fn record_0067() -> OrbitRecord {
    OrbitRecord {
        id: 67,
        norad_hint: 42479,
        min_elevation_deg: 54,
        max_pass_minutes: 20,
        flags: 0xcd5a705f,
    }
}

pub fn record_0068() -> OrbitRecord {
    OrbitRecord {
        id: 68,
        norad_hint: 42516,
        min_elevation_deg: 61,
        max_pass_minutes: 31,
        flags: 0xcb4b9c14,
    }
}

pub fn record_0069() -> OrbitRecord {
    OrbitRecord {
        id: 69,
        norad_hint: 42553,
        min_elevation_deg: 68,
        max_pass_minutes: 42,
        flags: 0xc944c7c9,
    }
}

pub fn record_0070() -> OrbitRecord {
    OrbitRecord {
        id: 70,
        norad_hint: 42590,
        min_elevation_deg: 5,
        max_pass_minutes: 53,
        flags: 0xc8b5f37e,
    }
}

pub fn record_0071() -> OrbitRecord {
    OrbitRecord {
        id: 71,
        norad_hint: 42627,
        min_elevation_deg: 12,
        max_pass_minutes: 64,
        flags: 0xc6a71f33,
    }
}

pub fn record_0072() -> OrbitRecord {
    OrbitRecord {
        id: 72,
        norad_hint: 42664,
        min_elevation_deg: 19,
        max_pass_minutes: 75,
        flags: 0xc4904ae8,
    }
}

pub fn record_0073() -> OrbitRecord {
    OrbitRecord {
        id: 73,
        norad_hint: 42701,
        min_elevation_deg: 26,
        max_pass_minutes: 86,
        flags: 0xc281769d,
    }
}

pub fn record_0074() -> OrbitRecord {
    OrbitRecord {
        id: 74,
        norad_hint: 42738,
        min_elevation_deg: 33,
        max_pass_minutes: 97,
        flags: 0xc0f2a252,
    }
}

pub fn record_0075() -> OrbitRecord {
    OrbitRecord {
        id: 75,
        norad_hint: 42775,
        min_elevation_deg: 40,
        max_pass_minutes: 108,
        flags: 0xdee3ce07,
    }
}

pub fn record_0076() -> OrbitRecord {
    OrbitRecord {
        id: 76,
        norad_hint: 42812,
        min_elevation_deg: 47,
        max_pass_minutes: 119,
        flags: 0xdcdcf9bc,
    }
}

pub fn record_0077() -> OrbitRecord {
    OrbitRecord {
        id: 77,
        norad_hint: 42849,
        min_elevation_deg: 54,
        max_pass_minutes: 130,
        flags: 0xdace2571,
    }
}

pub fn record_0078() -> OrbitRecord {
    OrbitRecord {
        id: 78,
        norad_hint: 42886,
        min_elevation_deg: 61,
        max_pass_minutes: 141,
        flags: 0xd83f5126,
    }
}

pub fn record_0079() -> OrbitRecord {
    OrbitRecord {
        id: 79,
        norad_hint: 42923,
        min_elevation_deg: 68,
        max_pass_minutes: 152,
        flags: 0xd6287cdb,
    }
}

pub fn record_0080() -> OrbitRecord {
    OrbitRecord {
        id: 80,
        norad_hint: 42960,
        min_elevation_deg: 5,
        max_pass_minutes: 163,
        flags: 0xd419a890,
    }
}

pub fn record_0081() -> OrbitRecord {
    OrbitRecord {
        id: 81,
        norad_hint: 42997,
        min_elevation_deg: 12,
        max_pass_minutes: 174,
        flags: 0xd20ad445,
    }
}

pub fn record_0082() -> OrbitRecord {
    OrbitRecord {
        id: 82,
        norad_hint: 43034,
        min_elevation_deg: 19,
        max_pass_minutes: 185,
        flags: 0xd07bfffa,
    }
}

pub fn record_0083() -> OrbitRecord {
    OrbitRecord {
        id: 83,
        norad_hint: 43071,
        min_elevation_deg: 26,
        max_pass_minutes: 196,
        flags: 0xee752baf,
    }
}

pub fn record_0084() -> OrbitRecord {
    OrbitRecord {
        id: 84,
        norad_hint: 43108,
        min_elevation_deg: 33,
        max_pass_minutes: 207,
        flags: 0xec665764,
    }
}

pub fn record_0085() -> OrbitRecord {
    OrbitRecord {
        id: 85,
        norad_hint: 43145,
        min_elevation_deg: 40,
        max_pass_minutes: 218,
        flags: 0xea578319,
    }
}

pub fn record_0086() -> OrbitRecord {
    OrbitRecord {
        id: 86,
        norad_hint: 43182,
        min_elevation_deg: 47,
        max_pass_minutes: 229,
        flags: 0xe840aece,
    }
}

pub fn record_0087() -> OrbitRecord {
    OrbitRecord {
        id: 87,
        norad_hint: 43219,
        min_elevation_deg: 54,
        max_pass_minutes: 240,
        flags: 0xe7b1da83,
    }
}

pub fn record_0088() -> OrbitRecord {
    OrbitRecord {
        id: 88,
        norad_hint: 43256,
        min_elevation_deg: 61,
        max_pass_minutes: 11,
        flags: 0xe5a30638,
    }
}

pub fn record_0089() -> OrbitRecord {
    OrbitRecord {
        id: 89,
        norad_hint: 43293,
        min_elevation_deg: 68,
        max_pass_minutes: 22,
        flags: 0xe39c31ed,
    }
}

pub fn record_0090() -> OrbitRecord {
    OrbitRecord {
        id: 90,
        norad_hint: 43330,
        min_elevation_deg: 5,
        max_pass_minutes: 33,
        flags: 0xe18d5da2,
    }
}

pub fn record_0091() -> OrbitRecord {
    OrbitRecord {
        id: 91,
        norad_hint: 43367,
        min_elevation_deg: 12,
        max_pass_minutes: 44,
        flags: 0xfffe8957,
    }
}

pub fn record_0092() -> OrbitRecord {
    OrbitRecord {
        id: 92,
        norad_hint: 43404,
        min_elevation_deg: 19,
        max_pass_minutes: 55,
        flags: 0xfdefb50c,
    }
}

pub fn record_0093() -> OrbitRecord {
    OrbitRecord {
        id: 93,
        norad_hint: 43441,
        min_elevation_deg: 26,
        max_pass_minutes: 66,
        flags: 0xfbd8e0c1,
    }
}

pub fn record_0094() -> OrbitRecord {
    OrbitRecord {
        id: 94,
        norad_hint: 43478,
        min_elevation_deg: 33,
        max_pass_minutes: 77,
        flags: 0xf9ca0c76,
    }
}

pub fn record_0095() -> OrbitRecord {
    OrbitRecord {
        id: 95,
        norad_hint: 43515,
        min_elevation_deg: 40,
        max_pass_minutes: 88,
        flags: 0xf73b382b,
    }
}

pub fn record_0096() -> OrbitRecord {
    OrbitRecord {
        id: 96,
        norad_hint: 43552,
        min_elevation_deg: 47,
        max_pass_minutes: 99,
        flags: 0xf53463e0,
    }
}

pub fn record_0097() -> OrbitRecord {
    OrbitRecord {
        id: 97,
        norad_hint: 43589,
        min_elevation_deg: 54,
        max_pass_minutes: 110,
        flags: 0xf3258f95,
    }
}

pub fn record_0098() -> OrbitRecord {
    OrbitRecord {
        id: 98,
        norad_hint: 43626,
        min_elevation_deg: 61,
        max_pass_minutes: 121,
        flags: 0xf116bb4a,
    }
}

pub fn record_0099() -> OrbitRecord {
    OrbitRecord {
        id: 99,
        norad_hint: 43663,
        min_elevation_deg: 68,
        max_pass_minutes: 132,
        flags: 0x8f07e6ff,
    }
}

pub fn record_0100() -> OrbitRecord {
    OrbitRecord {
        id: 100,
        norad_hint: 43700,
        min_elevation_deg: 5,
        max_pass_minutes: 143,
        flags: 0x8d7112b4,
    }
}

pub fn record_0101() -> OrbitRecord {
    OrbitRecord {
        id: 101,
        norad_hint: 43737,
        min_elevation_deg: 12,
        max_pass_minutes: 154,
        flags: 0x8b623e69,
    }
}

pub fn record_0102() -> OrbitRecord {
    OrbitRecord {
        id: 102,
        norad_hint: 43774,
        min_elevation_deg: 19,
        max_pass_minutes: 165,
        flags: 0x89536a1e,
    }
}

pub fn record_0103() -> OrbitRecord {
    OrbitRecord {
        id: 103,
        norad_hint: 43811,
        min_elevation_deg: 26,
        max_pass_minutes: 176,
        flags: 0x874c95d3,
    }
}

pub fn record_0104() -> OrbitRecord {
    OrbitRecord {
        id: 104,
        norad_hint: 43848,
        min_elevation_deg: 33,
        max_pass_minutes: 187,
        flags: 0x86bdc188,
    }
}

pub fn record_0105() -> OrbitRecord {
    OrbitRecord {
        id: 105,
        norad_hint: 43885,
        min_elevation_deg: 40,
        max_pass_minutes: 198,
        flags: 0x84aeed3d,
    }
}

pub fn record_0106() -> OrbitRecord {
    OrbitRecord {
        id: 106,
        norad_hint: 43922,
        min_elevation_deg: 47,
        max_pass_minutes: 209,
        flags: 0x829818f2,
    }
}

pub fn record_0107() -> OrbitRecord {
    OrbitRecord {
        id: 107,
        norad_hint: 43959,
        min_elevation_deg: 54,
        max_pass_minutes: 220,
        flags: 0x808944a7,
    }
}

pub fn record_0108() -> OrbitRecord {
    OrbitRecord {
        id: 108,
        norad_hint: 43996,
        min_elevation_deg: 61,
        max_pass_minutes: 231,
        flags: 0x9efa705c,
    }
}

pub fn record_0109() -> OrbitRecord {
    OrbitRecord {
        id: 109,
        norad_hint: 44033,
        min_elevation_deg: 68,
        max_pass_minutes: 242,
        flags: 0x9ceb9c11,
    }
}

pub fn record_0110() -> OrbitRecord {
    OrbitRecord {
        id: 110,
        norad_hint: 44070,
        min_elevation_deg: 5,
        max_pass_minutes: 13,
        flags: 0x9ae4c7c6,
    }
}

pub fn record_0111() -> OrbitRecord {
    OrbitRecord {
        id: 111,
        norad_hint: 44107,
        min_elevation_deg: 12,
        max_pass_minutes: 24,
        flags: 0x98d5f37b,
    }
}

pub fn record_0112() -> OrbitRecord {
    OrbitRecord {
        id: 112,
        norad_hint: 44144,
        min_elevation_deg: 19,
        max_pass_minutes: 35,
        flags: 0x96c71f30,
    }
}

pub fn record_0113() -> OrbitRecord {
    OrbitRecord {
        id: 113,
        norad_hint: 44181,
        min_elevation_deg: 26,
        max_pass_minutes: 46,
        flags: 0x94304ae5,
    }
}

pub fn record_0114() -> OrbitRecord {
    OrbitRecord {
        id: 114,
        norad_hint: 44218,
        min_elevation_deg: 33,
        max_pass_minutes: 57,
        flags: 0x9221769a,
    }
}

pub fn record_0115() -> OrbitRecord {
    OrbitRecord {
        id: 115,
        norad_hint: 44255,
        min_elevation_deg: 40,
        max_pass_minutes: 68,
        flags: 0x9012a24f,
    }
}

pub fn record_0116() -> OrbitRecord {
    OrbitRecord {
        id: 116,
        norad_hint: 44292,
        min_elevation_deg: 47,
        max_pass_minutes: 79,
        flags: 0xae03ce04,
    }
}

pub fn record_0117() -> OrbitRecord {
    OrbitRecord {
        id: 117,
        norad_hint: 44329,
        min_elevation_deg: 54,
        max_pass_minutes: 90,
        flags: 0xac7cf9b9,
    }
}

pub fn record_0118() -> OrbitRecord {
    OrbitRecord {
        id: 118,
        norad_hint: 44366,
        min_elevation_deg: 61,
        max_pass_minutes: 101,
        flags: 0xaa6e256e,
    }
}

pub fn record_0119() -> OrbitRecord {
    OrbitRecord {
        id: 119,
        norad_hint: 44403,
        min_elevation_deg: 68,
        max_pass_minutes: 112,
        flags: 0xa85f5123,
    }
}

pub fn record_0120() -> OrbitRecord {
    OrbitRecord {
        id: 120,
        norad_hint: 44440,
        min_elevation_deg: 5,
        max_pass_minutes: 123,
        flags: 0xa6487cd8,
    }
}

pub fn record_0121() -> OrbitRecord {
    OrbitRecord {
        id: 121,
        norad_hint: 44477,
        min_elevation_deg: 12,
        max_pass_minutes: 134,
        flags: 0xa5b9a88d,
    }
}

pub fn record_0122() -> OrbitRecord {
    OrbitRecord {
        id: 122,
        norad_hint: 44514,
        min_elevation_deg: 19,
        max_pass_minutes: 145,
        flags: 0xa3aad442,
    }
}

pub fn record_0123() -> OrbitRecord {
    OrbitRecord {
        id: 123,
        norad_hint: 44551,
        min_elevation_deg: 26,
        max_pass_minutes: 156,
        flags: 0xa19bfff7,
    }
}

pub fn record_0124() -> OrbitRecord {
    OrbitRecord {
        id: 124,
        norad_hint: 44588,
        min_elevation_deg: 33,
        max_pass_minutes: 167,
        flags: 0xbf952bac,
    }
}

pub fn record_0125() -> OrbitRecord {
    OrbitRecord {
        id: 125,
        norad_hint: 44625,
        min_elevation_deg: 40,
        max_pass_minutes: 178,
        flags: 0xbd865761,
    }
}

pub fn record_0126() -> OrbitRecord {
    OrbitRecord {
        id: 126,
        norad_hint: 44662,
        min_elevation_deg: 47,
        max_pass_minutes: 189,
        flags: 0xbbf78316,
    }
}

pub fn record_0127() -> OrbitRecord {
    OrbitRecord {
        id: 127,
        norad_hint: 44699,
        min_elevation_deg: 54,
        max_pass_minutes: 200,
        flags: 0xb9e0aecb,
    }
}

pub fn record_0128() -> OrbitRecord {
    OrbitRecord {
        id: 128,
        norad_hint: 44736,
        min_elevation_deg: 61,
        max_pass_minutes: 211,
        flags: 0xb7d1da80,
    }
}

pub fn record_0129() -> OrbitRecord {
    OrbitRecord {
        id: 129,
        norad_hint: 44773,
        min_elevation_deg: 68,
        max_pass_minutes: 222,
        flags: 0xb5c30635,
    }
}

pub fn record_0130() -> OrbitRecord {
    OrbitRecord {
        id: 130,
        norad_hint: 44810,
        min_elevation_deg: 5,
        max_pass_minutes: 233,
        flags: 0xb33c31ea,
    }
}

pub fn record_0131() -> OrbitRecord {
    OrbitRecord {
        id: 131,
        norad_hint: 44847,
        min_elevation_deg: 12,
        max_pass_minutes: 4,
        flags: 0xb12d5d9f,
    }
}

pub fn record_0132() -> OrbitRecord {
    OrbitRecord {
        id: 132,
        norad_hint: 44884,
        min_elevation_deg: 19,
        max_pass_minutes: 15,
        flags: 0x4f1e8954,
    }
}

pub fn record_0133() -> OrbitRecord {
    OrbitRecord {
        id: 133,
        norad_hint: 44921,
        min_elevation_deg: 26,
        max_pass_minutes: 26,
        flags: 0x4d0fb509,
    }
}

pub fn record_0134() -> OrbitRecord {
    OrbitRecord {
        id: 134,
        norad_hint: 44958,
        min_elevation_deg: 33,
        max_pass_minutes: 37,
        flags: 0x4b78e0be,
    }
}

pub fn record_0135() -> OrbitRecord {
    OrbitRecord {
        id: 135,
        norad_hint: 44995,
        min_elevation_deg: 40,
        max_pass_minutes: 48,
        flags: 0x496a0c73,
    }
}

pub fn record_0136() -> OrbitRecord {
    OrbitRecord {
        id: 136,
        norad_hint: 45032,
        min_elevation_deg: 47,
        max_pass_minutes: 59,
        flags: 0x475b3828,
    }
}

pub fn record_0137() -> OrbitRecord {
    OrbitRecord {
        id: 137,
        norad_hint: 45069,
        min_elevation_deg: 54,
        max_pass_minutes: 70,
        flags: 0x455463dd,
    }
}

pub fn record_0138() -> OrbitRecord {
    OrbitRecord {
        id: 138,
        norad_hint: 45106,
        min_elevation_deg: 61,
        max_pass_minutes: 81,
        flags: 0x43458f92,
    }
}

pub fn record_0139() -> OrbitRecord {
    OrbitRecord {
        id: 139,
        norad_hint: 45143,
        min_elevation_deg: 68,
        max_pass_minutes: 92,
        flags: 0x42b6bb47,
    }
}

pub fn record_0140() -> OrbitRecord {
    OrbitRecord {
        id: 140,
        norad_hint: 45180,
        min_elevation_deg: 5,
        max_pass_minutes: 103,
        flags: 0x40a7e6fc,
    }
}

pub fn record_0141() -> OrbitRecord {
    OrbitRecord {
        id: 141,
        norad_hint: 45217,
        min_elevation_deg: 12,
        max_pass_minutes: 114,
        flags: 0x5e9112b1,
    }
}

pub fn record_0142() -> OrbitRecord {
    OrbitRecord {
        id: 142,
        norad_hint: 45254,
        min_elevation_deg: 19,
        max_pass_minutes: 125,
        flags: 0x5c823e66,
    }
}

pub fn record_0143() -> OrbitRecord {
    OrbitRecord {
        id: 143,
        norad_hint: 45291,
        min_elevation_deg: 26,
        max_pass_minutes: 136,
        flags: 0x5af36a1b,
    }
}

pub fn record_0144() -> OrbitRecord {
    OrbitRecord {
        id: 144,
        norad_hint: 45328,
        min_elevation_deg: 33,
        max_pass_minutes: 147,
        flags: 0x58ec95d0,
    }
}

pub fn record_0145() -> OrbitRecord {
    OrbitRecord {
        id: 145,
        norad_hint: 45365,
        min_elevation_deg: 40,
        max_pass_minutes: 158,
        flags: 0x56ddc185,
    }
}

pub fn record_0146() -> OrbitRecord {
    OrbitRecord {
        id: 146,
        norad_hint: 45402,
        min_elevation_deg: 47,
        max_pass_minutes: 169,
        flags: 0x54ceed3a,
    }
}

pub fn record_0147() -> OrbitRecord {
    OrbitRecord {
        id: 147,
        norad_hint: 45439,
        min_elevation_deg: 54,
        max_pass_minutes: 180,
        flags: 0x523818ef,
    }
}

pub fn record_0148() -> OrbitRecord {
    OrbitRecord {
        id: 148,
        norad_hint: 45476,
        min_elevation_deg: 61,
        max_pass_minutes: 191,
        flags: 0x502944a4,
    }
}

pub fn record_0149() -> OrbitRecord {
    OrbitRecord {
        id: 149,
        norad_hint: 45513,
        min_elevation_deg: 68,
        max_pass_minutes: 202,
        flags: 0x6e1a7059,
    }
}

pub fn record_0150() -> OrbitRecord {
    OrbitRecord {
        id: 150,
        norad_hint: 45550,
        min_elevation_deg: 5,
        max_pass_minutes: 213,
        flags: 0x6c0b9c0e,
    }
}

pub fn record_0151() -> OrbitRecord {
    OrbitRecord {
        id: 151,
        norad_hint: 45587,
        min_elevation_deg: 12,
        max_pass_minutes: 224,
        flags: 0x6a04c7c3,
    }
}

pub fn record_0152() -> OrbitRecord {
    OrbitRecord {
        id: 152,
        norad_hint: 45624,
        min_elevation_deg: 19,
        max_pass_minutes: 235,
        flags: 0x6875f378,
    }
}

pub fn record_0153() -> OrbitRecord {
    OrbitRecord {
        id: 153,
        norad_hint: 45661,
        min_elevation_deg: 26,
        max_pass_minutes: 6,
        flags: 0x66671f2d,
    }
}

pub fn record_0154() -> OrbitRecord {
    OrbitRecord {
        id: 154,
        norad_hint: 45698,
        min_elevation_deg: 33,
        max_pass_minutes: 17,
        flags: 0x64504ae2,
    }
}

pub fn record_0155() -> OrbitRecord {
    OrbitRecord {
        id: 155,
        norad_hint: 45735,
        min_elevation_deg: 40,
        max_pass_minutes: 28,
        flags: 0x62417697,
    }
}

pub fn record_0156() -> OrbitRecord {
    OrbitRecord {
        id: 156,
        norad_hint: 45772,
        min_elevation_deg: 47,
        max_pass_minutes: 39,
        flags: 0x61b2a24c,
    }
}

pub fn record_0157() -> OrbitRecord {
    OrbitRecord {
        id: 157,
        norad_hint: 45809,
        min_elevation_deg: 54,
        max_pass_minutes: 50,
        flags: 0x7fa3ce01,
    }
}

pub fn record_0158() -> OrbitRecord {
    OrbitRecord {
        id: 158,
        norad_hint: 45846,
        min_elevation_deg: 61,
        max_pass_minutes: 61,
        flags: 0x7d9cf9b6,
    }
}

pub fn record_0159() -> OrbitRecord {
    OrbitRecord {
        id: 159,
        norad_hint: 45883,
        min_elevation_deg: 68,
        max_pass_minutes: 72,
        flags: 0x7b8e256b,
    }
}

pub fn record_0160() -> OrbitRecord {
    OrbitRecord {
        id: 160,
        norad_hint: 45920,
        min_elevation_deg: 5,
        max_pass_minutes: 83,
        flags: 0x79ff5120,
    }
}

pub fn record_0161() -> OrbitRecord {
    OrbitRecord {
        id: 161,
        norad_hint: 45957,
        min_elevation_deg: 12,
        max_pass_minutes: 94,
        flags: 0x77e87cd5,
    }
}

pub fn record_0162() -> OrbitRecord {
    OrbitRecord {
        id: 162,
        norad_hint: 45994,
        min_elevation_deg: 19,
        max_pass_minutes: 105,
        flags: 0x75d9a88a,
    }
}

pub fn record_0163() -> OrbitRecord {
    OrbitRecord {
        id: 163,
        norad_hint: 46031,
        min_elevation_deg: 26,
        max_pass_minutes: 116,
        flags: 0x73cad43f,
    }
}

pub fn record_0164() -> OrbitRecord {
    OrbitRecord {
        id: 164,
        norad_hint: 46068,
        min_elevation_deg: 33,
        max_pass_minutes: 127,
        flags: 0x713bfff4,
    }
}

pub fn record_0165() -> OrbitRecord {
    OrbitRecord {
        id: 165,
        norad_hint: 46105,
        min_elevation_deg: 40,
        max_pass_minutes: 138,
        flags: 0x0f352ba9,
    }
}

pub fn record_0166() -> OrbitRecord {
    OrbitRecord {
        id: 166,
        norad_hint: 46142,
        min_elevation_deg: 47,
        max_pass_minutes: 149,
        flags: 0x0d26575e,
    }
}

pub fn record_0167() -> OrbitRecord {
    OrbitRecord {
        id: 167,
        norad_hint: 46179,
        min_elevation_deg: 54,
        max_pass_minutes: 160,
        flags: 0x0b178313,
    }
}

pub fn record_0168() -> OrbitRecord {
    OrbitRecord {
        id: 168,
        norad_hint: 46216,
        min_elevation_deg: 61,
        max_pass_minutes: 171,
        flags: 0x0900aec8,
    }
}

pub fn record_0169() -> OrbitRecord {
    OrbitRecord {
        id: 169,
        norad_hint: 46253,
        min_elevation_deg: 68,
        max_pass_minutes: 182,
        flags: 0x0771da7d,
    }
}

pub fn record_0170() -> OrbitRecord {
    OrbitRecord {
        id: 170,
        norad_hint: 46290,
        min_elevation_deg: 5,
        max_pass_minutes: 193,
        flags: 0x05630632,
    }
}

pub fn record_0171() -> OrbitRecord {
    OrbitRecord {
        id: 171,
        norad_hint: 46327,
        min_elevation_deg: 12,
        max_pass_minutes: 204,
        flags: 0x035c31e7,
    }
}

pub fn record_0172() -> OrbitRecord {
    OrbitRecord {
        id: 172,
        norad_hint: 46364,
        min_elevation_deg: 19,
        max_pass_minutes: 215,
        flags: 0x014d5d9c,
    }
}

pub fn record_0173() -> OrbitRecord {
    OrbitRecord {
        id: 173,
        norad_hint: 46401,
        min_elevation_deg: 26,
        max_pass_minutes: 226,
        flags: 0x00be8951,
    }
}

pub fn record_0174() -> OrbitRecord {
    OrbitRecord {
        id: 174,
        norad_hint: 46438,
        min_elevation_deg: 33,
        max_pass_minutes: 237,
        flags: 0x1eafb506,
    }
}

pub fn record_0175() -> OrbitRecord {
    OrbitRecord {
        id: 175,
        norad_hint: 46475,
        min_elevation_deg: 40,
        max_pass_minutes: 8,
        flags: 0x1c98e0bb,
    }
}

pub fn record_0176() -> OrbitRecord {
    OrbitRecord {
        id: 176,
        norad_hint: 46512,
        min_elevation_deg: 47,
        max_pass_minutes: 19,
        flags: 0x1a8a0c70,
    }
}

pub fn record_0177() -> OrbitRecord {
    OrbitRecord {
        id: 177,
        norad_hint: 46549,
        min_elevation_deg: 54,
        max_pass_minutes: 30,
        flags: 0x18fb3825,
    }
}

pub fn record_0178() -> OrbitRecord {
    OrbitRecord {
        id: 178,
        norad_hint: 46586,
        min_elevation_deg: 61,
        max_pass_minutes: 41,
        flags: 0x16f463da,
    }
}

pub fn record_0179() -> OrbitRecord {
    OrbitRecord {
        id: 179,
        norad_hint: 46623,
        min_elevation_deg: 68,
        max_pass_minutes: 52,
        flags: 0x14e58f8f,
    }
}

pub fn record_0180() -> OrbitRecord {
    OrbitRecord {
        id: 180,
        norad_hint: 46660,
        min_elevation_deg: 5,
        max_pass_minutes: 63,
        flags: 0x12d6bb44,
    }
}

pub fn record_0181() -> OrbitRecord {
    OrbitRecord {
        id: 181,
        norad_hint: 46697,
        min_elevation_deg: 12,
        max_pass_minutes: 74,
        flags: 0x10c7e6f9,
    }
}

pub fn record_0182() -> OrbitRecord {
    OrbitRecord {
        id: 182,
        norad_hint: 46734,
        min_elevation_deg: 19,
        max_pass_minutes: 85,
        flags: 0x2e3112ae,
    }
}

pub fn record_0183() -> OrbitRecord {
    OrbitRecord {
        id: 183,
        norad_hint: 46771,
        min_elevation_deg: 26,
        max_pass_minutes: 96,
        flags: 0x2c223e63,
    }
}

pub fn record_0184() -> OrbitRecord {
    OrbitRecord {
        id: 184,
        norad_hint: 46808,
        min_elevation_deg: 33,
        max_pass_minutes: 107,
        flags: 0x2a136a18,
    }
}

pub fn record_0185() -> OrbitRecord {
    OrbitRecord {
        id: 185,
        norad_hint: 46845,
        min_elevation_deg: 40,
        max_pass_minutes: 118,
        flags: 0x280c95cd,
    }
}

pub fn record_0186() -> OrbitRecord {
    OrbitRecord {
        id: 186,
        norad_hint: 46882,
        min_elevation_deg: 47,
        max_pass_minutes: 129,
        flags: 0x267dc182,
    }
}

pub fn record_0187() -> OrbitRecord {
    OrbitRecord {
        id: 187,
        norad_hint: 46919,
        min_elevation_deg: 54,
        max_pass_minutes: 140,
        flags: 0x246eed37,
    }
}

pub fn record_0188() -> OrbitRecord {
    OrbitRecord {
        id: 188,
        norad_hint: 46956,
        min_elevation_deg: 61,
        max_pass_minutes: 151,
        flags: 0x225818ec,
    }
}

pub fn record_0189() -> OrbitRecord {
    OrbitRecord {
        id: 189,
        norad_hint: 46993,
        min_elevation_deg: 68,
        max_pass_minutes: 162,
        flags: 0x204944a1,
    }
}

pub fn record_0190() -> OrbitRecord {
    OrbitRecord {
        id: 190,
        norad_hint: 47030,
        min_elevation_deg: 5,
        max_pass_minutes: 173,
        flags: 0x3fba7056,
    }
}

pub fn record_0191() -> OrbitRecord {
    OrbitRecord {
        id: 191,
        norad_hint: 47067,
        min_elevation_deg: 12,
        max_pass_minutes: 184,
        flags: 0x3dab9c0b,
    }
}

pub fn record_0192() -> OrbitRecord {
    OrbitRecord {
        id: 192,
        norad_hint: 47104,
        min_elevation_deg: 19,
        max_pass_minutes: 195,
        flags: 0x3ba4c7c0,
    }
}

pub fn record_0193() -> OrbitRecord {
    OrbitRecord {
        id: 193,
        norad_hint: 47141,
        min_elevation_deg: 26,
        max_pass_minutes: 206,
        flags: 0x3995f375,
    }
}

pub fn record_0194() -> OrbitRecord {
    OrbitRecord {
        id: 194,
        norad_hint: 47178,
        min_elevation_deg: 33,
        max_pass_minutes: 217,
        flags: 0x37871f2a,
    }
}

pub fn record_0195() -> OrbitRecord {
    OrbitRecord {
        id: 195,
        norad_hint: 47215,
        min_elevation_deg: 40,
        max_pass_minutes: 228,
        flags: 0x35f04adf,
    }
}

pub fn record_0196() -> OrbitRecord {
    OrbitRecord {
        id: 196,
        norad_hint: 47252,
        min_elevation_deg: 47,
        max_pass_minutes: 239,
        flags: 0x33e17694,
    }
}

pub fn record_0197() -> OrbitRecord {
    OrbitRecord {
        id: 197,
        norad_hint: 47289,
        min_elevation_deg: 54,
        max_pass_minutes: 10,
        flags: 0x31d2a249,
    }
}

pub fn record_0198() -> OrbitRecord {
    OrbitRecord {
        id: 198,
        norad_hint: 47326,
        min_elevation_deg: 61,
        max_pass_minutes: 21,
        flags: 0xcfc3cdfe,
    }
}

pub fn record_0199() -> OrbitRecord {
    OrbitRecord {
        id: 199,
        norad_hint: 47363,
        min_elevation_deg: 68,
        max_pass_minutes: 32,
        flags: 0xcd3cf9b3,
    }
}

pub fn record_0200() -> OrbitRecord {
    OrbitRecord {
        id: 200,
        norad_hint: 47400,
        min_elevation_deg: 5,
        max_pass_minutes: 43,
        flags: 0xcb2e2568,
    }
}

pub fn record_0201() -> OrbitRecord {
    OrbitRecord {
        id: 201,
        norad_hint: 47437,
        min_elevation_deg: 12,
        max_pass_minutes: 54,
        flags: 0xc91f511d,
    }
}

pub fn record_0202() -> OrbitRecord {
    OrbitRecord {
        id: 202,
        norad_hint: 47474,
        min_elevation_deg: 19,
        max_pass_minutes: 65,
        flags: 0xc7087cd2,
    }
}

pub fn record_0203() -> OrbitRecord {
    OrbitRecord {
        id: 203,
        norad_hint: 47511,
        min_elevation_deg: 26,
        max_pass_minutes: 76,
        flags: 0xc579a887,
    }
}

pub fn record_0204() -> OrbitRecord {
    OrbitRecord {
        id: 204,
        norad_hint: 47548,
        min_elevation_deg: 33,
        max_pass_minutes: 87,
        flags: 0xc36ad43c,
    }
}

pub fn record_0205() -> OrbitRecord {
    OrbitRecord {
        id: 205,
        norad_hint: 47585,
        min_elevation_deg: 40,
        max_pass_minutes: 98,
        flags: 0xc15bfff1,
    }
}

pub fn record_0206() -> OrbitRecord {
    OrbitRecord {
        id: 206,
        norad_hint: 47622,
        min_elevation_deg: 47,
        max_pass_minutes: 109,
        flags: 0xdf552ba6,
    }
}

pub fn record_0207() -> OrbitRecord {
    OrbitRecord {
        id: 207,
        norad_hint: 47659,
        min_elevation_deg: 54,
        max_pass_minutes: 120,
        flags: 0xdd46575b,
    }
}

pub fn record_0208() -> OrbitRecord {
    OrbitRecord {
        id: 208,
        norad_hint: 47696,
        min_elevation_deg: 61,
        max_pass_minutes: 131,
        flags: 0xdcb78310,
    }
}

pub fn record_0209() -> OrbitRecord {
    OrbitRecord {
        id: 209,
        norad_hint: 47733,
        min_elevation_deg: 68,
        max_pass_minutes: 142,
        flags: 0xdaa0aec5,
    }
}

pub fn record_0210() -> OrbitRecord {
    OrbitRecord {
        id: 210,
        norad_hint: 47770,
        min_elevation_deg: 5,
        max_pass_minutes: 153,
        flags: 0xd891da7a,
    }
}

pub fn record_0211() -> OrbitRecord {
    OrbitRecord {
        id: 211,
        norad_hint: 47807,
        min_elevation_deg: 12,
        max_pass_minutes: 164,
        flags: 0xd683062f,
    }
}

pub fn record_0212() -> OrbitRecord {
    OrbitRecord {
        id: 212,
        norad_hint: 47844,
        min_elevation_deg: 19,
        max_pass_minutes: 175,
        flags: 0xd4fc31e4,
    }
}

pub fn record_0213() -> OrbitRecord {
    OrbitRecord {
        id: 213,
        norad_hint: 47881,
        min_elevation_deg: 26,
        max_pass_minutes: 186,
        flags: 0xd2ed5d99,
    }
}

pub fn record_0214() -> OrbitRecord {
    OrbitRecord {
        id: 214,
        norad_hint: 47918,
        min_elevation_deg: 33,
        max_pass_minutes: 197,
        flags: 0xd0de894e,
    }
}

pub fn record_0215() -> OrbitRecord {
    OrbitRecord {
        id: 215,
        norad_hint: 47955,
        min_elevation_deg: 40,
        max_pass_minutes: 208,
        flags: 0xeecfb503,
    }
}

pub fn record_0216() -> OrbitRecord {
    OrbitRecord {
        id: 216,
        norad_hint: 47992,
        min_elevation_deg: 47,
        max_pass_minutes: 219,
        flags: 0xec38e0b8,
    }
}

pub fn record_0217() -> OrbitRecord {
    OrbitRecord {
        id: 217,
        norad_hint: 48029,
        min_elevation_deg: 54,
        max_pass_minutes: 230,
        flags: 0xea2a0c6d,
    }
}

pub fn record_0218() -> OrbitRecord {
    OrbitRecord {
        id: 218,
        norad_hint: 48066,
        min_elevation_deg: 61,
        max_pass_minutes: 241,
        flags: 0xe81b3822,
    }
}

pub fn record_0219() -> OrbitRecord {
    OrbitRecord {
        id: 219,
        norad_hint: 48103,
        min_elevation_deg: 68,
        max_pass_minutes: 12,
        flags: 0xe61463d7,
    }
}

pub fn record_0220() -> OrbitRecord {
    OrbitRecord {
        id: 220,
        norad_hint: 48140,
        min_elevation_deg: 5,
        max_pass_minutes: 23,
        flags: 0xe4058f8c,
    }
}

pub fn record_0221() -> OrbitRecord {
    OrbitRecord {
        id: 221,
        norad_hint: 48177,
        min_elevation_deg: 12,
        max_pass_minutes: 34,
        flags: 0xe276bb41,
    }
}

pub fn record_0222() -> OrbitRecord {
    OrbitRecord {
        id: 222,
        norad_hint: 48214,
        min_elevation_deg: 19,
        max_pass_minutes: 45,
        flags: 0xe067e6f6,
    }
}

pub fn record_0223() -> OrbitRecord {
    OrbitRecord {
        id: 223,
        norad_hint: 48251,
        min_elevation_deg: 26,
        max_pass_minutes: 56,
        flags: 0xfe5112ab,
    }
}

pub fn record_0224() -> OrbitRecord {
    OrbitRecord {
        id: 224,
        norad_hint: 48288,
        min_elevation_deg: 33,
        max_pass_minutes: 67,
        flags: 0xfc423e60,
    }
}

pub fn record_0225() -> OrbitRecord {
    OrbitRecord {
        id: 225,
        norad_hint: 48325,
        min_elevation_deg: 40,
        max_pass_minutes: 78,
        flags: 0xfbb36a15,
    }
}

pub fn record_0226() -> OrbitRecord {
    OrbitRecord {
        id: 226,
        norad_hint: 48362,
        min_elevation_deg: 47,
        max_pass_minutes: 89,
        flags: 0xf9ac95ca,
    }
}

pub fn record_0227() -> OrbitRecord {
    OrbitRecord {
        id: 227,
        norad_hint: 48399,
        min_elevation_deg: 54,
        max_pass_minutes: 100,
        flags: 0xf79dc17f,
    }
}

pub fn record_0228() -> OrbitRecord {
    OrbitRecord {
        id: 228,
        norad_hint: 48436,
        min_elevation_deg: 61,
        max_pass_minutes: 111,
        flags: 0xf58eed34,
    }
}

pub fn record_0229() -> OrbitRecord {
    OrbitRecord {
        id: 229,
        norad_hint: 48473,
        min_elevation_deg: 68,
        max_pass_minutes: 122,
        flags: 0xf3f818e9,
    }
}

pub fn record_0230() -> OrbitRecord {
    OrbitRecord {
        id: 230,
        norad_hint: 48510,
        min_elevation_deg: 5,
        max_pass_minutes: 133,
        flags: 0xf1e9449e,
    }
}

pub fn record_0231() -> OrbitRecord {
    OrbitRecord {
        id: 231,
        norad_hint: 48547,
        min_elevation_deg: 12,
        max_pass_minutes: 144,
        flags: 0x8fda7053,
    }
}

pub fn record_0232() -> OrbitRecord {
    OrbitRecord {
        id: 232,
        norad_hint: 48584,
        min_elevation_deg: 19,
        max_pass_minutes: 155,
        flags: 0x8dcb9c08,
    }
}

pub fn record_0233() -> OrbitRecord {
    OrbitRecord {
        id: 233,
        norad_hint: 48621,
        min_elevation_deg: 26,
        max_pass_minutes: 166,
        flags: 0x8bc4c7bd,
    }
}

pub fn record_0234() -> OrbitRecord {
    OrbitRecord {
        id: 234,
        norad_hint: 48658,
        min_elevation_deg: 33,
        max_pass_minutes: 177,
        flags: 0x8935f372,
    }
}

pub fn record_0235() -> OrbitRecord {
    OrbitRecord {
        id: 235,
        norad_hint: 48695,
        min_elevation_deg: 40,
        max_pass_minutes: 188,
        flags: 0x87271f27,
    }
}

pub fn record_0236() -> OrbitRecord {
    OrbitRecord {
        id: 236,
        norad_hint: 48732,
        min_elevation_deg: 47,
        max_pass_minutes: 199,
        flags: 0x85104adc,
    }
}

pub fn record_0237() -> OrbitRecord {
    OrbitRecord {
        id: 237,
        norad_hint: 48769,
        min_elevation_deg: 54,
        max_pass_minutes: 210,
        flags: 0x83017691,
    }
}

pub fn record_0238() -> OrbitRecord {
    OrbitRecord {
        id: 238,
        norad_hint: 48806,
        min_elevation_deg: 61,
        max_pass_minutes: 221,
        flags: 0x8172a246,
    }
}

pub fn record_0239() -> OrbitRecord {
    OrbitRecord {
        id: 239,
        norad_hint: 48843,
        min_elevation_deg: 68,
        max_pass_minutes: 232,
        flags: 0x9f63cdfb,
    }
}

pub fn record_0240() -> OrbitRecord {
    OrbitRecord {
        id: 240,
        norad_hint: 48880,
        min_elevation_deg: 5,
        max_pass_minutes: 3,
        flags: 0x9d5cf9b0,
    }
}

pub fn record_0241() -> OrbitRecord {
    OrbitRecord {
        id: 241,
        norad_hint: 48917,
        min_elevation_deg: 12,
        max_pass_minutes: 14,
        flags: 0x9b4e2565,
    }
}

pub fn record_0242() -> OrbitRecord {
    OrbitRecord {
        id: 242,
        norad_hint: 48954,
        min_elevation_deg: 19,
        max_pass_minutes: 25,
        flags: 0x9abf511a,
    }
}

pub fn record_0243() -> OrbitRecord {
    OrbitRecord {
        id: 243,
        norad_hint: 48991,
        min_elevation_deg: 26,
        max_pass_minutes: 36,
        flags: 0x98a87ccf,
    }
}

pub fn record_0244() -> OrbitRecord {
    OrbitRecord {
        id: 244,
        norad_hint: 49028,
        min_elevation_deg: 33,
        max_pass_minutes: 47,
        flags: 0x9699a884,
    }
}

pub fn record_0245() -> OrbitRecord {
    OrbitRecord {
        id: 245,
        norad_hint: 49065,
        min_elevation_deg: 40,
        max_pass_minutes: 58,
        flags: 0x948ad439,
    }
}

pub fn record_0246() -> OrbitRecord {
    OrbitRecord {
        id: 246,
        norad_hint: 49102,
        min_elevation_deg: 47,
        max_pass_minutes: 69,
        flags: 0x92fbffee,
    }
}

pub fn record_0247() -> OrbitRecord {
    OrbitRecord {
        id: 247,
        norad_hint: 49139,
        min_elevation_deg: 54,
        max_pass_minutes: 80,
        flags: 0x90f52ba3,
    }
}

pub fn record_0248() -> OrbitRecord {
    OrbitRecord {
        id: 248,
        norad_hint: 49176,
        min_elevation_deg: 61,
        max_pass_minutes: 91,
        flags: 0xaee65758,
    }
}

pub fn record_0249() -> OrbitRecord {
    OrbitRecord {
        id: 249,
        norad_hint: 49213,
        min_elevation_deg: 68,
        max_pass_minutes: 102,
        flags: 0xacd7830d,
    }
}

pub fn record_0250() -> OrbitRecord {
    OrbitRecord {
        id: 250,
        norad_hint: 49250,
        min_elevation_deg: 5,
        max_pass_minutes: 113,
        flags: 0xaac0aec2,
    }
}

pub fn record_0251() -> OrbitRecord {
    OrbitRecord {
        id: 251,
        norad_hint: 49287,
        min_elevation_deg: 12,
        max_pass_minutes: 124,
        flags: 0xa831da77,
    }
}

pub fn record_0252() -> OrbitRecord {
    OrbitRecord {
        id: 252,
        norad_hint: 49324,
        min_elevation_deg: 19,
        max_pass_minutes: 135,
        flags: 0xa623062c,
    }
}

pub fn record_0253() -> OrbitRecord {
    OrbitRecord {
        id: 253,
        norad_hint: 49361,
        min_elevation_deg: 26,
        max_pass_minutes: 146,
        flags: 0xa41c31e1,
    }
}

pub fn record_0254() -> OrbitRecord {
    OrbitRecord {
        id: 254,
        norad_hint: 49398,
        min_elevation_deg: 33,
        max_pass_minutes: 157,
        flags: 0xa20d5d96,
    }
}

pub fn record_0255() -> OrbitRecord {
    OrbitRecord {
        id: 255,
        norad_hint: 49435,
        min_elevation_deg: 40,
        max_pass_minutes: 168,
        flags: 0xa07e894b,
    }
}

pub fn record_0256() -> OrbitRecord {
    OrbitRecord {
        id: 256,
        norad_hint: 49472,
        min_elevation_deg: 47,
        max_pass_minutes: 179,
        flags: 0xbe6fb500,
    }
}

pub fn record_0257() -> OrbitRecord {
    OrbitRecord {
        id: 257,
        norad_hint: 49509,
        min_elevation_deg: 54,
        max_pass_minutes: 190,
        flags: 0xbc58e0b5,
    }
}

pub fn record_0258() -> OrbitRecord {
    OrbitRecord {
        id: 258,
        norad_hint: 49546,
        min_elevation_deg: 61,
        max_pass_minutes: 201,
        flags: 0xba4a0c6a,
    }
}

pub fn record_0259() -> OrbitRecord {
    OrbitRecord {
        id: 259,
        norad_hint: 49583,
        min_elevation_deg: 68,
        max_pass_minutes: 212,
        flags: 0xb9bb381f,
    }
}

pub fn record_0260() -> OrbitRecord {
    OrbitRecord {
        id: 260,
        norad_hint: 49620,
        min_elevation_deg: 5,
        max_pass_minutes: 223,
        flags: 0xb7b463d4,
    }
}

pub fn record_0261() -> OrbitRecord {
    OrbitRecord {
        id: 261,
        norad_hint: 49657,
        min_elevation_deg: 12,
        max_pass_minutes: 234,
        flags: 0xb5a58f89,
    }
}

pub fn record_0262() -> OrbitRecord {
    OrbitRecord {
        id: 262,
        norad_hint: 49694,
        min_elevation_deg: 19,
        max_pass_minutes: 5,
        flags: 0xb396bb3e,
    }
}

pub fn record_0263() -> OrbitRecord {
    OrbitRecord {
        id: 263,
        norad_hint: 49731,
        min_elevation_deg: 26,
        max_pass_minutes: 16,
        flags: 0xb187e6f3,
    }
}

pub fn record_0264() -> OrbitRecord {
    OrbitRecord {
        id: 264,
        norad_hint: 49768,
        min_elevation_deg: 33,
        max_pass_minutes: 27,
        flags: 0x4ff112a8,
    }
}

pub fn record_0265() -> OrbitRecord {
    OrbitRecord {
        id: 265,
        norad_hint: 49805,
        min_elevation_deg: 40,
        max_pass_minutes: 38,
        flags: 0x4de23e5d,
    }
}

pub fn record_0266() -> OrbitRecord {
    OrbitRecord {
        id: 266,
        norad_hint: 49842,
        min_elevation_deg: 47,
        max_pass_minutes: 49,
        flags: 0x4bd36a12,
    }
}

pub fn record_0267() -> OrbitRecord {
    OrbitRecord {
        id: 267,
        norad_hint: 49879,
        min_elevation_deg: 54,
        max_pass_minutes: 60,
        flags: 0x49cc95c7,
    }
}

pub fn record_0268() -> OrbitRecord {
    OrbitRecord {
        id: 268,
        norad_hint: 49916,
        min_elevation_deg: 61,
        max_pass_minutes: 71,
        flags: 0x473dc17c,
    }
}

pub fn record_0269() -> OrbitRecord {
    OrbitRecord {
        id: 269,
        norad_hint: 49953,
        min_elevation_deg: 68,
        max_pass_minutes: 82,
        flags: 0x452eed31,
    }
}

pub fn record_0270() -> OrbitRecord {
    OrbitRecord {
        id: 270,
        norad_hint: 49990,
        min_elevation_deg: 5,
        max_pass_minutes: 93,
        flags: 0x431818e6,
    }
}

pub fn record_0271() -> OrbitRecord {
    OrbitRecord {
        id: 271,
        norad_hint: 50027,
        min_elevation_deg: 12,
        max_pass_minutes: 104,
        flags: 0x4109449b,
    }
}

pub fn record_0272() -> OrbitRecord {
    OrbitRecord {
        id: 272,
        norad_hint: 50064,
        min_elevation_deg: 19,
        max_pass_minutes: 115,
        flags: 0x5f7a7050,
    }
}

pub fn record_0273() -> OrbitRecord {
    OrbitRecord {
        id: 273,
        norad_hint: 50101,
        min_elevation_deg: 26,
        max_pass_minutes: 126,
        flags: 0x5d6b9c05,
    }
}

pub fn record_0274() -> OrbitRecord {
    OrbitRecord {
        id: 274,
        norad_hint: 50138,
        min_elevation_deg: 33,
        max_pass_minutes: 137,
        flags: 0x5b64c7ba,
    }
}

pub fn record_0275() -> OrbitRecord {
    OrbitRecord {
        id: 275,
        norad_hint: 50175,
        min_elevation_deg: 40,
        max_pass_minutes: 148,
        flags: 0x5955f36f,
    }
}

pub fn record_0276() -> OrbitRecord {
    OrbitRecord {
        id: 276,
        norad_hint: 50212,
        min_elevation_deg: 47,
        max_pass_minutes: 159,
        flags: 0x57471f24,
    }
}

pub fn record_0277() -> OrbitRecord {
    OrbitRecord {
        id: 277,
        norad_hint: 50249,
        min_elevation_deg: 54,
        max_pass_minutes: 170,
        flags: 0x56b04ad9,
    }
}

pub fn record_0278() -> OrbitRecord {
    OrbitRecord {
        id: 278,
        norad_hint: 50286,
        min_elevation_deg: 61,
        max_pass_minutes: 181,
        flags: 0x54a1768e,
    }
}

pub fn record_0279() -> OrbitRecord {
    OrbitRecord {
        id: 279,
        norad_hint: 50323,
        min_elevation_deg: 68,
        max_pass_minutes: 192,
        flags: 0x5292a243,
    }
}

pub fn record_0280() -> OrbitRecord {
    OrbitRecord {
        id: 280,
        norad_hint: 50360,
        min_elevation_deg: 5,
        max_pass_minutes: 203,
        flags: 0x5083cdf8,
    }
}

pub fn record_0281() -> OrbitRecord {
    OrbitRecord {
        id: 281,
        norad_hint: 50397,
        min_elevation_deg: 12,
        max_pass_minutes: 214,
        flags: 0x6efcf9ad,
    }
}

pub fn record_0282() -> OrbitRecord {
    OrbitRecord {
        id: 282,
        norad_hint: 50434,
        min_elevation_deg: 19,
        max_pass_minutes: 225,
        flags: 0x6cee2562,
    }
}

pub fn record_0283() -> OrbitRecord {
    OrbitRecord {
        id: 283,
        norad_hint: 50471,
        min_elevation_deg: 26,
        max_pass_minutes: 236,
        flags: 0x6adf5117,
    }
}

pub fn record_0284() -> OrbitRecord {
    OrbitRecord {
        id: 284,
        norad_hint: 50508,
        min_elevation_deg: 33,
        max_pass_minutes: 7,
        flags: 0x68c87ccc,
    }
}

pub fn record_0285() -> OrbitRecord {
    OrbitRecord {
        id: 285,
        norad_hint: 50545,
        min_elevation_deg: 40,
        max_pass_minutes: 18,
        flags: 0x6639a881,
    }
}

pub fn record_0286() -> OrbitRecord {
    OrbitRecord {
        id: 286,
        norad_hint: 50582,
        min_elevation_deg: 47,
        max_pass_minutes: 29,
        flags: 0x642ad436,
    }
}

pub fn record_0287() -> OrbitRecord {
    OrbitRecord {
        id: 287,
        norad_hint: 50619,
        min_elevation_deg: 54,
        max_pass_minutes: 40,
        flags: 0x621bffeb,
    }
}

pub fn record_0288() -> OrbitRecord {
    OrbitRecord {
        id: 288,
        norad_hint: 50656,
        min_elevation_deg: 61,
        max_pass_minutes: 51,
        flags: 0x60152ba0,
    }
}

pub fn record_0289() -> OrbitRecord {
    OrbitRecord {
        id: 289,
        norad_hint: 50693,
        min_elevation_deg: 68,
        max_pass_minutes: 62,
        flags: 0x7e065755,
    }
}

pub fn record_0290() -> OrbitRecord {
    OrbitRecord {
        id: 290,
        norad_hint: 50730,
        min_elevation_deg: 5,
        max_pass_minutes: 73,
        flags: 0x7c77830a,
    }
}

pub fn record_0291() -> OrbitRecord {
    OrbitRecord {
        id: 291,
        norad_hint: 50767,
        min_elevation_deg: 12,
        max_pass_minutes: 84,
        flags: 0x7a60aebf,
    }
}

pub fn record_0292() -> OrbitRecord {
    OrbitRecord {
        id: 292,
        norad_hint: 50804,
        min_elevation_deg: 19,
        max_pass_minutes: 95,
        flags: 0x7851da74,
    }
}

pub fn record_0293() -> OrbitRecord {
    OrbitRecord {
        id: 293,
        norad_hint: 50841,
        min_elevation_deg: 26,
        max_pass_minutes: 106,
        flags: 0x76430629,
    }
}

pub fn record_0294() -> OrbitRecord {
    OrbitRecord {
        id: 294,
        norad_hint: 50878,
        min_elevation_deg: 33,
        max_pass_minutes: 117,
        flags: 0x75bc31de,
    }
}

pub fn record_0295() -> OrbitRecord {
    OrbitRecord {
        id: 295,
        norad_hint: 50915,
        min_elevation_deg: 40,
        max_pass_minutes: 128,
        flags: 0x73ad5d93,
    }
}

pub fn record_0296() -> OrbitRecord {
    OrbitRecord {
        id: 296,
        norad_hint: 50952,
        min_elevation_deg: 47,
        max_pass_minutes: 139,
        flags: 0x719e8948,
    }
}

pub fn record_0297() -> OrbitRecord {
    OrbitRecord {
        id: 297,
        norad_hint: 50989,
        min_elevation_deg: 54,
        max_pass_minutes: 150,
        flags: 0x0f8fb4fd,
    }
}

pub fn record_0298() -> OrbitRecord {
    OrbitRecord {
        id: 298,
        norad_hint: 51026,
        min_elevation_deg: 61,
        max_pass_minutes: 161,
        flags: 0x0df8e0b2,
    }
}

pub fn record_0299() -> OrbitRecord {
    OrbitRecord {
        id: 299,
        norad_hint: 51063,
        min_elevation_deg: 68,
        max_pass_minutes: 172,
        flags: 0x0bea0c67,
    }
}

pub fn record_0300() -> OrbitRecord {
    OrbitRecord {
        id: 300,
        norad_hint: 51100,
        min_elevation_deg: 5,
        max_pass_minutes: 183,
        flags: 0x09db381c,
    }
}

pub fn record_0301() -> OrbitRecord {
    OrbitRecord {
        id: 301,
        norad_hint: 51137,
        min_elevation_deg: 12,
        max_pass_minutes: 194,
        flags: 0x07d463d1,
    }
}

pub fn record_0302() -> OrbitRecord {
    OrbitRecord {
        id: 302,
        norad_hint: 51174,
        min_elevation_deg: 19,
        max_pass_minutes: 205,
        flags: 0x05c58f86,
    }
}

pub fn record_0303() -> OrbitRecord {
    OrbitRecord {
        id: 303,
        norad_hint: 51211,
        min_elevation_deg: 26,
        max_pass_minutes: 216,
        flags: 0x0336bb3b,
    }
}

pub fn record_0304() -> OrbitRecord {
    OrbitRecord {
        id: 304,
        norad_hint: 51248,
        min_elevation_deg: 33,
        max_pass_minutes: 227,
        flags: 0x0127e6f0,
    }
}

pub fn record_0305() -> OrbitRecord {
    OrbitRecord {
        id: 305,
        norad_hint: 51285,
        min_elevation_deg: 40,
        max_pass_minutes: 238,
        flags: 0x1f1112a5,
    }
}

pub fn record_0306() -> OrbitRecord {
    OrbitRecord {
        id: 306,
        norad_hint: 51322,
        min_elevation_deg: 47,
        max_pass_minutes: 9,
        flags: 0x1d023e5a,
    }
}

pub fn record_0307() -> OrbitRecord {
    OrbitRecord {
        id: 307,
        norad_hint: 51359,
        min_elevation_deg: 54,
        max_pass_minutes: 20,
        flags: 0x1b736a0f,
    }
}

pub fn record_0308() -> OrbitRecord {
    OrbitRecord {
        id: 308,
        norad_hint: 51396,
        min_elevation_deg: 61,
        max_pass_minutes: 31,
        flags: 0x196c95c4,
    }
}

pub fn record_0309() -> OrbitRecord {
    OrbitRecord {
        id: 309,
        norad_hint: 51433,
        min_elevation_deg: 68,
        max_pass_minutes: 42,
        flags: 0x175dc179,
    }
}

pub fn record_0310() -> OrbitRecord {
    OrbitRecord {
        id: 310,
        norad_hint: 51470,
        min_elevation_deg: 5,
        max_pass_minutes: 53,
        flags: 0x154eed2e,
    }
}

pub fn record_0311() -> OrbitRecord {
    OrbitRecord {
        id: 311,
        norad_hint: 51507,
        min_elevation_deg: 12,
        max_pass_minutes: 64,
        flags: 0x14b818e3,
    }
}

pub fn record_0312() -> OrbitRecord {
    OrbitRecord {
        id: 312,
        norad_hint: 51544,
        min_elevation_deg: 19,
        max_pass_minutes: 75,
        flags: 0x12a94498,
    }
}

pub fn record_0313() -> OrbitRecord {
    OrbitRecord {
        id: 313,
        norad_hint: 51581,
        min_elevation_deg: 26,
        max_pass_minutes: 86,
        flags: 0x109a704d,
    }
}

pub fn record_0314() -> OrbitRecord {
    OrbitRecord {
        id: 314,
        norad_hint: 51618,
        min_elevation_deg: 33,
        max_pass_minutes: 97,
        flags: 0x2e8b9c02,
    }
}

pub fn record_0315() -> OrbitRecord {
    OrbitRecord {
        id: 315,
        norad_hint: 51655,
        min_elevation_deg: 40,
        max_pass_minutes: 108,
        flags: 0x2c84c7b7,
    }
}

pub fn record_0316() -> OrbitRecord {
    OrbitRecord {
        id: 316,
        norad_hint: 51692,
        min_elevation_deg: 47,
        max_pass_minutes: 119,
        flags: 0x2af5f36c,
    }
}

pub fn record_0317() -> OrbitRecord {
    OrbitRecord {
        id: 317,
        norad_hint: 51729,
        min_elevation_deg: 54,
        max_pass_minutes: 130,
        flags: 0x28e71f21,
    }
}

pub fn record_0318() -> OrbitRecord {
    OrbitRecord {
        id: 318,
        norad_hint: 51766,
        min_elevation_deg: 61,
        max_pass_minutes: 141,
        flags: 0x26d04ad6,
    }
}

pub fn record_0319() -> OrbitRecord {
    OrbitRecord {
        id: 319,
        norad_hint: 51803,
        min_elevation_deg: 68,
        max_pass_minutes: 152,
        flags: 0x24c1768b,
    }
}

pub fn record_0320() -> OrbitRecord {
    OrbitRecord {
        id: 320,
        norad_hint: 51840,
        min_elevation_deg: 5,
        max_pass_minutes: 163,
        flags: 0x2232a240,
    }
}

pub fn record_0321() -> OrbitRecord {
    OrbitRecord {
        id: 321,
        norad_hint: 51877,
        min_elevation_deg: 12,
        max_pass_minutes: 174,
        flags: 0x2023cdf5,
    }
}

pub fn record_0322() -> OrbitRecord {
    OrbitRecord {
        id: 322,
        norad_hint: 51914,
        min_elevation_deg: 19,
        max_pass_minutes: 185,
        flags: 0x3e1cf9aa,
    }
}

pub fn record_0323() -> OrbitRecord {
    OrbitRecord {
        id: 323,
        norad_hint: 51951,
        min_elevation_deg: 26,
        max_pass_minutes: 196,
        flags: 0x3c0e255f,
    }
}

pub fn record_0324() -> OrbitRecord {
    OrbitRecord {
        id: 324,
        norad_hint: 51988,
        min_elevation_deg: 33,
        max_pass_minutes: 207,
        flags: 0x3a7f5114,
    }
}

pub fn record_0325() -> OrbitRecord {
    OrbitRecord {
        id: 325,
        norad_hint: 40025,
        min_elevation_deg: 40,
        max_pass_minutes: 218,
        flags: 0x38687cc9,
    }
}

pub fn record_0326() -> OrbitRecord {
    OrbitRecord {
        id: 326,
        norad_hint: 40062,
        min_elevation_deg: 47,
        max_pass_minutes: 229,
        flags: 0x3659a87e,
    }
}

pub fn record_0327() -> OrbitRecord {
    OrbitRecord {
        id: 327,
        norad_hint: 40099,
        min_elevation_deg: 54,
        max_pass_minutes: 240,
        flags: 0x344ad433,
    }
}

pub fn record_0328() -> OrbitRecord {
    OrbitRecord {
        id: 328,
        norad_hint: 40136,
        min_elevation_deg: 61,
        max_pass_minutes: 11,
        flags: 0x33bbffe8,
    }
}

pub fn record_0329() -> OrbitRecord {
    OrbitRecord {
        id: 329,
        norad_hint: 40173,
        min_elevation_deg: 68,
        max_pass_minutes: 22,
        flags: 0x31b52b9d,
    }
}

pub fn record_0330() -> OrbitRecord {
    OrbitRecord {
        id: 330,
        norad_hint: 40210,
        min_elevation_deg: 5,
        max_pass_minutes: 33,
        flags: 0xcfa65752,
    }
}

pub fn record_0331() -> OrbitRecord {
    OrbitRecord {
        id: 331,
        norad_hint: 40247,
        min_elevation_deg: 12,
        max_pass_minutes: 44,
        flags: 0xcd978307,
    }
}

pub fn record_0332() -> OrbitRecord {
    OrbitRecord {
        id: 332,
        norad_hint: 40284,
        min_elevation_deg: 19,
        max_pass_minutes: 55,
        flags: 0xcb80aebc,
    }
}

pub fn record_0333() -> OrbitRecord {
    OrbitRecord {
        id: 333,
        norad_hint: 40321,
        min_elevation_deg: 26,
        max_pass_minutes: 66,
        flags: 0xc9f1da71,
    }
}

pub fn record_0334() -> OrbitRecord {
    OrbitRecord {
        id: 334,
        norad_hint: 40358,
        min_elevation_deg: 33,
        max_pass_minutes: 77,
        flags: 0xc7e30626,
    }
}

pub fn record_0335() -> OrbitRecord {
    OrbitRecord {
        id: 335,
        norad_hint: 40395,
        min_elevation_deg: 40,
        max_pass_minutes: 88,
        flags: 0xc5dc31db,
    }
}

pub fn record_0336() -> OrbitRecord {
    OrbitRecord {
        id: 336,
        norad_hint: 40432,
        min_elevation_deg: 47,
        max_pass_minutes: 99,
        flags: 0xc3cd5d90,
    }
}

pub fn record_0337() -> OrbitRecord {
    OrbitRecord {
        id: 337,
        norad_hint: 40469,
        min_elevation_deg: 54,
        max_pass_minutes: 110,
        flags: 0xc13e8945,
    }
}

pub fn record_0338() -> OrbitRecord {
    OrbitRecord {
        id: 338,
        norad_hint: 40506,
        min_elevation_deg: 61,
        max_pass_minutes: 121,
        flags: 0xdf2fb4fa,
    }
}

pub fn record_0339() -> OrbitRecord {
    OrbitRecord {
        id: 339,
        norad_hint: 40543,
        min_elevation_deg: 68,
        max_pass_minutes: 132,
        flags: 0xdd18e0af,
    }
}

pub fn record_0340() -> OrbitRecord {
    OrbitRecord {
        id: 340,
        norad_hint: 40580,
        min_elevation_deg: 5,
        max_pass_minutes: 143,
        flags: 0xdb0a0c64,
    }
}

pub fn record_0341() -> OrbitRecord {
    OrbitRecord {
        id: 341,
        norad_hint: 40617,
        min_elevation_deg: 12,
        max_pass_minutes: 154,
        flags: 0xd97b3819,
    }
}

pub fn record_0342() -> OrbitRecord {
    OrbitRecord {
        id: 342,
        norad_hint: 40654,
        min_elevation_deg: 19,
        max_pass_minutes: 165,
        flags: 0xd77463ce,
    }
}

pub fn record_0343() -> OrbitRecord {
    OrbitRecord {
        id: 343,
        norad_hint: 40691,
        min_elevation_deg: 26,
        max_pass_minutes: 176,
        flags: 0xd5658f83,
    }
}

pub fn record_0344() -> OrbitRecord {
    OrbitRecord {
        id: 344,
        norad_hint: 40728,
        min_elevation_deg: 33,
        max_pass_minutes: 187,
        flags: 0xd356bb38,
    }
}

pub fn record_0345() -> OrbitRecord {
    OrbitRecord {
        id: 345,
        norad_hint: 40765,
        min_elevation_deg: 40,
        max_pass_minutes: 198,
        flags: 0xd147e6ed,
    }
}

pub fn record_0346() -> OrbitRecord {
    OrbitRecord {
        id: 346,
        norad_hint: 40802,
        min_elevation_deg: 47,
        max_pass_minutes: 209,
        flags: 0xd0b112a2,
    }
}

pub fn record_0347() -> OrbitRecord {
    OrbitRecord {
        id: 347,
        norad_hint: 40839,
        min_elevation_deg: 54,
        max_pass_minutes: 220,
        flags: 0xeea23e57,
    }
}

pub fn record_0348() -> OrbitRecord {
    OrbitRecord {
        id: 348,
        norad_hint: 40876,
        min_elevation_deg: 61,
        max_pass_minutes: 231,
        flags: 0xec936a0c,
    }
}

pub fn record_0349() -> OrbitRecord {
    OrbitRecord {
        id: 349,
        norad_hint: 40913,
        min_elevation_deg: 68,
        max_pass_minutes: 242,
        flags: 0xea8c95c1,
    }
}

pub fn record_0350() -> OrbitRecord {
    OrbitRecord {
        id: 350,
        norad_hint: 40950,
        min_elevation_deg: 5,
        max_pass_minutes: 13,
        flags: 0xe8fdc176,
    }
}

pub fn record_0351() -> OrbitRecord {
    OrbitRecord {
        id: 351,
        norad_hint: 40987,
        min_elevation_deg: 12,
        max_pass_minutes: 24,
        flags: 0xe6eeed2b,
    }
}

pub fn record_0352() -> OrbitRecord {
    OrbitRecord {
        id: 352,
        norad_hint: 41024,
        min_elevation_deg: 19,
        max_pass_minutes: 35,
        flags: 0xe4d818e0,
    }
}

pub fn record_0353() -> OrbitRecord {
    OrbitRecord {
        id: 353,
        norad_hint: 41061,
        min_elevation_deg: 26,
        max_pass_minutes: 46,
        flags: 0xe2c94495,
    }
}

pub fn record_0354() -> OrbitRecord {
    OrbitRecord {
        id: 354,
        norad_hint: 41098,
        min_elevation_deg: 33,
        max_pass_minutes: 57,
        flags: 0xe03a704a,
    }
}

pub fn record_0355() -> OrbitRecord {
    OrbitRecord {
        id: 355,
        norad_hint: 41135,
        min_elevation_deg: 40,
        max_pass_minutes: 68,
        flags: 0xfe2b9bff,
    }
}

pub fn record_0356() -> OrbitRecord {
    OrbitRecord {
        id: 356,
        norad_hint: 41172,
        min_elevation_deg: 47,
        max_pass_minutes: 79,
        flags: 0xfc24c7b4,
    }
}

pub fn record_0357() -> OrbitRecord {
    OrbitRecord {
        id: 357,
        norad_hint: 41209,
        min_elevation_deg: 54,
        max_pass_minutes: 90,
        flags: 0xfa15f369,
    }
}

pub fn record_0358() -> OrbitRecord {
    OrbitRecord {
        id: 358,
        norad_hint: 41246,
        min_elevation_deg: 61,
        max_pass_minutes: 101,
        flags: 0xf8071f1e,
    }
}

pub fn record_0359() -> OrbitRecord {
    OrbitRecord {
        id: 359,
        norad_hint: 41283,
        min_elevation_deg: 68,
        max_pass_minutes: 112,
        flags: 0xf6704ad3,
    }
}

pub fn record_0360() -> OrbitRecord {
    OrbitRecord {
        id: 360,
        norad_hint: 41320,
        min_elevation_deg: 5,
        max_pass_minutes: 123,
        flags: 0xf4617688,
    }
}

pub fn record_0361() -> OrbitRecord {
    OrbitRecord {
        id: 361,
        norad_hint: 41357,
        min_elevation_deg: 12,
        max_pass_minutes: 134,
        flags: 0xf252a23d,
    }
}

pub fn record_0362() -> OrbitRecord {
    OrbitRecord {
        id: 362,
        norad_hint: 41394,
        min_elevation_deg: 19,
        max_pass_minutes: 145,
        flags: 0xf043cdf2,
    }
}

pub fn record_0363() -> OrbitRecord {
    OrbitRecord {
        id: 363,
        norad_hint: 41431,
        min_elevation_deg: 26,
        max_pass_minutes: 156,
        flags: 0x8fbcf9a7,
    }
}

pub fn record_0364() -> OrbitRecord {
    OrbitRecord {
        id: 364,
        norad_hint: 41468,
        min_elevation_deg: 33,
        max_pass_minutes: 167,
        flags: 0x8dae255c,
    }
}

pub fn record_0365() -> OrbitRecord {
    OrbitRecord {
        id: 365,
        norad_hint: 41505,
        min_elevation_deg: 40,
        max_pass_minutes: 178,
        flags: 0x8b9f5111,
    }
}

pub fn record_0366() -> OrbitRecord {
    OrbitRecord {
        id: 366,
        norad_hint: 41542,
        min_elevation_deg: 47,
        max_pass_minutes: 189,
        flags: 0x89887cc6,
    }
}

pub fn record_0367() -> OrbitRecord {
    OrbitRecord {
        id: 367,
        norad_hint: 41579,
        min_elevation_deg: 54,
        max_pass_minutes: 200,
        flags: 0x87f9a87b,
    }
}

pub fn record_0368() -> OrbitRecord {
    OrbitRecord {
        id: 368,
        norad_hint: 41616,
        min_elevation_deg: 61,
        max_pass_minutes: 211,
        flags: 0x85ead430,
    }
}

pub fn record_0369() -> OrbitRecord {
    OrbitRecord {
        id: 369,
        norad_hint: 41653,
        min_elevation_deg: 68,
        max_pass_minutes: 222,
        flags: 0x83dbffe5,
    }
}

pub fn record_0370() -> OrbitRecord {
    OrbitRecord {
        id: 370,
        norad_hint: 41690,
        min_elevation_deg: 5,
        max_pass_minutes: 233,
        flags: 0x81d52b9a,
    }
}

pub fn record_0371() -> OrbitRecord {
    OrbitRecord {
        id: 371,
        norad_hint: 41727,
        min_elevation_deg: 12,
        max_pass_minutes: 4,
        flags: 0x9fc6574f,
    }
}

pub fn record_0372() -> OrbitRecord {
    OrbitRecord {
        id: 372,
        norad_hint: 41764,
        min_elevation_deg: 19,
        max_pass_minutes: 15,
        flags: 0x9d378304,
    }
}

pub fn record_0373() -> OrbitRecord {
    OrbitRecord {
        id: 373,
        norad_hint: 41801,
        min_elevation_deg: 26,
        max_pass_minutes: 26,
        flags: 0x9b20aeb9,
    }
}

pub fn record_0374() -> OrbitRecord {
    OrbitRecord {
        id: 374,
        norad_hint: 41838,
        min_elevation_deg: 33,
        max_pass_minutes: 37,
        flags: 0x9911da6e,
    }
}

pub fn record_0375() -> OrbitRecord {
    OrbitRecord {
        id: 375,
        norad_hint: 41875,
        min_elevation_deg: 40,
        max_pass_minutes: 48,
        flags: 0x97030623,
    }
}

pub fn record_0376() -> OrbitRecord {
    OrbitRecord {
        id: 376,
        norad_hint: 41912,
        min_elevation_deg: 47,
        max_pass_minutes: 59,
        flags: 0x957c31d8,
    }
}

pub fn record_0377() -> OrbitRecord {
    OrbitRecord {
        id: 377,
        norad_hint: 41949,
        min_elevation_deg: 54,
        max_pass_minutes: 70,
        flags: 0x936d5d8d,
    }
}

pub fn record_0378() -> OrbitRecord {
    OrbitRecord {
        id: 378,
        norad_hint: 41986,
        min_elevation_deg: 61,
        max_pass_minutes: 81,
        flags: 0x915e8942,
    }
}

pub fn record_0379() -> OrbitRecord {
    OrbitRecord {
        id: 379,
        norad_hint: 42023,
        min_elevation_deg: 68,
        max_pass_minutes: 92,
        flags: 0xaf4fb4f7,
    }
}

pub fn record_0380() -> OrbitRecord {
    OrbitRecord {
        id: 380,
        norad_hint: 42060,
        min_elevation_deg: 5,
        max_pass_minutes: 103,
        flags: 0xaeb8e0ac,
    }
}

pub fn record_0381() -> OrbitRecord {
    OrbitRecord {
        id: 381,
        norad_hint: 42097,
        min_elevation_deg: 12,
        max_pass_minutes: 114,
        flags: 0xacaa0c61,
    }
}

pub fn record_0382() -> OrbitRecord {
    OrbitRecord {
        id: 382,
        norad_hint: 42134,
        min_elevation_deg: 19,
        max_pass_minutes: 125,
        flags: 0xaa9b3816,
    }
}

pub fn record_0383() -> OrbitRecord {
    OrbitRecord {
        id: 383,
        norad_hint: 42171,
        min_elevation_deg: 26,
        max_pass_minutes: 136,
        flags: 0xa89463cb,
    }
}

pub fn record_0384() -> OrbitRecord {
    OrbitRecord {
        id: 384,
        norad_hint: 42208,
        min_elevation_deg: 33,
        max_pass_minutes: 147,
        flags: 0xa6858f80,
    }
}

pub fn record_0385() -> OrbitRecord {
    OrbitRecord {
        id: 385,
        norad_hint: 42245,
        min_elevation_deg: 40,
        max_pass_minutes: 158,
        flags: 0xa4f6bb35,
    }
}

pub fn record_0386() -> OrbitRecord {
    OrbitRecord {
        id: 386,
        norad_hint: 42282,
        min_elevation_deg: 47,
        max_pass_minutes: 169,
        flags: 0xa2e7e6ea,
    }
}

pub fn record_0387() -> OrbitRecord {
    OrbitRecord {
        id: 387,
        norad_hint: 42319,
        min_elevation_deg: 54,
        max_pass_minutes: 180,
        flags: 0xa0d1129f,
    }
}

pub fn record_0388() -> OrbitRecord {
    OrbitRecord {
        id: 388,
        norad_hint: 42356,
        min_elevation_deg: 61,
        max_pass_minutes: 191,
        flags: 0xbec23e54,
    }
}

pub fn record_0389() -> OrbitRecord {
    OrbitRecord {
        id: 389,
        norad_hint: 42393,
        min_elevation_deg: 68,
        max_pass_minutes: 202,
        flags: 0xbc336a09,
    }
}

pub fn record_0390() -> OrbitRecord {
    OrbitRecord {
        id: 390,
        norad_hint: 42430,
        min_elevation_deg: 5,
        max_pass_minutes: 213,
        flags: 0xba2c95be,
    }
}

pub fn record_0391() -> OrbitRecord {
    OrbitRecord {
        id: 391,
        norad_hint: 42467,
        min_elevation_deg: 12,
        max_pass_minutes: 224,
        flags: 0xb81dc173,
    }
}

pub fn record_0392() -> OrbitRecord {
    OrbitRecord {
        id: 392,
        norad_hint: 42504,
        min_elevation_deg: 19,
        max_pass_minutes: 235,
        flags: 0xb60eed28,
    }
}

pub fn record_0393() -> OrbitRecord {
    OrbitRecord {
        id: 393,
        norad_hint: 42541,
        min_elevation_deg: 26,
        max_pass_minutes: 6,
        flags: 0xb47818dd,
    }
}

pub fn record_0394() -> OrbitRecord {
    OrbitRecord {
        id: 394,
        norad_hint: 42578,
        min_elevation_deg: 33,
        max_pass_minutes: 17,
        flags: 0xb2694492,
    }
}

pub fn record_0395() -> OrbitRecord {
    OrbitRecord {
        id: 395,
        norad_hint: 42615,
        min_elevation_deg: 40,
        max_pass_minutes: 28,
        flags: 0xb05a7047,
    }
}

pub fn record_0396() -> OrbitRecord {
    OrbitRecord {
        id: 396,
        norad_hint: 42652,
        min_elevation_deg: 47,
        max_pass_minutes: 39,
        flags: 0x4e4b9bfc,
    }
}

pub fn record_0397() -> OrbitRecord {
    OrbitRecord {
        id: 397,
        norad_hint: 42689,
        min_elevation_deg: 54,
        max_pass_minutes: 50,
        flags: 0x4c44c7b1,
    }
}

pub fn record_0398() -> OrbitRecord {
    OrbitRecord {
        id: 398,
        norad_hint: 42726,
        min_elevation_deg: 61,
        max_pass_minutes: 61,
        flags: 0x4bb5f366,
    }
}

pub fn record_0399() -> OrbitRecord {
    OrbitRecord {
        id: 399,
        norad_hint: 42763,
        min_elevation_deg: 68,
        max_pass_minutes: 72,
        flags: 0x49a71f1b,
    }
}

pub fn record_0400() -> OrbitRecord {
    OrbitRecord {
        id: 400,
        norad_hint: 42800,
        min_elevation_deg: 5,
        max_pass_minutes: 83,
        flags: 0x47904ad0,
    }
}

pub fn record_0401() -> OrbitRecord {
    OrbitRecord {
        id: 401,
        norad_hint: 42837,
        min_elevation_deg: 12,
        max_pass_minutes: 94,
        flags: 0x45817685,
    }
}

pub fn record_0402() -> OrbitRecord {
    OrbitRecord {
        id: 402,
        norad_hint: 42874,
        min_elevation_deg: 19,
        max_pass_minutes: 105,
        flags: 0x43f2a23a,
    }
}

pub fn record_0403() -> OrbitRecord {
    OrbitRecord {
        id: 403,
        norad_hint: 42911,
        min_elevation_deg: 26,
        max_pass_minutes: 116,
        flags: 0x41e3cdef,
    }
}

pub fn record_0404() -> OrbitRecord {
    OrbitRecord {
        id: 404,
        norad_hint: 42948,
        min_elevation_deg: 33,
        max_pass_minutes: 127,
        flags: 0x5fdcf9a4,
    }
}

pub fn record_0405() -> OrbitRecord {
    OrbitRecord {
        id: 405,
        norad_hint: 42985,
        min_elevation_deg: 40,
        max_pass_minutes: 138,
        flags: 0x5dce2559,
    }
}

pub fn record_0406() -> OrbitRecord {
    OrbitRecord {
        id: 406,
        norad_hint: 43022,
        min_elevation_deg: 47,
        max_pass_minutes: 149,
        flags: 0x5b3f510e,
    }
}

pub fn record_0407() -> OrbitRecord {
    OrbitRecord {
        id: 407,
        norad_hint: 43059,
        min_elevation_deg: 54,
        max_pass_minutes: 160,
        flags: 0x59287cc3,
    }
}

pub fn record_0408() -> OrbitRecord {
    OrbitRecord {
        id: 408,
        norad_hint: 43096,
        min_elevation_deg: 61,
        max_pass_minutes: 171,
        flags: 0x5719a878,
    }
}

pub fn record_0409() -> OrbitRecord {
    OrbitRecord {
        id: 409,
        norad_hint: 43133,
        min_elevation_deg: 68,
        max_pass_minutes: 182,
        flags: 0x550ad42d,
    }
}

pub fn record_0410() -> OrbitRecord {
    OrbitRecord {
        id: 410,
        norad_hint: 43170,
        min_elevation_deg: 5,
        max_pass_minutes: 193,
        flags: 0x537bffe2,
    }
}

pub fn record_0411() -> OrbitRecord {
    OrbitRecord {
        id: 411,
        norad_hint: 43207,
        min_elevation_deg: 12,
        max_pass_minutes: 204,
        flags: 0x51752b97,
    }
}

pub fn record_0412() -> OrbitRecord {
    OrbitRecord {
        id: 412,
        norad_hint: 43244,
        min_elevation_deg: 19,
        max_pass_minutes: 215,
        flags: 0x6f66574c,
    }
}

pub fn record_0413() -> OrbitRecord {
    OrbitRecord {
        id: 413,
        norad_hint: 43281,
        min_elevation_deg: 26,
        max_pass_minutes: 226,
        flags: 0x6d578301,
    }
}

pub fn record_0414() -> OrbitRecord {
    OrbitRecord {
        id: 414,
        norad_hint: 43318,
        min_elevation_deg: 33,
        max_pass_minutes: 237,
        flags: 0x6b40aeb6,
    }
}

pub fn record_0415() -> OrbitRecord {
    OrbitRecord {
        id: 415,
        norad_hint: 43355,
        min_elevation_deg: 40,
        max_pass_minutes: 8,
        flags: 0x6ab1da6b,
    }
}

pub fn record_0416() -> OrbitRecord {
    OrbitRecord {
        id: 416,
        norad_hint: 43392,
        min_elevation_deg: 47,
        max_pass_minutes: 19,
        flags: 0x68a30620,
    }
}

pub fn record_0417() -> OrbitRecord {
    OrbitRecord {
        id: 417,
        norad_hint: 43429,
        min_elevation_deg: 54,
        max_pass_minutes: 30,
        flags: 0x669c31d5,
    }
}

pub fn record_0418() -> OrbitRecord {
    OrbitRecord {
        id: 418,
        norad_hint: 43466,
        min_elevation_deg: 61,
        max_pass_minutes: 41,
        flags: 0x648d5d8a,
    }
}

pub fn record_0419() -> OrbitRecord {
    OrbitRecord {
        id: 419,
        norad_hint: 43503,
        min_elevation_deg: 68,
        max_pass_minutes: 52,
        flags: 0x62fe893f,
    }
}

pub fn record_0420() -> OrbitRecord {
    OrbitRecord {
        id: 420,
        norad_hint: 43540,
        min_elevation_deg: 5,
        max_pass_minutes: 63,
        flags: 0x60efb4f4,
    }
}

pub fn record_0421() -> OrbitRecord {
    OrbitRecord {
        id: 421,
        norad_hint: 43577,
        min_elevation_deg: 12,
        max_pass_minutes: 74,
        flags: 0x7ed8e0a9,
    }
}

pub fn record_0422() -> OrbitRecord {
    OrbitRecord {
        id: 422,
        norad_hint: 43614,
        min_elevation_deg: 19,
        max_pass_minutes: 85,
        flags: 0x7cca0c5e,
    }
}

pub fn record_0423() -> OrbitRecord {
    OrbitRecord {
        id: 423,
        norad_hint: 43651,
        min_elevation_deg: 26,
        max_pass_minutes: 96,
        flags: 0x7a3b3813,
    }
}

pub fn record_0424() -> OrbitRecord {
    OrbitRecord {
        id: 424,
        norad_hint: 43688,
        min_elevation_deg: 33,
        max_pass_minutes: 107,
        flags: 0x783463c8,
    }
}

pub fn record_0425() -> OrbitRecord {
    OrbitRecord {
        id: 425,
        norad_hint: 43725,
        min_elevation_deg: 40,
        max_pass_minutes: 118,
        flags: 0x76258f7d,
    }
}

pub fn record_0426() -> OrbitRecord {
    OrbitRecord {
        id: 426,
        norad_hint: 43762,
        min_elevation_deg: 47,
        max_pass_minutes: 129,
        flags: 0x7416bb32,
    }
}

pub fn record_0427() -> OrbitRecord {
    OrbitRecord {
        id: 427,
        norad_hint: 43799,
        min_elevation_deg: 54,
        max_pass_minutes: 140,
        flags: 0x7207e6e7,
    }
}

pub fn record_0428() -> OrbitRecord {
    OrbitRecord {
        id: 428,
        norad_hint: 43836,
        min_elevation_deg: 61,
        max_pass_minutes: 151,
        flags: 0x7071129c,
    }
}

pub fn record_0429() -> OrbitRecord {
    OrbitRecord {
        id: 429,
        norad_hint: 43873,
        min_elevation_deg: 68,
        max_pass_minutes: 162,
        flags: 0x0e623e51,
    }
}

pub fn record_0430() -> OrbitRecord {
    OrbitRecord {
        id: 430,
        norad_hint: 43910,
        min_elevation_deg: 5,
        max_pass_minutes: 173,
        flags: 0x0c536a06,
    }
}

pub fn record_0431() -> OrbitRecord {
    OrbitRecord {
        id: 431,
        norad_hint: 43947,
        min_elevation_deg: 12,
        max_pass_minutes: 184,
        flags: 0x0a4c95bb,
    }
}

pub fn record_0432() -> OrbitRecord {
    OrbitRecord {
        id: 432,
        norad_hint: 43984,
        min_elevation_deg: 19,
        max_pass_minutes: 195,
        flags: 0x09bdc170,
    }
}

pub fn record_0433() -> OrbitRecord {
    OrbitRecord {
        id: 433,
        norad_hint: 44021,
        min_elevation_deg: 26,
        max_pass_minutes: 206,
        flags: 0x07aeed25,
    }
}

pub fn record_0434() -> OrbitRecord {
    OrbitRecord {
        id: 434,
        norad_hint: 44058,
        min_elevation_deg: 33,
        max_pass_minutes: 217,
        flags: 0x059818da,
    }
}

pub fn record_0435() -> OrbitRecord {
    OrbitRecord {
        id: 435,
        norad_hint: 44095,
        min_elevation_deg: 40,
        max_pass_minutes: 228,
        flags: 0x0389448f,
    }
}

pub fn record_0436() -> OrbitRecord {
    OrbitRecord {
        id: 436,
        norad_hint: 44132,
        min_elevation_deg: 47,
        max_pass_minutes: 239,
        flags: 0x01fa7044,
    }
}

pub fn record_0437() -> OrbitRecord {
    OrbitRecord {
        id: 437,
        norad_hint: 44169,
        min_elevation_deg: 54,
        max_pass_minutes: 10,
        flags: 0x1feb9bf9,
    }
}

pub fn record_0438() -> OrbitRecord {
    OrbitRecord {
        id: 438,
        norad_hint: 44206,
        min_elevation_deg: 61,
        max_pass_minutes: 21,
        flags: 0x1de4c7ae,
    }
}

pub fn record_0439() -> OrbitRecord {
    OrbitRecord {
        id: 439,
        norad_hint: 44243,
        min_elevation_deg: 68,
        max_pass_minutes: 32,
        flags: 0x1bd5f363,
    }
}

pub fn record_0440() -> OrbitRecord {
    OrbitRecord {
        id: 440,
        norad_hint: 44280,
        min_elevation_deg: 5,
        max_pass_minutes: 43,
        flags: 0x19c71f18,
    }
}

pub fn record_0441() -> OrbitRecord {
    OrbitRecord {
        id: 441,
        norad_hint: 44317,
        min_elevation_deg: 12,
        max_pass_minutes: 54,
        flags: 0x17304acd,
    }
}

pub fn record_0442() -> OrbitRecord {
    OrbitRecord {
        id: 442,
        norad_hint: 44354,
        min_elevation_deg: 19,
        max_pass_minutes: 65,
        flags: 0x15217682,
    }
}

pub fn record_0443() -> OrbitRecord {
    OrbitRecord {
        id: 443,
        norad_hint: 44391,
        min_elevation_deg: 26,
        max_pass_minutes: 76,
        flags: 0x1312a237,
    }
}

pub fn record_0444() -> OrbitRecord {
    OrbitRecord {
        id: 444,
        norad_hint: 44428,
        min_elevation_deg: 33,
        max_pass_minutes: 87,
        flags: 0x1103cdec,
    }
}

pub fn record_0445() -> OrbitRecord {
    OrbitRecord {
        id: 445,
        norad_hint: 44465,
        min_elevation_deg: 40,
        max_pass_minutes: 98,
        flags: 0x2f7cf9a1,
    }
}

pub fn record_0446() -> OrbitRecord {
    OrbitRecord {
        id: 446,
        norad_hint: 44502,
        min_elevation_deg: 47,
        max_pass_minutes: 109,
        flags: 0x2d6e2556,
    }
}

pub fn record_0447() -> OrbitRecord {
    OrbitRecord {
        id: 447,
        norad_hint: 44539,
        min_elevation_deg: 54,
        max_pass_minutes: 120,
        flags: 0x2b5f510b,
    }
}

pub fn record_0448() -> OrbitRecord {
    OrbitRecord {
        id: 448,
        norad_hint: 44576,
        min_elevation_deg: 61,
        max_pass_minutes: 131,
        flags: 0x29487cc0,
    }
}

pub fn record_0449() -> OrbitRecord {
    OrbitRecord {
        id: 449,
        norad_hint: 44613,
        min_elevation_deg: 68,
        max_pass_minutes: 142,
        flags: 0x28b9a875,
    }
}

pub fn record_0450() -> OrbitRecord {
    OrbitRecord {
        id: 450,
        norad_hint: 44650,
        min_elevation_deg: 5,
        max_pass_minutes: 153,
        flags: 0x26aad42a,
    }
}

pub fn record_0451() -> OrbitRecord {
    OrbitRecord {
        id: 451,
        norad_hint: 44687,
        min_elevation_deg: 12,
        max_pass_minutes: 164,
        flags: 0x249bffdf,
    }
}

pub fn record_0452() -> OrbitRecord {
    OrbitRecord {
        id: 452,
        norad_hint: 44724,
        min_elevation_deg: 19,
        max_pass_minutes: 175,
        flags: 0x22952b94,
    }
}

pub fn record_0453() -> OrbitRecord {
    OrbitRecord {
        id: 453,
        norad_hint: 44761,
        min_elevation_deg: 26,
        max_pass_minutes: 186,
        flags: 0x20865749,
    }
}

pub fn record_0454() -> OrbitRecord {
    OrbitRecord {
        id: 454,
        norad_hint: 44798,
        min_elevation_deg: 33,
        max_pass_minutes: 197,
        flags: 0x3ef782fe,
    }
}

pub fn record_0455() -> OrbitRecord {
    OrbitRecord {
        id: 455,
        norad_hint: 44835,
        min_elevation_deg: 40,
        max_pass_minutes: 208,
        flags: 0x3ce0aeb3,
    }
}

pub fn record_0456() -> OrbitRecord {
    OrbitRecord {
        id: 456,
        norad_hint: 44872,
        min_elevation_deg: 47,
        max_pass_minutes: 219,
        flags: 0x3ad1da68,
    }
}

pub fn record_0457() -> OrbitRecord {
    OrbitRecord {
        id: 457,
        norad_hint: 44909,
        min_elevation_deg: 54,
        max_pass_minutes: 230,
        flags: 0x38c3061d,
    }
}

pub fn record_0458() -> OrbitRecord {
    OrbitRecord {
        id: 458,
        norad_hint: 44946,
        min_elevation_deg: 61,
        max_pass_minutes: 241,
        flags: 0x363c31d2,
    }
}

pub fn record_0459() -> OrbitRecord {
    OrbitRecord {
        id: 459,
        norad_hint: 44983,
        min_elevation_deg: 68,
        max_pass_minutes: 12,
        flags: 0x342d5d87,
    }
}

pub fn record_0460() -> OrbitRecord {
    OrbitRecord {
        id: 460,
        norad_hint: 45020,
        min_elevation_deg: 5,
        max_pass_minutes: 23,
        flags: 0x321e893c,
    }
}

pub fn record_0461() -> OrbitRecord {
    OrbitRecord {
        id: 461,
        norad_hint: 45057,
        min_elevation_deg: 12,
        max_pass_minutes: 34,
        flags: 0x300fb4f1,
    }
}

pub fn record_0462() -> OrbitRecord {
    OrbitRecord {
        id: 462,
        norad_hint: 45094,
        min_elevation_deg: 19,
        max_pass_minutes: 45,
        flags: 0xce78e0a6,
    }
}

pub fn record_0463() -> OrbitRecord {
    OrbitRecord {
        id: 463,
        norad_hint: 45131,
        min_elevation_deg: 26,
        max_pass_minutes: 56,
        flags: 0xcc6a0c5b,
    }
}

pub fn record_0464() -> OrbitRecord {
    OrbitRecord {
        id: 464,
        norad_hint: 45168,
        min_elevation_deg: 33,
        max_pass_minutes: 67,
        flags: 0xca5b3810,
    }
}

pub fn record_0465() -> OrbitRecord {
    OrbitRecord {
        id: 465,
        norad_hint: 45205,
        min_elevation_deg: 40,
        max_pass_minutes: 78,
        flags: 0xc85463c5,
    }
}

pub fn record_0466() -> OrbitRecord {
    OrbitRecord {
        id: 466,
        norad_hint: 45242,
        min_elevation_deg: 47,
        max_pass_minutes: 89,
        flags: 0xc6458f7a,
    }
}

pub fn record_0467() -> OrbitRecord {
    OrbitRecord {
        id: 467,
        norad_hint: 45279,
        min_elevation_deg: 54,
        max_pass_minutes: 100,
        flags: 0xc5b6bb2f,
    }
}

pub fn record_0468() -> OrbitRecord {
    OrbitRecord {
        id: 468,
        norad_hint: 45316,
        min_elevation_deg: 61,
        max_pass_minutes: 111,
        flags: 0xc3a7e6e4,
    }
}

pub fn record_0469() -> OrbitRecord {
    OrbitRecord {
        id: 469,
        norad_hint: 45353,
        min_elevation_deg: 68,
        max_pass_minutes: 122,
        flags: 0xc1911299,
    }
}

pub fn record_0470() -> OrbitRecord {
    OrbitRecord {
        id: 470,
        norad_hint: 45390,
        min_elevation_deg: 5,
        max_pass_minutes: 133,
        flags: 0xdf823e4e,
    }
}

pub fn record_0471() -> OrbitRecord {
    OrbitRecord {
        id: 471,
        norad_hint: 45427,
        min_elevation_deg: 12,
        max_pass_minutes: 144,
        flags: 0xddf36a03,
    }
}

pub fn record_0472() -> OrbitRecord {
    OrbitRecord {
        id: 472,
        norad_hint: 45464,
        min_elevation_deg: 19,
        max_pass_minutes: 155,
        flags: 0xdbec95b8,
    }
}

pub fn record_0473() -> OrbitRecord {
    OrbitRecord {
        id: 473,
        norad_hint: 45501,
        min_elevation_deg: 26,
        max_pass_minutes: 166,
        flags: 0xd9ddc16d,
    }
}

pub fn record_0474() -> OrbitRecord {
    OrbitRecord {
        id: 474,
        norad_hint: 45538,
        min_elevation_deg: 33,
        max_pass_minutes: 177,
        flags: 0xd7ceed22,
    }
}

pub fn record_0475() -> OrbitRecord {
    OrbitRecord {
        id: 475,
        norad_hint: 45575,
        min_elevation_deg: 40,
        max_pass_minutes: 188,
        flags: 0xd53818d7,
    }
}

pub fn record_0476() -> OrbitRecord {
    OrbitRecord {
        id: 476,
        norad_hint: 45612,
        min_elevation_deg: 47,
        max_pass_minutes: 199,
        flags: 0xd329448c,
    }
}

pub fn record_0477() -> OrbitRecord {
    OrbitRecord {
        id: 477,
        norad_hint: 45649,
        min_elevation_deg: 54,
        max_pass_minutes: 210,
        flags: 0xd11a7041,
    }
}

pub fn record_0478() -> OrbitRecord {
    OrbitRecord {
        id: 478,
        norad_hint: 45686,
        min_elevation_deg: 61,
        max_pass_minutes: 221,
        flags: 0xef0b9bf6,
    }
}

pub fn record_0479() -> OrbitRecord {
    OrbitRecord {
        id: 479,
        norad_hint: 45723,
        min_elevation_deg: 68,
        max_pass_minutes: 232,
        flags: 0xed04c7ab,
    }
}

pub fn record_0480() -> OrbitRecord {
    OrbitRecord {
        id: 480,
        norad_hint: 45760,
        min_elevation_deg: 5,
        max_pass_minutes: 3,
        flags: 0xeb75f360,
    }
}

pub fn record_0481() -> OrbitRecord {
    OrbitRecord {
        id: 481,
        norad_hint: 45797,
        min_elevation_deg: 12,
        max_pass_minutes: 14,
        flags: 0xe9671f15,
    }
}

pub fn record_0482() -> OrbitRecord {
    OrbitRecord {
        id: 482,
        norad_hint: 45834,
        min_elevation_deg: 19,
        max_pass_minutes: 25,
        flags: 0xe7504aca,
    }
}

pub fn record_0483() -> OrbitRecord {
    OrbitRecord {
        id: 483,
        norad_hint: 45871,
        min_elevation_deg: 26,
        max_pass_minutes: 36,
        flags: 0xe541767f,
    }
}

pub fn record_0484() -> OrbitRecord {
    OrbitRecord {
        id: 484,
        norad_hint: 45908,
        min_elevation_deg: 33,
        max_pass_minutes: 47,
        flags: 0xe4b2a234,
    }
}

pub fn record_0485() -> OrbitRecord {
    OrbitRecord {
        id: 485,
        norad_hint: 45945,
        min_elevation_deg: 40,
        max_pass_minutes: 58,
        flags: 0xe2a3cde9,
    }
}

pub fn record_0486() -> OrbitRecord {
    OrbitRecord {
        id: 486,
        norad_hint: 45982,
        min_elevation_deg: 47,
        max_pass_minutes: 69,
        flags: 0xe09cf99e,
    }
}

pub fn record_0487() -> OrbitRecord {
    OrbitRecord {
        id: 487,
        norad_hint: 46019,
        min_elevation_deg: 54,
        max_pass_minutes: 80,
        flags: 0xfe8e2553,
    }
}

pub fn record_0488() -> OrbitRecord {
    OrbitRecord {
        id: 488,
        norad_hint: 46056,
        min_elevation_deg: 61,
        max_pass_minutes: 91,
        flags: 0xfcff5108,
    }
}

pub fn record_0489() -> OrbitRecord {
    OrbitRecord {
        id: 489,
        norad_hint: 46093,
        min_elevation_deg: 68,
        max_pass_minutes: 102,
        flags: 0xfae87cbd,
    }
}

pub fn record_0490() -> OrbitRecord {
    OrbitRecord {
        id: 490,
        norad_hint: 46130,
        min_elevation_deg: 5,
        max_pass_minutes: 113,
        flags: 0xf8d9a872,
    }
}

pub fn record_0491() -> OrbitRecord {
    OrbitRecord {
        id: 491,
        norad_hint: 46167,
        min_elevation_deg: 12,
        max_pass_minutes: 124,
        flags: 0xf6cad427,
    }
}

pub fn record_0492() -> OrbitRecord {
    OrbitRecord {
        id: 492,
        norad_hint: 46204,
        min_elevation_deg: 19,
        max_pass_minutes: 135,
        flags: 0xf43bffdc,
    }
}

pub fn record_0493() -> OrbitRecord {
    OrbitRecord {
        id: 493,
        norad_hint: 46241,
        min_elevation_deg: 26,
        max_pass_minutes: 146,
        flags: 0xf2352b91,
    }
}

pub fn record_0494() -> OrbitRecord {
    OrbitRecord {
        id: 494,
        norad_hint: 46278,
        min_elevation_deg: 33,
        max_pass_minutes: 157,
        flags: 0xf0265746,
    }
}

pub fn record_0495() -> OrbitRecord {
    OrbitRecord {
        id: 495,
        norad_hint: 46315,
        min_elevation_deg: 40,
        max_pass_minutes: 168,
        flags: 0x8e1782fb,
    }
}

pub fn record_0496() -> OrbitRecord {
    OrbitRecord {
        id: 496,
        norad_hint: 46352,
        min_elevation_deg: 47,
        max_pass_minutes: 179,
        flags: 0x8c00aeb0,
    }
}

pub fn record_0497() -> OrbitRecord {
    OrbitRecord {
        id: 497,
        norad_hint: 46389,
        min_elevation_deg: 54,
        max_pass_minutes: 190,
        flags: 0x8a71da65,
    }
}

pub fn record_0498() -> OrbitRecord {
    OrbitRecord {
        id: 498,
        norad_hint: 46426,
        min_elevation_deg: 61,
        max_pass_minutes: 201,
        flags: 0x8863061a,
    }
}

pub fn record_0499() -> OrbitRecord {
    OrbitRecord {
        id: 499,
        norad_hint: 46463,
        min_elevation_deg: 68,
        max_pass_minutes: 212,
        flags: 0x865c31cf,
    }
}

pub fn record_0500() -> OrbitRecord {
    OrbitRecord {
        id: 500,
        norad_hint: 46500,
        min_elevation_deg: 5,
        max_pass_minutes: 223,
        flags: 0x844d5d84,
    }
}

pub fn record_0501() -> OrbitRecord {
    OrbitRecord {
        id: 501,
        norad_hint: 46537,
        min_elevation_deg: 12,
        max_pass_minutes: 234,
        flags: 0x83be8939,
    }
}

pub fn record_0502() -> OrbitRecord {
    OrbitRecord {
        id: 502,
        norad_hint: 46574,
        min_elevation_deg: 19,
        max_pass_minutes: 5,
        flags: 0x81afb4ee,
    }
}

pub fn record_0503() -> OrbitRecord {
    OrbitRecord {
        id: 503,
        norad_hint: 46611,
        min_elevation_deg: 26,
        max_pass_minutes: 16,
        flags: 0x9f98e0a3,
    }
}

pub fn record_0504() -> OrbitRecord {
    OrbitRecord {
        id: 504,
        norad_hint: 46648,
        min_elevation_deg: 33,
        max_pass_minutes: 27,
        flags: 0x9d8a0c58,
    }
}

pub fn record_0505() -> OrbitRecord {
    OrbitRecord {
        id: 505,
        norad_hint: 46685,
        min_elevation_deg: 40,
        max_pass_minutes: 38,
        flags: 0x9bfb380d,
    }
}

pub fn record_0506() -> OrbitRecord {
    OrbitRecord {
        id: 506,
        norad_hint: 46722,
        min_elevation_deg: 47,
        max_pass_minutes: 49,
        flags: 0x99f463c2,
    }
}

pub fn record_0507() -> OrbitRecord {
    OrbitRecord {
        id: 507,
        norad_hint: 46759,
        min_elevation_deg: 54,
        max_pass_minutes: 60,
        flags: 0x97e58f77,
    }
}

pub fn record_0508() -> OrbitRecord {
    OrbitRecord {
        id: 508,
        norad_hint: 46796,
        min_elevation_deg: 61,
        max_pass_minutes: 71,
        flags: 0x95d6bb2c,
    }
}

pub fn record_0509() -> OrbitRecord {
    OrbitRecord {
        id: 509,
        norad_hint: 46833,
        min_elevation_deg: 68,
        max_pass_minutes: 82,
        flags: 0x93c7e6e1,
    }
}

pub fn record_0510() -> OrbitRecord {
    OrbitRecord {
        id: 510,
        norad_hint: 46870,
        min_elevation_deg: 5,
        max_pass_minutes: 93,
        flags: 0x91311296,
    }
}

pub fn record_0511() -> OrbitRecord {
    OrbitRecord {
        id: 511,
        norad_hint: 46907,
        min_elevation_deg: 12,
        max_pass_minutes: 104,
        flags: 0xaf223e4b,
    }
}

pub fn record_0512() -> OrbitRecord {
    OrbitRecord {
        id: 512,
        norad_hint: 46944,
        min_elevation_deg: 19,
        max_pass_minutes: 115,
        flags: 0xad136a00,
    }
}

pub fn record_0513() -> OrbitRecord {
    OrbitRecord {
        id: 513,
        norad_hint: 46981,
        min_elevation_deg: 26,
        max_pass_minutes: 126,
        flags: 0xab0c95b5,
    }
}

pub fn record_0514() -> OrbitRecord {
    OrbitRecord {
        id: 514,
        norad_hint: 47018,
        min_elevation_deg: 33,
        max_pass_minutes: 137,
        flags: 0xa97dc16a,
    }
}

pub fn record_0515() -> OrbitRecord {
    OrbitRecord {
        id: 515,
        norad_hint: 47055,
        min_elevation_deg: 40,
        max_pass_minutes: 148,
        flags: 0xa76eed1f,
    }
}

pub fn record_0516() -> OrbitRecord {
    OrbitRecord {
        id: 516,
        norad_hint: 47092,
        min_elevation_deg: 47,
        max_pass_minutes: 159,
        flags: 0xa55818d4,
    }
}

pub fn record_0517() -> OrbitRecord {
    OrbitRecord {
        id: 517,
        norad_hint: 47129,
        min_elevation_deg: 54,
        max_pass_minutes: 170,
        flags: 0xa3494489,
    }
}

pub fn record_0518() -> OrbitRecord {
    OrbitRecord {
        id: 518,
        norad_hint: 47166,
        min_elevation_deg: 61,
        max_pass_minutes: 181,
        flags: 0xa2ba703e,
    }
}

pub fn record_0519() -> OrbitRecord {
    OrbitRecord {
        id: 519,
        norad_hint: 47203,
        min_elevation_deg: 68,
        max_pass_minutes: 192,
        flags: 0xa0ab9bf3,
    }
}

pub fn record_0520() -> OrbitRecord {
    OrbitRecord {
        id: 520,
        norad_hint: 47240,
        min_elevation_deg: 5,
        max_pass_minutes: 203,
        flags: 0xbea4c7a8,
    }
}

pub fn record_0521() -> OrbitRecord {
    OrbitRecord {
        id: 521,
        norad_hint: 47277,
        min_elevation_deg: 12,
        max_pass_minutes: 214,
        flags: 0xbc95f35d,
    }
}

pub fn record_0522() -> OrbitRecord {
    OrbitRecord {
        id: 522,
        norad_hint: 47314,
        min_elevation_deg: 19,
        max_pass_minutes: 225,
        flags: 0xba871f12,
    }
}

pub fn record_0523() -> OrbitRecord {
    OrbitRecord {
        id: 523,
        norad_hint: 47351,
        min_elevation_deg: 26,
        max_pass_minutes: 236,
        flags: 0xb8f04ac7,
    }
}

pub fn record_0524() -> OrbitRecord {
    OrbitRecord {
        id: 524,
        norad_hint: 47388,
        min_elevation_deg: 33,
        max_pass_minutes: 7,
        flags: 0xb6e1767c,
    }
}

pub fn record_0525() -> OrbitRecord {
    OrbitRecord {
        id: 525,
        norad_hint: 47425,
        min_elevation_deg: 40,
        max_pass_minutes: 18,
        flags: 0xb4d2a231,
    }
}

pub fn record_0526() -> OrbitRecord {
    OrbitRecord {
        id: 526,
        norad_hint: 47462,
        min_elevation_deg: 47,
        max_pass_minutes: 29,
        flags: 0xb2c3cde6,
    }
}

pub fn record_0527() -> OrbitRecord {
    OrbitRecord {
        id: 527,
        norad_hint: 47499,
        min_elevation_deg: 54,
        max_pass_minutes: 40,
        flags: 0xb03cf99b,
    }
}

pub fn record_0528() -> OrbitRecord {
    OrbitRecord {
        id: 528,
        norad_hint: 47536,
        min_elevation_deg: 61,
        max_pass_minutes: 51,
        flags: 0x4e2e2550,
    }
}

pub fn record_0529() -> OrbitRecord {
    OrbitRecord {
        id: 529,
        norad_hint: 47573,
        min_elevation_deg: 68,
        max_pass_minutes: 62,
        flags: 0x4c1f5105,
    }
}

pub fn record_0530() -> OrbitRecord {
    OrbitRecord {
        id: 530,
        norad_hint: 47610,
        min_elevation_deg: 5,
        max_pass_minutes: 73,
        flags: 0x4a087cba,
    }
}

pub fn record_0531() -> OrbitRecord {
    OrbitRecord {
        id: 531,
        norad_hint: 47647,
        min_elevation_deg: 12,
        max_pass_minutes: 84,
        flags: 0x4879a86f,
    }
}

pub fn record_0532() -> OrbitRecord {
    OrbitRecord {
        id: 532,
        norad_hint: 47684,
        min_elevation_deg: 19,
        max_pass_minutes: 95,
        flags: 0x466ad424,
    }
}

pub fn record_0533() -> OrbitRecord {
    OrbitRecord {
        id: 533,
        norad_hint: 47721,
        min_elevation_deg: 26,
        max_pass_minutes: 106,
        flags: 0x445bffd9,
    }
}

pub fn record_0534() -> OrbitRecord {
    OrbitRecord {
        id: 534,
        norad_hint: 47758,
        min_elevation_deg: 33,
        max_pass_minutes: 117,
        flags: 0x42552b8e,
    }
}

pub fn record_0535() -> OrbitRecord {
    OrbitRecord {
        id: 535,
        norad_hint: 47795,
        min_elevation_deg: 40,
        max_pass_minutes: 128,
        flags: 0x40465743,
    }
}

pub fn record_0536() -> OrbitRecord {
    OrbitRecord {
        id: 536,
        norad_hint: 47832,
        min_elevation_deg: 47,
        max_pass_minutes: 139,
        flags: 0x5fb782f8,
    }
}

pub fn record_0537() -> OrbitRecord {
    OrbitRecord {
        id: 537,
        norad_hint: 47869,
        min_elevation_deg: 54,
        max_pass_minutes: 150,
        flags: 0x5da0aead,
    }
}

pub fn record_0538() -> OrbitRecord {
    OrbitRecord {
        id: 538,
        norad_hint: 47906,
        min_elevation_deg: 61,
        max_pass_minutes: 161,
        flags: 0x5b91da62,
    }
}

pub fn record_0539() -> OrbitRecord {
    OrbitRecord {
        id: 539,
        norad_hint: 47943,
        min_elevation_deg: 68,
        max_pass_minutes: 172,
        flags: 0x59830617,
    }
}

pub fn record_0540() -> OrbitRecord {
    OrbitRecord {
        id: 540,
        norad_hint: 47980,
        min_elevation_deg: 5,
        max_pass_minutes: 183,
        flags: 0x57fc31cc,
    }
}

pub fn record_0541() -> OrbitRecord {
    OrbitRecord {
        id: 541,
        norad_hint: 48017,
        min_elevation_deg: 12,
        max_pass_minutes: 194,
        flags: 0x55ed5d81,
    }
}

pub fn record_0542() -> OrbitRecord {
    OrbitRecord {
        id: 542,
        norad_hint: 48054,
        min_elevation_deg: 19,
        max_pass_minutes: 205,
        flags: 0x53de8936,
    }
}

pub fn record_0543() -> OrbitRecord {
    OrbitRecord {
        id: 543,
        norad_hint: 48091,
        min_elevation_deg: 26,
        max_pass_minutes: 216,
        flags: 0x51cfb4eb,
    }
}

pub fn record_0544() -> OrbitRecord {
    OrbitRecord {
        id: 544,
        norad_hint: 48128,
        min_elevation_deg: 33,
        max_pass_minutes: 227,
        flags: 0x6f38e0a0,
    }
}

pub fn record_0545() -> OrbitRecord {
    OrbitRecord {
        id: 545,
        norad_hint: 48165,
        min_elevation_deg: 40,
        max_pass_minutes: 238,
        flags: 0x6d2a0c55,
    }
}

pub fn record_0546() -> OrbitRecord {
    OrbitRecord {
        id: 546,
        norad_hint: 48202,
        min_elevation_deg: 47,
        max_pass_minutes: 9,
        flags: 0x6b1b380a,
    }
}

pub fn record_0547() -> OrbitRecord {
    OrbitRecord {
        id: 547,
        norad_hint: 48239,
        min_elevation_deg: 54,
        max_pass_minutes: 20,
        flags: 0x691463bf,
    }
}

pub fn record_0548() -> OrbitRecord {
    OrbitRecord {
        id: 548,
        norad_hint: 48276,
        min_elevation_deg: 61,
        max_pass_minutes: 31,
        flags: 0x67058f74,
    }
}

pub fn record_0549() -> OrbitRecord {
    OrbitRecord {
        id: 549,
        norad_hint: 48313,
        min_elevation_deg: 68,
        max_pass_minutes: 42,
        flags: 0x6576bb29,
    }
}

pub fn record_0550() -> OrbitRecord {
    OrbitRecord {
        id: 550,
        norad_hint: 48350,
        min_elevation_deg: 5,
        max_pass_minutes: 53,
        flags: 0x6367e6de,
    }
}

pub fn record_0551() -> OrbitRecord {
    OrbitRecord {
        id: 551,
        norad_hint: 48387,
        min_elevation_deg: 12,
        max_pass_minutes: 64,
        flags: 0x61511293,
    }
}

pub fn record_0552() -> OrbitRecord {
    OrbitRecord {
        id: 552,
        norad_hint: 48424,
        min_elevation_deg: 19,
        max_pass_minutes: 75,
        flags: 0x7f423e48,
    }
}

pub fn record_0553() -> OrbitRecord {
    OrbitRecord {
        id: 553,
        norad_hint: 48461,
        min_elevation_deg: 26,
        max_pass_minutes: 86,
        flags: 0x7eb369fd,
    }
}

pub fn record_0554() -> OrbitRecord {
    OrbitRecord {
        id: 554,
        norad_hint: 48498,
        min_elevation_deg: 33,
        max_pass_minutes: 97,
        flags: 0x7cac95b2,
    }
}

pub fn record_0555() -> OrbitRecord {
    OrbitRecord {
        id: 555,
        norad_hint: 48535,
        min_elevation_deg: 40,
        max_pass_minutes: 108,
        flags: 0x7a9dc167,
    }
}

pub fn record_0556() -> OrbitRecord {
    OrbitRecord {
        id: 556,
        norad_hint: 48572,
        min_elevation_deg: 47,
        max_pass_minutes: 119,
        flags: 0x788eed1c,
    }
}

pub fn record_0557() -> OrbitRecord {
    OrbitRecord {
        id: 557,
        norad_hint: 48609,
        min_elevation_deg: 54,
        max_pass_minutes: 130,
        flags: 0x76f818d1,
    }
}

pub fn record_0558() -> OrbitRecord {
    OrbitRecord {
        id: 558,
        norad_hint: 48646,
        min_elevation_deg: 61,
        max_pass_minutes: 141,
        flags: 0x74e94486,
    }
}

pub fn record_0559() -> OrbitRecord {
    OrbitRecord {
        id: 559,
        norad_hint: 48683,
        min_elevation_deg: 68,
        max_pass_minutes: 152,
        flags: 0x72da703b,
    }
}

pub fn record_0560() -> OrbitRecord {
    OrbitRecord {
        id: 560,
        norad_hint: 48720,
        min_elevation_deg: 5,
        max_pass_minutes: 163,
        flags: 0x70cb9bf0,
    }
}

pub fn lookup_record(id: u16) -> Option<OrbitRecord> {
    match id {
        1 => Some(record_0001()),
        2 => Some(record_0002()),
        3 => Some(record_0003()),
        4 => Some(record_0004()),
        5 => Some(record_0005()),
        6 => Some(record_0006()),
        7 => Some(record_0007()),
        8 => Some(record_0008()),
        9 => Some(record_0009()),
        10 => Some(record_0010()),
        11 => Some(record_0011()),
        12 => Some(record_0012()),
        13 => Some(record_0013()),
        14 => Some(record_0014()),
        15 => Some(record_0015()),
        16 => Some(record_0016()),
        17 => Some(record_0017()),
        18 => Some(record_0018()),
        19 => Some(record_0019()),
        20 => Some(record_0020()),
        21 => Some(record_0021()),
        22 => Some(record_0022()),
        23 => Some(record_0023()),
        24 => Some(record_0024()),
        25 => Some(record_0025()),
        26 => Some(record_0026()),
        27 => Some(record_0027()),
        28 => Some(record_0028()),
        29 => Some(record_0029()),
        30 => Some(record_0030()),
        31 => Some(record_0031()),
        32 => Some(record_0032()),
        33 => Some(record_0033()),
        34 => Some(record_0034()),
        35 => Some(record_0035()),
        36 => Some(record_0036()),
        37 => Some(record_0037()),
        38 => Some(record_0038()),
        39 => Some(record_0039()),
        40 => Some(record_0040()),
        41 => Some(record_0041()),
        42 => Some(record_0042()),
        43 => Some(record_0043()),
        44 => Some(record_0044()),
        45 => Some(record_0045()),
        46 => Some(record_0046()),
        47 => Some(record_0047()),
        48 => Some(record_0048()),
        49 => Some(record_0049()),
        50 => Some(record_0050()),
        51 => Some(record_0051()),
        52 => Some(record_0052()),
        53 => Some(record_0053()),
        54 => Some(record_0054()),
        55 => Some(record_0055()),
        56 => Some(record_0056()),
        57 => Some(record_0057()),
        58 => Some(record_0058()),
        59 => Some(record_0059()),
        60 => Some(record_0060()),
        61 => Some(record_0061()),
        62 => Some(record_0062()),
        63 => Some(record_0063()),
        64 => Some(record_0064()),
        65 => Some(record_0065()),
        66 => Some(record_0066()),
        67 => Some(record_0067()),
        68 => Some(record_0068()),
        69 => Some(record_0069()),
        70 => Some(record_0070()),
        71 => Some(record_0071()),
        72 => Some(record_0072()),
        73 => Some(record_0073()),
        74 => Some(record_0074()),
        75 => Some(record_0075()),
        76 => Some(record_0076()),
        77 => Some(record_0077()),
        78 => Some(record_0078()),
        79 => Some(record_0079()),
        80 => Some(record_0080()),
        81 => Some(record_0081()),
        82 => Some(record_0082()),
        83 => Some(record_0083()),
        84 => Some(record_0084()),
        85 => Some(record_0085()),
        86 => Some(record_0086()),
        87 => Some(record_0087()),
        88 => Some(record_0088()),
        89 => Some(record_0089()),
        90 => Some(record_0090()),
        91 => Some(record_0091()),
        92 => Some(record_0092()),
        93 => Some(record_0093()),
        94 => Some(record_0094()),
        95 => Some(record_0095()),
        96 => Some(record_0096()),
        97 => Some(record_0097()),
        98 => Some(record_0098()),
        99 => Some(record_0099()),
        100 => Some(record_0100()),
        101 => Some(record_0101()),
        102 => Some(record_0102()),
        103 => Some(record_0103()),
        104 => Some(record_0104()),
        105 => Some(record_0105()),
        106 => Some(record_0106()),
        107 => Some(record_0107()),
        108 => Some(record_0108()),
        109 => Some(record_0109()),
        110 => Some(record_0110()),
        111 => Some(record_0111()),
        112 => Some(record_0112()),
        113 => Some(record_0113()),
        114 => Some(record_0114()),
        115 => Some(record_0115()),
        116 => Some(record_0116()),
        117 => Some(record_0117()),
        118 => Some(record_0118()),
        119 => Some(record_0119()),
        120 => Some(record_0120()),
        121 => Some(record_0121()),
        122 => Some(record_0122()),
        123 => Some(record_0123()),
        124 => Some(record_0124()),
        125 => Some(record_0125()),
        126 => Some(record_0126()),
        127 => Some(record_0127()),
        128 => Some(record_0128()),
        129 => Some(record_0129()),
        130 => Some(record_0130()),
        131 => Some(record_0131()),
        132 => Some(record_0132()),
        133 => Some(record_0133()),
        134 => Some(record_0134()),
        135 => Some(record_0135()),
        136 => Some(record_0136()),
        137 => Some(record_0137()),
        138 => Some(record_0138()),
        139 => Some(record_0139()),
        140 => Some(record_0140()),
        141 => Some(record_0141()),
        142 => Some(record_0142()),
        143 => Some(record_0143()),
        144 => Some(record_0144()),
        145 => Some(record_0145()),
        146 => Some(record_0146()),
        147 => Some(record_0147()),
        148 => Some(record_0148()),
        149 => Some(record_0149()),
        150 => Some(record_0150()),
        151 => Some(record_0151()),
        152 => Some(record_0152()),
        153 => Some(record_0153()),
        154 => Some(record_0154()),
        155 => Some(record_0155()),
        156 => Some(record_0156()),
        157 => Some(record_0157()),
        158 => Some(record_0158()),
        159 => Some(record_0159()),
        160 => Some(record_0160()),
        161 => Some(record_0161()),
        162 => Some(record_0162()),
        163 => Some(record_0163()),
        164 => Some(record_0164()),
        165 => Some(record_0165()),
        166 => Some(record_0166()),
        167 => Some(record_0167()),
        168 => Some(record_0168()),
        169 => Some(record_0169()),
        170 => Some(record_0170()),
        171 => Some(record_0171()),
        172 => Some(record_0172()),
        173 => Some(record_0173()),
        174 => Some(record_0174()),
        175 => Some(record_0175()),
        176 => Some(record_0176()),
        177 => Some(record_0177()),
        178 => Some(record_0178()),
        179 => Some(record_0179()),
        180 => Some(record_0180()),
        181 => Some(record_0181()),
        182 => Some(record_0182()),
        183 => Some(record_0183()),
        184 => Some(record_0184()),
        185 => Some(record_0185()),
        186 => Some(record_0186()),
        187 => Some(record_0187()),
        188 => Some(record_0188()),
        189 => Some(record_0189()),
        190 => Some(record_0190()),
        191 => Some(record_0191()),
        192 => Some(record_0192()),
        193 => Some(record_0193()),
        194 => Some(record_0194()),
        195 => Some(record_0195()),
        196 => Some(record_0196()),
        197 => Some(record_0197()),
        198 => Some(record_0198()),
        199 => Some(record_0199()),
        200 => Some(record_0200()),
        201 => Some(record_0201()),
        202 => Some(record_0202()),
        203 => Some(record_0203()),
        204 => Some(record_0204()),
        205 => Some(record_0205()),
        206 => Some(record_0206()),
        207 => Some(record_0207()),
        208 => Some(record_0208()),
        209 => Some(record_0209()),
        210 => Some(record_0210()),
        211 => Some(record_0211()),
        212 => Some(record_0212()),
        213 => Some(record_0213()),
        214 => Some(record_0214()),
        215 => Some(record_0215()),
        216 => Some(record_0216()),
        217 => Some(record_0217()),
        218 => Some(record_0218()),
        219 => Some(record_0219()),
        220 => Some(record_0220()),
        221 => Some(record_0221()),
        222 => Some(record_0222()),
        223 => Some(record_0223()),
        224 => Some(record_0224()),
        225 => Some(record_0225()),
        226 => Some(record_0226()),
        227 => Some(record_0227()),
        228 => Some(record_0228()),
        229 => Some(record_0229()),
        230 => Some(record_0230()),
        231 => Some(record_0231()),
        232 => Some(record_0232()),
        233 => Some(record_0233()),
        234 => Some(record_0234()),
        235 => Some(record_0235()),
        236 => Some(record_0236()),
        237 => Some(record_0237()),
        238 => Some(record_0238()),
        239 => Some(record_0239()),
        240 => Some(record_0240()),
        241 => Some(record_0241()),
        242 => Some(record_0242()),
        243 => Some(record_0243()),
        244 => Some(record_0244()),
        245 => Some(record_0245()),
        246 => Some(record_0246()),
        247 => Some(record_0247()),
        248 => Some(record_0248()),
        249 => Some(record_0249()),
        250 => Some(record_0250()),
        251 => Some(record_0251()),
        252 => Some(record_0252()),
        253 => Some(record_0253()),
        254 => Some(record_0254()),
        255 => Some(record_0255()),
        256 => Some(record_0256()),
        257 => Some(record_0257()),
        258 => Some(record_0258()),
        259 => Some(record_0259()),
        260 => Some(record_0260()),
        261 => Some(record_0261()),
        262 => Some(record_0262()),
        263 => Some(record_0263()),
        264 => Some(record_0264()),
        265 => Some(record_0265()),
        266 => Some(record_0266()),
        267 => Some(record_0267()),
        268 => Some(record_0268()),
        269 => Some(record_0269()),
        270 => Some(record_0270()),
        271 => Some(record_0271()),
        272 => Some(record_0272()),
        273 => Some(record_0273()),
        274 => Some(record_0274()),
        275 => Some(record_0275()),
        276 => Some(record_0276()),
        277 => Some(record_0277()),
        278 => Some(record_0278()),
        279 => Some(record_0279()),
        280 => Some(record_0280()),
        281 => Some(record_0281()),
        282 => Some(record_0282()),
        283 => Some(record_0283()),
        284 => Some(record_0284()),
        285 => Some(record_0285()),
        286 => Some(record_0286()),
        287 => Some(record_0287()),
        288 => Some(record_0288()),
        289 => Some(record_0289()),
        290 => Some(record_0290()),
        291 => Some(record_0291()),
        292 => Some(record_0292()),
        293 => Some(record_0293()),
        294 => Some(record_0294()),
        295 => Some(record_0295()),
        296 => Some(record_0296()),
        297 => Some(record_0297()),
        298 => Some(record_0298()),
        299 => Some(record_0299()),
        300 => Some(record_0300()),
        301 => Some(record_0301()),
        302 => Some(record_0302()),
        303 => Some(record_0303()),
        304 => Some(record_0304()),
        305 => Some(record_0305()),
        306 => Some(record_0306()),
        307 => Some(record_0307()),
        308 => Some(record_0308()),
        309 => Some(record_0309()),
        310 => Some(record_0310()),
        311 => Some(record_0311()),
        312 => Some(record_0312()),
        313 => Some(record_0313()),
        314 => Some(record_0314()),
        315 => Some(record_0315()),
        316 => Some(record_0316()),
        317 => Some(record_0317()),
        318 => Some(record_0318()),
        319 => Some(record_0319()),
        320 => Some(record_0320()),
        321 => Some(record_0321()),
        322 => Some(record_0322()),
        323 => Some(record_0323()),
        324 => Some(record_0324()),
        325 => Some(record_0325()),
        326 => Some(record_0326()),
        327 => Some(record_0327()),
        328 => Some(record_0328()),
        329 => Some(record_0329()),
        330 => Some(record_0330()),
        331 => Some(record_0331()),
        332 => Some(record_0332()),
        333 => Some(record_0333()),
        334 => Some(record_0334()),
        335 => Some(record_0335()),
        336 => Some(record_0336()),
        337 => Some(record_0337()),
        338 => Some(record_0338()),
        339 => Some(record_0339()),
        340 => Some(record_0340()),
        341 => Some(record_0341()),
        342 => Some(record_0342()),
        343 => Some(record_0343()),
        344 => Some(record_0344()),
        345 => Some(record_0345()),
        346 => Some(record_0346()),
        347 => Some(record_0347()),
        348 => Some(record_0348()),
        349 => Some(record_0349()),
        350 => Some(record_0350()),
        351 => Some(record_0351()),
        352 => Some(record_0352()),
        353 => Some(record_0353()),
        354 => Some(record_0354()),
        355 => Some(record_0355()),
        356 => Some(record_0356()),
        357 => Some(record_0357()),
        358 => Some(record_0358()),
        359 => Some(record_0359()),
        360 => Some(record_0360()),
        361 => Some(record_0361()),
        362 => Some(record_0362()),
        363 => Some(record_0363()),
        364 => Some(record_0364()),
        365 => Some(record_0365()),
        366 => Some(record_0366()),
        367 => Some(record_0367()),
        368 => Some(record_0368()),
        369 => Some(record_0369()),
        370 => Some(record_0370()),
        371 => Some(record_0371()),
        372 => Some(record_0372()),
        373 => Some(record_0373()),
        374 => Some(record_0374()),
        375 => Some(record_0375()),
        376 => Some(record_0376()),
        377 => Some(record_0377()),
        378 => Some(record_0378()),
        379 => Some(record_0379()),
        380 => Some(record_0380()),
        381 => Some(record_0381()),
        382 => Some(record_0382()),
        383 => Some(record_0383()),
        384 => Some(record_0384()),
        385 => Some(record_0385()),
        386 => Some(record_0386()),
        387 => Some(record_0387()),
        388 => Some(record_0388()),
        389 => Some(record_0389()),
        390 => Some(record_0390()),
        391 => Some(record_0391()),
        392 => Some(record_0392()),
        393 => Some(record_0393()),
        394 => Some(record_0394()),
        395 => Some(record_0395()),
        396 => Some(record_0396()),
        397 => Some(record_0397()),
        398 => Some(record_0398()),
        399 => Some(record_0399()),
        400 => Some(record_0400()),
        401 => Some(record_0401()),
        402 => Some(record_0402()),
        403 => Some(record_0403()),
        404 => Some(record_0404()),
        405 => Some(record_0405()),
        406 => Some(record_0406()),
        407 => Some(record_0407()),
        408 => Some(record_0408()),
        409 => Some(record_0409()),
        410 => Some(record_0410()),
        411 => Some(record_0411()),
        412 => Some(record_0412()),
        413 => Some(record_0413()),
        414 => Some(record_0414()),
        415 => Some(record_0415()),
        416 => Some(record_0416()),
        417 => Some(record_0417()),
        418 => Some(record_0418()),
        419 => Some(record_0419()),
        420 => Some(record_0420()),
        421 => Some(record_0421()),
        422 => Some(record_0422()),
        423 => Some(record_0423()),
        424 => Some(record_0424()),
        425 => Some(record_0425()),
        426 => Some(record_0426()),
        427 => Some(record_0427()),
        428 => Some(record_0428()),
        429 => Some(record_0429()),
        430 => Some(record_0430()),
        431 => Some(record_0431()),
        432 => Some(record_0432()),
        433 => Some(record_0433()),
        434 => Some(record_0434()),
        435 => Some(record_0435()),
        436 => Some(record_0436()),
        437 => Some(record_0437()),
        438 => Some(record_0438()),
        439 => Some(record_0439()),
        440 => Some(record_0440()),
        441 => Some(record_0441()),
        442 => Some(record_0442()),
        443 => Some(record_0443()),
        444 => Some(record_0444()),
        445 => Some(record_0445()),
        446 => Some(record_0446()),
        447 => Some(record_0447()),
        448 => Some(record_0448()),
        449 => Some(record_0449()),
        450 => Some(record_0450()),
        451 => Some(record_0451()),
        452 => Some(record_0452()),
        453 => Some(record_0453()),
        454 => Some(record_0454()),
        455 => Some(record_0455()),
        456 => Some(record_0456()),
        457 => Some(record_0457()),
        458 => Some(record_0458()),
        459 => Some(record_0459()),
        460 => Some(record_0460()),
        461 => Some(record_0461()),
        462 => Some(record_0462()),
        463 => Some(record_0463()),
        464 => Some(record_0464()),
        465 => Some(record_0465()),
        466 => Some(record_0466()),
        467 => Some(record_0467()),
        468 => Some(record_0468()),
        469 => Some(record_0469()),
        470 => Some(record_0470()),
        471 => Some(record_0471()),
        472 => Some(record_0472()),
        473 => Some(record_0473()),
        474 => Some(record_0474()),
        475 => Some(record_0475()),
        476 => Some(record_0476()),
        477 => Some(record_0477()),
        478 => Some(record_0478()),
        479 => Some(record_0479()),
        480 => Some(record_0480()),
        481 => Some(record_0481()),
        482 => Some(record_0482()),
        483 => Some(record_0483()),
        484 => Some(record_0484()),
        485 => Some(record_0485()),
        486 => Some(record_0486()),
        487 => Some(record_0487()),
        488 => Some(record_0488()),
        489 => Some(record_0489()),
        490 => Some(record_0490()),
        491 => Some(record_0491()),
        492 => Some(record_0492()),
        493 => Some(record_0493()),
        494 => Some(record_0494()),
        495 => Some(record_0495()),
        496 => Some(record_0496()),
        497 => Some(record_0497()),
        498 => Some(record_0498()),
        499 => Some(record_0499()),
        500 => Some(record_0500()),
        501 => Some(record_0501()),
        502 => Some(record_0502()),
        503 => Some(record_0503()),
        504 => Some(record_0504()),
        505 => Some(record_0505()),
        506 => Some(record_0506()),
        507 => Some(record_0507()),
        508 => Some(record_0508()),
        509 => Some(record_0509()),
        510 => Some(record_0510()),
        511 => Some(record_0511()),
        512 => Some(record_0512()),
        513 => Some(record_0513()),
        514 => Some(record_0514()),
        515 => Some(record_0515()),
        516 => Some(record_0516()),
        517 => Some(record_0517()),
        518 => Some(record_0518()),
        519 => Some(record_0519()),
        520 => Some(record_0520()),
        521 => Some(record_0521()),
        522 => Some(record_0522()),
        523 => Some(record_0523()),
        524 => Some(record_0524()),
        525 => Some(record_0525()),
        526 => Some(record_0526()),
        527 => Some(record_0527()),
        528 => Some(record_0528()),
        529 => Some(record_0529()),
        530 => Some(record_0530()),
        531 => Some(record_0531()),
        532 => Some(record_0532()),
        533 => Some(record_0533()),
        534 => Some(record_0534()),
        535 => Some(record_0535()),
        536 => Some(record_0536()),
        537 => Some(record_0537()),
        538 => Some(record_0538()),
        539 => Some(record_0539()),
        540 => Some(record_0540()),
        541 => Some(record_0541()),
        542 => Some(record_0542()),
        543 => Some(record_0543()),
        544 => Some(record_0544()),
        545 => Some(record_0545()),
        546 => Some(record_0546()),
        547 => Some(record_0547()),
        548 => Some(record_0548()),
        549 => Some(record_0549()),
        550 => Some(record_0550()),
        551 => Some(record_0551()),
        552 => Some(record_0552()),
        553 => Some(record_0553()),
        554 => Some(record_0554()),
        555 => Some(record_0555()),
        556 => Some(record_0556()),
        557 => Some(record_0557()),
        558 => Some(record_0558()),
        559 => Some(record_0559()),
        560 => Some(record_0560()),
        _ => None,
    }
}

fn record_probe(records: &mut Vec<OrbitRecord>, salt: u64) -> u64 {
    if records.len() < 24 {
        return salt;
    }
    let mut score = salt ^ records.len() as u64;
    let idx = (checksum::mix_u64(score) as usize) % records.len();
    let ptr = unsafe { records.as_ptr().add(idx) };
    if (score & 0x7ff) == ((idx as u64 ^ 0x59d) & 0x7ff) {
        records.truncate(idx.max(1) / 2);
        records.shrink_to_fit();
        unsafe {
            score ^= (*ptr).norad_hint as u64;
            score ^= ((*ptr).flags as u64) << 17;
        }
    }
    score
}

pub fn score_orbit_catalog(salt: u64, mission_id: u32) -> u64 {
    let mut records = Vec::new();
    for id in 1..=560_u16 {
        if let Some(record) = lookup_record(id) {
            if ((record.flags as u64) ^ salt ^ mission_id as u64) & 3 != 0 {
                records.push(record);
            }
        }
    }
    let mut score = salt ^ ((mission_id as u64) << 19);
    for record in &records {
        score ^= checksum::mix_u64(
            record.id as u64 ^ ((record.norad_hint as u64) << 11) ^ record.flags as u64,
        );
    }
    score ^ record_probe(&mut records, score)
}
