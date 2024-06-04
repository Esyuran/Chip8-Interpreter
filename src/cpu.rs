use std::fs;
use rand::Rng;

const FONT: [u8;80] = [0xF0, 0x90, 0x90, 0x90, 0xF0,0x20, 0x60, 0x20, 0x20, 0x70,0xF0, 0x10, 0xF0, 0x80, 0xF0,0xF0, 0x10, 0xF0, 0x10, 0xF0,0x90, 0x90, 0xF0, 0x10, 0x10,0xF0, 0x80, 0xF0, 0x10, 0xF0,0xF0, 0x80, 0xF0, 0x90, 0xF0,0xF0, 0x10, 0x20, 0x40, 0x40,0xF0, 0x90, 0xF0, 0x90, 0xF0,0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80 ];


pub fn init(filename: &str) -> ([u8; 4096], [u8; 16], [u16; 16], [bool; 32*64], [bool; 16], u8, u8, u16, u16, u16) {
    let mut memory: [u8; 4096] = [0; 4096];
    memory[0..FONT.len()].copy_from_slice(&FONT);

    match fs::read(filename) {
        Ok(contents) => memory[0x200..0x200+contents.len()].copy_from_slice(&contents),
        Err(error) => {
            println!("Error reading file: {}", error);
            std::process::exit(5);
        }
    }

    let registers: [u8; 16] = [0; 16];

    let stack: [u16; 16] = [0; 16];

    let graphics: [bool; 32*64] = [false;32*64];

    let input : [bool;16] = [false;16];

    let delay_timer : u8 = 0;
    let sound_timer : u8 = 0;
    let pc : u16 = 0x200;
    let sp : u16 = 0;
    let index : u16 = 0;




    return (memory, registers, stack, graphics, input, delay_timer, sound_timer, pc, sp, index);
}


