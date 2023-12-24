Any sort of build-scripts in nano are done with the use of entry points and compile-time execution.

Consider deploying your code to a remote repository:

```nano
from nnc import { sh }
from standard import { print }

@entry fn main -> print "Hello, world!"

# Create a test that asserts if main works. Note it's another entry point!
@entry fn check -> (
    @assert main() == "Hello, world!"
)

# Run the check and, if it's successful, push it to the remote.
match check() with (
    Ok => sh.exec("git push")
)

```

This is a silly example, yes, but you can see how nano's philosophy allows you to write whatever build routine you see fit.

Instead of installing package managers or more programs, you simply write a bit more nano code.