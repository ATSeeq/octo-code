fn main() -> std::io::Result<()> {
    println!(
        "Line with the most vowels is {}.",
        std::fs::read_to_string("data/OCTO-Coding-Challenge-2024-Week-1-Part-2-input.txt")?
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let vowel_count = line
                    .chars()
                    .filter(|c| "aeiouy".contains(c.to_ascii_lowercase()))
                    .count();
                (index, vowel_count)
            })
            .max_by_key(|&(_, count)| count)
            .map(|(index, _)| index)
            .unwrap()
    );

    Ok(())
}
