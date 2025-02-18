use input_linux::sys;
use regex::{Captures, Regex, Replacer};
// use notify_rust::{Notification, NotificationHandle};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use xcb::Xid;

use x11::xlib;

const ACTION_REMAP: i32 = 0;
const ACTION_MACRO: i32 = -1;
const ACTION_PREFIX: i32 = -2;
const ACTION_FUNCTION: i32 = -3;

const REMAP_INDEX_OTHERS: usize = 2;
const REMAP_INDEX_SELECT: usize = 3;
const REMAP_INDEX_CTRL_X: usize = 4;
const REMAP_INDEX_CTRL_C: usize = 5;

const ACTION_MACRO_START: i32 = 0;
const ACTION_MACRO_STOP: i32 = 1;
const ACTION_MACRO_EXECUTE: i32 = 2;
const ACTION_FUNCTION_CLOSE: i32 = 0;

// [ DEBUG
// const KEY_NAMES: [&str; 249] = [
//   "KEY_RESERVED",
//   "KEY_ESC",
//   "KEY_1",
//   "KEY_2",
//   "KEY_3",
//   "KEY_4",
//   "KEY_5",
//   "KEY_6",
//   "KEY_7",
//   "KEY_8",
//   "KEY_9",
//   "KEY_0",
//   "KEY_MINUS",
//   "KEY_EQUAL",
//   "KEY_BACKSPACE",
//   "KEY_TAB",
//   "KEY_Q",
//   "KEY_W",
//   "KEY_E",
//   "KEY_R",
//   "KEY_T",
//   "KEY_Y",
//   "KEY_U",
//   "KEY_I",
//   "KEY_O",
//   "KEY_P",
//   "KEY_LEFTBRACE",
//   "KEY_RIGHTBRACE",
//   "KEY_ENTER",
//   "KEY_LEFTCTRL",
//   "KEY_A",
//   "KEY_S",
//   "KEY_D",
//   "KEY_F",
//   "KEY_G",
//   "KEY_H",
//   "KEY_J",
//   "KEY_K",
//   "KEY_L",
//   "KEY_SEMICOLON",
//   "KEY_APOSTROPHE",
//   "KEY_GRAVE",
//   "KEY_LEFTSHIFT",
//   "KEY_BACKSLASH",
//   "KEY_Z",
//   "KEY_X",
//   "KEY_C",
//   "KEY_V",
//   "KEY_B",
//   "KEY_N",
//   "KEY_M",
//   "KEY_COMMA",
//   "KEY_DOT",
//   "KEY_SLASH",
//   "KEY_RIGHTSHIFT",
//   "KEY_KPASTERISK",
//   "KEY_LEFTALT",
//   "KEY_SPACE",
//   "KEY_CAPSLOCK",
//   "KEY_F1",
//   "KEY_F2",
//   "KEY_F3",
//   "KEY_F4",
//   "KEY_F5",
//   "KEY_F6",
//   "KEY_F7",
//   "KEY_F8",
//   "KEY_F9",
//   "KEY_F10",
//   "KEY_NUMLOCK",
//   "KEY_SCROLLLOCK",
//   "KEY_KP7",
//   "KEY_KP8",
//   "KEY_KP9",
//   "KEY_KPMINUS",
//   "KEY_KP4",
//   "KEY_KP5",
//   "KEY_KP6",
//   "KEY_KPPLUS",
//   "KEY_KP1",
//   "KEY_KP2",
//   "KEY_KP3",
//   "KEY_KP0",
//   "KEY_KPDOT",
//   "unknown",
//   "KEY_ZENKAKUHANKAKU",
//   "KEY_102ND",
//   "KEY_F11",
//   "KEY_F12",
//   "KEY_RO",
//   "KEY_KATAKANA",
//   "KEY_HIRAGANA",
//   "KEY_HENKAN",
//   "KEY_KATAKANAHIRAGANA",
//   "KEY_MUHENKAN",
//   "KEY_KPJPCOMMA",
//   "KEY_KPENTER",
//   "KEY_RIGHTCTRL",
//   "KEY_KPSLASH",
//   "KEY_SYSRQ",
//   "KEY_RIGHTALT",
//   "KEY_LINEFEED",
//   "KEY_HOME",
//   "KEY_UP",
//   "KEY_PAGEUP",
//   "KEY_LEFT",
//   "KEY_RIGHT",
//   "KEY_END",
//   "KEY_DOWN",
//   "KEY_PAGEDOWN",
//   "KEY_INSERT",
//   "KEY_DELETE",
//   "KEY_MACRO",
//   "KEY_MUTE",
//   "KEY_VOLUMEDOWN",
//   "KEY_VOLUMEUP",
//   "KEY_POWER",
//   "KEY_KPEQUAL",
//   "KEY_KPPLUSMINUS",
//   "KEY_PAUSE",
//   "KEY_SCALE",
//   "KEY_KPCOMMA",
//   "KEY_HANGEUL",
//   "KEY_HANJA",
//   "KEY_YEN",
//   "KEY_LEFTMETA",
//   "KEY_RIGHTMETA",
//   "KEY_COMPOSE",
//   "KEY_STOP",
//   "KEY_AGAIN",
//   "KEY_PROPS",
//   "KEY_UNDO",
//   "KEY_FRONT",
//   "KEY_COPY",
//   "KEY_OPEN",
//   "KEY_PASTE",
//   "KEY_FIND",
//   "KEY_CUT",
//   "KEY_HELP",
//   "KEY_MENU",
//   "KEY_CALC",
//   "KEY_SETUP",
//   "KEY_SLEEP",
//   "KEY_WAKEUP",
//   "KEY_FILE",
//   "KEY_SENDFILE",
//   "KEY_DELETEFILE",
//   "KEY_XFER",
//   "KEY_PROG1",
//   "KEY_PROG2",
//   "KEY_WWW",
//   "KEY_MSDOS",
//   "KEY_COFFEE",
//   "KEY_ROTATE_DISPLAY",
//   "KEY_CYCLEWINDOWS",
//   "KEY_MAIL",
//   "KEY_BOOKMARKS",
//   "KEY_COMPUTER",
//   "KEY_BACK",
//   "KEY_FORWARD",
//   "KEY_CLOSECD",
//   "KEY_EJECTCD",
//   "KEY_EJECTCLOSECD",
//   "KEY_NEXTSONG",
//   "KEY_PLAYPAUSE",
//   "KEY_PREVIOUSSONG",
//   "KEY_STOPCD",
//   "KEY_RECORD",
//   "KEY_REWIND",
//   "KEY_PHONE",
//   "KEY_ISO",
//   "KEY_CONFIG",
//   "KEY_HOMEPAGE",
//   "KEY_REFRESH",
//   "KEY_EXIT",
//   "KEY_MOVE",
//   "KEY_EDIT",
//   "KEY_SCROLLUP",
//   "KEY_SCROLLDOWN",
//   "KEY_KPLEFTPAREN",
//   "KEY_KPRIGHTPAREN",
//   "KEY_NEW",
//   "KEY_REDO",
//   "KEY_F13",
//   "KEY_F14",
//   "KEY_F15",
//   "KEY_F16",
//   "KEY_F17",
//   "KEY_F18",
//   "KEY_F19",
//   "KEY_F20",
//   "KEY_F21",
//   "KEY_F22",
//   "KEY_F23",
//   "KEY_F24",
//   "unknown",
//   "unknown",
//   "unknown",
//   "unknown",
//   "unknown",
//   "KEY_PLAYCD",
//   "KEY_PAUSECD",
//   "KEY_PROG3",
//   "KEY_PROG4",
//   "KEY_DASHBOARD",
//   "KEY_SUSPEND",
//   "KEY_CLOSE",
//   "KEY_PLAY",
//   "KEY_FASTFORWARD",
//   "KEY_BASSBOOST",
//   "KEY_PRINT",
//   "KEY_HP",
//   "KEY_CAMERA",
//   "KEY_SOUND",
//   "KEY_QUESTION",
//   "KEY_EMAIL",
//   "KEY_CHAT",
//   "KEY_SEARCH",
//   "KEY_CONNECT",
//   "KEY_FINANCE",
//   "KEY_SPORT",
//   "KEY_SHOP",
//   "KEY_ALTERASE",
//   "KEY_CANCEL",
//   "KEY_BRIGHTNESSDOWN",
//   "KEY_BRIGHTNESSUP",
//   "KEY_MEDIA",
//   "KEY_SWITCHVIDEOMODE",
//   "KEY_KBDILLUMTOGGLE",
//   "KEY_KBDILLUMDOWN",
//   "KEY_KBDILLUMUP",
//   "KEY_SEND",
//   "KEY_REPLY",
//   "KEY_FORWARDMAIL",
//   "KEY_SAVE",
//   "KEY_DOCUMENTS",
//   "KEY_BATTERY",
//   "KEY_BLUETOOTH",
//   "KEY_WLAN",
//   "KEY_UWB",
//   "KEY_UNKNOWN",
//   "KEY_VIDEO_NEXT",
//   "KEY_VIDEO_PREV",
//   "KEY_BRIGHTNESS_CYCLE",
//   "KEY_BRIGHTNESS_AUTO",
//   "KEY_DISPLAY_OFF",
//   "KEY_WWAN",
//   "KEY_RFKILL",
//   "KEY_MICMUTE",
// ];
// ] DEBUG

