pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![];

    for &factor in factors {
        let mults = (1..limit)
            .map(|x| x * factor)
            .filter(|x| *x < limit)
            .filter(|x| !multiples.contains(x))
            .collect::<Vec<u32>>();
        multiples.extend(mults);
    }
    multiples.iter().sum()
}
