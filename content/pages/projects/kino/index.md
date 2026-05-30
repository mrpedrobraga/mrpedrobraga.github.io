---
title: kino (0.1.0)
description: Markup language for writing complex, rich scripts for movies, tv shows or interactive media.
tags:
  - software
  - compiler
  - dsl
  - writing
status: To Do
dependencies:
  - proj
---
<center><a href="https://github.com/mrpedrobraga/obsidian-kino"><img src="https://img.shields.io/badge/github-repo-blue?logo=github"></a></center>

Kino is a language for writing movie and videogame scripts,
built for the digital era.

With `kino`, you can write scripts in a rich syntax
that can be understood by humans and computers,
thus supporting database-like introspection features.
It's highly inspired by `fountain`, `markdown` and `sol`.

You can edit your scripts in a "Live-Preview"-like environment,
authoring beautiful and semantics-rich document.

`kino` is still a very early WIP, but it can already do a lot.
## Syntax & Features
Each file starts with a metadata section. This gives the file more information that can be used by the visualisation tools...
```kino
Title: MY MOVIE SCRIPT
Subtitle: The best ever
Author: Pedro Braga
```

After that, time to write your scenes. Scenes are sections of a file demarked by two pairs of equal signs (`====`).

```kino
====
```

You may give scenes names...

```kino
== Scene 01 ==
```

As well as metadata...

```kino
== Scene 01 ==
Where: [Hiere Forest]
When: 12:00
Music: [OST Hiere Forest]
```

The properties here are truly arbitrary, you can add whatever you want, but visualisation tools might be expecting certain kinds of properties. My [Obsidian Plugin](https://github.com/mrpedrobraga/obsidian-kino/) can understand these properties:

- `Where`;
- `When`;
- `Characters`;
- `Tags`;
- `Board`;
- `Author`;

> Where, When, Music, Characters, Tags, Board, Author

Inside a scene (after a scene header) you can write action lines
simply by typing your paragraph.

```kino
The early sun rises in the distance. Birds chirp quietly.
```

You can write additional notes preceding your paragraph by //.
These paragraphs will be ignored by certain processes.

```kino
The early sun rises in the distance. Birds chirp quietly.
// Make sure to draw the sun rising from the east.
```

For style and emphasis, you can use formatting.

```kino
The fog looks **ominous** but not _too_ scary, moreso lonely.
```

With square brackets you can add rich references, also known
as wikilinks, which are references that can be followed through
or hovered over in editing software. You can refer to "things"
(to be explained later) by their name, id, path;
or to files or websites.

```kino
At the bed, [Maple] and [Casper] are sleeping.
```

Rich references are REALLY powerful.
You can use them to be clear about characters, locations,
items, other scripts (and specific scenes within them)
and other concepts that might be ambiguous. Software
that uses kino is aware of what files connect to each other
and can help with navigation or make all sorts of visualisations.

You can write dialogues by declaring a new current speaker,
and following it with dialogue lines.

A lone reference to a character in a line declares a new speaker.
After that, you may use dashes (-) to add lines or parentheticals.

```kino
[Soren]
- (yawns)
- What a beautiful day today...
- (looking at Akiko)
- When do you wanna get up?

Akiko rolls to look at his eyes, with a sleepy smile.

[Akiko]
- How dare you ask me that
```

You can add tagged directives using `@`.
They are pretty like action lines,
but they contain a tag that they can be filtered by.

```kino
@TRANS: FADE OUT.
@CHYRON: November 17.
@CAMERA: Orbit slowly around subject.
@SFX: Loud train horn.
@TODO: Write this scene better.
```

#### Some Nice Features

- You can use wikilinks everywhere in the text.
- Dialogue speakers are implicitly Wikilinks!
    - You can use `[Characters/FileName|My Character]` to link to a file while having a different display name. Wikilink syntax is same as anywhere else in Obsidian.
- Using wikilinks in @BGM directives makes a clickable link... Clicking that link will play the BGM in a bottom panel in the writing view, so you can get in the mood, ooo!

### Upcoming ###

> [!WARNING]
> 
> This plugin is still in its early version, these features haven't been formalised yet.

#### Visualisations

I'm still wondering how to implement this in obsidian,
but the idea is to have something that feels a lot like Bases
for your scenes.

- [ ] Plotlines;
    - [ ] Map;
    - [ ] Matrix;
- [ ] **Hierarchical Outline with Search and Filter**. Some of these are actually implemented but commented out, as I'll split some of the convoluted functionality into a new plugin.
    - [ ] Filter by content, plotline or arbitrary metadata;
    - [ ] Sort by reading order or chronological order;
        - [ ] Customizable/fictional calendar;
    - [ ] State tracking: asking "what is X at this point in the story?";
    - [ ] Seeing all the SFX, BGM, TRANS, portraits, animations, locations, etc that need to be produced in one single place.

#### Interactivity

Lastly, you can write some special directives using
the at sign (%).

Usually scenes are considered to transition to the next
scene written below them. But in an interactive medium,
a scene might lead to many other scenes.

You can specify that with the `CONT` directive.

```kino
Eventually, the two of them get up.

// Look yourself in the mirror.
%CONT Scene 02

// Getting out of the door.
%CONT Scene 03
```

Or `IF` to add quick variations.

```kino
%IF the player opens the chest {
    A ball of light falls from the sky,
    bestowing you with an item.
}
```
