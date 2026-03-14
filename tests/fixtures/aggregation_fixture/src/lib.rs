pub fn aggregation_target(
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    i: bool,
    j: bool,
) -> u32 {
    let mut score = 0;
    if a {
        score += 1;
    }
    if b {
        score += 1;
    }
    if c {
        score += 1;
    }
    if d {
        score += 1;
    }
    if e {
        score += 1;
    }
    if f {
        score += 1;
    }
    if g {
        score += 1;
    }
    if h {
        score += 1;
    }
    if i {
        score += 1;
    }
    if j {
        score += 1;
    }
    score
}
