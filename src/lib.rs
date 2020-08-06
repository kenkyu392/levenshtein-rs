/// Computing the Levenshtein distance between strings.
///
/// # Examples
///
/// ```
/// let cat = "I Am a Cat";
/// let dog = "I Am a Dog";
///
/// assert_eq!(3, levenshtein::compute(cat, dog));
/// ```
pub fn compute(a: &str, b: &str) -> usize {
    let acnt = a.chars().count();
    let bcnt = b.chars().count();
    let mut _next: usize;
    let mut _prev: usize;
    let mut c = (0..=acnt).collect::<Vec<usize>>();
    for x in 1..=bcnt {
        c[0] = x;
        _next = x - 1;
        for y in 1..=acnt {
            _prev = c[y];
            let cost = if a.chars().nth(y - 1) != b.chars().nth(x - 1) {
                1
            } else {
                0
            };
            c[y] = min(c[y] + 1, c[y - 1] + 1, _next + cost);
            _next = _prev
        }
    }
    return c[acnt];
}

fn min(a: usize, b: usize, c: usize) -> usize {
    if a < b {
        if a < c {
            return a;
        }
    } else {
        if b < c {
            return b;
        }
    }
    return c;
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn it_compute() {
        let cat = "I Am a Cat";
        let dog = "I Am a Dog";
        assert_eq!(3, compute(cat, dog));
    }
}
