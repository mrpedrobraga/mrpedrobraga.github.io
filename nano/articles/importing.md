## Importing

If you have many files on disk, you can use other files as libraries/modules/whatever you wanna call them.

```nano
# my_library.nn
export (value, value2)

let value = 1
let value2 = 10

```

```nano
# main.nn
import 'my_library.nn' as MyLib
#                          ^ the imported file will be a struct

print MyLib.value
print MyLib.value2
```

You might want to import only a few values from the library as well as spill them into this file's global scope.

You can do that using that _destructuring_ syntax.

```nano
# main.nn
from MyLib import { value }

print value
```

You can also import symbols and values from libraries you have linked. `nnc`, the nano compiler, only comes with three libraries you can refer to by default:

```nano
# Core library, where you have the very basic
# building blocks needed to make a program.
from 'core' import { args, ... }

# Standard library, where there are some useful
# helpers to make writing programs easier and saner.
from 'standard' import { KeyNotFoundError, ... }

# Compiler library, which provides powerful meta-programming
# capabilities, allowing you to create your own build system
# and whatnot.
from 'nnc' import { NanoExpressionIRNode, ... }
```

But you can download and import new libraries to use pre-built functionality someone else made.
For example, assume a theoretical 'geometry' library:

```nano
from 'geometry.dll' import { Circle }

let a: Circle = Circle (10)
```

You can also export your own code as a library.

Wait, did I say 'download'?

```nano
from 'http://mrpedrobraga.com/lib/Geometry' import { Circle }
#      ^ nano will fetch this link once then
#        cache the content so it can compile.
#        If you keep this cache, even if the remote changes
#        or goes offline you can still work.

let a: Circle = Circle (10)
```

Needless to say, depending on remote library is unsafe and unreliable, so make sure you trust the person serving the library.

Still, after your code is compiled once, it doesn't depend on the library anymore.

You may also prefer to only import libraries from platforms you trust... such as 'github' or 'npm', but, if you can, though, nothing beats 'downloading a library' from the internet, or serving it yourself in a network you own instead of depending on some stranger on the web. You shouldn't need to trust even popular package managers such as 'npm'.

```nano
import 'npm'
import 'git'
from 'npm://Geometry' import { Circle }
from 'git://mrpedrobraga/culinary' import { Beef, Salt }
#      ^ nano will fetch this link using a protocol
#        offered by the 'git' library you just imported.

let a: Circle = Circle (10)
```

Imported libraries can create new importing protocols allowing you to import file formats, or import stuff over a network. You can create your own protocols, too (we'll see that in [Importing II](./?article=importing2)).

Anything can be 'imported', not just nano code. Whatever is imported is available as a nano value.

```nano
# Use data formats as if they were natively supported.
import 'json'
import 'my_file.json'

# Refer to code written in other languages???
import 'rust-interop'
import 'my_file.rs'

# Even images aren't off-limits!
import 'an-image-lib'
import 'my_cool_image.png'
```

The sky truly is the limit when it comes to importing.
Imagine doing this with an API:

```nano
from 'https://weather.api' import { get_weather, WeatherInfo, APIError }

let a: WeatherInfo!APIError = await get_weather()
#                               ^ ooo, mysterious 'await' keyword?
#                               It is for doing operations that
#                               take time, such as operations over
#                               networks. Don't worry about it yet.
```

### entry.nano

Now, writing 'https://some_link.a.com.nl.otherthing/path/otherpath' every time you want to import a library in one of your thousands of files sounds like hell.

What if the link changes? Is there truly no escape from this?

As you can guess from the fact that I brought this concern up at all, there is.

A recommended organizational practice is that you actually start compilation from a file that sets up aliases for your project, does whatever things your build needs and THEN calls your main function to execute your code.

```nano
# entry.nano
global import 'rust'
global alias WeatherAPI = 'https://weather.api?version=0.1'

import { main } from 'main.nn'

main()
```

```nano
# main.nn
from WeatherAPI import { get_weather }
from 'rust_file.rs' import { something }

fn main -> (
	print "Hello!"
	let b = get_weather()
	print something
)
```
