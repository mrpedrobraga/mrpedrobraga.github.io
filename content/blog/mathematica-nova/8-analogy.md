---
date: 2025-01-01
tags:
  - logic
  - algebra
title: Mathematica Nova - Introduction
description: ...
---

The last piece of the puzzle to understanding why all this is useful is the concept of analogy.

Analogy consists of using abstract structures (like bindings, values, expressions) to represent things we might care about in the real world, or vice-versa.

It is possible to go from a real world structure to an abstract structure, do transformations on it and go back to the real world. This process is called "reasoning."

The opposite is also possible, to go from an abstract structure to the real world, undergo a physical process, then come back to the abstract world. This process is call "computation."

> [!info]
> It is also possible to go from abstract concepts to other abstract concepts (morphisms) but let us not consider that yet!
#### Logical Reasoning
One very peculiar way of reasoning is by using algebra and variables to represent facts that a person may propose in an argument with someone else.

The analogy goes that a variable represents a proposition.

| Variables | Propositions          |
| --------- | --------------------- |
| $a$       | It is raining outside |
| $b$       | The ground is wet     |
And operators represent ways to combine propositions.

> [!TODO] Edit
> Instead of a table here, I should introduce the analogies one by one, explaining why they feel equivalent....

| Operators    | Phrase Connectors |
| ------------ | ----------------- |
| $\lnot$       | not               |
| $\land$      | and / but         |
| $\lor$       | or                |
| $\implies$   | if-then / implies |
| $\impliedby$ | because           |

And the variables' _values_ represents the _truthiness_ of the propositions, where if a variable holds $▽$, its associated proposition is $\text{FALSE}$, and if the variable holds $▲$, the proposition is $\text{TRUE}$.

Consider the statement:
$$
\begin{gather}
  a & \land & b.\\
  \text{It is raining outside} & \text{and} & \text{the ground is wet.}
\end{gather}
$$

From $a \land b$, we have learned that we can infer $a$, through the _Simplification_ rule.

$$
\begin{gather}
  a & \land & b & \therefore & a\\
  \text{It is raining outside} & \text{and} & \text{the ground is wet} & \text{therefore} & \text{it is raining outside.}
\end{gather}
$$

And it does not stop there! Any proposition that we can construct in the real world, if modelled by our algebra, can undergo inferences!

Here is an example of Hypothetical Syllogism:

$$
\begin{gather}
( & a & \implies & b & ) & \land & a & \therefore & b.\\
 & \text{It is raining outside} & \text{implies} & \text{the ground is wet} & & \text{and} & \text{it is raining outside} & \text{therefore} & \text{the ground is wet}.
\end{gather}
$$

\[...\] Incomplete.

##### A note on invalid reasoning

Invalid reasoning is any reasoning where the conclusion is not properly inferred from the premises.

Consider the proof:

$$a \therefore b$$

It is invalid, very notably so since $a$ does not fully constrain $b$.

| $a$ | $b$ | $a$ | $\therefore b?$ |
| :-: | :-: | :-: | :-------------: |
| $▽$ | $▽$ | $▽$ |                 |
| $▽$ | $▲$ | $▽$ |                 |
| $▲$ | $▽$ | $▲$ |       $▽$       |
| $▲$ | $▲$ | $▲$ |       $▲$       |

The motive why invalid logical reasoning is often attempted is because the structure of the arguments or the conclusion's analogues, appear to _make sense_ in an intuitive way.

$$
\begin{gather}
  a & \therefore & b\\
  \text{I want ice cream} & \text{therefore} & \text{you should give me ice cream.}
\end{gather}
$$

Nevertheless, the _content_ of the given propositions has no bearing on the inferences that can be made on them, no matter how many propositions you use.

$$
\begin{gather}
  a & \land & b & \land & c & \therefore & d\\
  \text{I want ice cream} & & \text{i really want it} & & \text{did I tell you you're beautiful?} & \text{therefore} & \text{you should give me ice cream.}
\end{gather}
$$

> [!TODO] Edit
> This is unfinished, more text will be here, lol.


