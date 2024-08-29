use crate::cpu::CPU;
use crate::header::Header;
use crate::keypad::{Key, KeyEvent};
use crate::time;

const FRAME_TIME: f64 = 1.0 / 30.0;
const CYCLES_PER_SECOND: u32 = 4_194_304;
const CYCLES_PER_FRAME: u32 = CYCLES_PER_SECOND / 60;

pub struct Gameboy {
    pub cpu: CPU,
    header: Option<Header>,
    
    render_callback: Box<dyn FnMut(&[u8; 160 * 144 * 3]) + 'static>,
    input_callback: Box<dyn FnMut() -> Option<KeyEvent> + 'static>,

    pub previous_time: f64,
    pub lag: f64,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            cpu: CPU::new(),
            header: None,
            render_callback: Box::new(|_| {
                panic!("No render callback set!");
            }),
            input_callback: Box::new(|| {
                panic!("No input callback set!");
            }),

            previous_time: 0.0,
            lag: 0.0,
        }
    }

    pub fn load_rom_from_filename(&mut self, filename: &str ) {

        if !std::path::Path::new(&filename).exists() {
            panic!("File not found: {}", filename);
        }

        let rom = std::fs::read(filename).unwrap();
        self.load_rom(rom);
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.cpu.memory.load_rom(&rom);
        self.header = Some(Header::load_rom(&rom));
    }

    pub fn get_screen_data(&self) -> &[u8; 160 * 144 * 3] {
        return &self.cpu.memory.gpu.screen_data;
    }

    pub fn set_render_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&[u8; 69120]) + 'static,
    {
        self.render_callback = Box::new(callback);
    }

    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut() -> Option<KeyEvent> + 'static,
    {
        self.input_callback = Box::new(callback);
    }

    fn game_loop(&mut self) {
        let current_time = time::now();
        let elapsed = current_time - self.previous_time;
        self.previous_time = current_time;
        self.lag += elapsed;
        let mut cycles = 0;
        while self.lag >= FRAME_TIME {
            self.update();
            self.lag -= FRAME_TIME;
            cycles += 1;
        }
        self.render();
        println!("FPS: {:.2} Cycles: {:.2} Lag: {:.2} keypad {:#04x}", 1.0 / elapsed, cycles, self.lag, self.cpu.memory.keypad.read());
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run(&mut self) {
        self.previous_time = time::now();
        loop {
            self.game_loop();
            let frame_duration = time::now() - self.previous_time;
            if frame_duration < FRAME_TIME {
                //panic!("Sleeping for: {}", FRAME_TIME - frame_duration);
                std::thread::sleep(std::time::Duration::from_secs_f64(
                    FRAME_TIME - frame_duration,
                ));
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run(&mut self) {
        use wasm_bindgen::JsCast;

        self.previous_time = time::now();
        use std::borrow::BorrowMut;
        let self_ptr: *mut Gameboy = self;

        let window = web_sys::window().unwrap();

        let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move || {
            let self_ref: &mut Gameboy = unsafe { &mut *self_ptr };
            self_ref.game_loop();
            //window.request_animation_frame(closure.as_ref().unchecked_ref());
        }) as Box<dyn FnMut()>);

        window.request_animation_frame(closure.as_ref().unchecked_ref());
        closure.forget();
    }

    fn update(&mut self) {
        let mut cycles = 0;

        if let Some(key) = (self.input_callback)() {
            // Test if the key is pressed

            match key {
                KeyEvent::Press(key) => {
                    self.cpu.memory.keypad.press(key);
                }
                KeyEvent::Release(key) => {
                    self.cpu.memory.keypad.release(key);
                }
                
            }


        }

        while cycles < CYCLES_PER_FRAME {
            cycles += self.cpu.step(false) as u32;
        }
    }

    fn render(&mut self) {
        (self.render_callback)(&self.cpu.memory.gpu.screen_data);
    }

    pub fn run_debug(&mut self) {
        // 2F2A --> Intro
        // 6A6B --> Title screen
        // 650C
        // 2CF --> CFFB est remis (64D3)

        while self.cpu.registers.pc != 0x6a6b {
            self.cpu.step(false);
        }

        while self.cpu.registers.pc != 0xffb8 {
            self.cpu.step(false);
        }
        /* for _ in 0..160_000_000 {
            self.cpu.step(false);
        } */
        /* for _ in 0..1_000 {
            self.cpu.step(false);
            for _ in 0..500_000 {
                self.cpu.step(false);
            }
            //println!("Registers: {:?}", self.cpu.registers);
        } */


        for _ in 0..19_000_000 {
            self.cpu.step(false);
        }

        for _ in 0..1 {
            self.cpu.step_debug();
        }

        println!("Registers: {:?}", self.cpu.registers);
        println!("0x9820: {:#04x}", self.cpu.memory.read(0x9820));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe10));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe11));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe12));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe13));
    }

    pub fn header(&self) -> &Header {
        if let Some(header) = &self.header {
            return header;
        } else {
            panic!("No header find. Please load a ROM first.");
        }
    }
}
