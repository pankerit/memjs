mod sig;
use sig::Signature;

use std::time::Instant;
use std::{mem, ptr};

use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

use windows::Win32::System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot;
use windows::Win32::System::Diagnostics::ToolHelp::Module32NextW;
use windows::Win32::System::Diagnostics::ToolHelp::Process32NextW;
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32W;
use windows::Win32::System::Diagnostics::ToolHelp::PROCESSENTRY32W;
use windows::Win32::System::Diagnostics::ToolHelp::{
    TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};

use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

use windows::Win32::System::ProcessStatus::K32GetModuleFileNameExW;

use windows::Win32::System::Memory::{
    VirtualAllocEx, VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_IMAGE, MEM_RESERVE,
    PAGE_EXECUTE_READWRITE,
};

#[derive(Debug)]
pub struct Process_ {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct Process {
    pub id: u32,
    pub name: String,
    pub handle: isize,
    pub path: String,
}

#[derive(Debug)]
pub struct Module {
    pub base_address: u64,
    pub size: usize,
    pub name: String,
    pub path: String,
}

pub fn open_process(process_name: &str) -> Result<Process, Box<dyn std::error::Error>> {
    unsafe {
        let hProcessId = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut entry: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
        entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut found = false;
        while Process32NextW(hProcessId, &mut entry).as_bool() {
            if wchar_to_string(&entry.szExeFile) == process_name.to_string() {
                found = true;
                break;
            }
        }
        if (!found) {
            return Err("Failed to open process".into());
        }
        CloseHandle(hProcessId);
        let hProcess = OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), entry.th32ProcessID)?;
        let mut path: [u16; 260] = [0; 260];
        let mut ret = K32GetModuleFileNameExW(hProcess, HINSTANCE(0), &mut path);
        if ret == 0 {
            return Err("Failed to get process path".into());
        }
        let path = wchar_to_string(&path);
        return Ok(Process {
            id: entry.th32ProcessID,
            name: wchar_to_string(&entry.szExeFile),
            handle: hProcess.0,
            path,
        });
    }
}

pub fn read_memory_buffer(handle: HANDLE, address: u64, size: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; size];
    unsafe {
        ReadProcessMemory(
            handle,
            address as *mut _,
            &mut buffer as *mut _ as *mut _,
            size,
            ptr::null_mut(),
        );
    }
    buffer
}

pub fn read_memory<T>(handle: HANDLE, address: u64) -> T {
    let mut val: T = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<T>();
    unsafe {
        ReadProcessMemory(
            handle,
            address as *mut _,
            &mut val as *mut _ as *mut _,
            size,
            ptr::null_mut(),
        );
    }
    val
}

pub fn write_memory_buffer(handle: HANDLE, aaddress: u64, buffer: &Vec<u8>) {
    let size = buffer.len();
    unsafe {
        WriteProcessMemory(
            handle,
            aaddress as *mut _,
            buffer as *const _ as *mut _,
            size,
            ptr::null_mut(),
        );
    }
}

pub fn write_memory<T>(handle: HANDLE, aaddress: u64, val: T) {
    let size = std::mem::size_of::<T>();
    unsafe {
        WriteProcessMemory(
            handle,
            aaddress as *mut _,
            &val as *const _ as *mut _,
            size,
            ptr::null_mut(),
        );
    }
}

pub fn alloc_memory(handle: HANDLE, size: usize) -> u64 {
    let address = unsafe {
        VirtualAllocEx(
            handle,
            ptr::null_mut(),
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };

    let a = format!("{:?}", address);
    u64::from_str_radix(&a[2..], 16).unwrap()
}

pub fn close_handle(handle: HANDLE) -> bool {
    unsafe { CloseHandle(handle).as_bool() }
}

pub fn get_modules(process_id: u32) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    unsafe {
        let hProcessId =
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id)?;
        let mut entry: MODULEENTRY32W = unsafe { std::mem::zeroed() };
        entry.dwSize = mem::size_of::<MODULEENTRY32W>() as u32;
        let mut modules = Vec::new();
        while Module32NextW(hProcessId, &mut entry).as_bool() {
            let module = Module {
                base_address: entry.modBaseAddr as u64,
                size: entry.modBaseSize as usize,
                name: wchar_to_string(&entry.szModule),
                path: wchar_to_string(&entry.szExePath),
            };
            modules.push(module);
        }
        CloseHandle(hProcessId);
        return Ok(modules);
    }
}

pub fn get_processes() -> Result<Vec<Process_>, Box<dyn std::error::Error>> {
    unsafe {
        let hProcessId = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut entry: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
        entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut processes = Vec::new();
        while Process32NextW(hProcessId, &mut entry).as_bool() {
            let process = Process_ {
                id: entry.th32ProcessID,
                name: wchar_to_string(&entry.szExeFile),
            };
            processes.push(process);
        }
        CloseHandle(hProcessId);
        return Ok(processes);
    }
}

pub fn sig_scan(handle: HANDLE, pattern: &str, start_address: u64) -> Option<u64> {
    let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let size = mem::size_of::<MEMORY_BASIC_INFORMATION>() as usize;
    let mut address = start_address;
    let sig = Signature::new(pattern);
    loop {
        unsafe {
            if VirtualQueryEx(handle, address as *mut _, &mut info, size) != size {
                return None;
            }
            address = info.BaseAddress as u64 + info.RegionSize as u64;
            if info.State != MEM_COMMIT || info.Type == MEM_IMAGE {
                continue;
            }
            let mut buffer: Vec<u8> = vec![0; info.RegionSize];
            ReadProcessMemory(
                handle,
                info.BaseAddress as *mut _,
                buffer.as_mut_ptr() as *mut _,
                info.RegionSize as usize,
                ptr::null_mut(),
            );

            match sig.scan(&buffer) {
                Some(x) => {
                    return Some(info.BaseAddress as u64 + x as u64);
                }
                None => {}
            };
        }
    }
}

fn wchar_to_string(wchar: &[u16]) -> String {
    wchar
        .iter()
        .take_while(|&x| *x != 0)
        .map(|&x| x as u8 as char)
        .collect::<String>()
}

// cargo test --release -- --nocapture
// #[test]
// fn allocate_memory() {
//     let process = open_process("Discord.exe").unwrap();
//     let handle = HANDLE(process.handle);
//     let address = alloc_memory(handle, 10);
//     write_memory(handle, address, 0x02);
//     let val = read_memory::<u8>(handle, address);
//     assert_eq!(val, 2);
// }

#[test]
fn test_osu() {
    let process = open_process("osu!.exe").unwrap();
    let now = Instant::now();
    let add = sig_scan(
        HANDLE(process.handle),
        "8D 65 F4 5B 5E 5F 5D C3 00 00 00 00 00 00 2C DF CF 14 00 00 00 00 24 DF CF 14",
        0,
    )
    .unwrap();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:x}", add);
    assert_eq!(2, 2);
}
