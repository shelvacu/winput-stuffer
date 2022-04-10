# win-input-stuffer

Utilities for safely interfacing with the windows API to simulate keyboard/mouse inputs.

## License

This project is licensed under GPLv2.

`src/layout.rs` is heavily based on the implementation of input stuffing in the [Plover project](https://github.com/openstenoproject/plover) ([licensed under GPLv2][license]), in particular [`winkeyboardcontrol.py`][winkeyboardcontrol] and [`winkeyboardlayout.py`][winkeyboardlayout].

[license](https://github.com/openstenoproject/plover/blob/2ada7c71cd25a114e1439817a44206ff0da8b70e/LICENSE.txt)
[winkeyboardcontrol](https://github.com/openstenoproject/plover/blob/2ada7c71cd25a114e1439817a44206ff0da8b70e/plover/oslayer/winkeyboardcontrol.py)
[winkeyboardlayout](https://github.com/openstenoproject/plover/blob/2ada7c71cd25a114e1439817a44206ff0da8b70e/plover/oslayer/winkeyboardlayout.py)