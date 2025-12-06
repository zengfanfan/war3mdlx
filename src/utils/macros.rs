//#region conditional operator

#[macro_export]
macro_rules! yesno {
    ($cond:expr, $y:expr, $n:expr) => {{ if $cond { $y } else { $n } }};
    ($cond:expr, $y:stmt, $n:stmt) => {{ if $cond { $y } else { $n } }};
    ($var:ident = $expr:expr, $y:expr, $n:expr) => {{
        if let $var = $expr { $y } else { $n }
    }};
    ($var:ident = $expr:expr, $y:stmt, $n:stmt) => {{
        if let $var = $expr { $y } else { $n }
    }};
    ($var:ident = $expr:expr, $cond:expr, $y:stmt, $n:stmt) => {{
        let $var = $expr;
        if $cond { $y } else { $n }
    }};
}

#[macro_export]
macro_rules! yes {
    ($cond:expr, $y:stmt) => {{ if $cond { $y } }};
    ($var:ident = $expr:expr, $cond:expr, $y:stmt) => {{
        let $var = $expr;
        if $cond { $y }
    }};
}

#[macro_export]
macro_rules! no {
    ($cond:expr, $y:stmt) => {{ if !($cond) { $y } }};
    ($var:ident = $expr:expr, $cond:expr, $y:stmt) => {{
        let $var = $expr;
        if !($cond) { $y }
    }};
}

//#endregion
