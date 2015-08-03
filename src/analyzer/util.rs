use num::Complex;
use rft;
use strided::Strided;

pub fn channels(samples: &[i16]) -> (Vec<Complex<f64>>, Vec<Complex<f64>>, Vec<Complex<f64>>) {
	// our frames are stereo packed i16, so split the two channels
	let (left, right) = samples.as_stride().substrides2();

	// average the two channels to get a mono channel
	let mono = left.iter().zip(right.iter())
		.map(|(&l, &r)| ((l as i32 + r as i32) / 2) as i16)
		.collect::<Vec<i16>>();

	// run a hamming window FFT on all of them
	(rft::forward(rft::window::hamming(mono)),
	 rft::forward(rft::window::hamming(left)),
	 rft::forward(rft::window::hamming(right)))
}
