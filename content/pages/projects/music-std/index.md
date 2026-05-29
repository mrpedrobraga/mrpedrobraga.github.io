---
title: The MUS Standards
description: A collection of standards for interchange of semantic music data.
tags:
  - software
  - library
  - music
dependencies:
status: To Do
---
The two most important formats in this project are MUSi (Musical Instrument) and MUSx (Musical Excerpt), both which go hand in hand.

This library was developed for [Overtone](/project/overtone) but may be used for any purposes!

### MUSi (Musical Instrument)
A simple sample library, similar to SoundFont, that can generate audio from music
using a format for arbitrary semantic music exchange (MUSx).

Each instruments carries:
- Samples, each annotated with parameters;
- A Sampling Scheme, which describes how to fulfil the request to play some note;

### The MUSx (Musical Excerpt) format
An excerpt of musical data that will be played by a MUSi (Musical Instrument) or another MUS-compliant plugin.

This format contains a bunch of notes in a sorted buffer, such that they can be seeked efficiently using binary search.

Each note has eight inherent parameters:
- `start`;
  - the time offset for this note from the beginning of the song;
- `duration`;
  - how long the note will last for;
- `voice`;
  - the voice that plays this note, for instruments with multiple voices;
- `intensity`;
  - the intensity with which the note is played (a.k.a. velocity);
- `pitch`;
  - the pitch of the note;
- `kind`;
  - the "kind" of the note, for instruments with many emitters (like percussion) or articulations (like strings);
- `sample_variant`;
  - the sample variant, which can be used to add variation to a song;
- `extra`;
  - an extra parameter that you can use for whatever;

It additionally contains "notation" — time-bound and note-bound markers, each which add additional information to be interpreted by the MUSi. These markers are also arbitrary. They may look something like this:

`std::staccato`, `std::chord_stagger`, `std::artificial_harmonics`.

#### Playback
When you play a song in Overtone, the play head scans the notes from left to right and emits note events in real time. This is how MUSi plays excerpts, and also how it works if you're recording an excerpt using a MIDI controller.

But the instruments don't need to react "in real time" to events, since they do get a view into the currently-playing excerpt and are able to _look ahead into the future_.

This might be useful for certain instruments — the ability to generate sounds for blocks of music instead of in a  note-by-note basis.

### Standard Markers
For the benefit of the community, here is a relation of standard markers your instruments should follow. This doesn't mean every instrument should do something for _every_ marker, but if you're making an instrument that provides any of these functionalities, you should consider using one of these names to tag it.

This is the standard used by Overtone plugins made by me!

>>>
#### A short aside on naming convention
The names included were chosen to be as general and applicable as possible, while still remaining understandable. Unlike a program like _MuseScore Studio_, which has a focus on the aesthetics of notation (and thus has many redundant ways of notating the same concept), we prioritise high interchangeability instead of specificity.

Then, it's up to the GUI that's editing a `.musx` file to render the notation appropriately according to the user choice and the current instrument. For example, a classical composer might prefer to read `pitch_slide` as "**Portamento**", while an EDM composer might prefer "**Slide**".

Lastly, it's convention to "namespace" strings to the standard that defined them, such that you know exactly what piece of technical information someone is refering to. So, if a marker here is called `vibrato` it'll be referred to as `std:vibrato`, as opposed to some third party's marker that would be called `thirdparty:vibrato` and could potentially work differently;
>>>

### Note markers

Markers that modify or add effects to a single note or a set of notes.

> [!TIP]
> All markers that affect "a single note" can be applied to multiple notes at once, but some markers only make sense when applied to several notes, like legato.

#### Articulations
- `accent` for heighted accent on an note;
    - **(a.k.a. Marcato)**;
- `legato` for notes that are played together;
    - **(a.k.a. Legato)**;
- `staccato` for notes that are not held for their full time;
    - **(a.k.a. Staccato)**; 
- `vibrato` for quickly sliding a note up and down;
    - **(a.k.a. Vibrato)**;
- `tremolo` for quickly sliding a note's volume up and down;
    - **(a.k.a. Tremolo)**;
- `harmonics` for natural and artificial harmonics;
    - **(a.k.a. harmonics)**;
- `method` for changing the tool and overall method used to emit sound;
    - Has a single `String` parameter, which can be:
        - `"default"`, for the default method of the MUSi;
        - `"plucking"`, useful for string instruments;
            - **(Plucked, pizz.)**;
        - `"pick"`, useful for the guitar family;
            - **(Picked)**;
        - `"bow"`, useful for the violin family;
            - **(Arco)**;
        - `"soft_mallet"`, useful for percussion;
        - `"hard_mallet"`, useful for percussion;
        - Or something else;

#### Pitch
- `pitch_shift` for shifting the pitch of a selected section by some amount;
    - **(a.k.a. ♯, ♭, 8va. Alta & Bassa, etc.)**;
- `pitch_slide ( Mode )` for gradually shifting pitch between two selected notes;
    - **(Glissando)**;
    - `continuous` **(portamento)**;
    - `stepped` **(glissando, chromatic)**;
    - `diatonic` **(glissando, in key)**;
    - `discrete` **(hammer on & pull off)**;
- `sustain` for allowing notes to vibrate after they are released;
    - **(a.k.a. Ped., let ring, laissez vibrez)**

#### Dynamics
- `intensity` for setting the intensity with which a note is triggered.

#### Ornaments
- `chord_derive` for deriving a chord from a single note. Useful for sketching harmony non-destructively;
- `chord_stagger` for arpeggiating/strumming a chord;
    - **(a.k.a. Arpeggio, Strum)**;
- `trill` for quickly arpeggiating a note up and down;
    - **(a.k.a. Trill)**;
- `lyrics` for adding lyrics to a certain note;

### Voice markers

Markers that affect the song itself, at least for a specific voice.

- `time_resubdivide ( Int )` for subdividing a stretch of time in a new amount of parts;
    - **(a.k.a. tuplets)**;

### Timeline markers

Markers that are affect or are bound to the timeline.

These markers _are_ seen by MUSI during playback, but most instruments aren't supposed to react to the tempo anyways. Instead, these are to be interpreted and played back by the MUSI's host.

#### Location
- `marker` a simple marker that simply marks a place or range in the song.

#### Song pace
- `tempo_define`, for defining a new pace for the song, in units (beats) per second;
- `tempo_slide` for accelerating or decelerating the tempo;
    - **(accel., rit., etc.)**;
- `tempo_divide`, for setting a new scheme for dividing the flowing tempo (what's called a 'time signature');
- `tempo_feel`, for defining a new tempo style;
    - Has a single parameter, which can be:
        - `"straight"` for **Straight** tempo;
        - `"swing"` for **Swing** tempo;
        - Or something else;
- `repeat` for repeating an area of the song some amount of times;
- `conditional_jump` for jumping to some specific marker given an input condition. This one is advanced and might be used for repeats or dynamic, interactive music;