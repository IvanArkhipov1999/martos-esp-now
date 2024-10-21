#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::entry;
use esp_println::println;
use esp_wifi::{
    current_millis,
    esp_now::{EspNow, BROADCAST_ADDRESS},
};
use martos::get_esp_now;
use martos::init_system;
use martos::task_manager::TaskManager;

/// Esp-now object for network
static mut ESP_NOW: Option<EspNow> = None;
/// Variable for saving time to send broadcast message
static mut NEXT_SEND_TIME: Option<u64> = None;
/// Value for local vote protocol.
static mut VALUE: u8 = 1;

/// Setup function for task to execute.
fn setup_fn() {
    println!("Setup hello world!");
    unsafe {
        ESP_NOW = Some(get_esp_now());
        NEXT_SEND_TIME = Some(current_millis() + 5 * 1000);
    }
}

/// Loop function for task to execute.
fn loop_fn() {
    unsafe {
        let mut esp_now = ESP_NOW.take().expect("Esp-now error in main");
        let mut value = VALUE;

        // Receive value from neighbours
        let r = esp_now.receive();
        if let Some(r) = r {
            println!("Received {:?}", r);
            let data = r.get_data();
            let received_value = data[0];
            // Local voting protocol
            value = (value + received_value) / 2;
        }

        // Send value to neighbours
        let mut next_send_time = NEXT_SEND_TIME.take().expect("Next send time error in main");
        if current_millis() >= next_send_time {
            next_send_time = current_millis() + 5 * 1000;
            println!("Send value: {:?}", value);
            let status = esp_now
                .send(&BROADCAST_ADDRESS, &[value.try_into().unwrap()])
                .unwrap()
                .wait();
            println!("Send broadcast status: {:?}", status)
        }

        VALUE = value;

        NEXT_SEND_TIME = Some(next_send_time);
        ESP_NOW = Some(esp_now);
    }
}

/// Stop condition function for task to execute.
fn stop_condition_fn() -> bool {
    return false;
}

#[entry]
fn main() -> ! {
    // Initialize Martos.
    init_system();
    // Add task to execute.
    TaskManager::add_task(setup_fn, loop_fn, stop_condition_fn);
    // Start task manager.
    TaskManager::start_task_manager();
}
