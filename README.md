# Iterify ![Downloads](https://img.shields.io/crates/v/iterify)

Iterate over anything with Iterify! This crates takes any type `T` and lets you iterate over a mutable reference, yielding any type you want.

It has similar behaviour to [`std::iter::successors`](https://doc.rust-lang.org/std/iter/fn.successors.html) except that Iterify lets you mutate the type in-place through a `&mut` reference.

#### Example

```rust
use iterify::Iterify;

fn main() {
    let mut num = 12;
    let collatz: Vec<_> = num
        .iterify(|num| {
            *num = match *num {
                0 | 1 => return None,
                n if n % 2 != 0 => n * 3 + 1,
                n => n / 2,
            };

            Some(*num)
        })
        .collect();

    assert_eq!(collatz, &[6, 3, 10, 5, 16, 8, 4, 2, 1]);
    assert_eq!(num, 1);
}
```
