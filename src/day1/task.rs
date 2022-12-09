pub fn run() -> color_eyre::Result<()> {
    let input = include_str!("./input.txt");

    let mut max = 0;
    for group in input.replace("\r\n", "\n").split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().unwrap();
            sum += value;
        }

        if sum > max {
            max = sum;
        }
    }
    println!("The fattest of asses ate {max} calories");
    Ok(())
}
