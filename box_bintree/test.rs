macro_rules! macursive {
    () => {
        0
    };

    (C $val:expr) => {
        $val
    };

    (M ($val1:expr, $val2:expr)) => {
        $val1 * $val2
    };

    (M ($val1:expr, $val2:expr), $( $tail:tt )* ) => {
        $val1 * $val2 + macursive!($($tail)*)
    };

    (C $val:expr, $( $tail:tt )* ) => {
        $val + macursive!($($tail)*)
    };

    // (($val1:expr, $val2:expr), $( $tail:tt )* ) => {
    //     ($val1 * $val2) + macursive!($($tail)*)
    // };
}

fn main() {
    let val: u32 = macursive!(C 2, M (5, 2), C 2, M (5, 2), C 10);

    let tree = build_tree!{
        3, L(1, 2), R(5, 7)
    };

    let tree2 = build_tree!{
        2;
        L {
            3; 
            L {
                1
            }; R {
                4
            }
        };
        R { 
            6; L {5}; R {7} 
        }
    };

    println!("Value is {}", val);
}