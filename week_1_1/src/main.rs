fn main() -> std::io::Result<()> {
    println!(
        "The number of 'r' characters in the file is {}",
        std::fs::read_to_string("data/OCTO-Coding-Challenge-2024-Week-1-Part-1-input.txt")?
            .chars()
            .filter(|&c| c == 'r' || c == 'R')
            .count()
    );
    Ok(())
}
