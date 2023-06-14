/*
 *  Crate: Wyn
 * Module: Win32 - Inputs
 */

//! Types for handling User-Input, such as through Mice/Pointers, Keyboards, and Controllers.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::errors::*;
use super::event_loop::EventLoop;

// ================================================================================================================================ //

/// Native OS Representation for Mouse Buttons.
pub type NativeMouseButton = sys::VIRTUAL_KEY;

/// Native OS Representation for Virtual Key Codes.
pub type NativeKeyCode = sys::VIRTUAL_KEY;

/// Unit for representing Scrolling distance.
pub type ScrollDelta = f64;

// -------------------------------------------------------------------------------------------------------------------------------- //

/// A button on a mouse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct MouseButton(pub NativeMouseButton);

/// A key on a keyboard.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct KeyCode(pub NativeKeyCode);

pub use kc_constants::*;
pub use mb_constants::*;

// -------------------------------------------------------------------------------------------------------------------------------- //

// The Mouse-Button and Key-Code constants do not have doc comments, as the name itself is the documentation.

#[rustfmt::skip]
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
mod mb_constants {
    use super::MouseButton;

    pub const MB_LEFT  : MouseButton = MouseButton(sys::VK_LBUTTON); // 1
    pub const MB_RIGHT : MouseButton = MouseButton(sys::VK_RBUTTON); // 2
    pub const MB_MIDDLE: MouseButton = MouseButton(sys::VK_MBUTTON); // 4
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
mod kc_constants {
    use super::KeyCode;
    
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(0);                                       //   0
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_LBUTTON);                         //   1
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_RBUTTON);                         //   2
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_CANCEL);                          //   3
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_MBUTTON);                         //   4
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_XBUTTON1);                        //   5
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_XBUTTON2);                        //   6
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(7 /* UNDEFINED */);                       //   7
    pub const KC_BACKSPACE    : KeyCode            = KeyCode(sys::VK_BACK);                            //   8
    pub const KC_TAB          : KeyCode            = KeyCode(sys::VK_TAB);                             //   9
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(10 /* RESERVED */);                       //  10
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(11 /* RESERVED */);                       //  11
    pub const KC_CLEAR        : KeyCode            = KeyCode(sys::VK_CLEAR);                           //  12
    pub const KC_ENTER        : KeyCode            = KeyCode(sys::VK_RETURN);                          //  13
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(14 /* UNDEFINED */);                      //  14
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(15 /* UNDEFINED */);                      //  15
    pub const KC_SHIFT        : KeyCode            = KeyCode(sys::VK_SHIFT);                           //  16
    pub const KC_CONTROL      : KeyCode            = KeyCode(sys::VK_CONTROL);                         //  17
    pub const KC_ALT          : KeyCode            = KeyCode(sys::VK_MENU);                            //  18
    pub const KC_PAUSE        : KeyCode            = KeyCode(sys::VK_PAUSE);                           //  19
    pub const KC_CAPSLOCK     : KeyCode            = KeyCode(sys::VK_CAPITAL);                         //  20
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_KANA);                            //  21
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_IME_ON);                          //  22
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_JUNJA);                           //  23
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_FINAL);                           //  24
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_KANJI);                           //  25
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_IME_OFF);                         //  26
    pub const KC_ESCAPE       : KeyCode            = KeyCode(sys::VK_ESCAPE);                          //  27
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_CONVERT);                         //  28
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NONCONVERT);                      //  29
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_ACCEPT);                          //  30
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_MODECHANGE);                      //  31
    pub const KC_SPACE        : KeyCode            = KeyCode(sys::VK_SPACE);                           //  32
    pub const KC_PAGEUP       : KeyCode            = KeyCode(sys::VK_PRIOR);                           //  33
    pub const KC_PAGEDOWN     : KeyCode            = KeyCode(sys::VK_NEXT);                            //  34
    pub const KC_END          : KeyCode            = KeyCode(sys::VK_END);                             //  35
    pub const KC_HOME         : KeyCode            = KeyCode(sys::VK_HOME);                            //  36
    pub const KC_LEFT         : KeyCode            = KeyCode(sys::VK_LEFT);                            //  37
    pub const KC_UP           : KeyCode            = KeyCode(sys::VK_UP);                              //  38
    pub const KC_RIGHT        : KeyCode            = KeyCode(sys::VK_RIGHT);                           //  39
    pub const KC_DOWN         : KeyCode            = KeyCode(sys::VK_DOWN);                            //  40
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_SELECT);                          //  41
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_PRINT);                           //  42
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_EXECUTE);                         //  43
    pub const KC_PRINTSCREEN  : KeyCode            = KeyCode(sys::VK_SNAPSHOT);                        //  44
    pub const KC_INSERT       : KeyCode            = KeyCode(sys::VK_INSERT);                          //  45
    pub const KC_DELETE       : KeyCode            = KeyCode(sys::VK_DELETE);                          //  46
    pub const KC_HELP         : KeyCode            = KeyCode(sys::VK_HELP);                            //  47
    pub const KC_NUM0         : KeyCode            = KeyCode(sys::VK_0);                               //  48
    pub const KC_NUM1         : KeyCode            = KeyCode(sys::VK_1);                               //  49
    pub const KC_NUM2         : KeyCode            = KeyCode(sys::VK_2);                               //  50
    pub const KC_NUM3         : KeyCode            = KeyCode(sys::VK_3);                               //  51
    pub const KC_NUM4         : KeyCode            = KeyCode(sys::VK_4);                               //  52
    pub const KC_NUM5         : KeyCode            = KeyCode(sys::VK_5);                               //  53
    pub const KC_NUM6         : KeyCode            = KeyCode(sys::VK_6);                               //  54
    pub const KC_NUM7         : KeyCode            = KeyCode(sys::VK_7);                               //  55
    pub const KC_NUM8         : KeyCode            = KeyCode(sys::VK_8);                               //  56
    pub const KC_NUM9         : KeyCode            = KeyCode(sys::VK_9);                               //  57
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(58 /* UNDEFINED */);                      //  58
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(59 /* UNDEFINED */);                      //  59
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(60 /* UNDEFINED */);                      //  60
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(61 /* UNDEFINED */);                      //  61
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(62 /* UNDEFINED */);                      //  62
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(63 /* UNDEFINED */);                      //  63
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(64 /* UNDEFINED */);                      //  64
    pub const KC_A            : KeyCode            = KeyCode(sys::VK_A);                               //  65
    pub const KC_B            : KeyCode            = KeyCode(sys::VK_B);                               //  66
    pub const KC_C            : KeyCode            = KeyCode(sys::VK_C);                               //  67
    pub const KC_D            : KeyCode            = KeyCode(sys::VK_D);                               //  68
    pub const KC_E            : KeyCode            = KeyCode(sys::VK_E);                               //  69
    pub const KC_F            : KeyCode            = KeyCode(sys::VK_F);                               //  70
    pub const KC_G            : KeyCode            = KeyCode(sys::VK_G);                               //  71
    pub const KC_H            : KeyCode            = KeyCode(sys::VK_H);                               //  72
    pub const KC_I            : KeyCode            = KeyCode(sys::VK_I);                               //  73
    pub const KC_J            : KeyCode            = KeyCode(sys::VK_J);                               //  74
    pub const KC_K            : KeyCode            = KeyCode(sys::VK_K);                               //  75
    pub const KC_L            : KeyCode            = KeyCode(sys::VK_L);                               //  76
    pub const KC_M            : KeyCode            = KeyCode(sys::VK_M);                               //  77
    pub const KC_N            : KeyCode            = KeyCode(sys::VK_N);                               //  78
    pub const KC_O            : KeyCode            = KeyCode(sys::VK_O);                               //  79
    pub const KC_P            : KeyCode            = KeyCode(sys::VK_P);                               //  80
    pub const KC_Q            : KeyCode            = KeyCode(sys::VK_Q);                               //  81
    pub const KC_R            : KeyCode            = KeyCode(sys::VK_R);                               //  82
    pub const KC_S            : KeyCode            = KeyCode(sys::VK_S);                               //  83
    pub const KC_T            : KeyCode            = KeyCode(sys::VK_T);                               //  84
    pub const KC_U            : KeyCode            = KeyCode(sys::VK_U);                               //  85
    pub const KC_V            : KeyCode            = KeyCode(sys::VK_V);                               //  86
    pub const KC_W            : KeyCode            = KeyCode(sys::VK_W);                               //  87
    pub const KC_X            : KeyCode            = KeyCode(sys::VK_X);                               //  88
    pub const KC_Y            : KeyCode            = KeyCode(sys::VK_Y);                               //  89
    pub const KC_Z            : KeyCode            = KeyCode(sys::VK_Z);                               //  90
    pub const KC_LSTART       : KeyCode            = KeyCode(sys::VK_LWIN);                            //  91
    pub const KC_RSTART       : KeyCode            = KeyCode(sys::VK_RWIN);                            //  92
    pub const KC_MENU         : KeyCode            = KeyCode(sys::VK_APPS);                            //  93
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(94 /* RESERVED */);                       //  94
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_SLEEP);                           //  95
    pub const KC_NUMPAD0      : KeyCode            = KeyCode(sys::VK_NUMPAD0);                         //  96
    pub const KC_NUMPAD1      : KeyCode            = KeyCode(sys::VK_NUMPAD1);                         //  97
    pub const KC_NUMPAD2      : KeyCode            = KeyCode(sys::VK_NUMPAD2);                         //  98
    pub const KC_NUMPAD3      : KeyCode            = KeyCode(sys::VK_NUMPAD3);                         //  99
    pub const KC_NUMPAD4      : KeyCode            = KeyCode(sys::VK_NUMPAD4);                         // 100
    pub const KC_NUMPAD5      : KeyCode            = KeyCode(sys::VK_NUMPAD5);                         // 101
    pub const KC_NUMPAD6      : KeyCode            = KeyCode(sys::VK_NUMPAD6);                         // 102
    pub const KC_NUMPAD7      : KeyCode            = KeyCode(sys::VK_NUMPAD7);                         // 103
    pub const KC_NUMPAD8      : KeyCode            = KeyCode(sys::VK_NUMPAD8);                         // 104
    pub const KC_NUMPAD9      : KeyCode            = KeyCode(sys::VK_NUMPAD9);                         // 105
    pub const KC_MULTIPLY     : KeyCode            = KeyCode(sys::VK_MULTIPLY);                        // 106
    pub const KC_ADD          : KeyCode            = KeyCode(sys::VK_ADD);                             // 107
    pub const KC_SEPARATOR    : KeyCode            = KeyCode(sys::VK_SEPARATOR);                       // 108
    pub const KC_SUBTRACT     : KeyCode            = KeyCode(sys::VK_SUBTRACT);                        // 109
    pub const KC_DECIMAL      : KeyCode            = KeyCode(sys::VK_DECIMAL);                         // 110
    pub const KC_DIVIDE       : KeyCode            = KeyCode(sys::VK_DIVIDE);                          // 111
    pub const KC_F1           : KeyCode            = KeyCode(sys::VK_F1);                              // 112
    pub const KC_F2           : KeyCode            = KeyCode(sys::VK_F2);                              // 113
    pub const KC_F3           : KeyCode            = KeyCode(sys::VK_F3);                              // 114
    pub const KC_F4           : KeyCode            = KeyCode(sys::VK_F4);                              // 115
    pub const KC_F5           : KeyCode            = KeyCode(sys::VK_F5);                              // 116
    pub const KC_F6           : KeyCode            = KeyCode(sys::VK_F6);                              // 117
    pub const KC_F7           : KeyCode            = KeyCode(sys::VK_F7);                              // 118
    pub const KC_F8           : KeyCode            = KeyCode(sys::VK_F8);                              // 119
    pub const KC_F9           : KeyCode            = KeyCode(sys::VK_F9);                              // 120
    pub const KC_F10          : KeyCode            = KeyCode(sys::VK_F10);                             // 121
    pub const KC_F11          : KeyCode            = KeyCode(sys::VK_F11);                             // 122
    pub const KC_F12          : KeyCode            = KeyCode(sys::VK_F12);                             // 123
    pub const KC_F13          : KeyCode            = KeyCode(sys::VK_F13);                             // 124
    pub const KC_F14          : KeyCode            = KeyCode(sys::VK_F14);                             // 125
    pub const KC_F15          : KeyCode            = KeyCode(sys::VK_F15);                             // 126
    pub const KC_F16          : KeyCode            = KeyCode(sys::VK_F16);                             // 127
    pub const KC_F17          : KeyCode            = KeyCode(sys::VK_F17);                             // 128
    pub const KC_F18          : KeyCode            = KeyCode(sys::VK_F18);                             // 129
    pub const KC_F19          : KeyCode            = KeyCode(sys::VK_F19);                             // 130
    pub const KC_F20          : KeyCode            = KeyCode(sys::VK_F20);                             // 131
    pub const KC_F21          : KeyCode            = KeyCode(sys::VK_F21);                             // 132
    pub const KC_F22          : KeyCode            = KeyCode(sys::VK_F22);                             // 133
    pub const KC_F23          : KeyCode            = KeyCode(sys::VK_F23);                             // 134
    pub const KC_F24          : KeyCode            = KeyCode(sys::VK_F24);                             // 135
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_VIEW);                 // 136
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_MENU);                 // 137
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_UP);                   // 138
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_DOWN);                 // 139
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_LEFT);                 // 140
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_RIGHT);                // 141
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_ACCEPT);               // 142
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NAVIGATION_CANCEL);               // 143
    pub const KC_NUMLOCK      : KeyCode            = KeyCode(sys::VK_NUMLOCK);                         // 144
    pub const KC_SCROLLLOCK   : KeyCode            = KeyCode(sys::VK_SCROLL);                          // 145
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_FJ_JISHO);                    // 146
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_FJ_MASSHOU);                  // 147
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_FJ_TOUROKU);                  // 148
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_FJ_LOYA);                     // 149
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_FJ_ROYA);                     // 150
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(151 /* UNASSIGNED */);                    // 151
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(152 /* UNASSIGNED */);                    // 152
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(153 /* UNASSIGNED */);                    // 153
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(154 /* UNASSIGNED */);                    // 154
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(155 /* UNASSIGNED */);                    // 155
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(156 /* UNASSIGNED */);                    // 156
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(157 /* UNASSIGNED */);                    // 157
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(158 /* UNASSIGNED */);                    // 158
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(159 /* UNASSIGNED */);                    // 159
    pub const KC_LSHIFT       : KeyCode            = KeyCode(sys::VK_LSHIFT);                          // 160
    pub const KC_RSHIFT       : KeyCode            = KeyCode(sys::VK_RSHIFT);                          // 161
    pub const KC_LCONTROL     : KeyCode            = KeyCode(sys::VK_LCONTROL);                        // 162
    pub const KC_RCONTROL     : KeyCode            = KeyCode(sys::VK_RCONTROL);                        // 163
    pub const KC_LALT         : KeyCode            = KeyCode(sys::VK_LMENU);                           // 164
    pub const KC_RALT         : KeyCode            = KeyCode(sys::VK_RMENU);                           // 165
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_BROWSER_BACK);                    // 166
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_BROWSER_FORWARD);                 // 167
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_BROWSER_REFRESH);                 // 168
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_BROWSER_STOP);                    // 169
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_BROWSER_SEARCH);                  // 170
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_BROWSER_FAVORITES);               // 171
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_BROWSER_HOME);                    // 172
    pub const KC_VOLUMEMUTE   : KeyCode            = KeyCode(sys::VK_VOLUME_MUTE);                     // 173
    pub const KC_VOLUMEDOWN   : KeyCode            = KeyCode(sys::VK_VOLUME_DOWN);                     // 174
    pub const KC_VOLUMEUP     : KeyCode            = KeyCode(sys::VK_VOLUME_UP);                       // 175
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_MEDIA_NEXT_TRACK);                // 176
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_MEDIA_PREV_TRACK);                // 177
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_MEDIA_STOP);                      // 178
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_MEDIA_PLAY_PAUSE);                // 179
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_LAUNCH_MAIL);                     // 180
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_LAUNCH_MEDIA_SELECT);             // 181
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_LAUNCH_APP1);                     // 182
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_LAUNCH_APP2);                     // 183
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(184 /* RESERVED */);                      // 184
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(185 /* RESERVED */);                      // 185
    pub const KC_COLON        : KeyCode            = KeyCode(sys::VK_OEM_1);                           // 186
    pub const KC_PLUS         : KeyCode            = KeyCode(sys::VK_OEM_PLUS);                        // 187
    pub const KC_COMMA        : KeyCode            = KeyCode(sys::VK_OEM_COMMA);                       // 188
    pub const KC_MINUS        : KeyCode            = KeyCode(sys::VK_OEM_MINUS);                       // 189
    pub const KC_PERIOD       : KeyCode            = KeyCode(sys::VK_OEM_PERIOD);                      // 190
    pub const KC_FORWARDSLASH : KeyCode            = KeyCode(sys::VK_OEM_2);                           // 191
    pub const KC_BACKTICK     : KeyCode            = KeyCode(sys::VK_OEM_3);                           // 192
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(193 /* RESERVED */);                      // 193
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(194 /* RESERVED */);                      // 194
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_A);                       // 195
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_B);                       // 196
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_X);                       // 197
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_Y);                       // 198
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_RIGHT_SHOULDER);          // 199
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_LEFT_SHOULDER);           // 200
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_LEFT_TRIGGER);            // 201
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_RIGHT_TRIGGER);           // 202
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_DPAD_UP);                 // 203
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_DPAD_DOWN);               // 204
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_DPAD_LEFT);               // 205
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_DPAD_RIGHT);              // 206
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_MENU);                    // 207
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_VIEW);                    // 208
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON);  // 209
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON); // 210
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_LEFT_THUMBSTICK_UP);      // 211
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_LEFT_THUMBSTICK_DOWN);    // 212
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT);   // 213
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_LEFT_THUMBSTICK_LEFT);    // 214
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_RIGHT_THUMBSTICK_UP);     // 215
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN);   // 216
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT);  // 217
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT);   // 218
    pub const KC_LBRACKET     : KeyCode            = KeyCode(sys::VK_OEM_4);                           // 219
    pub const KC_BACKSLASH    : KeyCode            = KeyCode(sys::VK_OEM_5);                           // 220
    pub const KC_RBRACKET     : KeyCode            = KeyCode(sys::VK_OEM_6);                           // 221
    pub const KC_QUOTE        : KeyCode            = KeyCode(sys::VK_OEM_7);                           // 222
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_8);                           // 223
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(224 /* RESERVED */);                      // 224
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_AX);                          // 225
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_102);                         // 226
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_ICO_HELP);                        // 227
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_ICO_00);                          // 228
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_PROCESSKEY);                      // 229
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_ICO_CLEAR);                       // 230
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_PACKET);                          // 231
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(232 /* UNASSIGNED */);                    // 232
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_RESET);                       // 233
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_JUMP);                        // 234
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_PA1);                         // 235
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_PA2);                         // 236
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_PA3);                         // 237
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_WSCTRL);                      // 238
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_CUSEL);                       // 239
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_ATTN);                        // 240
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_FINISH);                      // 241
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_COPY);                        // 242
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_AUTO);                        // 243
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_ENLW);                        // 244
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_BACKTAB);                     // 245
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_ATTN);                            // 246
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_CRSEL);                           // 247
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_EXSEL);                           // 248
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_EREOF);                           // 249
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_PLAY);                            // 250
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_ZOOM);                            // 251
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_NONAME);                          // 252
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_PA1);                             // 253
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(sys::VK_OEM_CLEAR);                       // 254
  //pub const KC_UNKNOWN      : KeyCode            = KeyCode(255 /* UNDOCUMENTED */);                  // 255
}

