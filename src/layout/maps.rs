#![allow(dead_code)]
use bimap::BiMap;

use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref DEAD_KEYNAME: BiMap<&'static str, &'static str> = BiMap::from_iter([
        ("asciicircum", "dead_circumflex"),
        ("asciitilde",  "dead_tilde"),
    ].into_iter());

    pub(crate) static ref VIRTUAL_KEY_TO_CODE: BiMap<&'static str, u8> = BiMap::from_iter([
        ("LBUTTON",            0x01),
        ("RBUTTON",            0x02),
        ("CANCEL",             0x03),
        ("MBUTTON",            0x04),
        ("XBUTTON1",           0x05),
        ("XBUTTON2",           0x06),
        ("BACK",               0x08),
        ("TAB",                0x09),
        ("CLEAR",              0x0C),
        ("RETURN",             0x0D),
        ("SHIFT",              0x10),
        ("CONTROL",            0x11),
        ("MENU",               0x12),
        ("PAUSE",              0x13),
        ("CAPITAL",            0x14),
        ("KANA",               0x15),
        ("JUNJA",              0x17),
        ("FINAL",              0x18),
        ("HANJA",              0x19),
        ("ESCAPE",             0x1B),
        ("CONVERT",            0x1C),
        ("NONCONVERT",         0x1D),
        ("ACCEPT",             0x1E),
        ("MODECHANGE",         0x1F),
        ("SPACE",              0x20),
        ("PRIOR",              0x21),
        ("NEXT",               0x22),
        ("END",                0x23),
        ("HOME",               0x24),
        ("LEFT",               0x25),
        ("UP",                 0x26),
        ("RIGHT",              0x27),
        ("DOWN",               0x28),
        ("SELECT",             0x29),
        ("PRINT",              0x2A),
        ("EXECUTE",            0x2B),
        ("SNAPSHOT",           0x2C),
        ("INSERT",             0x2D),
        ("DELETE",             0x2E),
        ("HELP",               0x2F),
        ("DIGIT0",             0x30),
        ("DIGIT1",             0x31),
        ("DIGIT2",             0x32),
        ("DIGIT3",             0x33),
        ("DIGIT4",             0x34),
        ("DIGIT5",             0x35),
        ("DIGIT6",             0x36),
        ("DIGIT7",             0x37),
        ("DIGIT8",             0x38),
        ("DIGIT9",             0x39),
        ("A",                  0x41),
        ("B",                  0x42),
        ("C",                  0x43),
        ("D",                  0x44),
        ("E",                  0x45),
        ("F",                  0x46),
        ("G",                  0x47),
        ("H",                  0x48),
        ("I",                  0x49),
        ("J",                  0x4A),
        ("K",                  0x4B),
        ("L",                  0x4C),
        ("M",                  0x4D),
        ("N",                  0x4E),
        ("O",                  0x4F),
        ("P",                  0x50),
        ("Q",                  0x51),
        ("R",                  0x52),
        ("S",                  0x53),
        ("T",                  0x54),
        ("U",                  0x55),
        ("V",                  0x56),
        ("W",                  0x57),
        ("X",                  0x58),
        ("Y",                  0x59),
        ("Z",                  0x5A),
        ("LWIN",               0x5B),
        ("RWIN",               0x5C),
        ("APPS",               0x5D),
        ("SLEEP",              0x5F),
        ("NUMPAD0",            0x60),
        ("NUMPAD1",            0x61),
        ("NUMPAD2",            0x62),
        ("NUMPAD3",            0x63),
        ("NUMPAD4",            0x64),
        ("NUMPAD5",            0x65),
        ("NUMPAD6",            0x66),
        ("NUMPAD7",            0x67),
        ("NUMPAD8",            0x68),
        ("NUMPAD9",            0x69),
        ("MULTIPLY",           0x6A),
        ("ADD",                0x6B),
        ("SEPARATOR",          0x6C),
        ("SUBTRACT",           0x6D),
        ("DECIMAL",            0x6E),
        ("DIVIDE",             0x6F),
        ("F1",                 0x70),
        ("F2",                 0x71),
        ("F3",                 0x72),
        ("F4",                 0x73),
        ("F5",                 0x74),
        ("F6",                 0x75),
        ("F7",                 0x76),
        ("F8",                 0x77),
        ("F9",                 0x78),
        ("F10",                0x79),
        ("F11",                0x7A),
        ("F12",                0x7B),
        ("F13",                0x7C),
        ("F14",                0x7D),
        ("F15",                0x7E),
        ("F16",                0x7F),
        ("F17",                0x80),
        ("F18",                0x81),
        ("F19",                0x82),
        ("F20",                0x83),
        ("F21",                0x84),
        ("F22",                0x85),
        ("F23",                0x86),
        ("F24",                0x87),
        ("NUMLOCK",            0x90),
        ("SCROLL",             0x91),
        ("OEM_NEC_EQUAL",      0x92),
        ("OEM_FJ_JISHO",       0x92),
        ("OEM_FJ_MASSHOU",     0x93),
        ("OEM_FJ_TOUROKU",     0x94),
        ("OEM_FJ_LOYA",        0x95),
        ("OEM_FJ_ROYA",        0x96),
        ("LSHIFT",             0xA0),
        ("RSHIFT",             0xA1),
        ("LCONTROL",           0xA2),
        ("RCONTROL",           0xA3),
        ("LMENU",              0xA4),
        ("RMENU",              0xA5),
        ("BROWSER_BACK",       0xA6),
        ("BROWSER_FORWARD",    0xA7),
        ("BROWSER_REFRESH",    0xA8),
        ("BROWSER_STOP",       0xA9),
        ("BROWSER_SEARCH",     0xAA),
        ("BROWSER_FAVORITES",  0xAB),
        ("BROWSER_HOME",       0xAC),
        ("VOLUME_MUTE",        0xAD),
        ("VOLUME_DOWN",        0xAE),
        ("VOLUME_UP",          0xAF),
        ("MEDIA_NEXT_TRACK",   0xB0),
        ("MEDIA_PREV_TRACK",   0xB1),
        ("MEDIA_STOP",         0xB2),
        ("MEDIA_PLAY_PAUSE",   0xB3),
        ("LAUNCH_MAIL",        0xB4),
        ("LAUNCH_MEDIA_SELECT",0xB5),
        ("LAUNCH_APP1",        0xB6),
        ("LAUNCH_APP2",        0xB7),
        ("OEM_1",              0xBA),
        ("OEM_PLUS",           0xBB),
        ("OEM_COMMA",          0xBC),
        ("OEM_MINUS",          0xBD),
        ("OEM_PERIOD",         0xBE),
        ("OEM_2",              0xBF),
        ("OEM_3",              0xC0),
        ("OEM_4",              0xDB),
        ("OEM_5",              0xDC),
        ("OEM_6",              0xDD),
        ("OEM_7",              0xDE),
        ("OEM_8",              0xDF),
        ("OEM_AX",             0xE1),
        ("OEM_102",            0xE2),
        ("ICO_HELP",           0xE3),
        ("ICO_00",             0xE4),
        ("PROCESSKEY",         0xE5),
        ("ICO_CLEAR",          0xE6),
        ("PACKET",             0xE7),
        ("OEM_RESET",          0xE9),
        ("OEM_JUMP",           0xEA),
        ("OEM_PA1",            0xEB),
        ("OEM_PA2",            0xEC),
        ("OEM_PA3",            0xED),
        ("OEM_WSCTRL",         0xEE),
        ("OEM_CUSEL",          0xEF),
        ("OEM_ATTN",           0xF0),
        ("OEM_FINISH",         0xF1),
        ("OEM_COPY",           0xF2),
        ("OEM_AUTO",           0xF3),
        ("OEM_ENLW",           0xF4),
        ("OEM_BACKTAB",        0xF5),
        ("ATTN",               0xF6),
        ("CRSEL",              0xF7),
        ("EXSEL",              0xF8),
        ("EREOF",              0xF9),
        ("PLAY",               0xFA),
        ("ZOOM",               0xFB),
        ("NONAME",             0xFC),
        ("PA1",                0xFD),
        ("OEM_CLEAR",          0xFE),
    ].into_iter());

    pub(crate) static ref VIRTUAL_CODE_TO_ALT_NAME: BiMap<u8, &'static str> = BiMap::from_iter([
        ("ADD",                "kp_add"),
        ("APPS",               "menu"),
        ("BACK",               "backspace"),
        ("BROWSER_BACK",       "back"),
        ("BROWSER_FAVORITES",  "favorites"),
        ("BROWSER_FORWARD",    "forward"),
        ("BROWSER_HOME",       "homepage"),
        ("BROWSER_HOME",       "www"),
        ("BROWSER_REFRESH",    "refresh"),
        ("BROWSER_SEARCH",     "search"),
        ("BROWSER_STOP",       "stop"),
        ("CANCEL",             "cancel"),
        ("CAPITAL",            "caps_lock"),
        ("CLEAR",              "clear"),
        ("DECIMAL",            "kp_decimal"),
        ("DELETE",             "delete"),
        ("DIVIDE",             "kp_divide"),
        ("DOWN",               "down"),
        ("END",                "end"),
        ("ESCAPE",             "escape"),
        ("EXECUTE",            "execute"),
        ("F1",                 "f1"),
        ("F2",                 "f2"),
        ("F3",                 "f3"),
        ("F4",                 "f4"),
        ("F5",                 "f5"),
        ("F6",                 "f6"),
        ("F7",                 "f7"),
        ("F8",                 "f8"),
        ("F9",                 "f9"),
        ("F10",                "f10"),
        ("F11",                "f11"),
        ("F12",                "f12"),
        ("F13",                "f13"),
        ("F14",                "f14"),
        ("F15",                "f15"),
        ("F16",                "f16"),
        ("F17",                "f17"),
        ("F18",                "f18"),
        ("F19",                "f19"),
        ("F20",                "f20"),
        ("F21",                "f21"),
        ("F22",                "f22"),
        ("F23",                "f23"),
        ("F24",                "f24"),
        ("HELP",               "help"),
        ("HOME",               "home"),
        ("INSERT",             "insert"),
        ("LAUNCH_APP1",        "mycomputer"),
        ("LAUNCH_APP2",        "calculator"),
        ("LAUNCH_MAIL",        "mail"),
        ("LAUNCH_MEDIA_SELECT","audiomedia"),
        ("LCONTROL",           "control_l"),
        ("LEFT",               "left"),
        ("LMENU",              "alt_l"),
        ("LSHIFT",             "shift_l"),
        ("LWIN",               "super_l"),
        ("MEDIA_NEXT_TRACK",   "audionext"),
        ("MEDIA_PLAY_PAUSE",   "audiopause"),
        ("MEDIA_PLAY_PAUSE",   "audioplay"),
        ("MEDIA_PREV_TRACK",   "audioprev"),
        ("MEDIA_STOP",         "audiostop"),
        ("MODECHANGE",         "mode_switch"),
        ("MULTIPLY",           "kp_multiply"),
        ("NEXT",               "page_down"),
        ("NUMLOCK",            "num_lock"),
        ("NUMPAD0",            "kp_0"),
        ("NUMPAD1",            "kp_1"),
        ("NUMPAD2",            "kp_2"),
        ("NUMPAD3",            "kp_3"),
        ("NUMPAD4",            "kp_4"),
        ("NUMPAD5",            "kp_5"),
        ("NUMPAD6",            "kp_6"),
        ("NUMPAD7",            "kp_7"),
        ("NUMPAD8",            "kp_8"),
        ("NUMPAD9",            "kp_9"),
        ("OEM_PLUS",           "kp_equal"),
        ("PAUSE",              "pause"),
        ("PRIOR",              "page_up"),
        ("RCONTROL",           "control_r"),
        ("RETURN",             "return"),
        ("RIGHT",              "right"),
        ("RMENU",              "alt_r"),
        ("RSHIFT",             "shift_r"),
        ("RWIN",               "super_r"),
        ("SCROLL",             "scroll_lock"),
        ("SELECT",             "select"),
        ("SLEEP",              "standby"),
        ("SNAPSHOT",           "print"),
        ("SUBTRACT",           "kp_subtract"),
        ("TAB",                "tab"),
        ("UP",                 "up"),
        ("VOLUME_DOWN",        "audiolowervolume"),
        ("VOLUME_MUTE",        "audiomute"),
        ("VOLUME_UP",          "audioraisevolume"),
    ].into_iter().map(|(old,new)| (*VIRTUAL_KEY_TO_CODE.get_by_left(&old).unwrap(), new)));

    pub(crate) static ref KEYNAME_TO_CHAR: BiMap<&'static str, char> = BiMap::from_iter([
        ("aacute"            ,'\u{e1}'), // á
        ("acircumflex"       ,'\u{e2}'), // â
        ("acute"             ,'\u{b4}'), // ´
        ("adiaeresis"        ,'\u{e4}'), // ä
        ("ae"                ,'\u{e6}'), // æ
        ("agrave"            ,'\u{e0}'), // à
        ("ampersand"         ,     '&'), // &
        ("apostrophe"        ,    '\''), // '
        ("aring"             ,'\u{e5}'), // å
        ("asciicircum"       ,     '^'), // ^
        ("asciitilde"        ,     '~'), // ~
        ("asterisk"          ,     '*'), // *
        ("at"                ,     '@'), // @
        ("atilde"            ,'\u{e3}'), // ã
        ("backslash"         ,    '\\'), // \
        ("bar"               ,     '|'), // |
        ("braceleft"         ,     '{'), // {
        ("braceright"        ,     '}'), // }
        ("bracketleft"       ,     '['), // [
        ("bracketright"      ,     ']'), // ]
        ("brokenbar"         ,'\u{a6}'), // ¦
        ("ccedilla"          ,'\u{e7}'), // ç
        ("cedilla"           ,'\u{b8}'), // ¸
        ("cent"              ,'\u{a2}'), // ¢
        ("clear"             ,'\u{0b}'), // 
        ("colon"             ,     ':'), // :
        ("comma"             ,     ','), // ,
        ("copyright"         ,'\u{a9}'), // ©
        ("currency"          ,'\u{a4}'), // ¤
        ("degree"            ,'\u{b0}'), // °
        ("diaeresis"         ,'\u{a8}'), // ¨
        ("division"          ,'\u{f7}'), // ÷
        ("dollar"            ,     '$'), // $
        ("eacute"            ,'\u{e9}'), // é
        ("ecircumflex"       ,'\u{ea}'), // ê
        ("ediaeresis"        ,'\u{eb}'), // ë
        ("egrave"            ,'\u{e8}'), // è
        ("equal"             ,     '='), // =
        ("eth"               ,'\u{f0}'), // ð
        ("exclam"            ,     '!'), // !
        ("exclamdown"        ,'\u{a1}'), // ¡
        ("grave"             ,     '`'), // `
        ("greater"           ,     '>'), // >
        ("guillemotleft"     ,'\u{ab}'), // «
        ("guillemotright"    ,'\u{bb}'), // »
        ("hyphen"            ,'\u{ad}'), // ­
        ("iacute"            ,'\u{ed}'), // í
        ("icircumflex"       ,'\u{ee}'), // î
        ("idiaeresis"        ,'\u{ef}'), // ï
        ("igrave"            ,'\u{ec}'), // ì
        ("less"              ,     '<'), // <
        ("macron"            ,'\u{af}'), // ¯
        ("masculine"         ,'\u{ba}'), // º
        ("minus"             ,     '-'), // -
        ("mu"                ,'\u{b5}'), // µ
        ("multiply"          ,'\u{d7}'), // ×
        ("nobreakspace"      ,'\u{a0}'), //  
        ("notsign"           ,'\u{ac}'), // ¬
        ("ntilde"            ,'\u{f1}'), // ñ
        ("numbersign"        ,     '#'), // #
        ("oacute"            ,'\u{f3}'), // ó
        ("ocircumflex"       ,'\u{f4}'), // ô
        ("odiaeresis"        ,'\u{f6}'), // ö
        ("ograve"            ,'\u{f2}'), // ò
        ("onehalf"           ,'\u{bd}'), // ½
        ("onequarter"        ,'\u{bc}'), // ¼
        ("onesuperior"       ,'\u{b9}'), // ¹
        ("ooblique"          ,'\u{d8}'), // Ø
        ("ordfeminine"       ,'\u{aa}'), // ª
        ("oslash"            ,'\u{f8}'), // ø
        ("otilde"            ,'\u{f5}'), // õ
        ("paragraph"         ,'\u{b6}'), // ¶
        ("parenleft"         ,     '('), // (
        ("parenright"        ,     ')'), // )
        ("percent"           ,     '%'), // %
        ("period"            ,     '.'), // .
        ("periodcentered"    ,'\u{b7}'), // ·
        ("plus"              ,     '+'), // +
        ("plusminus"         ,'\u{b1}'), // ±
        ("question"          ,     '?'), // ?
        ("questiondown"      ,'\u{bf}'), // ¿
        ("quotedbl"          ,     '"'), // "
        ("quoteleft"         ,     '`'), // `
        ("quoteright"        ,    '\''), // '
        ("registered"        ,'\u{ae}'), // ®
        ("return"            ,    '\r'), // 
        ("section"           ,'\u{a7}'), // §
        ("semicolon"         ,     ';'), // ;
        ("slash"             ,     '/'), // /
        ("space"             ,     ' '), //  
        ("ssharp"            ,'\u{df}'), // ß
        ("sterling"          ,'\u{a3}'), // £
        ("tab"               ,    '\t'), // 	
        ("thorn"             ,'\u{fe}'), // þ
        ("threequarters"     ,'\u{be}'), // ¾
        ("threesuperior"     ,'\u{b3}'), // ³
        ("twosuperior"       ,'\u{b2}'), // ²
        ("uacute"            ,'\u{fa}'), // ú
        ("ucircumflex"       ,'\u{fb}'), // û
        ("udiaeresis"        ,'\u{fc}'), // ü
        ("ugrave"            ,'\u{f9}'), // ù
        ("underscore"        ,     '_'), // _
        ("yacute"            ,'\u{fd}'), // ý
        ("ydiaeresis"        ,'\u{ff}'), // ÿ
        ("yen"               ,'\u{a5}'), // ¥
    ].into_iter());
}



