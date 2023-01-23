
// fn retar_solution(ov: char, mv: char) -> i32 {
//     let mut totalpoint = 0;
//     if mv == 'X' {
//         if ov == 'A' {
//             totalpoint += 3;
//         }
//         // Win
//         else if ov == 'B' {
//             totalpoint += 0; // paper
//         } else if ov == 'C' {
//             totalpoint += 6; // scissors
//         }
//         totalpoint += 1;
//     } else if mv == 'Y' {
//         if ov == 'A' {
//             totalpoint += 6;
//         } else if ov == 'B' {
//             totalpoint += 3;
//         } else if ov == 'C' {
//             totalpoint += 0;
//         }
//         totalpoint += 2;
//     } else if mv == 'Z' {
//         if ov == 'A' {
//             totalpoint += 0;
//         } else if ov == 'B' {
//             totalpoint += 6;
//         } else if ov == 'C' {
//             totalpoint += 3;
//         }
//         totalpoint += 3;
//     }
//     totalpoint
// }
pub(crate) fn solve(buf:Option<String>) {
    let mut msg: String = String::new();
    if let Some(str) = buf {
        msg = str;
    } else {
        println!("Unable to load the file\n");
    }

    let mut good_total: i64 = 0;
    // let mut retar_total: i64 = 0;

    msg.lines().for_each(|line| {
        let truncated_line = line.replace(" ", "");
        let bytes = truncated_line.as_bytes();
        let ov = (bytes[0] - b'A') as i64; // ascii haha 0 if A, 1 if B, 3 if C
        let mv = (bytes[1] - b'X') as i64; // same as above, juse uses xyz

        good_total += mv + 1; // shape score, 0+1 = 1 if rock (since X will return 0) and so on
                              // find outcome using division
        let outcome = (mv - ov + 1).rem_euclid(3); // % doesnt work here
        good_total += 3 * outcome;

        // let opponent_move = truncated_line.chars().nth(0).unwrap();
        // let my_move = truncated_line.chars().nth(1).unwrap();
        // retar_total += retar_solution(opponent_move, my_move) as i64;
    });
    // println!("Good solution returns {good_total} and Bad solution also returns {retar_total}");

    println!("\tPart1 : {good_total}");

    // part2 
    let mut part2_total:i64 = 0;
    msg.lines().for_each(|line| {
        let truncated_line = line.replace(" ", "");
        let bytes = truncated_line.as_bytes();
        let ov = (bytes[0] - b'A') as i64; // ascii haha 0 if A, 1 if B, 3 if C
        let outcome = (bytes[1] - b'X') as i64; 

        let mv: i64 = (ov-1+ outcome).rem_euclid(3);
        part2_total += (mv+1) + (outcome*3);
    });
    println!("\tPart2 : {part2_total}");


}