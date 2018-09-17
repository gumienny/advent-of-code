extern crate md5;

use md5::compute;

static puzzle: &'static str = "bgvyzdsv";

fn test_range(start: usize, end: usize, zeros: usize) -> usize {
    let mut answer = 0;

    for salt in start..end {
        let f = format!("{}{}", puzzle, salt);

        let digest = format!("{:02x}", compute(f.as_str()));

        if test_digest(digest.as_str(), zeros) {
            println!("{} -> {}", salt, digest);
            answer = salt;
            break;
        }
    }

    answer
}

fn test_digest(s: &str, zeros: usize) -> bool {
    s.get(0..zeros).unwrap().chars().all(|ch: char| ch == '0')
}

fn main() {
    // std::usize::MAX is equal to !0usize
    let part_1_answer = test_range(1, !0, 5);

    println!("Part 1: {}", part_1_answer);

    let part_2_answer = test_range(part_1_answer, !0, 6);

    println!("Part 2: {}", part_2_answer);
}

// 254575  -> 000004b30d481662b9cb0c105f6549b2
// 1038736 -> 000000b1b64bf5eb55aad89986126953
