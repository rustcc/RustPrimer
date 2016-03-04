use std::thread;
use std::sync:Mutex;

fn main() {
   let mut data = Mutex::new(vec![1u32, 2, 3]);
   
   for i in 0..3 {
       let data = data.lock().unwrap();
       thread::spawn(move || {
       	  data[i] += 1;
       });
   }

   thread::sleep_ms(50);
}