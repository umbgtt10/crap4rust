pub fn empty_function() {}

pub fn single_if(x: bool) -> u32 {
    if x { 1 } else { 0 }
}

pub fn nested_if(x: bool, y: bool) -> u32 {
    if x { if y { 2 } else { 1 } } else { 0 }
}

pub fn match_three_arms(x: u32) -> u32 {
    match x {
        0 => 0,
        1 => 1,
        _ => 2,
    }
}

pub fn for_loop(items: &[u32]) -> u32 {
    let mut sum = 0;
    for item in items {
        sum += item;
    }
    sum
}

pub fn logical_and_or(a: bool, b: bool, c: bool) -> bool {
    a && b || c
}

pub fn try_operator(x: Option<u32>) -> Option<u32> {
    let val = x?;
    Some(val + 1)
}
