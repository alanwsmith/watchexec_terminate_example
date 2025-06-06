# watchexec_terminate_example

## Overview

This is a demonstration repo that shows
a use case where a terminal hangs and you 
can no longer CTRL+c it if you check for:

```rust
Signal::Interrupt
```

Things work as expected if you check for
that and:

```rust
Signal::Terminate
```


## Running The Demo

Run the `start-cargo-watch` script in the
root of the repo to see the behavior. 
(Note that if it works the way it does
on my machine you'll have to use 
something like `ps -A` to find the 
process and kill it)

## Seeing The Fix

To see it working as expected, uncomment the
block with:

```rust
.any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
```

## Suggestion

Update the docs to change the example on 
[the home page](https://docs.rs/watchexec/latest/watchexec/)
from:


```rust
if action.signals().any(|sig| sig == Signal::Interrupt) {
    action.quit();
}
```

To:

```rust
if action.signals()
    .any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
{
    action.quit();
}
```



