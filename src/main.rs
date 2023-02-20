#[macro_use]
mod geom;
mod color;
mod context;
mod device;
mod gesture;
mod helpers;
mod input;
mod metadata;
mod rtc;
mod sketch;
mod unit;

pub mod battery;
pub mod document;
pub mod font;
pub mod framebuffer;
pub mod frontlight;
pub mod lightsensor;
pub mod settings;
pub mod view;

use battery::*;
use context::*;
use device::*;
use font::*;
use framebuffer::*;
use frontlight::*;
use gesture::*;
use helpers::*;
use input::*;
use lightsensor::*;
use settings::*;

use sketch::Sketch;
use view::common::*;
use view::menu::*;
use view::{handle_event, process_render_queue, wait_for_all};
use view::{AppCmd, EntryId, EntryKind, Event, RenderData, RenderQueue, UpdateData, View, ViewId};

use std::collections::VecDeque;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

const FB_DEVICE: &str = "/dev/fb0";

const TOUCH_INPUTS: [&str; 3] = [
    "/dev/input/by-path/platform-1-0010-event",
    "/dev/input/by-path/platform-0-0010-event",
    "/dev/input/event1",
];
const BUTTON_INPUTS: [&str; 4] = [
    "/dev/input/by-path/platform-gpio-keys-event",
    "/dev/input/by-path/platform-ntx_event0-event",
    "/dev/input/by-path/platform-mxckpd-event",
    "/dev/input/event0",
];
const CLOCK_REFRESH_INTERVAL: Duration = Duration::from_secs(60);
const BATTERY_REFRESH_INTERVAL: Duration = Duration::from_secs(299);

fn main() {
    /*
    let mut sketch = Sketch::new();
    sketch.run();
    */
    //context.fb.set_monochrome(true);

    let path = String::from("./settings.toml");
    let mut settings = load_toml::<Settings, _>(path)
        .map_err(|e| eprintln!("Can't load settings: {:#}.", e))
        .unwrap_or_default();

    let battery = Box::new(FakeBattery::new()) as Box<dyn Battery>;
    let frontlight = Box::new(LightLevels::default()) as Box<dyn Frontlight>;
    let lightsensor = Box::new(0u16) as Box<dyn LightSensor>;
    let fonts = Fonts::load().unwrap();

    let mut fb: Box<dyn Framebuffer> = if CURRENT_DEVICE.mark() != 8 {
        Box::new(KoboFramebuffer1::new(FB_DEVICE).unwrap())
    } else {
        Box::new(KoboFramebuffer2::new(FB_DEVICE).unwrap())
    };
    let initial_rotation = CURRENT_DEVICE.transformed_rotation(fb.rotation());
    let startup_rotation = CURRENT_DEVICE.startup_rotation();
    fb.set_rotation(initial_rotation).ok();

    let mut rq = RenderQueue::new();
    let mut context: Context = Context::new(fb, settings, fonts, battery, frontlight, lightsensor);

    println!("Sketch is running on a Kobo {}.", CURRENT_DEVICE.model);
    println!(
        "The framebuffer resolution is {} by {}.",
        context.fb.rect().width(),
        context.fb.rect().height()
    );

    let mut paths = Vec::new();
    for ti in &TOUCH_INPUTS {
        if Path::new(ti).exists() {
            paths.push(ti.to_string());
            break;
        }
    }
    for bi in &BUTTON_INPUTS {
        if Path::new(bi).exists() {
            paths.push(bi.to_string());
            break;
        }
    }
    let (raw_sender, raw_receiver) = raw_events(paths);

    let touch_screen = gesture_events(device_events(
        raw_receiver,
        context.display,
        context.settings.button_scheme,
    ));

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        while let Ok(evt) = touch_screen.recv() {
            tx2.send(evt).ok();
        }
    });

    let tx4 = tx.clone();
    thread::spawn(move || loop {
        thread::sleep(CLOCK_REFRESH_INTERVAL);
        tx4.send(Event::ClockTick).ok();
    });

    let tx5 = tx.clone();
    thread::spawn(move || loop {
        thread::sleep(BATTERY_REFRESH_INTERVAL);
        tx5.send(Event::BatteryTick).ok();
    });

    context.fb.set_monochrome(true);

    let mut view: Box<dyn View> = Box::new(Sketch::new(context.fb.rect(), &mut rq, &mut context));
    let mut updating = Vec::new();
    let mut bus = VecDeque::with_capacity(4);

    tx.send(Event::WakeUp).ok();

    while let Ok(evt) = rx.recv() {
        match evt {
            Event::Select(EntryId::Quit) => {
                println!("Received quit message");
                // This doesnt work, clearing the screen after exit needs to be done outside
                //context.fb.set_monochrome(true);
                break;
            }
            Event::Close(id) => {
                if let Some(index) = locate_by_id(view.as_ref(), id) {
                    let rect = overlapping_rectangle(view.child(index));
                    rq.add(RenderData::expose(rect, UpdateMode::Gui));
                    view.children_mut().remove(index);
                }
            },
            _ => {
                handle_event(view.as_mut(), &evt, &tx, &mut bus, &mut rq, &mut context);
            }
        }

        process_render_queue(view.as_ref(), &mut rq, &mut context, &mut updating);

        while let Some(ce) = bus.pop_front() {
            tx.send(ce).ok();
        }
    }
    println!("Exiting");
}
