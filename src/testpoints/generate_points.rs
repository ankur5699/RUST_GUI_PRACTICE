use std::f64::consts::PI;

const NUM_SAMPLES: usize = 4096;

//Shall Generate moving points
fn sine_array(fs: f64, freq: f64) -> [f64; NUM_SAMPLES] {
    //Assume all to be in Mhz, This is just a test function
    //Can try 30.72 Mhz as fs
    let mut buffer = [0.0 as f64; NUM_SAMPLES];
    let w = 2.0 * PI * freq;
    let mut t: f64;
    let dt = 1.0 / fs;
    for index in 0..NUM_SAMPLES {
        t = (index as f64) * dt;
        let i = index as usize;
        buffer[i] = f64::sin(w * t);
    }
    return buffer;
}

fn convert2hex_16bit(buffer: [f64; NUM_SAMPLES]) -> [i16; NUM_SAMPLES] {
    //MIN = -32768 and MAX = 32767
    let min: i32 = -32768;
    let max: i32 = 32767;
    let mut quantized_buffer = [0 as i16; NUM_SAMPLES];

    for i in 0..NUM_SAMPLES {
        let quantized1: i32 = (buffer[i] * (max as f64)) as i32;
        let quantized2: i32 = if quantized1 > max { max } else { quantized1 };
        let quantized3: i16 = if quantized2 < min {
            min as i16
        } else {
            quantized2 as i16
        };
        quantized_buffer[i] = quantized3;
    }
    return quantized_buffer;
}

#[cfg(test)]
mod unit_tests {
    #[test]
    fn test_convert2hex_1() {
        todo!();
        assert!(0, 0);
    }
    fn test_convert2hex_2() {
        todo!();
        assert!(0, 0);
    }
    fn test_convert2hex_3() {
        todo!();
        assert!(0, 0);
    }
}
