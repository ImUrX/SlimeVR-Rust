#![no_std]
#![no_main]
// Needed for embassy macros
#![feature(type_alias_impl_trait, const_option)]
#![deny(unsafe_op_in_unsafe_fn)]

mod aliases;
mod globals;
mod imu;
mod networking;
mod peripherals;
mod utils;

use cfg_if as ඞ;
use defmt::{debug, trace};
use embassy_executor::{task, Executor};
use embassy_futures::yield_now;
use embedded_svc::wifi::Wifi;
use riscv_rt::entry;
use static_cell::StaticCell;

#[entry]
fn main() -> ! {
	self::globals::setup();
	debug!("Booted");

	let p = self::peripherals::ඞ::get_peripherals();
	debug!("Initialized peripherals");
	p.delay.delay(1000);

	static EXECUTOR: StaticCell<Executor> = StaticCell::new();
	EXECUTOR.init(Executor::new()).run(move |spawner| {
		spawner.spawn(network_task()).unwrap();
		spawner.spawn(imu_task(p.i2c, p.delay)).unwrap();
	});
}

#[task]
async fn network_task() {
	debug!("Started network_task");

	ඞ::cfg_if! {
		if #[cfg(feature = "esp-wifi")] {
			use esp_wifi::{
				create_network_stack_storage,
				network_stack_storage,
				wifi::utils::create_network_interface
			};
			let mut storage = create_network_stack_storage!(3, 8, 1, 1);
			let ethernet = create_network_interface(network_stack_storage!(storage));
			let mut wifi = esp_wifi::wifi_interface::Wifi::new(ethernet);
		}
	}
	let mut i = 0;
	loop {
		trace!("In main(), i was {}", i);
		i += 1;
		yield_now().await
		//Timer::after(Duration::from_millis(1000)).await
	}
}

#[task]
async fn imu_task(
	i2c: crate::aliases::ඞ::I2cConcrete,
	delay: crate::aliases::ඞ::DelayConcrete,
) {
	crate::imu::imu_task(i2c, delay).await
}
