//#region conditional operator

#[macro_export]
macro_rules! yesno {
    ($cond:expr, $y:expr, $n:expr) => {{ if $cond { $y } else { $n } }};
}

#[macro_export]
macro_rules! yes {
    ($cond:expr, $y:stmt) => {{
        if $cond {
            $y
        }
    }};
}

#[macro_export]
macro_rules! no {
    ($cond:expr, $n:stmt) => {{
        if !($cond) {
            $n
        }
    }};
}

//#endregion
