use similar::{ChangeTag, TextDiff};

pub fn print_diff(old: &str, new: &str) {
    println!("%%%%% START OF DIFF %%%%%");
    let diff = TextDiff::from_lines(old, new);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }
    println!("@@@@@ END OF DIFF @@@@@");
}
