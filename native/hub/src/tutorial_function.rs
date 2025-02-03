use std::time::Duration;

use crate::messages::*;
use rinf::debug_print;

pub async fn stream_amazing_number() {
  let mut current_number: i32 = 1;
  loop {
      tokio::time::sleep(Duration::from_secs(1)).await;
      MyAmazingNumber { current_number }.send_signal_to_dart(); // GENERATED
      current_number += 1;
  }
}

pub async fn calculate_precious_data() {
  let receiver = MyPreciousData::get_dart_signal_receiver(); // GENERATED
  while let Some(dart_signal) = receiver.recv().await {
      let my_precious_data = dart_signal.message;

      let new_numbers: Vec<i32> = my_precious_data
          .input_numbers
          .into_iter()
          .map(|x| x + 1)
          .collect();
      let new_string = my_precious_data.input_string.to_uppercase();

      debug_print!("{new_numbers:?}");
      debug_print!("{new_string}");
  }
}