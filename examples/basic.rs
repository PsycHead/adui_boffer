use plotly::{Plot, Scatter};
use std::f32::consts::{E, FRAC_PI_2};

use adui_boffer::buffer::{InplaceBuffer, IoBuffer, OutOfPlaceBuffer};

fn main() {
    // create some sample times and signal
    let t: Vec<_> = (-96000..96000).map(|v| (v as f32 / 48000.0)).collect();
    let data: Vec<_> = t.iter().map(|v| (v * FRAC_PI_2).sin()).collect();

    // inplace and out of place implementations
    let ip_result = calculate_inplace(data.clone());
    let oop_result = calculate_outofplace(data.clone());

    assert_eq!(&ip_result.as_slice(), &oop_result.as_slice());
    plot(t, data, ip_result);
}

fn calculate_inplace(mut data: Vec<f32>) -> Vec<f32> {
    // inplace operations operate directly on one slice
    let mut buffer = InplaceBuffer::new(&mut data);
    some_distortion(&mut buffer);
    data
}

fn calculate_outofplace(src: Vec<f32>) -> Vec<f32> {
    // out of place operations copy from src to dst
    let mut dst = vec![0f32; src.len()];
    let mut buffer = OutOfPlaceBuffer::new(&src, &mut dst);
    some_distortion(&mut buffer);

    dst
}

// this function can be used both in place and out of place
fn some_distortion(buff: &mut impl IoBuffer<f32>) {
    for (src, dst) in buff.iter() {
        *dst = src * (1.0 - E.powf(-src.abs())) * 1.5;
    }
}

fn plot(t: Vec<f32>, orig: Vec<f32>, res: Vec<f32>) {
    let mut plot = Plot::new();

    let trace = Scatter::new(t.clone(), orig);
    plot.add_trace(trace);

    let trace = Scatter::new(t.clone(), res);
    plot.add_trace(trace);

    plot.show()
}