fn is_modifier(code: u16) -> bool {
  matches!(
    i32::from(code),
    sys::KEY_LEFTCTRL
      | sys::KEY_RIGHTCTRL
      | sys::KEY_LEFTALT
      | sys::KEY_RIGHTALT
      | sys::KEY_LEFTSHIFT
      | sys::KEY_RIGHTSHIFT
      | sys::KEY_LEFTMETA
      | sys::KEY_RIGHTMETA
      | sys::KEY_CAPSLOCK
      | sys::KEY_NUMLOCK
      | sys::KEY_SCROLLLOCK
  )
}

#[derive(Debug)]
enum GroupClass {
  Void,
  Emacs,
  Term,
  Others,
}

xcb::atoms_struct! {
  #[derive(Debug)]
  struct Atoms {
    wm_protocols    => b"WM_PROTOCOLS",
    wm_del_window   => b"WM_DELETE_WINDOW",
    wm_state        => b"_NET_WM_STATE",
    wm_state_maxv   => b"_NET_WM_STATE_MAXIMIZED_VERT",
    wm_state_maxh   => b"_NET_WM_STATE_MAXIMIZED_HORZ",
  }
}

struct WMConn {
  conn: xcb::Connection,
  screen_num: i32,
  atoms: Atoms,
  last_instant: Option<std::time::Instant>,
  focused: Option<xcb::x::Window>,
  child: Option<xcb::x::Window>,
  caps_lock_on: std::process::Command,
  caps_lock_off: std::process::Command,
  caps_lock_thread: Option<thread::JoinHandle<()>>,
  fehbg_path: std::path::PathBuf,
}

impl WMConn {
  fn new() -> Self {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let atoms = Atoms::intern_all(&conn).unwrap();
    let mut caps_lock_on = std::process::Command::new("hsetroot");
    caps_lock_on.args(["-solid", "#ff0000"]);
    let fehbg_path = std::path::Path::new("/home")
      .join(
        std::env::var("SUDO_USER")
          .unwrap_or_else(|_| std::env::var("USER").unwrap()),
      )
      .join(".fehbg");
    Self {
      conn,
      screen_num,
      atoms,
      last_instant: None,
      focused: None,
      child: None,
      caps_lock_on,
      caps_lock_off: std::process::Command::new(fehbg_path.clone()),
      caps_lock_thread: None,
      fehbg_path,
    }
  }

