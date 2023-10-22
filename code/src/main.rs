use macros::include_input;

fn main() {
    include_input! {
        numbers: ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8))
    };

    println!("{:?}", numbers);
}
