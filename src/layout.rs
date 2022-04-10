use bimap::BiMap;

use windows::Win32::UI::WindowsAndMessaging as wm_sys;
use windows::Win32::UI::Input::KeyboardAndMouse as km_sys;

pub use windows::Win32::UI::TextServices::HKL;

/// Calls GetKeyboardLayout(GetWindowThreadProcessId(GetForegroundWindow()), NULL)
/// Important note from windows docs:
///
/// > Since the keyboard layout can be dynamically changed, applications that cache information about the current keyboard layout should process the WM_INPUTLANGCHANGE message to be informed of changes in the input language.
fn current_layout_id() -> HKL {
    // "The foreground window can be NULL in certain circumstances, such as when a window is losing activation." augh
    let front_window = unsafe { wm_sys::GetForegroundWindow() };
    if front_window.0 == 0 { panic!("Result of GetForegroundWindow was 0 (NULL)"); }
    let pid = unsafe { wm_sys::GetWindowThreadProcessId(
        front_window,
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid
        // "A pointer to a variable that receives the process identifier. If this parameter is not NULL, ..." ie this can be null
        std::ptr::null_mut(),
    ) };
    unsafe { km_sys::GetKeyboardLayout(pid) }
}

pub struct KeyboardLayout {
    sc_to_vk: BiMap<u8, u32>,
    state: [u8; 256],
    strbuf: String,
}

impl KeyboardLayout {
    pub fn new(layout_id: HKL) -> Self {
        let mut sc_to_vk = BiMap::new();
        for sc in 1..=0x7f {
            let vk = unsafe { km_sys::MapVirtualKeyExW(sc.into(), wm_sys::MAPVK_VSC_TO_VK_EX, layout_id) };
            sc_to_vk.insert(sc, vk);
        }

        // This is for lpKeyState param of ToUnicodeEx, "A pointer to a 256-byte array that contains the current keyboard state. Each element (byte) in the array contains the state of one key. If the high-order bit of a byte is set, the key is down."
        let state = [0u8; 256];


        Self{
            sc_to_vk,
            state: [0u8; 256],
            strbuf: String::new(),
        }
    }
}