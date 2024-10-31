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
/// Variable for old received value. For костыль.
static mut PREV_NEIGHBOUR_VALUE: u8 = 0;

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
        let mut next_send_time = NEXT_SEND_TIME.take().expect("Next send time error in main");

        // One "tick"
        if current_millis() >= next_send_time {
            // Send value to neighbours
            next_send_time = current_millis() + 5 * 1000;
            println!("Send value: {:?}", VALUE);
            let status = esp_now
                .send(&BROADCAST_ADDRESS, &[VALUE])
                .unwrap()
                .wait();
            println!("Send broadcast status: {:?}", status);

            // Receive value from neighbours
            let r = esp_now.receive();
            if let Some(r) = r {
                println!("Received data {:?}", r.get_data());
                let data = r.get_data();
                let received_value = data[0];
                // Костыль, works only for 2 agents
                if PREV_NEIGHBOUR_VALUE != received_value {
                    // Local voting protocol
                    VALUE = (VALUE + received_value) / 2;
                }
                PREV_NEIGHBOUR_VALUE = received_value;
            }
        }

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
