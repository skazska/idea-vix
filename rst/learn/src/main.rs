
mod stat;
mod sorts;
mod registry;


mod text_conversion;

fn main() {
    // let mut vec = vec![4,5];
    let mut vec = vec![5.0, 3.0, 1.0, 4.0, 2.0];
    // Example usage of the median function
    let median = stat::median(&mut vec);
    let mode = stat::mode(&vec);

    println!("Sorted Vector: {:?}", vec);
    println!("Median: {:?}", median);
    print!("Mode: {:?}", mode);

    println!("In Pig-latin: {}", text_conversion::to_pig_latin("Hello, world!"));
}