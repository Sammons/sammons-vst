

use crate::Arc;

pub struct MyReverbParams {
    // sample_rate: i32,
    decay: f32
}
pub struct RingBuffer {
    buffer: Vec<f32>,
    read_position: i32,
    write_position: i32,   
}
impl RingBuffer {
    fn new(vec: Vec<f32>) -> RingBuffer {
        return RingBuffer {
            buffer: vec,
            read_position: 1,
            write_position: 0,
        };
    }
    fn read_next(&mut self) -> f32 {
        let cur = self.buffer[self.read_position as usize];
        self.read_position = ((self.read_position + 1) as usize % self.buffer.len()) as i32;
        return cur;
    }
    fn write_next(&mut self, sample: &f32) {
        self.buffer[self.write_position as usize] = *sample;
        self.write_position = ((self.write_position + 1) as usize % self.buffer.len()) as i32;
    }
}

pub struct MyReverb {
    params: Arc<MyReverbParams>,
    buffers: Vec<RingBuffer>,
}

impl MyReverb {
    pub fn new(sample_rate: i32) -> MyReverb {
        let decay_per_bounce = 0.1f32; // only 10% remaining upon bounce
        let mut echo_buffers = Vec::new();
        let min_duration_echo_time = 0.10f32;
        let max_duration_echo_time = 2.500f32;
        let gradient = 0.25;
        let mut iter_val = min_duration_echo_time;
        loop {
            if iter_val > max_duration_echo_time {
                break;
            }
            iter_val += gradient;
            let sample_length = (iter_val * (sample_rate as f32)) as usize;
            if sample_length > 1 {
                let mut new_buff = Vec::with_capacity(sample_length);
                new_buff.resize(sample_length, 0f32);
                echo_buffers.push(RingBuffer::new(new_buff));
            }
        }
        return MyReverb {
            params: Arc::new(MyReverbParams {decay: decay_per_bounce}),
            buffers: echo_buffers
        }
    }
    pub fn process(&mut self, sample: &mut f32) {
        let mut receive = 0f32;
        let max_gradient = 0.6;
        for x in 0..self.buffers.len() {
            // consume and degrade each echo
            // this gives a linear decay per buffer
            // but we have a wide variety of buffer lengths so it should give something interesting
            // let polarity = if x % 2 == 0 { 1.0f32 } else { -1.0f32 };
            let midpoint = self.buffers.len()/3;
            let gradient: f32 = if x > midpoint { (1.0 - ((x as f32 - midpoint as f32) / midpoint as f32)) * max_gradient } else { ((x as f32) / midpoint as f32) * max_gradient };
            receive += self.buffers[x].read_next() * gradient;
        }
        // receive = (receive / (self.buffers.len() as f32)) + *sample;
        receive = (receive / self.buffers.len() as f32) + ((*sample) * 100.0);
        for rb in self.buffers.iter_mut() {
            (*rb).write_next(&receive);
        }
        *sample = receive / 100.0;
    }
}