  fn get_focused_window(&mut self) -> xcb::Result<xcb::x::Window> {
    static ELAPSED: Duration = Duration::from_millis(400);
    if let Some(window) = self.focused {
      if let Some(last) = self.last_instant {
        if last.elapsed() < ELAPSED {
          self.last_instant = Some(std::time::Instant::now());
          return Ok(window);
        }
      }
    }
    let cookie = self.conn.send_request(&xcb::x::GetInputFocus {});
    let reply = self.conn.wait_for_reply(cookie)?;
    let focus = reply.focus();
    self.last_instant = Some(std::time::Instant::now());
    self.focused = Some(focus);
    Ok(focus)
  }

  fn get_group_class(&mut self) -> GroupClass {
    if let Ok(window) = self.get_focused_window() {
      let cookie = self.conn.send_request(&xcb::x::GetProperty {
        delete: false,
        window,
        property: xcb::x::ATOM_WM_CLASS,
        r#type: xcb::x::ATOM_ANY,
        long_offset: 0,
        long_length: 32,
      });
      if let Ok(reply) = self.conn.wait_for_reply(cookie) {
        if let Ok(class) = std::str::from_utf8(reply.value()) {
          return match class {
            "st-256color\0st-256color\0"
            | "st-256color\0Scratchpad\0"
            | "Alacritty\0Alacritty\0"
            | "Alacritty\0Scratchpad\0"
            | "emacs\0Emacs\0" => GroupClass::Emacs,
            "urxvt\0Urxvt\0" | "xterm\0Xterm\0" => GroupClass::Term,
            _ => GroupClass::Others,
          };
        }
      }
    }
    GroupClass::Void
  }

  // https://github.com/rust-x-bindings/rust-xcb/issues/178
  fn close_focused_window(&mut self) {
    if let Ok(window) = self.get_focused_window() {
      self.conn.flush().unwrap();
      let cookie = self.conn.send_request(&xcb::x::GetProperty {
        delete: false,
        window,
        property: self.atoms.wm_protocols,
        r#type: xcb::x::ATOM_ATOM,
        long_offset: 0,
        long_length: u32::MAX,
      });
      'wm_delete: {
        if let Ok(reply) = self.conn.wait_for_reply(cookie) {
          let supports_wm_delete = reply
            .value::<xcb::x::Atom>()
            .contains(&self.atoms.wm_del_window);
          if supports_wm_delete {
            let event = xcb::x::ClientMessageEvent::new(
              window,
              self.atoms.wm_protocols,
              xcb::x::ClientMessageData::Data32([
                self.atoms.wm_del_window.resource_id(),
                xcb::x::CURRENT_TIME,
                0,
                0,
                0,
              ]),
            );
            self.conn.send_request(&xcb::x::SendEvent {
              propagate: false,
              destination: xcb::x::SendEventDest::Window(window),
              event_mask: xcb::x::EventMask::NO_EVENT,
              event: &event,
            });
            break 'wm_delete;
          }
        }
        self.conn.send_request(&xcb::x::KillClient {
          resource: window.resource_id(),
        });
      }
      self.conn.flush().unwrap();
      // self.conn.send_request(&xcb::x::UnmapSubwindows { window });
      // self.conn.send_request(&xcb::x::UnmapWindow { window });
      // self.conn.send_request(&xcb::x::DestroySubwindows { window });
      // self.conn.send_request(&xcb::x::DestroyWindow { window });
      // self.conn.flush().unwrap();
    }
  }

  fn create_text_window(
    &self,
    text: &[u8],
    x: i16,
    y: i16,
    fg: u32,
    bg: u32,
  ) -> xcb::x::Window {
    let cookie = self.conn.send_request(&xcb::x::GetInputFocus {});
    let reply = self.conn.wait_for_reply(cookie);

    let (parent, visual) = match reply {
      Ok(reply) => (reply.focus(), xcb::x::COPY_FROM_PARENT),
      Err(_) => {
        let screen = self
          .conn
          .get_setup()
          .roots()
          .nth(self.screen_num as usize)
          .unwrap();
        (screen.root(), screen.root_visual())
      }
    };
    let child: xcb::x::Window = self.conn.generate_id();
    self.conn.send_request(&xcb::x::CreateWindow {
      depth: xcb::x::COPY_FROM_PARENT as u8,
      wid: child,
      parent,
      x,
      y,
      width: text.len() as u16 * 9 + 6,
      height: 18,
      border_width: 0,
      class: xcb::x::WindowClass::CopyFromParent,
      visual,
      value_list: &[
        xcb::x::Cw::BackPixel(bg),
        // xcb::x::Cw::EventMask(xcb::x::EventMask::EXPOSURE),
      ],
    });
    self.conn.send_request(&xcb::x::MapWindow { window: child });

    let font: xcb::x::Font = self.conn.generate_id();
    self.conn.send_request(&xcb::x::OpenFont {
      fid: font,
      // xfontsel: command to preview font
      name: b"-*-fixed-bold-*-*-*-18-*-*-*-*-*-iso8859-*",
    });

    let drawable = xcb::x::Drawable::Window(child);
    let gc: xcb::x::Gcontext = self.conn.generate_id();
    self.conn.send_request(&xcb::x::CreateGc {
      cid: gc,
      drawable,
      value_list: &[
        xcb::x::Gc::Foreground(fg),
        xcb::x::Gc::Background(bg),
        xcb::x::Gc::Font(font),
      ],
    });

    self.conn.send_request(&xcb::x::ImageText8 {
      drawable,
      gc,
      x: 4,
      y: 14,
      string: text,
    });
    self.conn.flush().unwrap();
    child
  }

  fn show_text(&mut self, text: &str) {
    if let Some(child) = self.child {
      self
        .conn
        .send_request(&xcb::x::DestroyWindow { window: child });
      self.conn.flush().unwrap();
      self.child = None;
    }
    if !text.is_empty() {
      self.child = Some(self.create_text_window(
        text.as_bytes(),
        0,
        0,
        0xffffffff,
        0xff0818a8,
      ));
    }
  }

  fn toggle_caps_lock(&mut self) {
    if let Err(e) = {
      if !get_caps_lock_state() {
        let caps_lock_on = self.caps_lock_on.spawn();
        self.caps_lock_thread_spawn();
        caps_lock_on
      } else {
        self.caps_lock_off.spawn()
      }
    } {
      eprintln!("Failed to spawn process: {}", e);
    }
  }

  fn caps_lock_thread_spawn(&mut self) {
    if let Some(thread) = &self.caps_lock_thread {
      if !thread.is_finished() {
        return;
      }
    }
    let fehbg_path = self.fehbg_path.clone();
    self.caps_lock_thread = Some(thread::spawn(move || {
      let mut caps_lock_off = std::process::Command::new(fehbg_path);
      unsafe {
        // Open a connection to the X server
        let display = xlib::XOpenDisplay(std::ptr::null());
        if display.is_null() {
          eprintln!("Unable to open X display");
          return;
        }

        // Get the default keyboard device
        let mut keys_return: xlib::XKeyboardState = std::mem::zeroed();
        xlib::XGetKeyboardControl(display, &mut keys_return);
        while keys_return.led_mask & 1 != 0 {
          thread::sleep(Duration::from_millis(500));
          xlib::XGetKeyboardControl(display, &mut keys_return);
        }
        if let Err(e) = caps_lock_off.spawn() {
          eprintln!("Failed to spawn process: {}", e);
        }

        // Close the connection to the X server
        xlib::XCloseDisplay(display);
      }
    }));
  }
}

