Consider this code:

```nano
let url = 'http://mrpedrobraga.com/nano?article=overview'
let response = fetch(url)

if response is NetworkError then (err response)

let article_length = response.data::as_float()

if article_length is ParseError then (err article_length)

print ('Article length is {article_length}')
```
So much error checking ewwww!

Let's make it better >:)

```nano
let url = 'http://mrpedrobraga.com/nano?article=overview'
let article_length: uint!NetworkError!ParseError = fetch(url)!.data::as_float()

if article_length is NetworkError | ParseError then (err article_length)

print ('Article length is {article_length}')
```

Yes!

```nano
let url = 'http://mrpedrobraga.com/nano?article=overview'
let article_length: uint = fetch(url)!.data::as_float() ||> (err _)

print ('Article length is {article_length}')
```
Ugh, yeah, YES!

```nano
with ForwardErrorHandler

let url = 'http://mrpedrobraga.com/nano?article=overview'
let article_length: uint = fetch(url)!.data::as_float()

print ('Article length is {article_length}')
```
YESSSSS no error treating at all!!! If on another file you define this:

```nano
err_handler ForwardErrorHandler (e) -> (
    err e
)
```