use windows::{
    Win32::Foundation::*, 
    Win32::System::SystemServices::*,
    core::*, 
    Win32::UI::WindowsAndMessaging::MessageBoxA,
    Win32::System::Threading::GetCurrentProcessId,
};
use winapi::shared::minwindef::HINSTANCE;   // HINSTANCE defined here

use std::process::Command;                  // cmd exec


#[no_mangle]            // if u want the symbol to be exported, we have to do this in addition to pub extern
pub extern fn add(left: usize, right: usize) -> usize {
    println!("aa");     // no stdout when called via rundll32
    left + right
}

#[no_mangle]
pub extern fn hello() {
    let _output = Command::new("cmd")
    .args(["/C", "echo hello from dll > proof.txt"])          
    .spawn()
    .expect("failed to execute process");
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,                  // https://github.com/rust-lang/rust/issues/84981 found hinstance location
    call_reason: u32,
    _: *mut ()
) -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH  => attach(),
        DLL_PROCESS_DETACH  => detach(),
        DLL_THREAD_ATTACH   => attach(),
        DLL_THREAD_DETACH   => detach(),
        _ => ()
    }

    true
}

fn attach() {
    unsafe {
        let pid = GetCurrentProcessId();

        // Create a message box
        MessageBoxA(
            HWND(0),
            // s!("DLL_ATTACHED!"),
            PCSTR( std::format!("Hello from Process: {}\0", pid).as_ptr() ),
            s!("hello.dll"),    // s! is macro for: A literal UTF-8 string with a trailing null terminator.
            Default::default()
        );

    };

    // example of cmd exec
    let _output = Command::new("cmd")
                    // .args(["/C", "echo hello > proof.txt"])
                    // need a cmd context to use 'start', powershell start-process is not supported everywhere
                    .args(["/C", "start calc"])                  
                    .spawn()
                    .expect("failed to execute process");
}

fn detach() {
    unsafe {
        // Create a message box
        MessageBoxA(HWND(0),
            s!("DLL_DETACHED!"),
            s!("hello.dll"),
            Default::default()
        );
    };
}

