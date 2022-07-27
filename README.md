# lazytry

`lazytry` aims for lazy data evaluation which may fail.

```rust
use lazytry::unsync::LazyTry;

let lazy: LazyTry<i32, _> = LazyTry::new(|| "1".parse());

assert_eq!(lazy.force().unwrap(), &1);
assert_eq!(lazy.force().unwrap(), &1);
```

```rust
use lazytry::unsync::LazyTryFn;
use std::num::{IntErrorKind, ParseIntError};

let lazy: LazyTryFn<i32, ParseIntError> = LazyTry::new(|| "a".parse());

assert_eq!(
    *lazy.force().unwrap_err().into_err().unwrap().kind(),
    IntErrorKind::InvalidDigit
);
```

The current code is quite experimental, including unproved use of unsafe (impl Sync), lack of documentation and a weird and incompleted api.