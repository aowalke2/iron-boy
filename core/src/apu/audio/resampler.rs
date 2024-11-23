use super::Sample;

const PI: f32 = std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct CosineResampler {
    phase: f32,
    last_input_sample: Sample<f32>,
    input_frequency: f32,
    output_frequency: f32,
}

impl CosineResampler {
    pub fn new(in_freq: f32, out_freq: f32) -> CosineResampler {
        CosineResampler {
            phase: 0.0,
            last_input_sample: Default::default(),
            input_frequency: in_freq,
            output_frequency: out_freq,
        }
    }

    pub fn feed(&mut self, s: &Sample<f32>, output: &mut Vec<Sample<f32>>) {
        while self.phase < 1.0 {
            let left = cosine_interpolation(self.last_input_sample[0], s[0], self.phase);
            let right = cosine_interpolation(self.last_input_sample[1], s[1], self.phase);
            output.push([left, right]);
            self.phase += self.input_frequency / self.output_frequency;
        }
        self.phase -= 1.0;
        self.last_input_sample = *s;
    }
}

fn cosine_interpolation(y1: f32, y2: f32, phase: f32) -> f32 {
    let mu = (1.0 - (PI * phase).cos()) / 2.0;
    y2 * (1.0 - mu) + y1 * mu
}
