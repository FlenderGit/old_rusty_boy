use crate::cpu::CPU;
use crate::gpu::SCREEN_SIZE_RGB;
use crate::header::Header;
use crate::keypad::KeyEvent;
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

            render_callback: Box::new(|_| { panic!("No render callback set!"); }),
            input_callback: Box::new(|| { panic!("No input callback set!"); }),

            previous_time: 0.0,
            lag: 0.0,
        }
    }

    /// Load a ROM from the given filename
    /// The ROM is loaded from the file system and then loaded into the Gameboy memory
    /// This function will panic if the file is not found
    /// 
    /// # Arguments
    /// * `filename` - A string slice that holds the filename of the ROM to load
    /// 
    /// # Example
    /// ```
    /// use rusty_boy::gameboy::Gameboy;
    /// let mut game = Gameboy::new();
    /// game.load_rom_from_filename("roms/tetris.gb");
    /// ```
    pub fn load_rom_from_filename(&mut self, filename: &'static str ) {

        if !std::path::Path::new(&filename).exists() {
            panic!("File not found: {}", filename);
        }

        let rom = std::fs::read(filename).unwrap();
        self.load_rom(&rom);
    }

    /// Load a ROM from a vector of bytes
    /// The ROM is loaded into the Gameboy memory
    /// The ROM must be at least 0x8000 bytes long - the minimum size for a Gameboy ROM. Other tests will be done after to veriy if the size provided in the ROMs' header is correct.
    /// 
    /// # Arguments
    /// * `rom` - A vector of bytes that holds the ROM to load.
    /// 
    /// # Example
    /// ```
    /// use rusty_boy::gameboy::Gameboy;
    /// let mut game = Gameboy::new();
    /// let rom = std::fs::read("roms/tetris.gb").unwrap();
    /// game.load_rom(&rom);
    /// ```
    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        
        if rom.len() < 0x8000 {
            panic!("ROM is too small. Minimum size is 0x8000 bytes.");
        }

        self.cpu.memory.load_rom(&rom);
        self.header = Some(Header::load_rom(&rom));
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

        // Call the input callback to get the input from the user
        if let Some(key) = (self.input_callback)() {
            match key {
                KeyEvent::Press(key)    => { self.cpu.memory.keypad.press(key); }
                KeyEvent::Release(key)  => { self.cpu.memory.keypad.release(key); }
            }
        }

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

        while self.cpu.registers.pc != 0x6a6b {
            self.cpu.step();
        }

        while self.cpu.registers.pc != 0xffb8 {
            self.cpu.step();
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
            self.cpu.step();
        }

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
        if let Some(header) = &self.header {
            return header;
        } else {
            panic!("No header find. Please load a ROM first.");
        }
    }
}
