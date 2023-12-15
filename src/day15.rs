use std::fs;
use std::collections::HashMap;

fn hash_algorithm(v: &str) -> usize {
    // Just looping maths.

    v.chars().fold(0, |acc, x| {
        return ((acc + u64::from(x) as usize) * 17) % 256;
    })
}

fn hash_result() -> usize {
    if let Some(input) = fs::read_to_string("data/15.input").ok() {
        return input
            .trim()
            .split(',')
            .map(hash_algorithm)
            .sum();
    }

    panic!("file not found")
}

fn hashmap_algorithm(boxes: &mut HashMap<usize, Vec<(String, usize)>>, step: &str) {
    // Complex logic today.
    // Every “HASHMAP” step is either a “removal” label or an “insert/modify” label + focal length.
    //  Removal: if our boxes contain the label, remove it.
    //  Insert/modify: if our boxes contain the label, modify it in place to the new focal length.
    //                 otherwise, insert it.
    // The box a lens goes to is the result of HASH from part one, applied to the label.

    if let Some((label, focal_length)) = step.split_once('=') {
        let light_box = hash_algorithm(label);
        let focal_length = focal_length.parse::<usize>().unwrap();

        // Insert/modify.
        boxes.entry(light_box).and_modify(|lens_box| {
            // If exists...
            if let Some((index, _)) = lens_box
                .iter()
                .enumerate()
                .filter(|(_, lens)| lens.0 == label)
                .nth(0)
            {
                // ...modify.
                lens_box[index] = (label.to_string(), focal_length);
            } else {
                // Else: insert.
                lens_box.push((label.to_string(), focal_length));
            }
        }).or_insert(vec![(label.to_string(), focal_length)]);

    } else {
        if let Some((label, _)) = step.split_once('-') {
            let light_box = hash_algorithm(label);

            // Removal.
            boxes.entry(light_box).and_modify(|lens_box| {
                // If exists.
                if let Some((index, _)) = lens_box
                    .iter()
                    .enumerate()
                    .filter(|(_, lens)| lens.0 == label)
                    .nth(0)
                {
                    lens_box.remove(index);
                }
            });
        }
    }
}

fn hashmap_result() -> usize {
    if let Some(input) = fs::read_to_string("data/15.input").ok() {
        let mut boxes = HashMap::new();

        input
            .trim()
            .split(',')
            .for_each(|step| hashmap_algorithm(&mut boxes, step));

        return boxes
            .into_iter()
            .map(|(box_num, vals)| {
                return vals.iter().enumerate().map(|(idx, lens)| {
                    // Box index * lens index in box * lens strength.
                    return (box_num + 1) * (idx + 1) * lens.1;
                }).sum::<usize>();
            }).sum();
    }

    panic!("file not found");
}

fn main() {
    println!("part one: {}", hash_result());
    println!("part two: {}", hashmap_result());
}
