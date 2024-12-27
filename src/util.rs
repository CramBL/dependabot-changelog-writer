use similar::{ChangeTag, TextDiff};

pub fn print_diff(old: &str, new: &str) {
    let diff = TextDiff::from_lines(new, old);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }
}
