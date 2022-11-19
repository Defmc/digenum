## An simple enum variant extractor
Can extract a (mutable) reference or turn into a tuple from fields. When variant is different than the expected, return `None`.

## Example
```rust
use digenum::DigEnum;
use std::io::Read;

#[derive(DigEnum)]
enum Login<T: Read> {
    Local,
    Stream(T)
    Token([u8; 64]),
    UserAndPasswd(String, String),
}
```

Maps to:
| Variant | Reference | Mutable Reference | Owned |
| ------- | --------- | ----------------- | ----- |
| `Local` | None | None | None |
| `Stream(T)` | `as_Stream -> Option<&T>` | `as_mut_Stream -> Option<&mut T>` | `into_Stream -> Option<T>` |
| `Token([u8; 64])` | `as_Token -> Option<&[u8; 64]>` | `as_mut_Token -> Option<&mut [u8; 64]>` | `into_Token -> Option<[u8; 64]>` |
| `UserAndPasswd(String, String)` | `as_UserAndPasswd -> Option<(&String, &String)>` | `as_mut_UserAndPasswd -> Option<(&mut String, &mut String)>` | `into_UserAndPasswd -> Option<(String, String)>` |

## "Why do not turn `snake_case`?"
Consider this example:
```rust
use digenum::DigEnum;

#[derive(DigEnum)]
enum Foo {
    FOO,
    foo,
    Foo
}
```
How should I handle it?

## Works on
- [x] Empty enums
- [x] Empty variants
- [x] Single-fielded variants
- [x] Multiple-fielded variants
- [x] Different-case variants (like `Foo::FOO` and `Foo::foo`)
- [x] Generic variants
- [x] Unit variants
- [ ] Struct-like variants
