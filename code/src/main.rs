use macros::include_input;

fn main() {
    include_input! {
        numbers: [[u8; 3]; 3]
    };

    println!("{:?}", numbers);
}