// ================================================================================================================================ //

/// An XInput Controller.
#[repr(transparent)]
pub struct XInputController {
    /// Internal XInput State.
    state: sys::XINPUT_STATE,
}

/// Maximum number of connected controllers.
pub const MAX_CONTROLLERS: usize = sys::XUSER_MAX_COUNT as usize;

// -------------------------------------------------------------------------------------------------------------------------------- //

impl XInputController {
    /// Returns an array of all potentially connected controllers.
    pub fn collect(_events: &EventLoop) -> WinResult<[Option<XInputController>; MAX_CONTROLLERS]> {
        let mut controllers: [Option<XInputController>; MAX_CONTROLLERS] = Default::default();

        for (i, controller) in controllers.iter_mut().enumerate() {
            // SAFETY: C-structs are safe to zero-initialize.
            let mut state = unsafe { zeroed() };
            // SAFETY: The pointer is guaranteed to be valid.
            let res = unsafe { sys::XInputGetState(i as u32, addr_of_mut!(state)) };

            match WinError::new(res) {
                None => {
                    // No Error, controller data is valid
                    *controller = Some(XInputController { state })
                }
                Some(err) => {
                    if err.code() == sys::ERROR_DEVICE_NOT_CONNECTED {
                        // No Controller
                        *controller = None;
                    } else {
                        // Error reading data
                        return Err(err);
                    }
                }
            }
        }

        Ok(controllers)
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl XInputController {
    /// Returns the state of the A Button.
    pub fn a(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_A != 0
    }

    /// Returns the state of the B Button.
    pub fn b(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_B != 0
    }

    /// Returns the state of the X Button.
    pub fn x(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_X != 0
    }

    /// Returns the state of the Y Button.
    pub fn y(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_Y != 0
    }

    /// Returns the state of the BACK Button.
    pub fn back(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_BACK != 0
    }

    /// Returns the state of the START Button.
    pub fn start(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_START != 0
    }

    /// Returns the state of the D-Pad Up Button.
    pub fn dpad_u(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_DPAD_UP != 0
    }

    /// Returns the state of the D-Pad Down Button.
    pub fn dpad_d(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_DPAD_DOWN != 0
    }

    /// Returns the state of the D-Pad Left Button.
    pub fn dpad_l(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_DPAD_LEFT != 0
    }

    /// Returns the state of the D-Pad Right Button.
    pub fn dpad_r(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_DPAD_RIGHT != 0
    }

    /// Returns the state of the Left Bumper.
    pub fn bumper_l(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_LEFT_SHOULDER != 0
    }

    /// Returns the state of the Right Bumper.
    pub fn bumper_r(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_RIGHT_SHOULDER != 0
    }

    /// Returns the state of the Left Trigger.
    /// The float is a normalized [0..1] value indicating how far the trigger was pressed.
    /// The boolean indicates whether or not the register-threshold was exceeded.
    pub fn trigger_l(&self) -> (f32, bool) {
        let bf = self.state.Gamepad.bLeftTrigger as f32 / 255.0;
        let bt = self.state.Gamepad.bLeftTrigger as u16 > sys::XINPUT_GAMEPAD_TRIGGER_THRESHOLD;
        (bf, bt)
    }

    /// Returns the state of the Right Trigger.
    /// The float is a normalized [0..1] value indicating how far the trigger was pressed.
    /// The boolean indicates whether or not the register-threshold was exceeded.
    pub fn trigger_r(&self) -> (f32, bool) {
        let bf = self.state.Gamepad.bRightTrigger as f32 / 255.0;
        let bt = self.state.Gamepad.bRightTrigger as u16 > sys::XINPUT_GAMEPAD_TRIGGER_THRESHOLD;
        (bf, bt)
    }

    /// Returns the state of the Left Thumbstick Button.
    pub fn thumb_l(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_LEFT_THUMB != 0
    }

    /// Returns the state of the Right Thumbstick Button.
    pub fn thumb_r(&self) -> bool {
        self.state.Gamepad.wButtons & sys::XINPUT_GAMEPAD_RIGHT_THUMB != 0
    }

    /// Returns the normalized [-1..1] values indicating the (x, y) coordinates of the Left Stick.
    pub fn stick_l(&self) -> (f32, f32) {
        let x = self.state.Gamepad.sThumbLX as f32 / 32768.0;
        let y = self.state.Gamepad.sThumbLY as f32 / 32768.0;
        (x, y)
    }

    /// Returns the normalized [-1..1] values indicating the (x, y) coordinates of the Right Stick.
    pub fn stick_r(&self) -> (f32, f32) {
        let x = self.state.Gamepad.sThumbRX as f32 / 32768.0;
        let y = self.state.Gamepad.sThumbRY as f32 / 32768.0;
        (x, y)
    }

    /// The Packet-Number associated with this controller.
    pub fn packet(&self) -> sys::DWORD {
        self.state.dwPacketNumber
    }
}

// ================================================================================================================================ //
