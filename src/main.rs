use glob::glob;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::{string::String, vec::Vec};
use texture_synthesis as ts;

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    for entry in glob("inputs/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => inputs.push(String::from(path.to_str().unwrap())),
            Err(e) => println!("{:?}", e),
        }
    }
    for count in 1..1000 {
        inputs.shuffle(&mut thread_rng());
        helper(&inputs[0], &inputs[1], count);
        println!("Generated image {}.", count);
    }
    println!("Done!");
}

fn helper(img_1: &str, img_2: &str, count: i32) -> Result<(), ts::Error> {
    let texsynth = ts::Session::builder()
        .add_examples(&[img_1, img_2])
        .resize_input(ts::Dims {
            width: 2000,
            height: 2000,
        })
        .output_size(ts::Dims {
            width: 2000,
            height: 2000,
        })
        .random_init(10)
        .seed(thread_rng().gen::<u64>())
        .max_thread_count(4)
        .build()?;

    let output = texsynth.run(None);
    output.save(format!("outputs/{}.png", count))?;
    output.save_debug("debug/")
}