fn get_caps_lock_state() -> bool {
  unsafe {
    // Open a connection to the X server
    let display = xlib::XOpenDisplay(std::ptr::null());
    if display.is_null() {
      eprintln!("Unable to open X display");
      return false;
    }

    // Get the default keyboard device
    let mut keys_return: xlib::XKeyboardState = std::mem::zeroed();
    xlib::XGetKeyboardControl(display, &mut keys_return);

    // Check if the Caps Lock mask is set
    let caps_lock_state = keys_return.led_mask & 1;

    // Close the connection to the X server
    xlib::XCloseDisplay(display);
    caps_lock_state != 0
  }
}

fn read_input_event(
  stdin: &mut std::io::Stdin,
) -> std::io::Result<input_linux::InputEvent> {
  let mut buf = [0; std::mem::size_of::<input_linux::InputEvent>()];
  stdin.read_exact(&mut buf)?;
  let input_event: input_linux::InputEvent =
    unsafe { std::mem::transmute(buf) };
  Ok(input_event)
}

struct Stdoutput {
  grab: bool,
  events: Vec<input_linux::InputEvent>,
  stdout: std::io::Stdout,
}

impl Stdoutput {
  fn new() -> Self {
    Self {
      grab: false,
      events: Vec::new(),
      stdout: std::io::stdout(),
    }
  }

  fn start_recording(&mut self) {
    self.events.clear();
    self.grab = true;
  }

  fn stop_recording(&mut self) {
    self.grab = false;
  }

  fn write_events(&mut self) -> std::io::Result<()> {
    for event in &self.events {
      self.stdout.write_all(event.as_bytes())?;
      self.stdout.flush()?;
    }
    Ok(())
  }

  fn write_event(
    &mut self,
    event: input_linux::InputEvent,
  ) -> std::io::Result<()> {
    // [ DEBUG
    // eprintln!(
    //   "{:?}\t{:?}\t{}\t{}",
    //   event.time, event.kind, KEY_NAMES[event.code as usize], event.value
    // );
    // ] DEBUG
    self.stdout.write_all(event.as_bytes())?;
    self.stdout.flush()?;
    if self.grab {
      self.events.push(event);
    }
    Ok(())
  }
}

const SHIFT_CHARS: [char; 10] =
  ['=', '!', '"', '·', '$', '%', '&', '/', '(', ')'];

struct ShiftTranslator;

impl Replacer for ShiftTranslator {
  fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
    dst.push(SHIFT_CHARS[caps[2].parse::<usize>().unwrap()]);
  }
}

