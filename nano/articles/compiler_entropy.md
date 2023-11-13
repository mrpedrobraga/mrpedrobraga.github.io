## Compiler Entropy

nnc, the nano compiler, offers more than compliation functionality.

It is also a parser, a linter, a language server, a tester, etc. So, yeah, I don't know about you, but that sounds like a lot of bloat if all you wanna do is compile some code.

I have a plan to make all this reasonable, however, by modelling the internal information flow of the compiler in a way that allows all of those things to be possible with nano alone, without having them be builitn.

Let's try to do several operations with this repository:

```nano
# main.nn
from './utils.nn' import { double }
```
```nano
# utils.nn

fn double x -> x * 2
```

### Parsing

Everything begins at parsing. Parsing will load your source file and transform it into a list_view of tokens, then a CST, then IR.

### Indexing

Indexing will walk that CST and keep resolve all the symbols -- it'll keep track of which symbols exist (globally) and which identifiers refer to what symbol.

After this state, you'll be able to do literally anything else.

### Compilation

```nano
import nnc

let index = nnc::index(entry: './main.nano')

fn compile(index: NanoProjectIndex) -> ( 
    let render_target = nnc::new_render_target()

    render_target::render( index.entry_file ) )
)
```

Where a node's repr is supplied by the language definition somewhere within the a library (for primitive functions).

```nano
## Repr for the 'if' statement.

fn IF (rc: RenderContext, node: Expression) ->
(
    from node let {
        fragments[1] as condition,
        fragments[3] as consequence
    }

    if not Type.is_assignable( condition.return_value.type, Symbol(Boolean) ) then (
        rc::error("Condition of if statement must be of type 'boolean'.")
    )

    # Render the 
    let c = rc::render( condition )
    let d = rc::render( consequence )
    
    return + [
        c,
        rc::jump_if_zero( d.end_location ),
        d
    ]
)
```

### Formatting

Formatting is like compiling, but the render target is text. By sharing the same context as the compiler it has the same information.

```nano
## Repr for the 'if' statement.

fn IF (rc: RenderContext, node): string | KEEP ->
(
    if (rc.current_line_width > rc.desired_line_width)
    then ('if {rc::render(node.fragments[1])} then {rc::render(node.fragments[3])}')
    else KEEP
)

## Repr for groupins
fn $"()" (rc: RenderContext, condition: Expression, consequence: Expression): string | KEEP ->
(
    if (rc.current_line_width > rc.desired_line_width)
    then ('({rc::render(condition)} > {rc::render(consequence)})')
    else KEEP
)
```

### Linting

The linter is like a compiler, but it observes the aspects of nodes and adds complaints to a list -- it doesn't render anything.

```nano
import nnc

let index = nnc::index(entry: './main.nano')

fn lint(index: NanoProjectIndex) -> (
    let linter_problems = list<LinterProblem> []

    let rules = [
        {
            description: "Local variables must be snake case"
            
            fn get_bad_fragment(e) -> (
                e.fragments[1]    # let >>helloThere<< = 3
            )
            
            fn accepts(e) -> (
                if e.kind == DECLARATION and e.keyword = LET then e.name matches o/[\w_]+/
            )
                
        }
    ]

    index.entry_file.recursive_map (
        fn node -> (
            for rule in rules do (
                if not rule.accepts(node) then (
                    linter_problems.push( LinterProblem (at: node) )
                )
            )
        )
    )
)
```