fn main() -> std::io::Result<()> {
    println!(
        "Sum of primes found in file: {}",
        std::fs::read_to_string("data/OCTO-Coding-Challenge-2024-Week-2-Part-2-input.txt")?
            .lines()
            .filter_map(|line| {
                line.parse::<usize>()
                    .ok()
                    .filter(|&n| n > 1 && (2..n).all(|d| n % d != 0))
            })
            .sum::<usize>()
    );

    Ok(())
}
// 999

