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

/// Helper macro to define `xkcd_unreachable!()`. This avoids having to
/// copy-paste the error text for each variant.
///
/// ## Parameters:
///   * `$dollar:tt`
///     * A dollar sign token (`$`). This exists to allow higher-order
///       macro definition.
///       <https://twitter.com/comex/status/979949654610604032>
///   * `$main_msg:expr`
///     * The text of the xkcd comic ("If you're seeing this, ...")
macro_rules! def_xkcd_unreachable {
    ($dollar:tt, $main_msg:expr) => {
        #[macro_export]
        /// Macro indicating unreachable code. This is equivalent to
        /// [`unreachable!()`](unreachable), except that it displays the message
        /// in [xkcd 2200](https://xkcd.com/2200/).
        ///
        /// ## Example
        /// ```rust
	/// use xkcd_unreachable::xkcd_unreachable;
	///
        /// fn foo(x: Option<i32>) {
        ///     match x {
        ///         Some(n) if n >= 0 => println!("Some(Non-negative)"),
        ///         Some(n) if n <  0 => println!("Some(Negative)"),
        ///         Some(_)           => xkcd_unreachable!(),
        ///         None              => println!("None")
        ///     }
        /// }
        /// ```
        ///
        /// ## Output if reached
        /// ```text
        /// ⚠ ERROR
        ///
        /// If you're seeing this, the code is in what
        /// I thought was an unreachable state.
        ///
        /// I could give you advice for what to do.
        /// But honestly, why should you trust me?
        /// I clearly screwed this up. I'm writing a
        /// message that should never appear, yet
        /// I know it will probably appear someday.
        ///
        /// On a deep level, I know I'm not
        /// up to this task. I'm so sorry.
        ///
        /// <https://xkcd.com/2200/>
        /// ```
        macro_rules! xkcd_unreachable {
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

def_xkcd_unreachable!(
    $,
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
