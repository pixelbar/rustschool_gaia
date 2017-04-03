macro_rules! try_get {
    ($e:expr) => {
        match $e {
            Some(val) => val,
            None => return None
        }
    };
    ($e:expr, $msg:expr) => {
        match $e {
            Some(val) => val,
            None => {
                println!("Could not get value {:?}", $msg);
                return None
            }
        }
    }
}

macro_rules! try_get_nonempty_string {
    ($e:expr, $name:expr) => {{
        let val = $e;
        match val {
            Some(val) if val.len() > 0 => val,
            _ => {
                println!("{:?} is {:?}", $name, val);
                return None;
            }
        }
    }}
}

macro_rules! get_optional_string {
    ($e:expr, $name:expr) => {{
        let val = $e;
        match val {
            Some(val) if val.len() > 0 => Some(val),
            _ => None
        }
    }}
}

macro_rules! try_get_bool {
    ($e:expr, $name:expr) => {{
        let val = $e;
        match val {
            Some("true") => true,
            Some("false") => false,
            x => {
                println!("Could not parse {:?} ({:?}) into an bool: {:?}", $name, val, x);
                return None;
            }
        }
    }}
}

macro_rules! make_parser_macros {
    ($tryparse_name:ident, $macro_type:ty) => {
        macro_rules! $tryparse_name {
            ($e:expr, $name:expr) => {{
                let val = $e;
                match val.map(|v| v.parse::<$macro_type>()) {
                    Some(Ok(val)) => val,
                    x => {
                        println!("Could not parse {:?} ({:?}) into an {:?}: {:?}", $name, val, stringify!($macro_type), x);
                        return None;
                    }
                }
            }}
        }
    };
    ($tryparse_name:ident, $optionparse_name:ident, $macro_type:ty) => {
        make_parser_macros!($tryparse_name, $macro_type);
        macro_rules! $optionparse_name {
            ($e:expr, $name:expr) => {{
                let val = $e;
                match val.map(|v| v.parse::<$macro_type>()) {
                    Some(Ok(val)) => Some(val),
                    _ => None
                }
            }}
        }
    }
}

make_parser_macros!(try_get_i16, i16);
make_parser_macros!(try_get_i32, get_optional_i32, i32);

make_parser_macros!(try_get_u16, u16);
make_parser_macros!(try_get_u32, get_optional_u32, u32);
make_parser_macros!(try_get_u64, u64);

make_parser_macros!(try_get_f32, get_optional_f32, f32);
make_parser_macros!(try_get_f64, f64);
