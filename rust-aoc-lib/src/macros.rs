#[macro_export]
macro_rules! read_till {
    ($iter:expr, $delimiter:expr) => {
        while !matches!($iter.next(), Some($delimiter)) {}
    };
}

#[macro_export]
macro_rules! read_number {
    ($iter:expr) => {{
        let mut val = 0;

        while let Some(&c @ b'0'..=b'9') = $iter.next() {
            rust_aoc_lib::incr_num!(val, c);
        }

        val
    }};

    ($val:expr, $iter:expr) => {{
        while let Some(&c @ b'0'..=b'9') = $iter.next() {
            incr_num!($val, c);
        }

        $val
    }};
}

#[macro_export]
macro_rules! incr_num {
    ($val:expr, $c:expr) => {
        $val = $val * 10 + ($c - b'0') as usize;
    };
}
