use crate::cpu::CPU;
use crate::gpu::SCREEN_SIZE_RGB;
use crate::header::Header;
use crate::keypad::KeyEvent;
use crate::{mbc, time};

const FRAME_TIME: f64 = 1.0 / 60.0;
const CYCLES_PER_SECOND: u32 = 4_194_304;
const CYCLES_PER_FRAME: u32 = CYCLES_PER_SECOND / 60;

#[derive(PartialEq)]
pub enum GBMode {
    DMG,
    CGB,
}
 
pub struct Gameboy {
    pub cpu: CPU,
    header: Header,
    
    render_callback: Box<dyn FnMut(&[u8; 160 * 144 * 3]) + 'static>,
    input_callback: Box<dyn FnMut() -> Option<KeyEvent> + 'static>,

    pub previous_time: f64,
    pub lag: f64,
}

impl Gameboy {
    pub fn new(rom: &Vec<u8>) -> Gameboy {

        let header = Header::load_rom(&rom);
        let mbc = crate::mbc::from_rom(&rom);

        Gameboy {
            cpu: CPU::new(mbc),
            header,

            render_callback: Box::new(|_| { panic!("No render callback set!"); }),
            input_callback: Box::new(|| { panic!("No input callback set!"); }),

            previous_time: 0.0,
            lag: 0.0,
        }
    }

    pub fn new_from_file(file: &str) -> Gameboy {
        let rom = std::fs::read(file).unwrap();
        Gameboy::new(&rom)
    }


    /// Get the screen data
    pub fn get_screen_data(&self) -> &[u8; SCREEN_SIZE_RGB] {
        return self.cpu.memory.gpu.screen_data();
    }

    /// Set the render callback
    /// The render callback is a function that will be called every frame to render the screen
    /// The function must take a slice of 160*144*3 u8 as argument -- 160*144 pixels with 3 bytes per pixel (RGB)
    /// 
    /// # Example
    /// ```
    /// use rusty_boy::gameboy::Gameboy;
    /// let mut game = Gameboy::new();
    /// game.set_render_callback(|screen_data| {
    ///     for y in 0..144 {
    ///         for x in 0..160 {
    ///             print!( "{}", 
    ///                 match screen_data[y * 160 * 3 + x * 3] {
    ///                     3 => "  ",
    ///                     2 => "░░",
    ///                     1 => "▒▒",
    ///                     0 => "▓▓",
    ///                     _ => "  ",
    ///                 }
    ///             );
    ///         }
    ///     println!();
    ///    }
    /// });
    /// ```
    pub fn set_render_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&[u8; SCREEN_SIZE_RGB]) + 'static,
    {
        self.render_callback = Box::new(callback);
    }

    /// Set the input callback
    /// The input callback is a function that will be called every frame to get the input from the user
    /// The function must return an Option<KeyEvent>
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut() -> Option<KeyEvent> + 'static,
    {
        self.input_callback = Box::new(callback);
    }

    /// Run the game loop
    /// A game loop is a loop that will run the game at a fixed frame rate
    /// This function it called in a loop by the run function
    fn game_loop(&mut self) {
        let current_time = time::now();
        let elapsed = current_time - self.previous_time;
        self.previous_time = current_time;
        self.lag += elapsed;

        // Call the input callback to get the input from the user
        if let Some(key) = (self.input_callback)() {
            match key {
                KeyEvent::Press(key)    => { self.cpu.memory.keypad.press(key); }
                KeyEvent::Release(key)  => { self.cpu.memory.keypad.release(key); }
            }
        }


        let mut cycles = 0;
        while self.lag >= FRAME_TIME {
            self.update();
            self.lag -= FRAME_TIME;
            cycles += 1;
        }
        self.render();
        println!("FPS: {:.2} Cycles: {:.2} Lag: {:.2} keypad {:#04x}", 1.0 / elapsed, cycles, self.lag, self.cpu.memory.keypad.read());
    }

    /// Start the emultation
    /// This function have two implementations: one for the native target and one for the wasm target.
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

    /// Update the game state
    /// This function will update the game state by running the CPU for a fixed number of cycles
    fn update(&mut self) {

        // Execute the CPU for a fixed number of cycles
        let mut cycles = 0;
        while cycles < CYCLES_PER_FRAME {
            cycles += self.cpu.step() as u32;
        }
    }

    /// Render the screen
    /// This function will call the render callback to render the screen
    fn render(&mut self) {
        (self.render_callback)(self.cpu.memory.gpu.screen_data());
    }

    #[deprecated]
    // This function was used to debug opcodes
    pub fn run_debug(&mut self) {
        // 2F2A --> Intro
        // 6A6B --> Title screen
        // 650C
        // 2CF --> CFFB est remis (64D3)

        // 0x5b7 --> 0x3c5

        // List of 10 last opcodes
        let mut last_addr = [0u16; 5_000];

        //while self.cpu.registers.pc != 0x01 {
        let mut c = 0;
        while true {
            self.cpu.step();
            if self.cpu.registers.pc == 0x2892 && self.cpu.registers.hl() == 0x6f94 {
                c += 1;
                if c == 982 {
                    break;
                }
            }
        }

        //0x40e --> load scx into a
        //0x7b9e --> increment 0xC103
        while true {
            self.cpu.step();
        }

        for _ in 0..100 {
            self.cpu.step_debug();
        }

        println!("Registers: {:?}", self.cpu.registers);
        println!("0xC103: {:#04x}", self.cpu.memory.read(0xC103));
        
        /* let mut str_buffer = String::new();
        for addr in last_addr.iter() {
            let s = format!("0x{:04x} :: ", addr);
            str_buffer.push_str(&s);
        }
        println!("{}", str_buffer); */
        println!("ROM: {:?}", self.cpu.memory.mbc.info());

        /* for _ in 0..1_000 {
            self.cpu.step(false);
            for _ in 0..500_000 {
                self.cpu.step(false);
            }
            //println!("Registers: {:?}", self.cpu.registers);
        } */


        println!("Registers: {:?}", self.cpu.registers);
        println!("0x9820: {:#04x}", self.cpu.memory.read(0x9820));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe10));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe11));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe12));
        println!("OAM: {:#04x}", self.cpu.memory.read(0xfe13));
    }

    /// Get the header of the loaded ROM
    /// This function will return the header of the loaded ROM or panic if no ROM is loaded
    pub fn header(&self) -> &Header {
        &self.header
    }
}
