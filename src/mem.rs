use std::mem::size_of;
use std::{ ptr};
use neon::prelude::Context;
use skidscan::{Signature};
use neon::types::Finalize;

use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{ReadProcessMemory, VirtualQueryEx, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{
    HANDLE, MEM_COMMIT, MEM_IMAGE, MEMORY_BASIC_INFORMATION, PROCESS_ALL_ACCESS,
};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};


#[derive(Debug)]
pub struct Process {
    pub process_id: u32,
    pub handle: HANDLE,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

impl Process {
    pub fn new(proc: &str) -> Result<Process, Box<dyn std::error::Error>> {
        let hProcessId = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        let mut entry: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
        entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;
        let mut found = false;
        while unsafe { Process32NextW(hProcessId, &mut entry) } != 0 {
            if entry
                .szExeFile
                .iter()
                .take_while(|&x| *x != 0)
                .map(|&x| x as u8 as char)
                .collect::<String>()
                == proc
            {
                found = true;
                break;
            }
        }
        if !found {
            return Err("Process not found".into());
        }
        let process_id = entry.th32ProcessID;
        let handle = unsafe {
            let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
            if handle == std::ptr::null_mut() {
                return Err("Failed to open process".into());
            }
            handle
        };
        Ok(Process { handle, process_id })
    }

    pub fn close_handle(&self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }   

    pub fn read_memory(
        &self,
        address: u64,
        size: usize,
    ) -> Vec<u8> {
        let mut buffer = vec![0; size];
        unsafe {
            ReadProcessMemory(
                self.handle,
                address as *mut _,
                buffer.as_mut_ptr() as *mut _,
                size,
                ptr::null_mut(),
            ); 
        };
        buffer
    }

    pub fn write_memory(
        &self,
        address: u64,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let size = data.len();
        let written = unsafe {
            WriteProcessMemory(
                self.handle,
                address as *mut _,
                data.as_ptr() as *mut _,
                size,
                ptr::null_mut(),
            )
        };
        if written == 0 {
            return Err("Failed to write memory".into());
        };
        Ok(())
    }

    pub fn sig_scan(&self, pattern: &str, start_address: u64) -> Option<u64> {
        let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        let size = size_of::<MEMORY_BASIC_INFORMATION>() as usize;
        let mut address = start_address;
        loop {
            unsafe {
                if VirtualQueryEx(
                    self.handle,
                    address as *mut _,
                    &mut info,
                    size,
                ) != size
                {
                    return None;
                }
                address = info.BaseAddress as u64 + info.RegionSize as u64;
                if info.State != MEM_COMMIT || info.Type == MEM_IMAGE {
                    continue;
                }
                let _buffer = self
                    .read_memory(info.BaseAddress as u64, info.RegionSize);
                let sig = Signature::from(pattern.split(" ").map(|x| {
                    match x {
                        "?" => None,
                        _ => Some(u8::from_str_radix(x, 16).unwrap()),
                    }
                }).collect::<Vec<Option<u8>>>());
                match sig.scan(&_buffer) {
                    Some(x) => {
                        return Some(info.BaseAddress as u64 + x as u64);
                    }
                    None => {},
                };
            }
        }
    }

}

impl Finalize for Process {}

// cargo test -- --nocapture
#[test]
fn check_sig_scan() {
    let process = Process::new("osu!.exe").unwrap();
    println!("{:?}", process);
    // process.read_memory(0x106b000, 12288);
    let find = process.sig_scan("64 00 00 00 00 00 00 00 18 5A 55 01 01 00 00 00 00 00 00 00", 0);
    match find {
        Some(x) => println!("{:x}", x),
        None => println!("Not found"),
    };
    assert_eq!(2 + 2, 4);
}

#[test]
fn keystone() {
    // let engine = Keystone::new(Arch::X86, Mode::Mode32)
    //     .expect("Could not initialize Keystone engine");

    // engine.option(OptionType::Syntax, OptionValue::SyntaxNASM);
    //     // .expect("Could not set option to nasm syntax");

    // // let result = engine.asm("mov ah, 0x80".to_string(), 0)
    // //     .expect("Could not assemble");

    // print!("{}", result);
    assert_eq!(2 + 2, 4);
}
