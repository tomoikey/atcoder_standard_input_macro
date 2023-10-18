use play_ground::include_input;

fn main() {
    include_input! {
        n: u8,
        strings1: [String; n],
        m: u8,
        strings2: [(String, u32); m],
        numbers: [u64; 3]
    };

    println!(
        "{}\n{:?}\n{}\n{:?}\n{:?}",
        n, strings1, m, strings2, numbers
    );
}
