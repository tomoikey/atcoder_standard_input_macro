use play_ground::include_input;

fn main() {
    include_input! {
        n: u8,
        strings: [String; n],
        favorite_food: String
    };
    println!("{}\n{:?}\n{}", n, strings, favorite_food);
}
