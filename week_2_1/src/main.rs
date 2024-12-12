fn main() -> std::io::Result<()> {
    println!(
        "Sum of squares of non-negative integers: {}",
        std::fs::read_to_string("data/OCTO-Coding-Challenge-2024-Week-2-Part-1-input.txt")?
            .lines()
            .filter_map(|line| line.parse::<i32>().ok().filter(|&num| num >= 0))
            .fold(0, |acc, num| acc + num * num)
    );
    Ok(())
}
// 2606911