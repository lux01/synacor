//! Convenience macros

#[macro_export]
macro_rules! send {
    ($tx:ident, $str:expr) => {{
        $tx.send(format!("{}", $str)).unwrap();
    }};
    ($tx:ident, $($args:tt)*) => {{
        $tx.send(format!($($args)*)).unwrap();
    }}
}

#[macro_export]
macro_rules! recv_line {
    ($rx:ident) => {{
        let mut buf = String::new();
        while let Ok(c) = $rx.recv() {
            if c == '\n' {
                break;
            }
        }
    }}
}
