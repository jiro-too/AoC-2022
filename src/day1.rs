pub fn solve(buf: Option<String>) -> i64{
    let mut msg: String = String::new();
    if let Some(str) = buf {
        msg = str;
    } else {
        println!("Unable to load the file\n");
    }

    // split this shit into various groups

    let mut current_max = 0;

    msg.split("\n\n").for_each(|grp| {
        // println!("ELF:");
        let total = grp
            .lines()
            .map(|calories| {
                // println!("\t+{calories}");
                calories.parse::<i64>().unwrap()
            })
            .sum::<i64>();
        if total > current_max {
            current_max = total;
        }
    });
    // println!("DAY 1: ELF number {index} got {current_max} calories");
    current_max
}