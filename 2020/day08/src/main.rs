use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("/tmp/input8").unwrap();
    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();
    let input = s.trim();

    let width = 25;
    let height = 6;
    let layer_pixels = width * height;
    let mut offset = 0;
    let mut layers = Vec::new();

    let mut min_zeros = layer_pixels+1;
    let mut winning_layer = 0;
    let mut current_layer = 0;
    
    while offset < input.len() {
        let layer = &input[offset..offset+layer_pixels];
        layers.push(layer);

        let zeros = layer.chars().filter(|c| c == &'0').count();
        if zeros < min_zeros {
            min_zeros = zeros;
            winning_layer = current_layer;
        }

        current_layer += 1;
        offset += layer_pixels;
    }

    let layer = layers[winning_layer];
    let result = layer.chars().filter(|c| c == &'1').count() * layer.chars().filter(|c| c == &'2').count();
    println!("Result: {}", result);
}