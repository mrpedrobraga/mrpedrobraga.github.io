## Introduction

From this tutorial, even a begginer to programming will be able to read and write nano code.

However, this resource walks at a fast pace with direct and concise language, so take as much time as you want to make sure a section really sinks in.

By the end of it, you'll completely understand this code:

```nano
# sum.nn
from core import { args }

if args.size < 2 then err "Not enough arguments supplied!"

with err_handler (p: ParseError) -> (
	err 'Parameter {p.string_content} is invalid.'
)

let result: float =
	for acc=0, arg across args select
		acc + float::from_string(arg)
```

But since nano IS pretty clear, you probably already make out a bit of it.

Anyways, go check [Primitives](./primitives) already!
