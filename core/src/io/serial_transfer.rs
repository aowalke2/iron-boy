use crate::bus::Memory;

pub type SerialTransferCallBack = Box<dyn FnMut(u8) -> Option<u8> + Send>;

fn no_callback(_: u8) -> Option<u8> {
    None
}

fn print_to_std(data: u8) -> Option<u8> {
    use std::io::Write;

    print!("{}", data as char);
    let _ = ::std::io::stdout().flush();

    None
}

pub struct SerialTransfer {
    data: u8,
    control: u8,
    callback: SerialTransferCallBack,
    pub interrupt: u8,
}

impl SerialTransfer {
    pub fn new() -> Self {
        SerialTransfer {
            data: 0,
            control: 0,
            callback: Box::new(print_to_std),
            interrupt: 0,
        }
    }
}

impl Default for SerialTransfer {
    fn default() -> Self {
        SerialTransfer {
            data: 0,
            control: 0,
            callback: Box::new(no_callback),
            interrupt: 0,
        }
    }
}

impl Memory for SerialTransfer {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => {
                if self.data != 0 {
                    println!("{}", self.data)
                }
                self.data
            }
            0xFF02 => self.control | 0b0111_1110,
            _ => panic!("Serial Transfer does not handle read to address {:4X}", address),
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF01 => {
                self.data = data;
                if data != 0 {
                    println!("{}", data)
                }
            }
            0xFF02 => {
                self.control = data;
                if data == 0x81 {
                    match (self.callback)(self.data) {
                        Some(data) => {
                            self.data = data;
                            self.interrupt = 0b1000;
                        }
                        None => {}
                    }
                }
            }
            _ => panic!("Serial Transfer does not handle write to address {:4X}", address),
        }
    }
}