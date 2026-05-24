# Writing music down!

Partitune is a syntax for rigorously writing down music in text files.

```partitune
Title: "My Song"
Composer: "Pedro Braga"

TREBLE C 4/4:
||:
2/4 +f T"Swing"  C4, E4,  G4;
1/4             ..., D4, ...;
1/8              C4, F4,  A4;
1/8             ..., G4,  B4;
|
2/4              C4, E4,  G4;
1/4 +tr         ..., F4, ...;
1/4              C4, D4,  A4;
:||

BASS C 4/4:
||:
2/4 +f C3;
2/4  --- ; 
|
4/4    C3
:||
```

## Parser

Here is the entirety of the parser's grammar.

```pest
DOCUMENT = { SONG_INFO* ~ STAVES }
SONG_INFO = { ("Title" | "Subtitle" | "Composer") ~ ":" ~ string }
STAVES = { STAFF+ }

STAFF = { STAFF_HEADER ~ ":" ~ BARS ~ END_STAFF }
STAFF_HEADER = { CLEF ~ FULL_NOTE ~ BEAT_LENGTH }
FULL_NOTE = @{ NOTE_LETTER ~ ACCIDENTAL? }
NOTE_LETTER = { "A" | "B" | "C" | "D" | "E" | "F" | "G" }
ACCIDENTAL = { "##" | "bb" | "#" | "b" }
CLEF = { "TREBLE" | "BASS" }
BARS = { MEASURE_SEPARATOR? ~ BAR ~ (MEASURE_SEPARATOR ~ BAR)* }

MEASURE_SEPARATOR = @{ "||:" | ":||" | ":|||:" | "|" }
END_STAFF = @{ "||" | ":||" }

BAR = { FRAGMENT ~ (";" ~ FRAGMENT)* ~ ";"? }
FRAGMENT = { BEAT_LENGTH ~ (FRAGMENT_MODIFIER | TEXT)* ~ VOICE_FRAGMENT ~ ("," ~ VOICE_FRAGMENT)* ~ ","? }
FRAGMENT_MODIFIER = { MODIFIER_MODE ~ FRAGMENT_MODIFIER_NAME }
FRAGMENT_MODIFIER_NAME = { "tr" | "f" | "ff" | "fff" | "p" | "pp" | "ppp" }
VOICE_FRAGMENT = { ((PRECISE_NOTE | REST) ~ VOICE_FRAGMENT_MODIFIER* | CONTINUE) }
PRECISE_NOTE = @{ NOTE_LETTER ~ NUMBER }
VOICE_FRAGMENT_MODIFIER = { MODIFIER_MODE ~ VOICE_FRAGMENT_MODIFIER_NAME }
VOICE_FRAGMENT_MODIFIER_NAME = { ACCIDENTAL | "tie" | "." }
BEAT_LENGTH = { NUMBER ~ "/" ~ NUMBER }
REST = { "---" }
CONTINUE = { "..." }

MODIFIER_MODE = { "+" | "-" | "&" }

TEXT = @{ ("Ly" | "Ch" | "Sys" | "T") ~ string }

string = ${ "\"" ~ inner ~ "\"" }
inner = @{ char* }
char = {
    !("\"" | "\\") ~ ANY
    | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
}

NUMBER = @{ ASCII_DIGIT+ }
WHITESPACE = _{ " " | NEWLINE }
```