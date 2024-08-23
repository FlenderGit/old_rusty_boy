use log::info;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0xA0;

#[derive(PartialEq)]
enum Mode {
    HBlank, // 204 cycles : termine le rendu d'une ligne horizontale et attend la prochaine ligne à dessiner
    VBlank, //4560 cycles (10 lignes * 456 cycles/ligne) : La PPU a fini de dessiner toutes les lignes, et il est temps d'envoyer l'image au framebuffer (ce qui provoque une interruption VBlank)
    OAM,    // 80 cycles : La PPU lit les sprites de la mémoire OAM
    DRAWING, // 172 cycles : dessine les pixels de la ligne actuelle
}

pub struct GPU {
    mode: Mode,
    clock: u32,
    pub interrupt: u8,

    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],

    lcdc: u8, // 0xff40 LCD Control (LCDC)
    stat: u8, // 0xff41 STAT
    scy: u8,  // 0xff42 SCY -- Background Vertical Scrolling
    scx: u8,  // 0xff43 SCX -- Background Horizontal Scrolling
    ly: u8,   // 0xff44 LY -- Current scanline
    lyc: u8,  // 0xff45 LYC -- Scanline Comparaison
    dma: u8,  // 0xff46 DMA -- DMA Transfer and Start Address
    bgp: u8,  // 0xff47 BGP -- Background Palette Data
    obp0: u8, // 0xff48 OBP0 -- Object Palette 0 Data
    obp1: u8, // 0xff49 OBP1 -- Object Palette 1 Data
    wy: u8,   // 0xff4a WY -- Window Y Position
    wx: u8,   // 0xff4b WX -- Window X Position
    pub vram_bank: u8,

    pub screen_data: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            mode: Mode::OAM,
            clock: 0,
            interrupt: 0,
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
            screen_data: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
        }
    }

    pub fn step(&mut self, cycles: u8, draw: bool) {
        self.clock += cycles as u32;

        match self.mode {
            Mode::HBlank => {
                if self.clock >= 204 {
                    self.clock -= 204;
                    self.ly += 1;
                    self.check_interrupt_lyc();
    
                    if self.ly == SCREEN_HEIGHT as u8 {
                        self.mode = Mode::VBlank;
                        self.interrupt = 0x01;
                        // Début de VBlank, appeler la routine d'interruption ici si nécessaire
                    } else {
                        self.mode = Mode::OAM;
                    }
                }
            }
            Mode::VBlank => {
                if self.clock >= 456 {
                    self.clock -= 456;
                    self.ly += 1;
    
                    if self.ly > 153 {
                        self.ly = 0;
                        self.mode = Mode::OAM;
                    }
                }
            }
            Mode::OAM => {
                if self.clock >= 80 {
                    self.clock -= 80;
                    self.mode = Mode::DRAWING;
                }
            }
            Mode::DRAWING => {
                if self.clock >= 172 {
                    self.clock -= 172;
                    self.mode = Mode::HBlank;    
                    if draw {
                        self.render_scanline();
                        self.print_line();
                    }
                }
            }
        }

        if self.ly == self.lyc {
            self.stat |= 0x04;
        } else {
            self.stat &= !0x04;
        }

        if self.ly >= 144 {
            self.mode = Mode::VBlank;
        }

        if self.mode == Mode::VBlank && self.ly == 153 {
            self.ly = 0;
            self.mode = Mode::OAM;
        }
    }

    pub fn render_scanline(&mut self) {
        self.draw_tiles();
    }

    pub fn draw_tiles(&mut self) {
        let win_on = self.lcdc & 0x20 == 0x20;
        let bg_on = self.lcdc & 0x01 == 0x01;

        let bg_y = self.ly.wrapping_add(self.scy);
        let bg_tile_y = bg_y >> 3;
        let win_y = self.ly.wrapping_add(self.wy);
        let win_tile_y = win_y >> 3 & 0x31;

        //info!("LY: {}, BG_TILE_Y: {}", self.ly, bg_tile_y);
        for x in 0..SCREEN_WIDTH {
            let win_x = x as u8 - self.wx + 7;
            let bg_x = x as u8 + self.scx;

            let (tilemap_addr, tile_y, tile_x, pixel_y, pixel_x) = if win_y >= 0 && win_x >= 0 && false {
                (
                    0x9800,
                    win_tile_y,
                    win_x >> 3,
                    win_y & 0x07,
                    win_x & 0x07,
                )
            } else {
                (
                    0x9800,
                    bg_tile_y,
                    bg_x >> 3,
                    bg_y & 0x07,
                    bg_x & 0x07,
                )
            };

            //info!("tilemap_addr: {:#06x}, tile_y: {:#06x}, tile_x: {:#06x}, pixel_y: {:#06x}, pixel_x: {:#06x}", tilemap_addr, tile_y, tile_x, pixel_y, pixel_x);
            let tile_addr = tilemap_addr + tile_y as u16 * 32 + tile_x as u16;
            //info!("tile_addr: {:#06x}", tile_addr);
            let tile_num = self.read_vram(tile_addr) as u16;
            //info!("tile_num: {:#06x}", tile_num);

            let tile_data_addr = if tilemap_addr == 0x9800 {
                0x8000 + tile_num * 16
            } else {
                0x8800 + ((tile_num as i8 as i16 + 128) * 16) as u16
            };

            let low_byte = self.read_vram(tile_data_addr + pixel_y as u16 * 2);
            let high_byte = self.read_vram(tile_data_addr + pixel_y as u16 * 2 + 1);

            let color_bit = 7 - pixel_x;
            let color_id = ((high_byte >> color_bit) & 0x1) << 1 | ((low_byte >> color_bit) & 0x1);

            let color = match color_id {
                0 => self.bgp & 0x03,
                1 => (self.bgp >> 2) & 0x03,
                2 => (self.bgp >> 4) & 0x03,
                3 => (self.bgp >> 6) & 0x03,
                _ => 0,
            };

            self.set_color(x, color);
            //self.screen_data[(self.ly as usize * SCREEN_WIDTH + x) as usize] = color;
        }
    }

    pub fn set_color(&mut self, x: usize, color: u8) {
        self.screen_data[self.ly as usize * SCREEN_WIDTH * 3 + x * 3] = color;
        self.screen_data[self.ly as usize * SCREEN_WIDTH * 3 + x * 3 + 1] = color;
        self.screen_data[self.ly as usize * SCREEN_WIDTH * 3 + x * 3 + 2] = color;
    }

    fn print_line(&self) {
        for x in 0..SCREEN_WIDTH {
            match self.screen_data[self.ly as usize * SCREEN_WIDTH * 3 + x * 3] {
                3 => print!("  "),
                2 => print!("░░"),
                1 => print!("▒▒"),
                0 => print!("▓▓"),
                _ => print!(" "),
            }
        }
        println!();
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        self.vram[address as usize & 0x1FFF]
    }

    fn check_interrupt_lyc(&mut self) {
        if self.ly == self.lyc {
            self.stat |= 0x04;
        }
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        self.oam[address as usize]
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xff40 => self.lcdc, // LCD Control (LCDC)
            0xff41 => self.stat, // STAT
            0xff42 => self.scy,  // SCY
            0xff43 => self.scx,  // SCX
            0xff44 => self.ly,   // LY
            0xff45 => self.lyc,  // LYC
            0xff46 => self.dma,  // DMA
            0xff47 => self.bgp,  // BGP
            0xff48 => self.obp0, // OBP0
            0xff49 => self.obp1, // OBP1
            0xff4a => self.wy,   // WY
            0xff4b => self.wx,   // WX
            _ => {
                panic!("Unimplemented GPU read at address: {:#06x}", address);
            }
        }
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {
        self.vram[address as usize] = value;
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        self.oam[address as usize] = value;
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xff40 => self.lcdc = value, // LCD Control (LCDC)
            0xff41 => self.stat = value, // STAT
            0xff42 => self.scy = value,  // SCY
            0xff43 => self.scx = value,  // SCX
            0xff44 => self.ly = value,   // LY
            0xff45 => {
                self.lyc = value;
                self.check_interrupt_lyc();
            } // LYC
            0xff46 => self.dma = value,  // DMA
            0xff47 => self.bgp = value,  // BGP
            0xff48 => self.obp0 = value, // OBP0
            0xff49 => self.obp1 = value, // OBP1
            0xff4a => self.wy = value,   // WY
            0xff4b => self.wx = value,   // WX
            _ => {
                panic!("Unimplemented GPU write at address: {:#06x}", address);
            }
        }
    }
}