pub fn run(memory: &mut [u8; 4096], registers: &mut [u8; 16], stack : &mut[u16; 16], graphics: &mut[bool; 32*64], input: &mut[bool; 16], delay_timer: &mut u8, sound_timer: &mut u8, pc: &mut u16, sp: &mut u16, index: &mut u16) {
    let instruction : u16 = (memory[*pc as usize] as u16) << 8 | memory[(*pc+1) as usize] as u16;


    match instruction >> 12 {
        0x0 => {
            match instruction {
                0x00E0 => {
                    for pixel in graphics.iter_mut() { *pixel = false; }
                }
                0x00EE => {
                    *sp -= 1;
                    *pc = stack[*sp as usize];
                }

                _ => {return;}
            }

        }
        0x1 => {
            *pc = instruction & 0x0FFF;
            return;
        }
        0x2 => {
            stack[*sp as usize] = *pc;
            *sp += 1;
            *pc = instruction & 0x0FFF;
            return;
        }
        0x3 => {
            if registers[((instruction & 0x0F00) >> 8) as usize] == (instruction & 0x00FF) as u8 {
                *pc += 2;
            }
        }

        0x4 => {
            if registers[((instruction & 0x0F00) >> 8) as usize] != (instruction & 0x00FF) as u8 {
                *pc += 2;
            }
        }

        0x5 => {
            if registers[((instruction & 0x0F00) >> 8) as usize] == registers[((instruction & 0x00F0) >> 4) as usize] {
                *pc += 2;
            }
        }

        0x6 => {
            registers[((instruction & 0x0F00) >> 8) as usize] = (instruction & 0x00FF) as u8 //move into register
        }

        0x7 => {
            registers[((instruction & 0x0F00) >> 8) as usize] = registers[((instruction & 0x0F00) >> 8) as usize].wrapping_add((instruction & 0x00FF) as u8);

        }

        0x8 => {
            match instruction & 0x000F {
                0x0 => {
                    registers[((instruction & 0x0F00) >> 8) as usize] = registers[((instruction & 0x00F0) >> 4) as usize]
                }
                0x1 => {
                    registers[((instruction & 0x0F00) >> 8) as usize] |= registers[((instruction & 0x00F0) >> 4) as usize]
                }
                0x2 => {
                    registers[((instruction & 0x0F00) >> 8) as usize] &= registers[((instruction & 0x00F0) >> 4) as usize]
                }
                0x3 => {
                    registers[((instruction & 0x0F00) >> 8) as usize] ^= registers[((instruction & 0x00F0) >> 4) as usize]
                }
                0x4 => {
                    let sum : u16 = registers[((instruction & 0x0F00) >> 8) as usize] as u16 + (registers[((instruction & 0x00F0) >> 4) as usize] as u16);
                    registers[((instruction & 0x0F00) >> 8) as usize] = (0x00FF & sum) as u8;
                    registers[0xF] = if sum > 255 {1} else {0};

                }
                0x5 => {
                    let flag :u8 = if registers[((instruction & 0x0F00) >> 8) as usize] >= (registers[((instruction & 0x00F0) >> 4) as usize]) {1} else {0};
                    registers[((instruction & 0x0F00) >> 8) as usize] = registers[((instruction & 0x0F00) >> 8) as usize].wrapping_sub(registers[((instruction & 0x00F0) >> 4) as usize]);
                    registers[0xF] = flag;
                }
                0x6 => {
                    let flag :u8 = registers[((instruction & 0x0F00) >> 8) as usize] & 0x0001;
                    registers[((instruction & 0x0F00) >> 8) as usize] >>= 1;
                    registers[0xF] = flag;
                }
                0x7 => {
                    let flag :u8 = if registers[((instruction & 0x0F00) >> 8) as usize] <= (registers[((instruction & 0x00F0) >> 4) as usize]) {1} else {0};
                    registers[((instruction & 0x0F00) >> 8) as usize] = registers[((instruction & 0x00F0) >> 4) as usize].wrapping_sub(registers[((instruction & 0x0F00) >> 8) as usize]);
                    registers[0xF] = flag;
                }
                0xE => {
                    let flag :u8 = (registers[((instruction & 0x0F00) >> 8) as usize] & 0b10000000) >> 7;
                    registers[((instruction & 0x0F00) >> 8) as usize] <<= 1;
                    registers[0xF] = flag;
                }

                _ => {println!("Found not yet implemented instruction: {:#06x}", instruction)}
            }
        }
        0x9 => {
            if registers[((instruction & 0x0F00) >> 8) as usize] != registers[((instruction & 0x00F0) >> 4) as usize] {
                *pc += 2;
            }
        }


        0xA => {
            *index = instruction & 0x0FFF
        }

        0xB => {
            *pc = (instruction & 0x0FFF) + registers[0x0] as u16;
            return;
        }

        0xC => {
            let mut rng = rand::thread_rng();
            let rb: u8 = rng.gen();
            registers[((instruction & 0x0F00) >> 8) as usize] = rb & ((instruction & 0x00FF) as u8);
        }


        0xD => {
            let reg_x = registers[((instruction & 0x0F00) >> 8) as usize];
            let reg_y = registers[((instruction & 0x00F0) >> 4) as usize];
            let height: u16 = instruction & 0x000F;

            for y in 0..height {
                let row = memory[(*index + y) as usize];
                for x in 0..8 {
                    if 0x80 >> x & row != 0 {
                        let graphics_x = (reg_x +x) % 64;
                        let graphics_y = (reg_y as u16 + y) % 32;
                        let idx= graphics_x as u16 + (graphics_y * 64);

                        graphics[idx as usize] ^= true;

                        if graphics[idx as usize] == false {
                            registers[0xF] = 1;
                        }


                    }
                }
            }
        }

        0xE => {
            match instruction & 0x00FF {
                0x9E => {
                    if input[registers[((instruction & 0x0F00) >> 8) as usize] as usize] {
                        *pc += 2;
                    }
                }
                0xA1 => {
                    if !input[registers[((instruction & 0x0F00) >> 8) as usize] as usize] {
                        *pc += 2;
                    }
                }
                _ => {println!("This should be impossible")}
            }

        }
        0xF => {
            match instruction & 0x00FF {
                0x07 => {
                    registers[((instruction & 0x0F00) >> 8) as usize] = *delay_timer;
                }

                0x0A => {
                    for (i, key) in input.iter().enumerate() {
                        if *key == true {
                            registers[((instruction & 0x0F00) >> 8) as usize] = i as u8;
                            *pc += 2;
                            return;
                        }
                    }
                    return;
                }

                0x15 => {
                    *delay_timer = registers[((instruction & 0x0F00) >> 8) as usize];
                }

                0x18 => {
                    *sound_timer = registers[((instruction & 0x0F00) >> 8) as usize];
                }

                0x1E => {
                    *index += registers[((instruction & 0x0F00) >> 8) as usize] as u16;
                }

                0x29 => {
                    *index = registers[((instruction & 0x0F00) >> 8) as usize] as u16 * 0x5;
                }
                0x33 => {
                    memory[*index as usize] = registers[((instruction & 0x0F00) >> 8) as usize] / 100;
                    memory[(*index + 1) as usize ] = (registers[((instruction & 0x0F00) >> 8) as usize] % 100) / 10;
                    memory[(*index + 2) as usize] = registers[((instruction & 0x0F00) >> 8) as usize] % 10;
                }
                0x55 => {
                    for i in 0..=((instruction & 0x0F00) >> 8) as usize {
                        memory[(*index + i as u16) as usize] = registers[i];
                    }
                }
                0x65 => {
                    for i in 0..=((instruction & 0x0F00) >> 8) as usize {
                         registers[i] = memory[(*index + i as u16) as usize];
                    }
                }



                _ => {}
            }
        }

        _ => {println!("Found not yet implemented instruction: {:#06x}", instruction)}
    }

    *pc += 2
}

