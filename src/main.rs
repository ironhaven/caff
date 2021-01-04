
/*
typedef struct tagINPUT {
  DWORD type;
  union {
    MOUSEINPUT    mi;
    KEYBDINPUT    ki;
    HARDWAREINPUT hi;
  } DUMMYUNIONNAME;
} INPUT, *PINPUT, *LPINPUT;

typedef struct tagKEYBDINPUT {
  WORD      wVk;
  WORD      wScan;
  DWORD     dwFlags;
  DWORD     time;
  ULONG_PTR dwExtraInfo;
} KEYBDINPUT, *PKEYBDINPUT, *LPKEYBDINPUT;



*/


use winapi::um::winuser::SendInput;
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_F24};
use winapi::ctypes::{c_int, c_uint};

//use winapi::um::winbase::STD_OUTPUT_HANDLE;
//use winapi::um::processenv::GetStdHandle;
//use winapi::um::consoleapi::WriteConsoleA;

//use winapi::um::synchapi::Sleep;


#[repr(C)]
struct Input {
    type_: u32,
    ki: KEYBDINPUT
}



pub fn press_f24() {
    let mut inputs = unsafe {
        [
            INPUT {
                type_: INPUT_KEYBOARD,
                // Why does the union not implment From?
                u: core::mem::transmute_copy(&KEYBDINPUT {
                    wVk: VK_F24 as u16,
                    ..Default::default()
                }),
            },
            INPUT {
                type_: INPUT_KEYBOARD,
                u: core::mem::transmute_copy(&KEYBDINPUT {
                    wVk: VK_F24 as u16,
                    dwFlags: KEYEVENTF_KEYUP,
                    ..Default::default()
                }),
            },
        ]
    };
    unsafe {
        assert_eq!(inputs.len() as c_uint, SendInput(inputs.len() as c_uint, inputs.as_mut_ptr(), core::mem::size_of::<INPUT>() as c_int));
    };
}
/*fn main() {
    unsafe {
        let stdout =  GetStdHandle(STD_OUTPUT_HANDLE);
        let buf = "pressed\n";
        loop {
            press_f24();
            assert!(WriteConsoleA(stdout, buf.as_ptr() as *mut _, buf.len() as u32, 0 as *mut _, 0 as *mut _) != 0);
            Sleep(5 * 1000);
        }
    }
}*/

fn main() {
    loop {
        press_f24();
        println!("pressed");
        std::thread::sleep(std::time::Duration::from_secs(59));
    }
}


