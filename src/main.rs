use hound;
use std::i16;

fn main() {
    let mut reader1 = hound::WavReader::open("../files/audio2.wav").unwrap();
    let spec1 = reader1.spec();
    let samples1: Vec<i16> = reader1.samples::<i16>().map(|s| s.unwrap()).collect();

    let mut reader2 = hound::WavReader::open("../files/audio1.wav").unwrap();
    let spec2 = reader2.spec();
    let samples2: Vec<i16> = reader2.samples::<i16>().map(|s| s.unwrap()).collect();
    let samples2: Vec<i16> = samples2.iter().step_by(1).copied().collect(); // step by responsible for acceleration audio

    let length = samples1.len().max(samples2.len());

    let mut extended_samples2 = Vec::with_capacity(length);
    extended_samples2.extend_from_slice(&samples2);
    extended_samples2.resize(length, 0);

    let mut output_samples: Vec<i16> = Vec::with_capacity(length);

    for i in 0..length {
        let sample1 = if i < samples1.len() { samples1[i] } else { 0 };
        let sample2 = if i < extended_samples2.len() { extended_samples2[i] } else { 0 };

        let mixed_sample = sample1.saturating_add(sample2);
        output_samples.push(mixed_sample);
    }

    let spec = hound::WavSpec {
        channels: spec1.channels,
        sample_rate: spec1.sample_rate,
        bits_per_sample: spec1.bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();

    for sample in output_samples {
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("Audio files successfully merged into 'output.wav'");
}
