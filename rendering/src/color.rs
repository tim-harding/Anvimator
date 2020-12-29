pub struct HexRgba(u32);

impl From<u32> for HexRgba {
    fn from(n: u32) -> Self {
        HexRgba(n)
    }
}

impl From<HexRgba> for u32 {
    fn from(color: HexRgba) -> u32 {
        color.0
    }
}

impl From<HexRgba> for wgpu::Color {
    fn from(hex: HexRgba) -> Self {
        let parts: [f64; 4] = hex.into();
        Self {
            r: parts[0],
            g: parts[1],
            b: parts[2],
            a: parts[3],
        }
    }
}

impl From<HexRgba> for [f32; 4] {
    fn from(hex: HexRgba) -> [f32; 4] {
        let hex = hex.into_hex();
        [
            gamma_f32(hex[0]),
            gamma_f32(hex[1]),
            gamma_f32(hex[2]),
            gamma_f32(hex[3]),
        ]
    }
}

impl From<HexRgba> for [f64; 4] {
    fn from(hex: HexRgba) -> [f64; 4] {
        let hex = hex.into_hex();
        [
            gamma_f64(hex[0]),
            gamma_f64(hex[1]),
            gamma_f64(hex[2]),
            gamma_f64(hex[3]),
        ]
    }
}

impl HexRgba {
    const fn into_hex(self) -> [u8; 4] {
        let n: u32 = self.0;
        let r = (n >> 24) as u8;
        let g = (n >> 16) as u8;
        let b = (n >> 8) as u8;
        let a = n as u8;
        [r, g, b, a]
    }
}

fn gamma_f64(n: u8) -> f64 {
    let f = n as f64 / 255.0;
    f.powf(2.2)
}

fn gamma_f32(n: u8) -> f32 {
    let f = n as f32 / 255.0;
    f.powf(2.2)
}

#[repr(u32)]
#[allow(dead_code)]
pub enum Nord {
    PolarNight0 = 0x2e3440ff,
    PolarNight1 = 0x3b4252ff,
    PolarNight2 = 0x434c5eff,
    PolarNight3 = 0x4c566aff,
    SnowStorm0 = 0xd8dee9ff,
    SnowStorm1 = 0xe5e9f0ff,
    SnowStorm2 = 0xeceff4ff,
    Frost0 = 0x8fbcbbff,
    Frost1 = 0x88c0d0ff,
    Frost2 = 0x81a1c1ff,
    Frost3 = 0x5e81acff,
    Aurora0 = 0xbf616aff,
    Aurora1 = 0xd08770ff,
    Aurora2 = 0xebcb8bff,
    Aurora3 = 0xa3be8cff,
    Aurora4 = 0xb48eadff,
}

impl Nord {
    pub fn hex(self) -> HexRgba {
        self.into()
    }
}

impl From<Nord> for HexRgba {
    fn from(nord: Nord) -> Self {
        Self(nord as _)
    }
}