#![no_main]
use libfuzzer_sys::fuzz_target;
use priority_queue::DoublePriorityQueue;
use priority_queue::PriorityQueue;

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }

    let opt = data[0] % 2;

    let mut pq = PriorityQueue::new();
    let mut dpq = DoublePriorityQueue::new();

    let input_val_size: usize = data[1].into();
    let new_data = &data[2..];

    let mut idx = 0;
    let mut end_idx = 0;
    let mut do_break = false;

    loop {
        end_idx = idx + input_val_size;
        if end_idx >= new_data.len() {
            end_idx = new_data.len() - 1;
            do_break = true;
        }

        if opt == 0 {
            pq.push(&new_data[idx..end_idx], new_data[end_idx]);
        } else {
            dpq.push(&new_data[idx..end_idx], new_data[end_idx]);
        }

        if do_break {
            break;
        }

        idx = end_idx + 1;

        if idx >= new_data.len() {
            break;
        }
    }

    if opt == 0 {
        pq.into_sorted_iter().count();
    } else {
        dpq.into_sorted_iter().count();
    }
});
