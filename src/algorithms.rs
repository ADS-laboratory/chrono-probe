pub struct Algorithm {
    pub name: &'static str,
    pub function: fn(&[u8]) -> usize,
}

pub const PERIOD_NAIVE1: Algorithm = Algorithm {
    name: "period naive1",
    function: period_naive1,
};

pub const PERIOD_NAIVE2: Algorithm = Algorithm {
    name: "period naive2",
    function: period_naive2,
};

pub const PERIOD_SMART: Algorithm = Algorithm {
    name: "period smart",
    function: period_smart,
};

fn period_naive1(s: &[u8]) -> usize {
    let n = s.len();

    'outer: for i in 1..n {
        for j in 0..n - i {
            if s[j] != s[j + i] {
                continue 'outer;
            }
        }
        return i;
    }
    n
}

fn period_naive2(s: &[u8]) -> usize {
    let n = s.len();
    for i in 1..n {
        if s[..n - i] == s[i..] {
            return i;
        }
    }
    n
}

fn period_smart(s: &[u8]) -> usize {
    let size = s.len();

    // b[i] represents the maximum edge length of s[0..i]
    let mut b = vec![0; size];

    // current maximum edge length
    let mut x;

    // update b[i] as a function of b[0..i-1]
    for i in 1..size {
        // update x with the length of the maximum edge of s[0..i-1]
        x = b[i - 1];
        // if the new character (of the suffix) is not equal to the character
        // following the prefix then the next candidate for the maximum edge is
        // the maximum edge of the prefix
        while s[x] != s[i] && x > 0 {
            x = b[x - 1];
            //print b
        }

        // if they are equal then the length of the maximum edge is increased
        if s[x] == s[i] {
            x += 1;
        }

        // update b[i]
        b[i] = x;
    }
    // the maximum border of the entire string
    let max_border = b[size - 1];
    // The minimum fractional period is the length of the string minus the maximum border
    size - max_border
}