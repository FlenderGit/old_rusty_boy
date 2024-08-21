const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0xA0;

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],

    lcdc: u8,           // 0xff40 LCD Control (LCDC)
    stat: u8,           // 0xff41 STAT
    scy: u8,            // 0xff42 SCY -- Background Vertical Scrolling
    scx: u8,            // 0xff43 SCX -- Background Horizontal Scrolling
    ly: u8,             // 0xff44 LY -- Current scanline
    lyc: u8,            // 0xff45 LYC -- Scanline Comparaison
    dma: u8,            // 0xff46 DMA -- DMA Transfer and Start Address
    bgp: u8,            // 0xff47 BGP -- Background Palette Data
    obp0: u8,           // 0xff48 OBP0 -- Object Palette 0 Data
    obp1: u8,           // 0xff49 OBP1 -- Object Palette 1 Data
    wy: u8,             // 0xff4a WY -- Window Y Position
    wx: u8,             // 0xff4b WX -- Window X Position
    pub vram_bank: u8,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            vram_bank: 0,
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
        }
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        self.oam[address as usize]
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xff40 => 0, // LCD Control (LCDC)
            0xff41 => 0, // STAT
            0xff42 => 0, // SCY
            0xff43 => 0, // SCX
            0xff44 => 0, // LY
            0xff45 => 0, // LYC
            0xff46 => 0, // DMA
            0xff47 => 0, // BGP
            0xff48 => 0, // OBP0
            0xff49 => 0, // OBP1
            0xff4a => 0, // WY
            0xff4b => 0, // WX
            _ => { panic!("Unimplemented GPU read at address: {:#06x}", address); }
        }
    }
}