pub(crate) const VK_LBUTTON:u8             = 0x01;
pub(crate) const VK_RBUTTON:u8             = 0x02;
pub(crate) const VK_CANCEL:u8              = 0x03;
pub(crate) const VK_MBUTTON:u8             = 0x04;
pub(crate) const VK_XBUTTON1:u8            = 0x05;
pub(crate) const VK_XBUTTON2:u8            = 0x06;
pub(crate) const VK_BACK:u8                = 0x08;
pub(crate) const VK_TAB:u8                 = 0x09;
pub(crate) const VK_CLEAR:u8               = 0x0C;
pub(crate) const VK_RETURN:u8              = 0x0D;
pub(crate) const VK_SHIFT:u8               = 0x10;
pub(crate) const VK_CONTROL:u8             = 0x11;
pub(crate) const VK_MENU:u8                = 0x12;
pub(crate) const VK_PAUSE:u8               = 0x13;
pub(crate) const VK_CAPITAL:u8             = 0x14;
pub(crate) const VK_KANA:u8                = 0x15;
pub(crate) const VK_JUNJA:u8               = 0x17;
pub(crate) const VK_FINAL:u8               = 0x18;
pub(crate) const VK_HANJA:u8               = 0x19;
pub(crate) const VK_ESCAPE:u8              = 0x1B;
pub(crate) const VK_CONVERT:u8             = 0x1C;
pub(crate) const VK_NONCONVERT:u8          = 0x1D;
pub(crate) const VK_ACCEPT:u8              = 0x1E;
pub(crate) const VK_MODECHANGE:u8          = 0x1F;
pub(crate) const VK_SPACE:u8               = 0x20;
pub(crate) const VK_PRIOR:u8               = 0x21;
pub(crate) const VK_NEXT:u8                = 0x22;
pub(crate) const VK_END:u8                 = 0x23;
pub(crate) const VK_HOME:u8                = 0x24;
pub(crate) const VK_LEFT:u8                = 0x25;
pub(crate) const VK_UP:u8                  = 0x26;
pub(crate) const VK_RIGHT:u8               = 0x27;
pub(crate) const VK_DOWN:u8                = 0x28;
pub(crate) const VK_SELECT:u8              = 0x29;
pub(crate) const VK_PRINT:u8               = 0x2A;
pub(crate) const VK_EXECUTE:u8             = 0x2B;
pub(crate) const VK_SNAPSHOT:u8            = 0x2C;
pub(crate) const VK_INSERT:u8              = 0x2D;
pub(crate) const VK_DELETE:u8              = 0x2E;
pub(crate) const VK_HELP:u8                = 0x2F;
pub(crate) const VK_DIGIT0:u8              = 0x30;
pub(crate) const VK_DIGIT1:u8              = 0x31;
pub(crate) const VK_DIGIT2:u8              = 0x32;
pub(crate) const VK_DIGIT3:u8              = 0x33;
pub(crate) const VK_DIGIT4:u8              = 0x34;
pub(crate) const VK_DIGIT5:u8              = 0x35;
pub(crate) const VK_DIGIT6:u8              = 0x36;
pub(crate) const VK_DIGIT7:u8              = 0x37;
pub(crate) const VK_DIGIT8:u8              = 0x38;
pub(crate) const VK_DIGIT9:u8              = 0x39;
pub(crate) const VK_A:u8                   = 0x41;
pub(crate) const VK_B:u8                   = 0x42;
pub(crate) const VK_C:u8                   = 0x43;
pub(crate) const VK_D:u8                   = 0x44;
pub(crate) const VK_E:u8                   = 0x45;
pub(crate) const VK_F:u8                   = 0x46;
pub(crate) const VK_G:u8                   = 0x47;
pub(crate) const VK_H:u8                   = 0x48;
pub(crate) const VK_I:u8                   = 0x49;
pub(crate) const VK_J:u8                   = 0x4A;
pub(crate) const VK_K:u8                   = 0x4B;
pub(crate) const VK_L:u8                   = 0x4C;
pub(crate) const VK_M:u8                   = 0x4D;
pub(crate) const VK_N:u8                   = 0x4E;
pub(crate) const VK_O:u8                   = 0x4F;
pub(crate) const VK_P:u8                   = 0x50;
pub(crate) const VK_Q:u8                   = 0x51;
pub(crate) const VK_R:u8                   = 0x52;
pub(crate) const VK_S:u8                   = 0x53;
pub(crate) const VK_T:u8                   = 0x54;
pub(crate) const VK_U:u8                   = 0x55;
pub(crate) const VK_V:u8                   = 0x56;
pub(crate) const VK_W:u8                   = 0x57;
pub(crate) const VK_X:u8                   = 0x58;
pub(crate) const VK_Y:u8                   = 0x59;
pub(crate) const VK_Z:u8                   = 0x5A;
pub(crate) const VK_LWIN:u8                = 0x5B;
pub(crate) const VK_RWIN:u8                = 0x5C;
pub(crate) const VK_APPS:u8                = 0x5D;
pub(crate) const VK_SLEEP:u8               = 0x5F;
pub(crate) const VK_NUMPAD0:u8             = 0x60;
pub(crate) const VK_NUMPAD1:u8             = 0x61;
pub(crate) const VK_NUMPAD2:u8             = 0x62;
pub(crate) const VK_NUMPAD3:u8             = 0x63;
pub(crate) const VK_NUMPAD4:u8             = 0x64;
pub(crate) const VK_NUMPAD5:u8             = 0x65;
pub(crate) const VK_NUMPAD6:u8             = 0x66;
pub(crate) const VK_NUMPAD7:u8             = 0x67;
pub(crate) const VK_NUMPAD8:u8             = 0x68;
pub(crate) const VK_NUMPAD9:u8             = 0x69;
pub(crate) const VK_MULTIPLY:u8            = 0x6A;
pub(crate) const VK_ADD:u8                 = 0x6B;
pub(crate) const VK_SEPARATOR:u8           = 0x6C;
pub(crate) const VK_SUBTRACT:u8            = 0x6D;
pub(crate) const VK_DECIMAL:u8             = 0x6E;
pub(crate) const VK_DIVIDE:u8              = 0x6F;
pub(crate) const VK_F1:u8                  = 0x70;
pub(crate) const VK_F2:u8                  = 0x71;
pub(crate) const VK_F3:u8                  = 0x72;
pub(crate) const VK_F4:u8                  = 0x73;
pub(crate) const VK_F5:u8                  = 0x74;
pub(crate) const VK_F6:u8                  = 0x75;
pub(crate) const VK_F7:u8                  = 0x76;
pub(crate) const VK_F8:u8                  = 0x77;
pub(crate) const VK_F9:u8                  = 0x78;
pub(crate) const VK_F10:u8                 = 0x79;
pub(crate) const VK_F11:u8                 = 0x7A;
pub(crate) const VK_F12:u8                 = 0x7B;
pub(crate) const VK_F13:u8                 = 0x7C;
pub(crate) const VK_F14:u8                 = 0x7D;
pub(crate) const VK_F15:u8                 = 0x7E;
pub(crate) const VK_F16:u8                 = 0x7F;
pub(crate) const VK_F17:u8                 = 0x80;
pub(crate) const VK_F18:u8                 = 0x81;
pub(crate) const VK_F19:u8                 = 0x82;
pub(crate) const VK_F20:u8                 = 0x83;
pub(crate) const VK_F21:u8                 = 0x84;
pub(crate) const VK_F22:u8                 = 0x85;
pub(crate) const VK_F23:u8                 = 0x86;
pub(crate) const VK_F24:u8                 = 0x87;
pub(crate) const VK_NUMLOCK:u8             = 0x90;
pub(crate) const VK_SCROLL:u8              = 0x91;
pub(crate) const VK_OEM_NEC_EQUAL:u8       = 0x92;
pub(crate) const VK_OEM_FJ_JISHO:u8        = 0x92;
pub(crate) const VK_OEM_FJ_MASSHOU:u8      = 0x93;
pub(crate) const VK_OEM_FJ_TOUROKU:u8      = 0x94;
pub(crate) const VK_OEM_FJ_LOYA:u8         = 0x95;
pub(crate) const VK_OEM_FJ_ROYA:u8         = 0x96;
pub(crate) const VK_LSHIFT:u8              = 0xA0;
pub(crate) const VK_RSHIFT:u8              = 0xA1;
pub(crate) const VK_LCONTROL:u8            = 0xA2;
pub(crate) const VK_RCONTROL:u8            = 0xA3;
pub(crate) const VK_LMENU:u8               = 0xA4;
pub(crate) const VK_RMENU:u8               = 0xA5;
pub(crate) const VK_BROWSER_BACK:u8        = 0xA6;
pub(crate) const VK_BROWSER_FORWARD:u8     = 0xA7;
pub(crate) const VK_BROWSER_REFRESH:u8     = 0xA8;
pub(crate) const VK_BROWSER_STOP:u8        = 0xA9;
pub(crate) const VK_BROWSER_SEARCH:u8      = 0xAA;
pub(crate) const VK_BROWSER_FAVORITES:u8   = 0xAB;
pub(crate) const VK_BROWSER_HOME:u8        = 0xAC;
pub(crate) const VK_VOLUME_MUTE:u8         = 0xAD;
pub(crate) const VK_VOLUME_DOWN:u8         = 0xAE;
pub(crate) const VK_VOLUME_UP:u8           = 0xAF;
pub(crate) const VK_MEDIA_NEXT_TRACK:u8    = 0xB0;
pub(crate) const VK_MEDIA_PREV_TRACK:u8    = 0xB1;
pub(crate) const VK_MEDIA_STOP:u8          = 0xB2;
pub(crate) const VK_MEDIA_PLAY_PAUSE:u8    = 0xB3;
pub(crate) const VK_LAUNCH_MAIL:u8         = 0xB4;
pub(crate) const VK_LAUNCH_MEDIA_SELECT:u8 = 0xB5;
pub(crate) const VK_LAUNCH_APP1:u8         = 0xB6;
pub(crate) const VK_LAUNCH_APP2:u8         = 0xB7;
pub(crate) const VK_OEM_1:u8               = 0xBA;
pub(crate) const VK_OEM_PLUS:u8            = 0xBB;
pub(crate) const VK_OEM_COMMA:u8           = 0xBC;
pub(crate) const VK_OEM_MINUS:u8           = 0xBD;
pub(crate) const VK_OEM_PERIOD:u8          = 0xBE;
pub(crate) const VK_OEM_2:u8               = 0xBF;
pub(crate) const VK_OEM_3:u8               = 0xC0;
pub(crate) const VK_OEM_4:u8               = 0xDB;
pub(crate) const VK_OEM_5:u8               = 0xDC;
pub(crate) const VK_OEM_6:u8               = 0xDD;
pub(crate) const VK_OEM_7:u8               = 0xDE;
pub(crate) const VK_OEM_8:u8               = 0xDF;
pub(crate) const VK_OEM_AX:u8              = 0xE1;
pub(crate) const VK_OEM_102:u8             = 0xE2;
pub(crate) const VK_ICO_HELP:u8            = 0xE3;
pub(crate) const VK_ICO_00:u8              = 0xE4;
pub(crate) const VK_PROCESSKEY:u8          = 0xE5;
pub(crate) const VK_ICO_CLEAR:u8           = 0xE6;
pub(crate) const VK_PACKET:u8              = 0xE7;
pub(crate) const VK_OEM_RESET:u8           = 0xE9;
pub(crate) const VK_OEM_JUMP:u8            = 0xEA;
pub(crate) const VK_OEM_PA1:u8             = 0xEB;
pub(crate) const VK_OEM_PA2:u8             = 0xEC;
pub(crate) const VK_OEM_PA3:u8             = 0xED;
pub(crate) const VK_OEM_WSCTRL:u8          = 0xEE;
pub(crate) const VK_OEM_CUSEL:u8           = 0xEF;
pub(crate) const VK_OEM_ATTN:u8            = 0xF0;
pub(crate) const VK_OEM_FINISH:u8          = 0xF1;
pub(crate) const VK_OEM_COPY:u8            = 0xF2;
pub(crate) const VK_OEM_AUTO:u8            = 0xF3;
pub(crate) const VK_OEM_ENLW:u8            = 0xF4;
pub(crate) const VK_OEM_BACKTAB:u8         = 0xF5;
pub(crate) const VK_ATTN:u8                = 0xF6;
pub(crate) const VK_CRSEL:u8               = 0xF7;
pub(crate) const VK_EXSEL:u8               = 0xF8;
pub(crate) const VK_EREOF:u8               = 0xF9;
pub(crate) const VK_PLAY:u8                = 0xFA;
pub(crate) const VK_ZOOM:u8                = 0xFB;
pub(crate) const VK_NONAME:u8              = 0xFC;
pub(crate) const VK_PA1:u8                 = 0xFD;
pub(crate) const VK_OEM_CLEAR:u8           = 0xFE;