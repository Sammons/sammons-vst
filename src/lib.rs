#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Info,Plugin,Category,PluginParameters};
use vst::util::{AtomicFloat};

use crate::my_reverb::{MyReverb};
mod my_reverb;

use std::sync::Arc;

struct MyEffect {
    params: Arc<EffectParams>,
    // do not change after the initial setup
    reverb: MyReverb
}

struct EffectParams {
    sample_rate: AtomicFloat,
    pre_amp_linear: AtomicFloat,
    post_amp_linear: AtomicFloat,
}

impl Default for EffectParams {
    fn default() -> EffectParams {
        EffectParams {
            pre_amp_linear: AtomicFloat::new(1.0),
            post_amp_linear: AtomicFloat::new(1.0),
            sample_rate: AtomicFloat::new(44100.0)
        }
    }
}

impl Default for MyEffect {
    fn default() -> MyEffect {
        let params = EffectParams::default();
        return MyEffect {
            reverb: MyReverb::new(
                params.sample_rate.get() as i32,
            ),
            params: Arc::new(params),
        }
    }
}

impl Plugin for MyEffect {
    fn get_info(&self) -> Info {
        Info {
            name: "Sammons VST2.4".to_string(),
            vendor: "Sammons".to_string(),
            unique_id: 3141591,
            version: 1,
            inputs: 1,
            outputs: 1,
            parameters: 2,
            category: Category::Effect,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = self.params.pre_amp_linear.get() * (*input_sample);
                self.reverb.process(output_sample);
                *output_sample = self.params.post_amp_linear.get() * (*output_sample);
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for EffectParams {
    fn get_parameter(&self, index: i32) -> f32 {
        #[allow(clippy::single_match)]
        match index {
            0 => self.pre_amp_linear.get(),
            1 => self.post_amp_linear.get(),
            _ => 0.0
        }
    }
    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.pre_amp_linear.set(val),
            1 => self.post_amp_linear.set(val),
            _ => ()
        }
    }
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // 0 would be -100%
            // 1 would be +100%
            0 => format!("{:.0}", 100f32 * (self.pre_amp_linear.get() - 0.5) * 2f32),
            1 => format!("{:.0}", 100f32 * (self.post_amp_linear.get() - 0.5) * 2f32),
            _ => "".to_string()
        }
    }
    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Pre Gain".to_string(),
            1 => "Post Gain".to_string(),
            _ => "".to_string()
        }
    }
}

plugin_main!(MyEffect);