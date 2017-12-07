#[cfg(test)]
mod tests {
    use super::bottom;

    #[test]
    fn it_works() {
        let progs = vec![
            (String::from("pbga"), 66, vec![]),
            (String::from("xhth"), 57, vec![]),
            (String::from("havc"), 66, vec![]),
            (String::from("ktlj"), 57, vec![]),
            (String::from("fwft"), 72, vec![String::from("ktlj"), String::from("cntj"), String::from("xhth")]),
            (String::from("qoyq"), 66, vec![]),
            (String::from("padx"), 45, vec![String::from("pbga"), String::from("havc"), String::from("qoyq")]),
            (String::from("tknk"), 41, vec![String::from("ugml"), String::from("padx"), String::from("fwft")]),
            (String::from("jptl"), 61, vec![]),
            (String::from("ugml"), 68, vec![String::from("gyxo"), String::from("ebii"), String::from("jptl")]),
            (String::from("gyxo"), 61, vec![]),
            (String::from("cntj"), 57, vec![]),
        ];
        assert_eq!(String::from("tknk"), bottom(&progs));
    }
}

use std::collections::HashMap;

pub fn bottom(programs: &Vec<(String, u32, Vec<String>)>) -> String {
    let mut refs: HashMap<&str, usize> = HashMap::new();
    for &(ref name, _, ref subs) in programs.iter().filter(|&&(_, _, ref subs)| !subs.is_empty()) {
        refs.entry(name).or_insert(0);
        for sub_name in subs {
            let entry = refs.entry(sub_name).or_insert(0);
            *entry += 1;
        }
    }
    refs.iter()
        .filter(|&(_, count)| *count == 0)
        .map(|(name, _)| name.to_string())
        .next().expect("This should be here")
}
