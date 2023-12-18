use d07::*;
mod p2;

fn main() {
    let mut hands = parse_input("./test.txt");

    println!("hands");
    for hand in &hands {
        println!("{:?}", hand);
    }
    hands.sort_by_key(|h| h.key.clone());

    println!("sorted");
    for hand in &hands {
        println!("{:?}", hand);
    }

    let sum = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| -> i32 {(1 + idx as i32) * (val.bid as i32) })
        .sum::<i32>();
    println!("sum {:?}", sum);


    let mut hands = parse_input("./input.txt");
    
    hands.sort_by_key(|h| h.key.clone());

    // for hand in &hands {
    //     println!("{:?}", hand);
    // }

    let sum = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| -> i32 {(1 + idx as i32) * (val.bid as i32) })
        .sum::<i32>();
    println!("sum {:?}", sum);

    // part 2: Test
    let mut hands = p2::parse_input("./test.txt");

    println!("hands");
    for hand in &hands {
        println!("{:?}", hand);
    }
    hands.sort_by_key(|h| h.key.clone());

    println!("sorted");
    for hand in &hands {
        println!("{:?}", hand);
    }

    let sum = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| -> i32 {(1 + idx as i32) * (val.bid as i32) })
        .sum::<i32>();
    println!("sum {:?}", sum);


    // part 2
    let mut hands = p2::parse_input("./input.txt");
    
    hands.sort_by_key(|h| h.key.clone());

    // for hand in &hands {
    //     println!("{:?}", hand);
    // }

    let sum = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| -> i32 {(1 + idx as i32) * (val.bid as i32) })
        .sum::<i32>();
    println!("sum {:?}", sum);

}
