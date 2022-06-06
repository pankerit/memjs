mod sig;
use sig::Signature;

use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::ProcessStatus::K32GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

pub fn wchar_to_string(wchar: &[u16]) -> String {
    wchar
        .iter()
        .take_while(|&x| *x != 0)
        .map(|&x| x as u8 as char)
        .collect::<String>()
}

#[derive(Debug)]
pub struct Process {
    pub id: u32,
    pub name: String,
    pub handle: HANDLE,
}

#[derive(Debug)]
pub struct Module {
    pub base_address: u32,
    pub size: usize,
    pub name: String,
    pub path: String,
}

pub fn open_process(process_name: &str) -> Result<Process, Box<dyn std::error::Error>> {
    unsafe {
        let hProcessId = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut found = false;
        while Process32NextW(hProcessId, &mut entry).as_bool() {
            if wchar_to_string(&entry.szExeFile) == process_name.to_string() {
                found = true;
                break;
            }
        }
        if !found {
            return Err("Process not found".into());
        }
        CloseHandle(hProcessId);
        let hProcess = OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), entry.th32ProcessID)?;

        Ok(Process {
            id: entry.th32ProcessID,
            name: wchar_to_string(&entry.szExeFile),
            handle: hProcess,
        })
    }
}

pub fn read_memory<T>(handle: HANDLE, address: u32) -> T {
    unsafe {
        let mut val: T = std::mem::zeroed();
        let size = std::mem::size_of::<T>();

        ReadProcessMemory(
            handle,
            address as *mut _,
            &mut val as *mut _ as *mut _,
            size,
            std::ptr::null_mut(),
        );

        val
    }
}

pub fn read_memory_buffer(handle: HANDLE, address: u32, size: usize) -> Vec<u8> {
    unsafe {
        let buffer: Vec<u8> = vec![0; size];
        ReadProcessMemory(
            handle,
            address as *mut _,
            buffer.as_ptr() as *mut _,
            size,
            std::ptr::null_mut(),
        );
        buffer
    }
}

pub fn read_memory_from_pointer<T>(handle: HANDLE, addresses: &Vec<u32>) -> T {
    let size = addresses.len();
    let mut pointer = read_memory::<u32>(handle, addresses[0]);
    if size > 2 {
        for i in 1..addresses.len() - 1 {
            pointer = read_memory::<u32>(handle, pointer + addresses[i]);
        }
    }

    read_memory::<T>(handle, pointer + addresses.last().unwrap())
}

pub fn write_memory<T>(handle: HANDLE, address: u32, val: T) {
    let size = std::mem::size_of::<T>();
    unsafe {
        WriteProcessMemory(
            handle,
            address as *mut _,
            &val as *const _ as *mut _,
            size,
            std::ptr::null_mut(),
        );
    }
}

pub fn write_memory_buffer(handle: HANDLE, address: u32, buffer: &Vec<u8>) {
    unsafe {
        let size = buffer.len();
        WriteProcessMemory(
            handle,
            address as *mut _,
            buffer.as_ptr() as *mut _,
            size,
            std::ptr::null_mut(),
        );
    }
}

pub fn alloc_memory(handle: HANDLE, size: usize) -> u32 {
    let address = unsafe {
        VirtualAllocEx(
            handle,
            std::ptr::null_mut(),
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };

    let a = format!("{:?}", address);
    u32::from_str_radix(&a[2..], 16).unwrap()
}

pub fn close_handle(handle: HANDLE) -> bool {
    unsafe { CloseHandle(handle).as_bool() }
}

pub fn sig_scan(handle: HANDLE, pattern: &str, start_address: u32) -> Option<u32> {
    let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<MEMORY_BASIC_INFORMATION>() as usize;
    let mut address = start_address as u64;
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
            let buffer = read_memory_buffer(handle, info.BaseAddress as u32, info.RegionSize);
            match sig.scan(&buffer) {
                Some(x) => {
                    return Some(info.BaseAddress as u32 + x);
                }
                None => {}
            };
        }
    }
}

pub fn sig_scan_module(
    handle: HANDLE,
    process_id: u32,
    pattern: &str,
    module_name: &str,
) -> Option<u32> {
    let modules = get_process_modules(process_id);
    for module in modules {
        if module.name == module_name {
            let sig = Signature::new(pattern);
            let buffer = read_memory_buffer(handle, module.base_address, module.size);
            match sig.scan(&buffer) {
                Some(x) => {
                    return Some(module.base_address + x);
                }
                None => return None,
            };
        }
    }
    None
}

pub fn get_process_path(handle: HANDLE) -> Option<String> {
    unsafe {
        let mut path: [u16; 260] = [0; 260];
        let mut ret = K32GetModuleFileNameExW(handle, HINSTANCE(0), &mut path);
        if ret == 0 {
            return None;
        }
        Some(wchar_to_string(&path))
    }
}

pub fn get_process_modules(process_id: u32) -> Vec<Module> {
    unsafe {
        let hProcessId =
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id).unwrap();
        let mut entry: MODULEENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<MODULEENTRY32W>() as u32;
        let mut modules = Vec::new();
        while Module32NextW(hProcessId, &mut entry).as_bool() {
            let module = Module {
                base_address: entry.modBaseAddr as u32,
                size: entry.modBaseSize as usize,
                name: wchar_to_string(&entry.szModule),
                path: wchar_to_string(&entry.szExePath),
            };
            modules.push(module);
        }
        CloseHandle(hProcessId);
        modules
    }
}

// cargo test --release -- --nocapture
#[test]
fn test() {
    let process = open_process("osu!.exe").unwrap();
    let now = std::time::Instant::now();
    sig_scan(
        process.handle,
        "55 8B EC 57 56 53 83 EC 14 33 C0 89 45 E8 83 3D AD EC DF",
        0,
    );
    println!("{:?}", now.elapsed().as_millis());
    assert_eq!(2, 2);
}
