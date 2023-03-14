# Rust notes

## Things to avoid

- [Blog post on common newbie mistakes](https://adventures.michaelfbryan.com/posts/rust-best-practices/bad-habits/)
- [Thread on the Rust forum which birthed the above post](https://users.rust-lang.org/t/common-newbie-mistakes-or-bad-practices/64821)



## Techniques

Article on [Slice patterns](https://adventures.michaelfbryan.com/posts/daily/slice-patterns/) is useful; can match on elements of a slice kiiiiiinda like Elixir can, though it's a wee bit verbose. Anyway, the `@` symbol binds a new variable to whatever it matches. Therefore can loop and process. The argument parser in the article is probably the most interesting part.

The guy's whole blog has interesting stuff on various ways of approaching problems in Rust: [Michael F Bryan blog](https://adventures.michaelfbryan.com/posts/).


## Errors

Fundamentals of error handling + note on `unwrap`:

- https://blog.burntsushi.net/rust-error-handling/
- https://blog.burntsushi.net/unwrap/

tl/dr: amongst other things, [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) is pretty excellent:

```rust
use anyhow::Result;

fn thing_that_might_blow_up() -> Result<str> {
  Ok("hiya");
}

fn main() -> Result<()> {
  let _ = thing_that_might_blow_up()?;
  // do stuff
  Ok(())
}
```


## Bit stuff

- [Bit manipulation at realtimecollisiondetection.net (bit techniques)](https://realtimecollisiondetection.net/blog/?p=78)
- [Bit twiddling hacks (bit techniques)](http://graphics.stanford.edu/~seander/bithacks.html)
- [Aggregate magic algorithms (bit techniques)](http://aggregate.org/MAGIC/)
- [The bit twiddler (bit techniques)](https://bits.stephan-brumme.com/)
- [Comparing packed bitfields without unpacking each field](https://devblogs.microsoft.com/oldnewthing/20190301-00/?p=101076)
- [Modulus without division](http://homepage.divms.uiowa.edu/~jones/bcd/mod.shtml)
- [Bitboards at chessprogramming.org](https://www.chessprogramming.org/Bitboards)

- [Poker hand analyzer in JS](https://www.codeproject.com/Articles/569271/A-Poker-hand-analyzer-in-JavaScript-using-bit-math) -- note that it uses floats, hence the weird layout of bits.
- [Analysis of the above poker hand analyzer](https://knowles.co.za/analyzing-a-poker-hand-analyzer/)
- [Another overview of the above](https://jonathanhsiao.com/blog/evaluating-poker-hands-with-bit-math)


