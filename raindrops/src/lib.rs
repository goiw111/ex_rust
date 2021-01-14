pub fn raindrops(n: u32) -> String {

        match n {
            i if i % 7 == 0 
                && i % 5 == 0 
                && i % 3 == 0 => String::from("PlingPlangPlong"),
            i if i % 3 == 0 
                && i % 7 == 0 => String::from("PlingPlong"),
            i if i % 3 == 0 
                && i % 5 == 0 => String::from("PlingPlang"),
            i if i % 5 == 0 
                && i % 7 == 0 => String::from("PlangPlong"),
            i if i % 3 == 0 => String::from("Pling"),
            i if i % 5 == 0 => String::from("Plang"),
            i if i % 7 == 0 => String::from("Plong"),
                          _ => format!("{}",n),
        }
}