#[allow(clippy::type_complexity)]
fn get_config() -> (
  HashMap<Vec<i32>, HashMap<usize, (Vec<i32>, Vec<[i32; 2]>)>>,
  HashMap<usize, String>,
) {
  let key_repr: HashMap<i32, &'static str> = HashMap::from([
    (sys::KEY_0, "0"),
    (sys::KEY_1, "1"),
    (sys::KEY_2, "2"),
    (sys::KEY_3, "3"),
    (sys::KEY_4, "4"),
    (sys::KEY_5, "5"),
    (sys::KEY_6, "6"),
    (sys::KEY_7, "7"),
    (sys::KEY_8, "8"),
    (sys::KEY_9, "9"),
    (sys::KEY_A, "A"),
    (sys::KEY_B, "B"),
    (sys::KEY_C, "C"),
    (sys::KEY_D, "D"),
    (sys::KEY_E, "E"),
    (sys::KEY_F, "F"),
    (sys::KEY_G, "G"),
    (sys::KEY_H, "H"),
    (sys::KEY_I, "I"),
    (sys::KEY_J, "J"),
    (sys::KEY_K, "K"),
    (sys::KEY_L, "L"),
    (sys::KEY_M, "M"),
    (sys::KEY_N, "N"),
    (sys::KEY_O, "O"),
    (sys::KEY_P, "P"),
    (sys::KEY_Q, "Q"),
    (sys::KEY_R, "R"),
    (sys::KEY_S, "S"),
    (sys::KEY_T, "T"),
    (sys::KEY_U, "U"),
    (sys::KEY_V, "V"),
    (sys::KEY_W, "W"),
    (sys::KEY_X, "X"),
    (sys::KEY_Y, "Y"),
    (sys::KEY_Z, "Z"),
    (sys::KEY_COMMA, ","),
    (sys::KEY_DOT, "."),
    (sys::KEY_ENTER, "RET"),
    (sys::KEY_EQUAL, "="),
    (sys::KEY_ESC, "ESC"),
    (sys::KEY_LEFTALT, "A+"),
    (sys::KEY_LEFTBRACE, "["),
    (sys::KEY_LEFTCTRL, "C+"),
    (sys::KEY_LEFTSHIFT, "S+"),
    (sys::KEY_MINUS, "-"),
    (sys::KEY_RIGHTALT, "A+"),
    (sys::KEY_RIGHTBRACE, "]"),
    (sys::KEY_RIGHTCTRL, "C+"),
    (sys::KEY_RIGHTSHIFT, "S+"),
    (sys::KEY_SEMICOLON, ";"),
    (sys::KEY_SLASH, "/"),
    (sys::KEY_SPACE, "SPC"),
    (sys::KEY_TAB, "TAB"),
  ]);
  let remaps = [
    // 0 - Emacs
    vec![(
      vec![sys::KEY_RIGHTALT, sys::KEY_ESC],
      vec![sys::KEY_RIGHTALT],
      vec![[sys::KEY_GRAVE, 1], [sys::KEY_GRAVE, 0]],
    )],
    // 1 - Term
    vec![(
      vec![sys::KEY_RIGHTALT, sys::KEY_ESC],
      vec![sys::KEY_RIGHTALT],
      vec![[sys::KEY_GRAVE, 1], [sys::KEY_GRAVE, 0]],
    )],
    // 2 - Others (REMAP_INDEX_OTHERS)
    vec![
      (
        vec![sys::KEY_RIGHTALT, sys::KEY_ESC],
        vec![sys::KEY_RIGHTALT],
        vec![[sys::KEY_GRAVE, 1], [sys::KEY_GRAVE, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_N],
        vec![],
        vec![[sys::KEY_DOWN, 1], [sys::KEY_DOWN, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_P],
        vec![],
        vec![[sys::KEY_UP, 1], [sys::KEY_UP, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_F],
        vec![],
        vec![[sys::KEY_RIGHT, 1], [sys::KEY_RIGHT, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_B],
        vec![],
        vec![[sys::KEY_LEFT, 1], [sys::KEY_LEFT, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_A],
        vec![],
        vec![[sys::KEY_HOME, 1], [sys::KEY_HOME, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_E],
        vec![],
        vec![[sys::KEY_END, 1], [sys::KEY_END, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_V],
        vec![],
        vec![[sys::KEY_PAGEDOWN, 1], [sys::KEY_PAGEDOWN, 0]],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_V],
        vec![],
        vec![[sys::KEY_PAGEUP, 1], [sys::KEY_PAGEUP, 0]],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_N],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_DOWN, 1],
          [sys::KEY_DOWN, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_P],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_UP, 1],
          [sys::KEY_UP, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_F],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_RIGHT, 1],
          [sys::KEY_RIGHT, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_B],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_LEFT, 1],
          [sys::KEY_LEFT, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_102ND],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_HOME, 1],
          [sys::KEY_HOME, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_X],
        vec![],
        vec![[sys::KEY_F6, 1], [sys::KEY_F6, 0]],
      ),
      // COPY-PASTE
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_Y],
        vec![sys::KEY_LEFTCTRL],
        vec![[sys::KEY_V, 1], [sys::KEY_V, 0]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_W],
        vec![sys::KEY_LEFTCTRL],
        vec![[sys::KEY_X, 1], [sys::KEY_X, 0]],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_W],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_C, 1],
          [sys::KEY_C, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      // EDITION
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_D],
        vec![],
        vec![[sys::KEY_DELETE, 1], [sys::KEY_DELETE, 0]],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_D],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_DELETE, 1],
          [sys::KEY_DELETE, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_LEFTSHIFT, sys::KEY_SLASH],
        vec![sys::KEY_LEFTCTRL],
        vec![[sys::KEY_Z, 1], [sys::KEY_Z, 0]],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_LEFTSHIFT, sys::KEY_SLASH],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_Y, 1],
          [sys::KEY_Y, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_K],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_END, 1],
          [sys::KEY_END, 0],
          [sys::KEY_DELETE, 1],
          [sys::KEY_DELETE, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_U],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_HOME, 1],
          [sys::KEY_HOME, 0],
          [sys::KEY_DELETE, 1],
          [sys::KEY_DELETE, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_BACKSPACE],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_BACKSPACE, 1],
          [sys::KEY_BACKSPACE, 0],
          [sys::KEY_LEFTCTRL, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_LEFTSHIFT, sys::KEY_BACKSPACE],
        vec![],
        vec![
          [sys::KEY_HOME, 1],
          [sys::KEY_HOME, 0],
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_END, 1],
          [sys::KEY_END, 0],
          [sys::KEY_DELETE, 1],
          [sys::KEY_DELETE, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      // SEARCH
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_S],
        vec![sys::KEY_LEFTCTRL],
        vec![[sys::KEY_F, 1], [sys::KEY_F, 0]],
      ),
      // Change remap
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_SPACE],
        vec![sys::KEY_LEFTCTRL],
        vec![[ACTION_REMAP, REMAP_INDEX_SELECT as i32]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_X],
        vec![sys::KEY_LEFTCTRL],
        vec![[ACTION_REMAP, REMAP_INDEX_CTRL_X as i32]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_C],
        vec![sys::KEY_LEFTCTRL],
        vec![[ACTION_REMAP, REMAP_INDEX_CTRL_C as i32]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_1],
        vec![],
        vec![[ACTION_PREFIX, 1]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_2],
        vec![],
        vec![[ACTION_PREFIX, 2]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_3],
        vec![],
        vec![[ACTION_PREFIX, 3]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_4],
        vec![],
        vec![[ACTION_PREFIX, 4]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_5],
        vec![],
        vec![[ACTION_PREFIX, 5]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_6],
        vec![],
        vec![[ACTION_PREFIX, 6]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_7],
        vec![],
        vec![[ACTION_PREFIX, 7]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_8],
        vec![],
        vec![[ACTION_PREFIX, 8]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_9],
        vec![],
        vec![[ACTION_PREFIX, 9]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_0],
        vec![],
        vec![[ACTION_PREFIX, 0]],
      ),
    ],
    // 3 - Select Mode (REMAP_INDEX_SELECT)
    vec![
      (
        vec![sys::KEY_ESC],
        vec![],
        vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_SPACE],
        vec![sys::KEY_LEFTCTRL],
        vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_G],
        vec![],
        vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
      ),
      (
        vec![sys::KEY_DELETE],
        vec![],
        vec![
          [sys::KEY_DELETE, 1],
          [sys::KEY_DELETE, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_BACKSPACE],
        vec![],
        vec![
          [sys::KEY_BACKSPACE, 1],
          [sys::KEY_BACKSPACE, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_N],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_DOWN, 1],
          [sys::KEY_DOWN, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_P],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_UP, 1],
          [sys::KEY_UP, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_F],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_RIGHT, 1],
          [sys::KEY_RIGHT, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_B],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_LEFT, 1],
          [sys::KEY_LEFT, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_A],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_HOME, 1],
          [sys::KEY_HOME, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_E],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_END, 1],
          [sys::KEY_END, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_V],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_PAGEDOWN, 1],
          [sys::KEY_PAGEDOWN, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_V],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_PAGEUP, 1],
          [sys::KEY_PAGEUP, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_N],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_DOWN, 1],
          [sys::KEY_DOWN, 0],
          [sys::KEY_LEFTCTRL, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_P],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_UP, 1],
          [sys::KEY_UP, 0],
          [sys::KEY_LEFTCTRL, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_F],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_RIGHT, 1],
          [sys::KEY_RIGHT, 0],
          [sys::KEY_LEFTCTRL, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_B],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_LEFT, 1],
          [sys::KEY_LEFT, 0],
          [sys::KEY_LEFTCTRL, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_102ND],
        vec![],
        vec![
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_HOME, 1],
          [sys::KEY_HOME, 0],
          [sys::KEY_LEFTCTRL, 0],
          [sys::KEY_LEFTSHIFT, 0],
        ],
      ),
      // COPY-PASTE
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_Y],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_V, 1],
          [sys::KEY_V, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_W],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_X, 1],
          [sys::KEY_X, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_W],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_C, 1],
          [sys::KEY_C, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      // EDITION
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_D],
        vec![],
        vec![
          [sys::KEY_DELETE, 1],
          [sys::KEY_DELETE, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTALT, sys::KEY_D],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_DELETE, 1],
          [sys::KEY_DELETE, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
    ],
    // 4 - Ctrl x (REMAP_INDEX_CTRL_X)
    vec![
      (
        vec![sys::KEY_ESC],
        vec![],
        vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_G],
        vec![],
        vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
      ),
      (
        vec![sys::KEY_B],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_A, 1],
          [sys::KEY_A, 0],
          [sys::KEY_LEFTSHIFT, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_U],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_LEFTSHIFT, 1],
          [sys::KEY_T, 1],
          [sys::KEY_T, 0],
          [sys::KEY_LEFTSHIFT, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_K],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_W, 1],
          [sys::KEY_W, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_O],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_TAB, 1],
          [sys::KEY_TAB, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTSHIFT, sys::KEY_O],
        vec![sys::KEY_LEFTSHIFT],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_TAB, 1],
          [sys::KEY_TAB, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_2],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_T, 1],
          [sys::KEY_T, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_3],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_T, 1],
          [sys::KEY_T, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_5],
        vec![],
        vec![
          [sys::KEY_LEFTCTRL, 1],
          [sys::KEY_N, 1],
          [sys::KEY_N, 0],
          [sys::KEY_LEFTCTRL, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_S],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_S, 1],
          [sys::KEY_S, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_F],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_O, 1],
          [sys::KEY_O, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_C],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [ACTION_FUNCTION, ACTION_FUNCTION_CLOSE],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTSHIFT, sys::KEY_8],
        vec![],
        vec![
          [ACTION_MACRO, ACTION_MACRO_START],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTSHIFT, sys::KEY_9],
        vec![],
        vec![
          [ACTION_MACRO, ACTION_MACRO_STOP],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_E],
        vec![],
        vec![
          [ACTION_MACRO, ACTION_MACRO_EXECUTE],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
    ],
    // 5 - Ctrl c (REMAP_INDEX_CTRL_C)
    vec![
      (
        vec![sys::KEY_ESC],
        vec![],
        vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_G],
        vec![],
        vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
      ),
      (
        vec![sys::KEY_T],
        vec![],
        vec![
          [sys::KEY_F6, 1],
          [sys::KEY_F6, 0],
          [sys::KEY_F6, 1],
          [sys::KEY_F6, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_A],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_A, 1],
          [sys::KEY_A, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_B],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_B, 1],
          [sys::KEY_B, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_C],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_C, 1],
          [sys::KEY_C, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_D],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_D, 1],
          [sys::KEY_D, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_E],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_E, 1],
          [sys::KEY_E, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_F],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_F, 1],
          [sys::KEY_F, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_K],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_K, 1],
          [sys::KEY_K, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_N],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_N, 1],
          [sys::KEY_N, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_P],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_P, 1],
          [sys::KEY_P, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_U],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_U, 1],
          [sys::KEY_U, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_1],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_1, 1],
          [sys::KEY_1, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_2],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_2, 1],
          [sys::KEY_2, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_3],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_3, 1],
          [sys::KEY_3, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_4],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_4, 1],
          [sys::KEY_4, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_5],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_5, 1],
          [sys::KEY_5, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_6],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_6, 1],
          [sys::KEY_6, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_7],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_7, 1],
          [sys::KEY_7, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_8],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_8, 1],
          [sys::KEY_8, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_9],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_9, 1],
          [sys::KEY_9, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
      (
        vec![sys::KEY_LEFTCTRL, sys::KEY_0],
        vec![sys::KEY_LEFTCTRL],
        vec![
          [sys::KEY_0, 1],
          [sys::KEY_0, 0],
          [ACTION_REMAP, REMAP_INDEX_OTHERS as i32],
        ],
      ),
    ],
    // 6 - Vi mode
    // vec![
    //   (
    //     vec![sys::KEY_ESC],
    //     vec![],
    //     vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
    //   ),
    //   (
    //     vec![sys::KEY_LEFTCTRL, sys::KEY_G],
    //     vec![],
    //     vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
    //   ),
    //   (
    //     vec![sys::KEY_I],
    //     vec![],
    //     vec![[ACTION_REMAP, REMAP_INDEX_OTHERS as i32]],
    //   ),
    //   (
    //     vec![sys::KEY_J],
    //     vec![],
    //     vec![[sys::KEY_DOWN, 1], [sys::KEY_DOWN, 0]],
    //   ),
    // ],
  ];
  let mut ctrl_c = "C+C >".to_string();
  let mut ctrl_x = "C+X >".to_string();
  let mut config = HashMap::new();
  for (index, remap) in remaps.iter().enumerate() {
    let mut title_ref = match index {
      REMAP_INDEX_CTRL_C => Some(&mut ctrl_c),
      REMAP_INDEX_CTRL_X => Some(&mut ctrl_x),
      _ => None,
    };
    for (pressed_keys, keep_keys, fake_keys) in remap {
      if let Some(title) = &mut title_ref {
        title.push(' ');
        for &key in pressed_keys.iter() {
          title.push_str(key_repr.get(&key).unwrap_or(&"¬"));
        }
      }
      let mut pressed_keys = pressed_keys.clone();
      pressed_keys.sort();
      if !config.contains_key(&pressed_keys) {
        config.insert(pressed_keys.clone(), HashMap::new());
      }
      config
        .get_mut(&pressed_keys)
        .unwrap()
        .insert(index, (keep_keys.clone(), fake_keys.clone()));
    }
  }
  let mut remap_titles = HashMap::new();
  remap_titles.insert(REMAP_INDEX_SELECT, "C+SPC".to_string());

  let re = Regex::new(r"(S\+([0-9]))").unwrap();
  remap_titles.insert(
    REMAP_INDEX_CTRL_X,
    re.replace_all(ctrl_x.as_str(), ShiftTranslator).to_string(),
  );
  remap_titles.insert(REMAP_INDEX_CTRL_C, ctrl_c);
  (config, remap_titles)
}

