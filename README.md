[![](https://img.shields.io/crates/v/xkcd_unreachable)](https://crates.io/crates/xkcd_unreachable)
[![](https://docs.rs/xkcd_unreachable/badge.svg)](https://docs.rs/xkcd_unreachable)

# `xkcd_unreachable`

A Rust macro `xkcd_unreachable!()` inspired by [xkcd 2200](https://xkcd.com/2200/)

## Example
```rust
use xkcd_unreachable::xkcd_unreachable;

fn foo(x: Option<i32>) {
    match x {
        Some(n) if n >= 0 => println!("Some(Non-negative)"),
        Some(n) if n <  0 => println!("Some(Negative)"),
        Some(_)           => xkcd_unreachable!(),
        None              => println!("None")
    }
}
```

## Output if reached
```text
⚠ ERROR

If you're seeing this, the code is in what
I thought was an unreachable state.

I could give you advice for what to do.
But honestly, why should you trust me?
I clearly screwed this up. I'm writing a
message that should never appear, yet
I know it will probably appear someday.

On a deep level, I know I'm not
up to this task. I'm so sorry.

<https://xkcd.com/2200/>
```

For more information, see the [documentation](https://docs.rs/xkcd_unreachable/).
