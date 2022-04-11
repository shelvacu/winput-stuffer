use std::mem::MaybeUninit;
use std::io;
use std::num::NonZeroU32;

use windows::Win32::UI::Input::KeyboardAndMouse as km_sys;
use windows::Win32::UI::WindowsAndMessaging as wm_sys;

pub const WHEEL_DELTA:u8 = 120;

pub use km_sys::{
    MOUSEINPUT,
    KEYBDINPUT,
    HARDWAREINPUT,
    VIRTUAL_KEY as VirtualKey,
};

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct MouseInputSys(MOUSEINPUT);

impl MouseInputSys {
    pub unsafe fn new(inner: MOUSEINPUT) -> Self {
        Self(inner)
    }
}

impl From<MouseInput> for MouseInputSys {
    fn from(rusty: MouseInput) -> Self {
        let mut inner:MOUSEINPUT = unsafe { std::mem::zeroed() };
        if let Some(msg) = rusty.msg {
            inner.dwExtraInfo = msg.into_inner() as usize;
        }
        if let Some(time) = rusty.time {
            inner.time = time.into();
        }

        match rusty.e {
            MouseInputEnum::Button{which: MouseButton::Left, button_up: false} => inner.dwFlags |= km_sys::MOUSEEVENTF_LEFTDOWN,
            MouseInputEnum::Button{which: MouseButton::Left, button_up: true} => inner.dwFlags |= km_sys::MOUSEEVENTF_LEFTUP,
            MouseInputEnum::Button{which: MouseButton::Right, button_up: false} => inner.dwFlags |= km_sys::MOUSEEVENTF_RIGHTDOWN,
            MouseInputEnum::Button{which: MouseButton::Right, button_up: true} => inner.dwFlags |= km_sys::MOUSEEVENTF_RIGHTUP,
            MouseInputEnum::Button{which: MouseButton::Middle, button_up: false} => inner.dwFlags |= km_sys::MOUSEEVENTF_MIDDLEDOWN,
            MouseInputEnum::Button{which: MouseButton::Middle, button_up: true} => inner.dwFlags |= km_sys::MOUSEEVENTF_MIDDLEUP,
            MouseInputEnum::Button{which: MouseButton::X1, button_up: true} => {
                inner.dwFlags |= km_sys::MOUSEEVENTF_XUP;
                inner.mouseData = wm_sys::XBUTTON1.0;
            },
            MouseInputEnum::Button{which: MouseButton::X1, button_up: false} => {
                inner.dwFlags |= km_sys::MOUSEEVENTF_XDOWN;
                inner.mouseData = wm_sys::XBUTTON1.0;
            },
            MouseInputEnum::Button{which: MouseButton::X2, button_up: true} => {
                inner.dwFlags |= km_sys::MOUSEEVENTF_XUP;
                inner.mouseData = wm_sys::XBUTTON1.0;
            },
            MouseInputEnum::Button{which: MouseButton::X2, button_up: false} => {
                inner.dwFlags |= km_sys::MOUSEEVENTF_XDOWN;
                inner.mouseData = wm_sys::XBUTTON1.0;
            },
            MouseInputEnum::Move{m: MouseMovement::AbsolutePrimaryMonitor{x, y}, coalesce} => {
                inner.dwFlags |= km_sys::MOUSEEVENTF_ABSOLUTE;
                if !coalesce {
                    inner.dwFlags |= km_sys::MOUSEEVENTF_MOVE_NOCOALESCE;
                }
                inner.dx = x.into();
                inner.dy = y.into();
            },
            MouseInputEnum::Move{m: MouseMovement::AbsoluteVirtualDesktop{x, y}, coalesce} => {
                inner.dwFlags |= km_sys::MOUSEEVENTF_ABSOLUTE & km_sys::MOUSEEVENTF_VIRTUALDESK;
                if !coalesce {
                    inner.dwFlags |= km_sys::MOUSEEVENTF_MOVE_NOCOALESCE;
                }
                inner.dx = x.into();
                inner.dy = y.into();
            },
            MouseInputEnum::Move{m: MouseMovement::Relative{dx, dy}, coalesce} => {
                if !coalesce {
                    inner.dwFlags |= km_sys::MOUSEEVENTF_MOVE_NOCOALESCE;
                }
                inner.dx = dx;
                inner.dy = dy;
            },
            MouseInputEnum::Wheel{horizontal, amount} => {
                inner.dwFlags |= if horizontal {
                    km_sys::MOUSEEVENTF_HWHEEL
                } else {
                    km_sys::MOUSEEVENTF_WHEEL
                };
                inner.mouseData = amount as u32;
            }
        }

        Self(inner)
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct MouseInput {
    pub e: MouseInputEnum,
    pub msg: Option<super::window_message::WindowMessage>,
    /// The time stamp for the event, in milliseconds. If this parameter is None, the system will provide its own time stamp.
    pub time: Option<NonZeroU32>,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum MouseInputEnum {
    Button{which: MouseButton, button_up: bool},
    Move{m: MouseMovement, coalesce: bool},
    /// When vertical (horizontal: false)
    /// ```
    ///  ^ + positive values are forward, away from the user
    ///  |
    ///  v - negative values are backward, towards the user
    /// ```
    /// 
    /// When horizontal: true
    /// ```
    /// negative values point left
    ///   |
    /// < - ................................ + >
    ///                                      |
    ///              positive values point right
    /// ```
    /// 
    /// In both cases, one wheel click is 120 (WHEEL_DELTA)
    Wheel{horizontal: bool, amount: i32,},
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum MouseMovement {
    /// Move the mouse to the given coordinates.
    /// Coordinates are absolute. 0,0 is the top-left of the primary monitor and 65535,65535 is the bottom-right of the primary monitor
    AbsolutePrimaryMonitor{x: u16, y: u16},
    /// Move the mouse to the given coordinates.
    /// Coordinates are absolute. 0,0 is the top-left of the entire virtual desktop (all monitors) and 65535,65535 is the bottom-right of the virtual desktop
    AbsoluteVirtualDesktop{x: u16, y: u16},
    /// Move the mouse by the given delta x and delta y. Positive dx goes right, and positive dy goes down
    Relative{dx: i32, dy: i32},
}





#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct KeyboardInputSys(KEYBDINPUT);

impl KeyboardInputSys {
    pub unsafe fn new(inner: KEYBDINPUT) -> Self {
        Self(inner)
    }
}

impl From<KeyboardInput> for KeyboardInputSys {
    fn from(rusty: KeyboardInput) -> Self {
        let mut inner:KEYBDINPUT = unsafe { std::mem::zeroed() };
        if let Some(msg) = rusty.msg {
            inner.dwExtraInfo = msg.into_inner().try_into().unwrap();
        }
        if let Some(time) = rusty.time {
            inner.time = time.into();
        }

        if rusty.key_up {
            inner.dwFlags |= km_sys::KEYEVENTF_KEYUP;
        }

        match rusty.e {
            KeyboardInputEnum::VirtualKeyCode{code, extended} => {
                if code.0 < 1 || code.0 > 254 {
                    panic!("VirtualKeyCode outside acceptable range");
                }
                if extended {
                    inner.dwFlags |= km_sys::KEYEVENTF_EXTENDEDKEY
                }
                inner.wVk = code;
            },
            KeyboardInputEnum::ScanCode{code, extended} => {
                inner.wScan = code;
                if extended {
                    inner.dwFlags |= km_sys::KEYEVENTF_EXTENDEDKEY
                }
                inner.dwFlags |= km_sys::KEYEVENTF_SCANCODE
            },
            KeyboardInputEnum::UnicodeCodeUnit(code) => {
                inner.wScan = code;
                inner.dwFlags |= km_sys::KEYEVENTF_UNICODE;
            }
        }

        Self(inner)
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct KeyboardInput {
    pub e: KeyboardInputEnum,
    pub key_up: bool,
    pub msg: Option<super::window_message::WindowMessage>,
    /// The time stamp for the event, in milliseconds. If this parameter is None, the system will provide its own time stamp.
    pub time: Option<NonZeroU32>,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum KeyboardInputEnum {
    /// Must be in the range 1..=254
    VirtualKeyCode{code: VirtualKey, extended: bool},
    ScanCode{code: u16, extended: bool},
    /// That's right, you have to encode your text as UTF-16 and if it's got two code units you have to send 2 events.
    UnicodeCodeUnit(u16),
}














#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct HardwareInputSys(HARDWAREINPUT);

impl HardwareInputSys {
    pub unsafe fn new(inner: HARDWAREINPUT) -> Self {
        Self(inner)
    }
}

impl From<HardwareInput> for HardwareInputSys {
    fn from(rusty: HardwareInput) -> Self {
        let mut inner:HARDWAREINPUT = unsafe { std::mem::zeroed() };
        inner.uMsg = rusty.u_msg;
        inner.wParamL = rusty.w_param_l;
        inner.wParamH = rusty.w_param_h;
        Self(inner)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct HardwareInput {
    u_msg: u32,
    w_param_l: u16,
    w_param_h: u16,
}















#[repr(transparent)]
pub struct Input (km_sys::INPUT);

impl Input {
    /// Safety: Must be a valid INPUT struct
    pub unsafe fn new(inner: km_sys::INPUT) -> Self {
        Self(inner)
    }

    pub fn from_mouse(t: &MouseInputSys) -> Self {
        let mut win_input_u = MaybeUninit::<km_sys::INPUT_0>::uninit();
        let src = t as *const MouseInputSys;
        let u = unsafe {
            std::ptr::copy_nonoverlapping(
                src,
                win_input_u.as_mut_ptr() as *mut MouseInputSys,
                1
            );
            win_input_u.assume_init()
        };
        let input = km_sys::INPUT {
            r#type: km_sys::INPUT_MOUSE,
            Anonymous: u,
        };
        unsafe { Self::new(input) }
    }

    pub fn from_keyboard(t: &KeyboardInputSys) -> Self {
        let mut win_input_u = MaybeUninit::<km_sys::INPUT_0>::uninit();
        let src = t as *const KeyboardInputSys;
        let u = unsafe {
            std::ptr::copy_nonoverlapping(
                src,
                win_input_u.as_mut_ptr() as *mut KeyboardInputSys,
                1
            );
            win_input_u.assume_init()
        };
        let input = km_sys::INPUT {
            r#type: km_sys::INPUT_KEYBOARD,
            Anonymous: u,
        };
        unsafe { Self::new(input) }
    }

    pub fn from_hardware(t: &HardwareInputSys) -> Self {
        let mut win_input_u = MaybeUninit::<km_sys::INPUT_0>::uninit();
        let src = t as *const HardwareInputSys;
        let u = unsafe {
            std::ptr::copy_nonoverlapping(
                src,
                win_input_u.as_mut_ptr() as *mut HardwareInputSys,
                1
            );
            win_input_u.assume_init()
        };
        let input = km_sys::INPUT {
            r#type: km_sys::INPUT_HARDWARE,
            Anonymous: u,
        };
        unsafe { Self::new(input) }
    }
}

/// Safe interface to windows SendInput function. [MS Docs](https://docs.microsoft.com/en-us/windows/win32/api/km_sys/nf-km_sys-sendinput)
/// 
/// Panics:
/// 
/// Panics if the length of the input slice >= i32::MAX
pub fn send_input(inputs: &[Input]) -> io::Result<u32> {
    if inputs.len() >= i32::MAX as usize { panic!() }
    let res = unsafe {
        km_sys::SendInput(
            std::mem::transmute(inputs),
            std::mem::size_of::<km_sys::INPUT>().try_into().unwrap(),
        )
    };
    if res == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}