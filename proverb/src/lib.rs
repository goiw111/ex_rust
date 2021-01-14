pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        x if x > 0 => {
            vec![list
                .windows(2)
                .map(|i|format!("For want of a {} the {} was lost.",i[0],i[1]))
                .collect::<Vec<String>>().join("\n"),
                format!("And all for the want of a {}.",list[0])]
                    .into_iter().filter(|i| !i.is_empty()).collect::<Vec<_>>()
                    .join("\n")},
        _               => String::new(),
    }
}