fn main() {
  let (config, remap_titles) = get_config();
  let mut pressed_keys = Vec::new();
  let mut wmconn = WMConn::new();

  // let mut notifier: Option<NotificationHandle> = None;
  let mut repetitions = 0;
  let mut remap_index: usize = 0;
  let mut remap_index_next: usize = 0;
  let mut last_remap: Option<Vec<i32>> = None;
  let mut avoid_keys = Vec::new();
  let mut fake_writed_keys = Vec::new();
  let mut fake_event = input_linux::InputEvent {
    time: input_linux::EventTime::new(0, 0),
    kind: input_linux::EventKind::Key,
    code: 0,
    value: 0,
  };
  let mut remap_title = "";
  let mut macro_title = "";
  let mut repeat_title = "".to_string();
  let mut stdin = std::io::stdin();
  let mut output = Stdoutput::new();
  while let std::io::Result::Ok(mut event) = read_input_event(&mut stdin) {
    match event.kind {
      input_linux::EventKind::Key => (),
      input_linux::EventKind::Misc => {
        if event.code != input_linux::MiscKind::Scancode as u16 {
          output.write_event(event).unwrap();
        }
        continue;
      }
      _ => {
        output.write_event(event).unwrap();
        continue;
      }
    }

    if let Ok(key) = input_linux::Key::try_from(event.code) {
      match key {
        input_linux::Key::RightAlt => {
          event.code = sys::KEY_LEFTCTRL as u16;
        }
        input_linux::Key::LeftCtrl | input_linux::Key::RightCtrl => {
          event.code = sys::KEY_RIGHTALT as u16;
        }
        input_linux::Key::RightShift => {
          event.code = sys::KEY_LEFTSHIFT as u16;
        }
        input_linux::Key::CapsLock => {
          if event.value == 1 {
            wmconn.toggle_caps_lock();
          }
        }
        _ => (),
      }
    } else {
      output.write_event(event).unwrap();
      continue;
    }

    fake_event.time = event.time;
    match event.value {
      0 => {
        while let Some(key) = fake_writed_keys.pop() {
          fake_event.code = key;
          fake_event.value = 1;
          output.write_event(fake_event).unwrap();
        }
        pressed_keys.retain(|&key| key != event.code as i32);
        if avoid_keys.contains(&event.code) {
          avoid_keys.retain(|&key| key != event.code);
          continue;
        }
        output.write_event(event).unwrap();
      }
      1 => {
        last_remap = None;
        while let Some(key) = fake_writed_keys.pop() {
          fake_event.code = key;
          fake_event.value = 1;
          output.write_event(fake_event).unwrap();
        }
        pressed_keys.push(event.code as i32);
        let mut pressed_keys = pressed_keys.clone();
        pressed_keys.sort();
        if avoid_keys.is_empty() {
          let mut avoid_repeat = false;
          if let Some(remap) = config.get(&pressed_keys) {
            if remap_index_next <= REMAP_INDEX_OTHERS
              && *remap.keys().min().unwrap() <= REMAP_INDEX_OTHERS
            {
              match wmconn.get_group_class() {
                GroupClass::Void => {
                  output.write_event(event).unwrap();
                  continue;
                }
                GroupClass::Emacs => {
                  remap_index = 0;
                }
                GroupClass::Term => {
                  remap_index = 1;
                }
                GroupClass::Others => {
                  remap_index = REMAP_INDEX_OTHERS;
                }
              }
            } else {
              remap_index = remap_index_next;
            }
            if let Some((keep_keys, fake_keys)) = remap.get(&remap_index) {
              for &pressed_key in pressed_keys.iter().rev() {
                if event.code == pressed_key as u16
                  || keep_keys.contains(&pressed_key)
                {
                  continue;
                }
                fake_event.code = pressed_key as u16;
                fake_event.value = 0;
                output.write_event(fake_event).unwrap();
                fake_writed_keys.push(fake_event.code);
              }
              last_remap = Some(pressed_keys);
              for fake_key in fake_keys.iter() {
                match fake_key[0] {
                  ACTION_PREFIX => {
                    repetitions = repetitions * 10 + fake_key[1];
                    repeat_title = format!("({repetitions})");
                    wmconn.show_text(
                      &(macro_title.to_owned()
                        + &(repeat_title.to_owned() + remap_title)),
                    );
                    avoid_repeat = true;
                  }
                  ACTION_MACRO => {
                    match fake_key[1] {
                      ACTION_MACRO_START => {
                        output.start_recording();
                        macro_title = "REC:";
                        wmconn.show_text(
                          &(macro_title.to_owned()
                            + &(repeat_title.to_owned() + remap_title)),
                        );
                      }
                      ACTION_MACRO_STOP => {
                        output.stop_recording();
                        macro_title = "";
                        wmconn.show_text(
                          &(macro_title.to_owned()
                            + &(repeat_title.to_owned() + remap_title)),
                        );
                      }
                      ACTION_MACRO_EXECUTE => {
                        output.write_events().unwrap();
                      }
                      _ => eprintln!("Unknown macro action: {}", fake_key[1]),
                    }

                    while let Some(key) = fake_writed_keys.pop() {
                      avoid_keys.push(key);
                    }
                  }
                  ACTION_REMAP => {
                    remap_index_next = fake_key[1] as usize;
                    remap_title = if let Some(remap_title) =
                      remap_titles.get(&remap_index_next)
                    {
                      remap_title
                    } else {
                      ""
                    };
                    if fake_keys.len() == 1 {
                      avoid_repeat = true;
                    }
                    wmconn.show_text(
                      &(macro_title.to_owned()
                        + &(repeat_title.to_owned() + remap_title)),
                    );

                    while let Some(key) = fake_writed_keys.pop() {
                      avoid_keys.push(key);
                    }
                  }
                  ACTION_FUNCTION => match fake_key[1] {
                    ACTION_FUNCTION_CLOSE => wmconn.close_focused_window(),
                    _ => eprintln!("Unknown function: {}", fake_key[1]),
                  },
                  _ => {
                    fake_event.code = fake_key[0] as u16;
                    fake_event.value = fake_key[1];
                    output.write_event(fake_event).unwrap();
                  }
                }
              }
              avoid_keys.push(event.code);
              if avoid_repeat {
                continue;
              }
              loop {
                match repetitions {
                  0 => break,
                  1 => {
                    repetitions = 0;
                    repeat_title = "".to_string();
                    wmconn.show_text(
                      &(macro_title.to_owned()
                        + &(repeat_title.to_owned() + remap_title)),
                    );
                    break;
                  }
                  _ => (),
                }
                repetitions -= 1;
                for fake_key in fake_keys.iter() {
                  match fake_key[0] {
                    ACTION_MACRO => {
                      if fake_key[1] == ACTION_MACRO_EXECUTE {
                        output.write_events().unwrap();
                      }
                    }
                    ACTION_REMAP => (),
                    _ => {
                      fake_event.code = fake_key[0] as u16;
                      fake_event.value = fake_key[1];
                      output.write_event(fake_event).unwrap();
                    }
                  }
                }
              }
              continue;
            }
          }
          output.write_event(event).unwrap();
          if avoid_repeat || is_modifier(event.code) {
            continue;
          }
          loop {
            match repetitions {
              0 => break,
              1 => {
                repetitions = 0;
                repeat_title = "".to_string();
                wmconn.show_text(
                  &(macro_title.to_owned()
                    + &(repeat_title.to_owned() + remap_title)),
                );
                break;
              }
              _ => (),
            }
            repetitions -= 1;
            event.value = 0;
            output.write_event(event).unwrap();
            event.value = 1;
            output.write_event(event).unwrap();
          }
        }
        if remap_index > REMAP_INDEX_OTHERS {
          continue;
        }
      }
      2 => {
        if let Some(ref last_remap) = last_remap {
          for fake_key in config
            .get(last_remap)
            .unwrap()
            .get(&remap_index)
            .unwrap()
            .1
            .iter()
          {
            fake_event.code = fake_key[0] as u16;
            fake_event.value = fake_key[1];
            output.write_event(fake_event).unwrap();
          }
          continue;
        }
        output.write_event(event).unwrap();
      }
      _ => (),
    }
  }
}
