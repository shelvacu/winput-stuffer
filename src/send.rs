///! The goal of this module is to convert from `str` to `Input`s that should produce that string when put through SendInput

use std::io;
use std::borrow::Cow;

use crate::layout::KeyboardLayout;
use crate::input::{Input, KeyboardInput, KeyboardInputEnum};

// This is taken directly from https://github.com/openstenoproject/plover/blob/2ada7c71cd25a114e1439817a44206ff0da8b70e/plover/oslayer/winkeyboardcontrol.py#L52 warts and all
static EXTENDED_KEYS:phf::Set<u8> = phf::phf_set! {
    0xA2u8, // Control
    0xA3u8,
    0xA4u8, // Alt
    0xA5u8,
    0x2Du8, // Ins
    0x2Eu8, // Del
    0x21u8, // Home
    0x22u8, // End
    0x24u8, // Pg Up
    0x23u8, // Pg Down
    0x25u8, // Arrows
    0x26u8,
    0x27u8,
    0x28u8,
    0x90u8, // NumLock
    0x03u8, // Break
    0x2Cu8, // PrintScreen
    0x6Fu8, // Divide
};

pub fn key_event(
    keycode: u8,
    key_down: bool,
    msg: Option<super::window_message::WindowMessage>,
) -> KeyboardInput {
    KeyboardInput{
        e: KeyboardInputEnum::VirtualKeyCode{
            code: crate::input::VirtualKey(keycode.into()),
            extended: EXTENDED_KEYS.contains(&keycode),
        },
        key_up: !key_down,
        msg,
        time: None,
    }
}

struct KeyPressIter {
    keycode_list: Vec<u8>,
    i: usize,
    second_iter: bool,
    msg: Option<super::window_message::WindowMessage>,
}

impl Iterator for KeyPressIter {
    type Item = Input;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.keycode_list.len() {
            if self.second_iter {
                return None;
            } else {
                self.i = 0;
                self.second_iter = true;
            }
        }

        let keycode = self.keycode_list[self.i];
        let ki = key_event(keycode, !self.second_iter, self.msg.clone());

        self.i += 1;
        Some(Input::from_keyboard(&ki.into()))
    }
}

fn key_press(
    c:char,
    layout:&KeyboardLayout,
    msg: Option<super::window_message::WindowMessage>,
) -> impl Iterator<Item = Input> {
    let (vk, ss) = layout.char_to_vk_ss()[&c];
    let mut keycode_list = vec![];
    keycode_list.extend(&layout.ss_to_vks()[&ss]);
    keycode_list.push(vk);

    KeyPressIter {
        keycode_list,
        i: 0,
        second_iter: false,
        msg,
    }
}

fn key_unicode(
    c:char,
    msg: Option<super::window_message::WindowMessage>,
) -> impl Iterator<Item = Input> {
    let s:String = c.into();
    let mut res = vec![];
    for key_up in [false, true] {
        for wc in s.encode_utf16() {
            let k = KeyboardInput{
                e:KeyboardInputEnum::UnicodeCodeUnit(wc),
                key_up,
                msg: msg.clone(),
                time: None,
            };
            res.push(Input::from_keyboard(&k.into()));
        }
    }
    res.into_iter()
}

pub fn send_text_with_msg_layout(
    text: &str,
    msg: Option<super::window_message::WindowMessage>,
    layout: &KeyboardLayout
) -> io::Result<()> {
    let mut inputs = Vec::with_capacity(text.len());
    for c in text.chars() {
        if layout.char_to_vk_ss().contains_key(&c) {
            inputs.extend(key_press(c, &layout, msg.clone()));
        } else {
            inputs.extend(key_unicode(c, msg.clone()));
        }
    }
    dbg!(inputs.len());
    dbg!(crate::input::send_input(&inputs)).map(|_| ())
}

pub fn inputs_for_text(
    text: &str,
    layout: &KeyboardLayout,
    out: &mut Vec<Input>
) {
    for c in text.chars() {
        if layout.char_to_vk_ss().contains_key(&c) {
            out.extend(key_press(c, &layout, None));
        } else {
            out.extend(key_unicode(c, None));
        }
    }
}

pub fn send_text(
    text: &str,
) -> io::Result<()> {
    let layout = KeyboardLayout::current();
    send_text_with_msg_layout(text, None, &layout)
}

pub fn send_key(
    key: &str,
    key_down: bool,
) -> io::Result<()> {
    let layout = KeyboardLayout::current();
    send_key_layout(key, key_down, &layout)
}

pub fn send_key_layout(
    key: &str,
    key_down: bool,
    layout: &KeyboardLayout,
) -> io::Result<()> {
    let inputs = vec![input_for_key(key, key_down, layout)];
    dbg!(crate::input::send_input(&inputs)).map(|count| assert_eq!(inputs.len() as u32, count))
}

pub fn input_for_key<'a>(
    key: &'a str,
    key_down: bool,
    layout: &KeyboardLayout,
) -> Input {
    let key_borrow_garbage:Cow<'a, str> = key.into();
    let vk = layout.keyname_to_vk().get(&key_borrow_garbage).unwrap();
    let ki = key_event(*vk, key_down, None);
    Input::from_keyboard(&ki.into())
}