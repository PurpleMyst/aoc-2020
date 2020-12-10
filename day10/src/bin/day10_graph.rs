const COLORS: [&str; 3] = ["red", "green", "blue"];

fn main() {
    let mut adapters = [false; 256];

    let highest = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse::<u8>().unwrap())
        .inspect(|&n| adapters[n as usize] = true)
        .max()
        .unwrap();

    adapters[0] = true;
    adapters[highest as usize + 3] = true;

    println!("digraph {{");

    println!("0 [shape=box]");
    println!("{} [shape=box]", highest + 3);

    for (n, _) in adapters.iter().enumerate().filter(|&(_, &val)| val) {
        for d in 1..=3 {
            if adapters[n + d] {
                println!("{} -> {} [color={}]", n, n + d, COLORS[d - 1]);
            }
        }
    }
    println!("}}");
}
