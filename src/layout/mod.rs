use std::borrow::Cow;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::collections::HashMap;

use bimap::BiMap;

use windows::Win32::UI::WindowsAndMessaging as wm_sys;
use windows::Win32::UI::Input::KeyboardAndMouse as km_sys;

pub use windows::Win32::UI::TextServices::HKL;

mod maps;
use maps::*;

const SHIFT_STATE_SHIFT:u8 = 0x01;
const SHIFT_STATE_CTRL:u8  = 0x02;
const SHIFT_STATE_MENU:u8  = 0x04;

const SHIFT_STATES:[u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7,];

fn shift_state_str(ss:u8) -> String {
    if ss == 0 {
        "Base".to_string()
    } else {
        let mut res = String::new();
        if ss & SHIFT_STATE_SHIFT > 0 {
            res.push_str("Shift ");
        }
        if ss & SHIFT_STATE_CTRL > 0 {
            res.push_str("Ctrl ");
        }
        if ss & SHIFT_STATE_MENU > 0 {
            res.push_str("Menu ");
        }
        res.truncate(res.len() - 1);
        res
    }
}

fn get_vk_code(mut name: &str) -> Option<u8> {
    if name == "HANGUEL" || name == "HANGUL" { name = "KANA" }
    if name == "KANJI" { name = "HANJA" }
    VIRTUAL_KEY_TO_CODE.get_by_left(&name).copied()
}

fn vk_to_str(vk:u8) -> Cow<'static, str> {
    if let Some(name) = VIRTUAL_KEY_TO_CODE.get_by_right(&vk) {
        Cow::Borrowed(*name)
    } else {
        Cow::Owned(format!("{}", vk))
    }
}

