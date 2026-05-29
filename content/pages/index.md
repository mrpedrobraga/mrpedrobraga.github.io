# Hi!

My name is Pedro, I'm a mid-level software engineer and a big computer nerd. This is my website, where all my cool things reside. Thanks for visiting `ദ്ദി˙∇˙)ว`.

I have **many** projects under my name, so it's likely you'll be interested in some of them. Please, take a look!
### Software
I make tools that enhance productivity/creativity, all free and open-source. If you're a software or game developer, you're bound to find some of these interesting...
```base
filters:
  and:
    - file.inFolder("content/pages/projects")
    - file.hasTag("software")
views:
  - type: cards
    name: Table
    order:
      - title
      - description
      - tags

```
### Video games
You may also recognise me from video game credits... Turns out I've been a small part in a few games over the years (and even have unannounced projects of my own).
```base
filters:
  and:
    - file.inFolder("content/pages/projects")
    - file.hasTag("videogame")
views:
  - type: cards
    name: Table
    order:
      - title
      - description
      - tags
```