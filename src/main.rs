
extern crate dbus;
extern crate gio;
extern crate gtk;

use std::cell::RefCell;
use std::thread;
use std::ops::Drop;
use std::sync::Arc;
use std::iter::IntoIterator;

use dbus::Connection;
use dbus::ConnectionItem;
use dbus::BusType;
use dbus::NameFlag;
use dbus::Message;
use dbus::MessageItem;
use dbus::tree::Signal;
use dbus::tree::MethodResult;
use dbus::tree::Factory;
use dbus::tree::MethodErr;

use gio::Settings;

struct Zone {
    zone_detected: bool,
    bottom_left_action: String,
    bottom_right_action: String,
    top_left_action: String,
    top_right_action: String,

    connection: Connection,
    s: RefCell<Option<Arc<Signal>>>,

    settings: Settings,
}

impl Zone {
    fn new() -> Zone {
        let mut zone = Zone {
            zone_detected: false,
            bottom_left_action: String::new(),
            bottom_right_action: String::new(),
            top_left_action: String::new(),
            top_right_action: String::new(),

            connection: Connection::get_private(BusType::Session).unwrap(),
            s: RefCell::new(None),

            settings: Settings::new("com.deepin.dde.zone"),
        };

        zone.settings.connect_changed(|se, st| {
            println!("{:?}", se);
            println!("{:?}", st);
        });

        zone.load_settings();
        zone
    }

    fn load_settings(&mut self) {
        self.top_left_action = self.settings.get_string("left-up").unwrap();
        self.top_right_action = self.settings.get_string("right-up").unwrap();
        self.bottom_left_action = self.settings.get_string("left-down").unwrap();
        self.bottom_right_action = self.settings.get_string("right-down").unwrap();
    }

    fn save_settings(&mut self) -> bool {
        self.settings.set_string("left-up", self.top_left_action.as_str()) &&
        self.settings.set_string("right-up", self.top_right_action.as_str()) &&
        self.settings.set_string("left-down", self.bottom_left_action.as_str()) &&
        self.settings.set_string("right-down", self.bottom_right_action.as_str())
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

        let mut v: Vec<Result<(String, MessageItem), &str>> = vec![];
        v.push(Ok(("a".into(), 3i32.into())));
        v.push(Ok(("b".into(), 2i32.into())));

        let msg = MessageItem::from_dict(v.into_iter()).unwrap();

        let s = self.s.borrow().as_ref().unwrap().emit(&vec!(msg));
        let _ = self.connection.send(s);

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

impl Drop for Zone {
    fn drop(&mut self) {
        self.save_settings();
    }
}

fn create_dbus_service() {
    let zone = RefCell::new(Zone::new());

    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("com.deepin.daemon.Zone2", NameFlag::ReplaceExisting as u32).unwrap();

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

    let mut inter = f.interface("com.deepin.daemon.Zone");

    // create signals
    let mut z = zone.borrow_mut();
    *z.s.borrow_mut() = Some(inter.add_s_ref(f.signal("TestSignal").arg(("arg", "a{sd}"))));

    let inter = inter.add_m(zone_detected)
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
            ConnectionItem::Nothing => {
            },
            _ => {
                println!("{:?}", item);
            },
        }
    }
}

fn main() {

    gtk::init().unwrap();

    thread::spawn(create_dbus_service);

    gtk::main();
}
