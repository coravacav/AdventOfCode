#[macro_export]
macro_rules! simple_benchmark {
    ($name:ident, $input:expr, $iterations:expr) => {{
        let stringified = stringify!($name);
        let now = std::time::Instant::now();
        for _ in 0..$iterations {
            $name(&$input);
        }
        println!("{stringified}: {:?}", now.elapsed() / $iterations);
    }};
    ($name:ident, $input:expr) => {
        simple_benchmark!($name, $input, 100_000);
    };
}

#[macro_export]
macro_rules! assert_eq_same_input {
    ($input:expr, $($name:ident),+) => {
        assert_eq!(
            $(
                $name(&$input)
            ),+
        );
    };
}

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
            incr_num!(val, c);
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
