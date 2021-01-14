pub fn verse(n: u32) -> String {
    return format!("{} {}",
        match n {
            1          => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around,"),
            i @ 2..=99 => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around,",i),
            _          => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more,")
        },
        match n {
            1        => format!("no more bottles of beer on the wall.\n"),
            2        => format!("1 bottle of beer on the wall.\n"),
            i @ _ => format!("{} bottles of beer on the wall.\n",if i > 2 {i-1} else {99})
        });
}

pub fn sing(start: u32, end: u32) -> String {
    if start > end {(end..=start)
        .map(|x|verse(x))
            .rev()
            .collect::<Vec<String>>()} 
    else {(start..=end)
        .map(|x|verse(x))
            .collect::<Vec<String>>()}
    .join("\n")
}