/// Calls GetKeyboardLayout(GetWindowThreadProcessId(GetForegroundWindow()), NULL)
/// Important note from windows docs:
///
/// > Since the keyboard layout can be dynamically changed, applications that cache information about the current keyboard layout should process the WM_INPUTLANGCHANGE message to be informed of changes in the input language.
pub fn current_layout_id() -> HKL {
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

#[derive(Debug)]
pub struct KeyboardLayout {
    layout_id: HKL,
    char_to_vk_ss: HashMap<char, (u8, u8)>,
    keyname_to_vk: HashMap<Cow<'static, str>,u8>,
    ss_to_vks: HashMap<u8, std::vec::Vec<u8>>,
}

impl KeyboardLayout {
    pub fn id(&self) -> HKL {
        self.layout_id
    }

    pub fn char_to_vk_ss(&self) -> &HashMap<char, (u8, u8)> {
        &self.char_to_vk_ss
    }

    pub fn keyname_to_vk(&self) -> &HashMap<Cow<'static, str>,u8> {
        &self.keyname_to_vk
    }

    pub fn ss_to_vks(&self) -> &HashMap<u8, std::vec::Vec<u8>> {
        &self.ss_to_vks
    }

    pub fn new(layout_id: HKL, debug: bool) -> Self {
        let mut sc_to_vk = BiMap::new();
        for sc in 1..=0x7fu8 {
            let vk = unsafe { km_sys::MapVirtualKeyExW(sc.into(), wm_sys::MAPVK_VSC_TO_VK_EX, layout_id) };
            if vk != 0 {
                sc_to_vk.insert(sc, vk.try_into().unwrap());
            }
        }

        // This is for lpKeyState param of ToUnicodeEx, "A pointer to a 256-byte array that contains the current keyboard state. Each element (byte) in the array contains the state of one key. If the high-order bit of a byte is set, the key is down."
        let mut state = [0u8; 256];
        let mut strbuf = [0u16; 8];

        let mut fill_state = |ss:u8, state: &mut [u8; 256]| {
            for (mod_state, mod_vk) in [
                (SHIFT_STATE_SHIFT, VK_SHIFT),
                (SHIFT_STATE_CTRL,  VK_CONTROL),
                (SHIFT_STATE_MENU,  VK_MENU),
            ] {
                state[mod_vk as usize] = if ss & mod_state != 0 { 0x80 } else { 0 }
            }
        };

        let mut to_unichr = |vk:u8, sc:u8, ss:u8| {
            fill_state(ss, &mut state);

            let rc = unsafe {
                km_sys::ToUnicodeEx(
                    vk.into(),
                    sc.into(),
                    &state,
                    strbuf.as_mut_slice(),
                    0,
                    layout_id,
                )
            };

            let dead_key;
            let mut c = None;
            if rc > 0 {
                dead_key = false;
                c = Some(strbuf[0..(rc as usize)].to_vec());
            } else if rc < 0 {
                // This was a "dead key"; it produces nothing by itself, but will aglutinate onto the next character. "Pressing" space should produce the character we want by itself.
                dead_key = true;
                fill_state(0, &mut state);
                let rc2 = unsafe {
                    km_sys::ToUnicodeEx(
                        VK_SPACE.into(),
                        (*sc_to_vk.get_by_right(&VK_SPACE).unwrap()).into(),
                        &state,
                        strbuf.as_mut_slice(),
                        0,
                        layout_id,
                    )
                };
                if rc2 > 0 {
                    c = Some(strbuf[0..(rc2 as usize)].to_vec());
                }
            } else { // rc == 0
                dead_key = false;
            }
            (c, dead_key)
        };

        fn sort_vk_ss_list(mut l:Vec<(u8,u8)>) -> Vec<(u8,u8)> {
            l.sort_unstable_by_key(|(_, ss)| ss.count_ones());
            l
        }

        let mut char_to_vk_sss:HashMap<char, Vec<(u8, u8)>> = HashMap::new();
        let mut keyname_to_vk_sss:HashMap<Cow<'static, str>, Vec<(u8, u8)>> = HashMap::new();

        let mut sc_vk:Vec<(u8, u8)> = sc_to_vk.iter().map(|(a, b)| (*a, *b)).collect();
        sc_vk.sort_unstable();
        for (sc, vk) in sc_vk {
            for ss_ref in &SHIFT_STATES {
                let ss = *ss_ref;
                if ss == SHIFT_STATE_MENU || ss == (SHIFT_STATE_SHIFT | SHIFT_STATE_MENU) {
                    // "Alt and Shift+Alt don't work, so skip them."
                    continue;
                }
                let (maybe_c, dead_key) = to_unichr(vk, sc, ss);
                if debug {
                    if let Some(c) = maybe_c.as_ref() {
                        println!(
                            "{}{} -> {:?} [{:?}] {}",
                            shift_state_str(ss),
                            vk_to_str(vk),
                            OsString::from_wide(c.as_ref()),
                            c,
                            if dead_key { "[dead key]" } else { "" }
                        );
                    }
                }
                let mut kn:Option<Cow<'static, str>> = None;
                if let Some(c_vec) = maybe_c.as_ref() {
                    if c_vec.len() == 1 {
                        let c_u16:u16 = c_vec[0];
                        let c_u8:u8   = c_u16.try_into().unwrap();
                        let c_u32:u32 = c_u16.into();
                        let c_char:char = c_u32.try_into().unwrap();
                        kn = KEYNAME_TO_CHAR.get_by_right(&c_char).copied().map(std::convert::Into::into);
                        if dead_key {
                            if let Some(old_kn) = kn {
                                let maybe_key:Option<Cow<'static, str>> = DEAD_KEYNAME.get_by_left(old_kn.as_ref()).copied().map(std::convert::Into::into);
                                kn = Some(maybe_key.unwrap_or(format!("dead_{}", old_kn.as_ref()).into()));
                            }
                        } else {
                            char_to_vk_sss.entry(c_char).or_default().push((vk, ss));
                        }
                        if let Some(kn) = kn {
                            keyname_to_vk_sss.entry(kn).or_default().push((vk, ss));
                        }
                    }
                }
            }
        }

        let mut char_to_vk_ss = HashMap::new();
        for (c, vk_ss_list) in char_to_vk_sss {
            char_to_vk_ss.insert(c, sort_vk_ss_list(vk_ss_list)[0]);
        }
        char_to_vk_ss.insert('\n',char_to_vk_ss[&'\r'].clone());

        let mut keyname_to_vk:HashMap<Cow<'static, str>,u8> = HashMap::new();
        for (vk, kn) in VIRTUAL_CODE_TO_ALT_NAME.iter() {
            keyname_to_vk.insert((*kn).into(), *vk);
        }
        keyname_to_vk.insert("next".into(), VK_NEXT);
        keyname_to_vk.insert("prior".into(), VK_PRIOR);
        keyname_to_vk.insert("kp_delete".into(), VK_DELETE);
        keyname_to_vk.insert("kp_enter".into(), VK_RETURN);
        keyname_to_vk.insert("break".into(), VK_CANCEL);
        for (kn, vk_ss_list) in keyname_to_vk_sss {
            keyname_to_vk.insert(kn, sort_vk_ss_list(vk_ss_list)[0].0);
        }
        // add_modifiers_aliases(&mut keyname_to_vk);

        let mut ss_to_vks = HashMap::new();
        for ss in SHIFT_STATES {
            let mut vk_list = vec![];
            for (mod_state, mod_vk) in [
                (SHIFT_STATE_SHIFT, VK_SHIFT),
                (SHIFT_STATE_CTRL,  VK_CONTROL),
                (SHIFT_STATE_MENU,  VK_MENU),
            ] {
                if ss & mod_state > 0 {
                    vk_list.push(mod_vk);
                }
            }
            ss_to_vks.insert(ss, vk_list);
        }

        Self{
            layout_id,
            char_to_vk_ss,
            keyname_to_vk,
            ss_to_vks,
        }
    }
}