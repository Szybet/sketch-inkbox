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
pub mod font;
pub mod framebuffer;
pub mod frontlight;
pub mod lightsensor;
pub mod settings;
pub mod view;

use battery::*;
use frontlight::*;
use lightsensor::*;
use font::*;
use device::*;
use framebuffer::*;
use sketch::Sketch;
use context::*;
use settings::*;
use helpers::*;
use view::RenderQueue;

const FB_DEVICE: &str = "/dev/fb0";

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

    let mut rq = RenderQueue::new();

    let mut context: Context = Context::new(fb, settings, fonts, battery, frontlight, lightsensor);
    Box::new(Sketch::new(context.fb.rect(), &mut rq, &mut context));
}
