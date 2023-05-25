use biquad::*;

pub struct LowPassFilter {
    fs: Hertz<f32>,
    f0: Hertz<f32>,
    biquad1: DirectForm1<f32>,
    biquad2: DirectForm2Transposed<f32>,
}

impl LowPassFilter {
    pub fn new(fs: f32, f0: f32) -> Self {
        // implementing a constructing function
        let coeffs = Coefficients::<f32>::from_params(Type::LowPass, fs.hz(), f0.hz(), Q_BUTTERWORTH_F32).unwrap();
        let mut lpf = LowPassFilter {
            fs: fs.hz(),
            f0: f0.hz(),
            biquad1: DirectForm1::<f32>::new(coeffs),
            biquad2: DirectForm2Transposed::<f32>::new(coeffs),
        };
        lpf //implicit return
    }

    pub fn process(&mut self, input: (f32, f32)) -> (f32, f32) {
        // a function that takes in reference to self, a pair of stereo samples
        // spits out a pair of output samples
        let mut out = (0.0, 0.0);
        out.0 = self.biquad1.run(input.0);
        out.1 = self.biquad1.run(input.1);
        out
    }

    pub fn set_f0(&mut self, value: f32) {
        self.f0 = value.hz();
        self.update_biquad();
    }

    pub fn set_fs(&mut self, value: f32) {
        self.fs = value.hz();
        self.update_biquad();
    }

    pub fn update_biquad(&mut self) {
        let coeffs = Coefficients::<f32>::from_params(Type::LowPass, self.fs, self.f0, Q_BUTTERWORTH_F32).unwrap();
        self.biquad1 = DirectForm1::<f32>::new(coeffs);
        self.biquad2 = DirectForm2Transposed::<f32>::new(coeffs);
    }
}