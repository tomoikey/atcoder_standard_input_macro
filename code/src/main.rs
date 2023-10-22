use macros::include_input;

fn main() {
    include_input! {
        hello: String,
        one: i128,
        array: [u8; 5],
        array_array: [[i32; 3]; 3],
        array_tuple1: [(u16, String); 2],
        array_tuple2: [(u16, String, usize, usize); 2],
    };

    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?}",
        hello, one, array, array_array, array_tuple1, array_tuple2
    );
}
