The basic building block of nano is a dop (data-operation).

That's right, a hyphenated word -- because they are actually one thing. Operation is literally simply data, zeroes and ones interpreted by a machine. But data is also only meaningful when coherent operations are described for it.

nano aims to be an expressive language that shortens the gap between a programmer and their codebase, comprised of data-operations.

## Hello, world!

A nano source file is a conversation between the programmer and the compiler.
The hello world program consists of two phrases.

```nano
# Include the 'print' symbol from the 'standard' library within my binary.
from standard import { print }

# There is an entry function named 'main'
# which calls the 'print' function with 'Hello world!' as parameter.
@entry fn main -> print "Hello, world!"
```

This model of compiler-conversation is very powerful. Consider this:

```nano
# Include the 'print' symbol from 'standard'.
from standard import { print }

# Print 'Hello world!'
print "Hello, world!"
```

This code will print "Hello, world!" onto the console in compilation time! You can see where this is going.
