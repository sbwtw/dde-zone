
extern crate dbus;
extern crate gio_sys;

use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;

use dbus::Connection;
use dbus::ConnectionItem;
use dbus::BusType;
use dbus::NameFlag;
use dbus::Message;
use dbus::tree::MethodResult;
use dbus::tree::Factory;
use dbus::tree::MethodErr;
// use dbus::tree::ObjectPath;
// use dbus::tree::Tree;
// use dbus::tree::MethodFn;

use gio_sys::GSettings;
use gio_sys::g_settings_new;
use gio_sys::g_settings_get_string;

struct Zone {
    zone_detected: bool,
    bottom_left_action: String,
    bottom_right_action: String,
    top_left_action: String,
    top_right_action: String,

    settings: *mut GSettings,
}

impl Zone {
    fn new() -> Zone {
        let settings;
        unsafe {
            settings = g_settings_new(CString::new("com.deepin.dde.zone").unwrap().as_ptr());
        }

        let mut zone = Zone {
            zone_detected: false,
            bottom_left_action: String::new(),
            bottom_right_action: String::new(),
            top_left_action: String::new(),
            top_right_action: String::new(),

            settings: settings,
        };

        zone.load_settings();
        zone
    }

    fn load_settings(&mut self) {
        let top_left_action;
        let top_right_action;
        let bottom_left_action;
        let bottom_right_action;

        unsafe {
            let raw_str = g_settings_get_string(self.settings, CString::new("left-up").unwrap().as_ptr());
            top_left_action = CStr::from_ptr(raw_str);
            let raw_str = g_settings_get_string(self.settings, CString::new("right-up").unwrap().as_ptr());
            top_right_action = CStr::from_ptr(raw_str);
            let raw_str = g_settings_get_string(self.settings, CString::new("left-down").unwrap().as_ptr());
            bottom_left_action = CStr::from_ptr(raw_str);
            let raw_str = g_settings_get_string(self.settings, CString::new("right-down").unwrap().as_ptr());
            bottom_right_action = CStr::from_ptr(raw_str);
        }

        self.top_left_action = top_left_action.to_string_lossy().into_owned();
        self.top_right_action = top_right_action.to_string_lossy().into_owned();
        self.bottom_left_action = bottom_left_action.to_string_lossy().into_owned();
        self.bottom_right_action = bottom_right_action.to_string_lossy().into_owned();
    }

    fn set_detected(&mut self, m: &Message) -> MethodResult {
        match m.get1::<bool>() {
            Some(boolean) => self.zone_detected = boolean,
            _ => return Err(MethodErr::no_arg())
        }

        Ok(vec!(m.method_return()))
    }

    fn bottom_left_action(&mut self, m: &Message) -> MethodResult {
        Ok(vec!(m.method_return().append(self.bottom_left_action.clone())))
    }

    fn set_bottom_left(&mut self, m: &Message) -> MethodResult {
        match m.get1::<&str>() {
            Some(str) => self.bottom_left_action = str.to_string(),
            _ => return Err(MethodErr::no_arg())
        }

        Ok(vec!(m.method_return()))
    }

    fn bottom_right_action(&mut self, m: &Message) -> MethodResult {
        Ok(vec!(m.method_return().append(self.bottom_right_action.clone())))
    }

    fn set_bottom_right(&mut self, m: &Message) -> MethodResult {
        match m.get1::<&str>() {
            Some(str) => self.bottom_right_action = str.to_string(),
            _ => return Err(MethodErr::no_arg())
        }

        Ok(vec!(m.method_return()))
    }

    fn top_left_action(&mut self, m: &Message) -> MethodResult {
        Ok(vec!(m.method_return().append(self.top_left_action.clone())))
    }

    fn set_top_left(&mut self, m: &Message) -> MethodResult {
        match m.get1::<&str>() {
            Some(str) => self.top_left_action = str.to_string(),
            _ => return Err(MethodErr::no_arg())
        }

        Ok(vec!(m.method_return()))
    }

    fn top_right_action(&mut self, m: &Message) -> MethodResult {
        Ok(vec!(m.method_return().append(self.top_right_action.clone())))
    }

    fn set_top_right(&mut self, m: &Message) -> MethodResult {
        match m.get1::<&str>() {
            Some(str) => self.top_right_action = str.to_string(),
            _ => return Err(MethodErr::no_arg())
        }

        Ok(vec!(m.method_return()))
    }
}

fn main() {

    let zone = RefCell::new(Zone::new());

    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("com.deepin.daemon.Zone1", NameFlag::ReplaceExisting as u32).unwrap();

    let f = Factory::new_fn();
    let zone_detected = f.method("EnableZoneDetected", |m, _, _| {
        zone.borrow_mut().set_detected(m)
    }).inarg::<bool, _>("detected");
    let bottom_left_action = f.method("ButtomLeftAction", |m, _, _| {
        zone.borrow_mut().bottom_left_action(m)
    }).outarg::<&str, _>("action");
    let set_bottom_left = f.method("SetBottomLeft", |m, _, _| {
        zone.borrow_mut().set_bottom_left(m)
    }).inarg::<&str, _>("action");
    let bottom_right_action = f.method("BottomRightAction", |m, _, _| {
        zone.borrow_mut().bottom_right_action(m)
    }).outarg::<&str, _>("action");
    let set_bottom_right = f.method("SetBottomRight", |m, _, _| {
        zone.borrow_mut().set_bottom_right(m)
    }).inarg::<&str, _>("action");
    let top_left_action = f.method("TopLeftAction", |m, _, _| {
        zone.borrow_mut().top_left_action(m)
    }).outarg::<&str, _>("action");
    let set_top_left = f.method("SetTopLeft", |m, _, _| {
        zone.borrow_mut().set_top_left(m)
    }).inarg::<&str, _>("action");
    let top_right_action = f.method("TopRightAction", |m, _, _| {
        zone.borrow_mut().top_right_action(m)
    }).outarg::<&str, _>("action");
    let set_top_right = f.method("SetTopRight", |m, _, _| {
        zone.borrow_mut().set_top_right(m)
    }).inarg::<&str, _>("action");

    let inter = f.interface("com.deepin.daemon.Zone").add_m(zone_detected)
                                                     .add_m(bottom_left_action)
                                                     .add_m(set_bottom_left)
                                                     .add_m(bottom_right_action)
                                                     .add_m(set_bottom_right)
                                                     .add_m(top_left_action)
                                                     .add_m(set_top_left)
                                                     .add_m(top_right_action)
                                                     .add_m(set_top_right);

    let path = f.object_path("/com/deepin/daemon/Zone").introspectable().add(inter);
    let tree = f.tree().add(path);

    tree.set_registered(&c, true).unwrap();

    for item in tree.run(&c, c.iter(1000)) {
        match item {
            ConnectionItem::Nothing => {},
            _ => {
                println!("{:?}", item);
            },
        }
    }
}
