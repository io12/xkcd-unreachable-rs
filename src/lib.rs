#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_no_args() {
        xkcd_unreachable!()
    }

    #[test]
    #[should_panic]
    fn test_msg() {
        xkcd_unreachable!("foo == bar")
    }

    #[test]
    #[should_panic]
    fn test_fmt() {
        xkcd_unreachable!("error code: {}", -11)
    }
}

macro_rules! def_unreachable {
    ($dollar:tt, $name:ident, $main_msg:expr) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                panic!($main_msg)
            };
            ($dollar sub_msg:expr) => {
                panic!(concat!($main_msg, $dollar sub_msg, "\n"))
            };
            ($dollar fmt:expr, $dollar ($dollar arg:tt)*) => {
                panic!(concat!($main_msg, $dollar fmt, "\n"), $dollar ($dollar arg)*)
            };
        }
    };
}

def_unreachable!(
    $,
    xkcd_unreachable,
    concat!(
        "\n",
        "⚠ ERROR\n",
        "\n",
        "If you're seeing this, the code is in what\n",
        "I thought was an unreachable state.\n",
        "\n",
        "I could give you advice for what to do.\n",
        "But honestly, why should you trust me?\n",
        " I clearly screwed this up. I'm writing a\n",
        "message that should never appear, yet\n",
        "I know it will probably appear someday.\n",
        "\n",
        "On a deep level, I know I'm not\n",
        "up to this task. I'm so sorry.\n",
        "\n",
        "<https://xkcd.com/2200/>\n",
        "\n",
    )
);
