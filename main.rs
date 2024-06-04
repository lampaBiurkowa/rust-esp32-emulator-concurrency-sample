use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use esp_idf_svc::hal::{
    peripherals::Peripherals,
    gpio::*,
    prelude::*,
};

fn main() {
    esp_idf_svc::sys::link_patches();

    let shared_data = Arc::new(Mutex::new(0));
    let peripherals = Peripherals::take().unwrap();
    let mut pins = peripherals.pins;

    let shared_data1 = Arc::clone(&shared_data);
    let handle1 = thread::spawn(move || {
        let pin1 = &mut pins.gpio2;
        for _ in 0..10 {
            increment_thread_safe("Task 1", shared_data1.clone());
            blink_led(pin1);
        }
    });

    let shared_data2 = Arc::clone(&shared_data);
    let handle2 = thread::spawn(move || {
        let pin2 = &mut pins.gpio4;
        for _ in 0..10 {
            increment_thread_safe("Task 2", shared_data2.clone());
            blink_led(pin2);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final value: {}", *shared_data.lock().unwrap());
}

fn increment_thread_safe(thread_name: &str, thread_arc: Arc<Mutex<i32>>) {
    //in brackets in order to release the lock after increment
    {
        let mut data = thread_arc.lock().unwrap();
        *data += 1;
        println!("{}: {}", thread_name, *data);
    }
}

fn blink_led<P>(pin: &mut P) where P: OutputPin {
    let mut led = PinDriver::output(pin).unwrap();
    led.set_low().unwrap();
    thread::sleep(Duration::from_millis(1000));
    led.set_high().unwrap();
}
