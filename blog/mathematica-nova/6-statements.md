---
date: 2025-01-01
tags:
  - logic
  - algebra
title: M. Nova Chapter 06 — Statements
description: ...
---
The concept of a statement is similar to that of an expression.

You create a statement by _affirming_ an expression. The process of affirmation is an exact complement of the process of evaluation.

In evaluation, you start from the components of an expression to determine the resulting value. In affirmation, you start from the outer value of the expression, to guess the values of the expression's components.

To affirm an expression is to say it evaluates to $▲$.

> Why $▲$ and not $▽$? Well, just because! I could have chosen $▽$ if I wanted. I do have to pick some value, so I will pick $▲$.

We can affirm the expression $\lnot▽$ like this:

$$\lnot▽.$$

Meanwhile can _not_ affirm the expression $\lnot▲$, after all, it does not evaluate to $▲$.

Meaning these are all examples of valid affirmations:

$$
\begin{equation}
	\begin{cases}
		\lnot▽\\
		▲\\
		\lnot\lnot\lnot▽
	\end{cases}
\end{equation}
$$

And these are not:

$$
\begin{equation}
	\begin{cases}
		\lnot▲\\
		▽\\
		\lnot\lnot\lnot\lnot\lnot\lnot▽
	\end{cases}
\end{equation}
$$

Now, admittedly, affirming statements like these does not _do_ anything.

The whole purpose with evaluation is that there is something you know (the parts of the expression) and something you do not know (the value of the expression).

For affirmations to be interesting, there must be _parts_ of the expression that you do not know the value to.

And for that, we will use a variable!!!

$$a$$

If you remember from the Lexical Bindings section, the value of an expression containing a single variable is the value of that variable. Now, I can not tell you what specific value it is, since "$a$" has no defined value. It is unknown.

Yet, this expression _can_ be affirmed without problem.

$$a.$$

What are the implications of affirming a variable like this?

Remember, affirmation is to say the expression evaluates to $▲$. But only some affirmations are valid. For a moment, imagine that the value of $a$ is $▽$.

Then the affirmation would be effectively:

$$▽.$$

Which is an invalid affirmation.

But, if the value of $a$ were $▲$...

$$▲.$$

You would have a valid affirmation.

In other words, if you _assume_ that the sentence '$a$' is valid, there is only one possible value $a$ could take: $▲$.

Take a moment for that to sink in.

When evaluating, we start from the parts of an expression and discover an outer unknown (the value of the expression). When affirming, we start from the outer value of the expression and discover inner unknowns (variables).

Consider this next affirmation:

$$\lnot b.$$

What value does this affirmation constrain the variable $b$ to hold?

...

That is right, $b$ is _forced_ to evaluate to $▽$, since '$\lnot▽$' is the only valid affirmation you can form out of $\lnot b.$

Let us try statements involving binary operations!

A statement containing two unknowns has four possible resolution choices, one for each combination of values the variables can hold. We can represent each possible combination on a table, with one column for each variable.

| $a$ | $b$ |
| --- | --- |
| $▽$ | $▽$ |
| $▽$ | $▲$ |
| $▲$ | $▽$ |
| $▲$ | $▲$ |

But, again, affirmations can constrain which of these choices are valid.

Consider this expression:
$$a \land b$$
We can observe all the possible evaluations of $a \land b$ in a table. The first columns still represent each possible combination of $a$ and $b$, but the last appended column shows what $a \land b$ evaluates to on each combination.

| $a$ | $b$ | $a \land b$ |
| --- | --- | ----------- |
| $▽$ | $▽$ | $▽$         |
| $▽$ | $▲$ | $▽$         |
| $▲$ | $▽$ | $▽$         |
| $▲$ | $▲$ | $▲$         |

By affirming that expression...

$$a \land b.$$

...we require $a \land b$ to evaluate to $▲$. There is only one case in which it does: the very last row. In this case, we can see that $a$ and $b$ both themselves evaluate to $▲$.

> [!tip]
> In other words, the pattern of $▽$ and $▲$ in the $a \land b$ column work as a stencil that limits which cases are valid. The cases where the stencil is $▲$ are valid, the others are not.

Because of this, wherever you find $a \land b$ affirmed, you can safely conclude ($\therefore$) $a$ as affirmed.

$$a \land b \therefore a$$

And, again, you can perform this transformation without knowing the values of the variables!

> [!info]
> "Rule of Inference" is a term you might hear when you do rewriting in algebra that utilises $▽$ and $▲$.
> 
> This specific rule of inference where you go from $a \land b$ to $a$ is very useful and gets a name: "Simplification."
> 
> In practice, you might remember a few rules of inference from the top of your head, but remember that they are not fundamental, i.e., that rules of inference exist is just a natural consequence of operators' evaluation tables + rewriting.

Let us try another expression!

$$a \lor b$$

And observing its evaluation table:

| $a$ | $b$ | $a \lor b$ |
| --- | --- | ---------- |
| $▽$ | $▽$ | $▽$        |
| $▽$ | $▲$ | $▲$        |
| $▲$ | $▽$ | $▲$        |
| $▲$ | $▲$ | $▲$        |

Hmm...

This time, we did not constrain $a$ and $b$ to specific values, even if we did limit which combinations of $a$ and $b$ are valid.

Now, check this out. On two of the valid cases, $b$ holds $▲$, and $a$ holds either $▽$ or $▲$. But on the one case where b holds $▽$, $a$ holds $▲$! If we could constrain $b$ to $▽$, it also constraints $a$ to $▲$!

We can constrain $b$ to $▽$ by affirming this:

$$\lnot b.$$

| $a$ | $b$ | $\lnot b$ |
| --- | --- | -------- |
| $▽$ | $▽$ | $▲$      |
| $▽$ | $▲$ | $▽$      |
| $▲$ | $▽$ | $▲$      |
| $▲$ | $▲$ | $▽$      |

We can merge both affirmations ($a \lor b$ with $\lnot b$) into a single one ($a \lor b) \land (\lnot b)$ and keeping count of which rows are valid cases, and which aren't.

| $a$ | $b$ | $(a \lor b) \land \lnot b$ | $\therefore$ |
| --- | --- | ------------------------- | ------------ |
| $▽$ | $▽$ | $▽$                       | ❌            |
| $▽$ | $▲$ | $▽$                       | ❌            |
| $▲$ | $▽$ | $▲$                       | ✅            |
| $▲$ | $▲$ | $▽$                       | ❌            |

Following, though, we will represent each part of the conjunction $\land$ as its own unique column. Valid cases are then rows that do not have any $▽$ (which is effective applying the $\land$ operator):

| $a$ | $b$ | $a \lor b$ | $\lnot b$ | $\therefore$ |
| --- | --- | ---------- | -------- | ------------ |
| $▽$ | $▽$ | $▽$        | $▲$      | ❌            |
| $▽$ | $▲$ | $▲$        | $▽$      | ❌            |
| $▲$ | $▽$ | $▲$        | $▲$      | ✅            |
| $▲$ | $▲$ | $▲$        | $▽$      | ❌            |

For simplicity, we can just _omit_ the rows of the table following a $▽$ in a stencil column, "blocking" that case from continuing: which is why I call these "stencil columns"!

| $a$ | $b$ | $a \lor b$ | $\lnot b$ | $\therefore$ |
| --- | --- | ---------- | -------- | ------------ |
| $▽$ | $▽$ | $▽$        |          |              |
| $▽$ | $▲$ | $▲$        | $▽$      |              |
| $▲$ | $▽$ | $▲$        | $▲$      | ✅            |
| $▲$ | $▲$ | $▲$        | $▽$      |              |

And there we go.

From a rule of inference point of view, we started with a pair of statements:

$$
\begin{equation}
	\begin{cases}
	a \lor b\\
	\lnot b
	\end{cases}
\end{equation}
$$

...and from them, inferred a third one:

$$\therefore a.$$

This specific rule of inference happens to be called "Disjunctive Syllogism." "Disjunction" is just how the $\lor$ operation is called by math nerds, and "Syllogism" is just a fancy name for rules of inference where a pair of statements infers a third one.

As an exercise, convince yourself that "Hypothetical Syllogism" (pictured below) is a valid rule of inference, by drawing the evaluation tables for it.
 $$
\begin{equation}
	\begin{cases}
	a \implies b\\
	a
	\end{cases}
\end{equation}
\quad \therefore \quad
b.
$$

#### Interesting Cases

Some affirmations evaluate to $▲$ no matter the values of the variables inside it, and therefore make no constraints on them.

| $a$ | $b$ | $a \lor \lnot a$ | $\therefore$ |
| --- | --- | --------------- | ------------ |
| $▽$ | $▽$ | $▲$             | ✅            |
| $▽$ | $▲$ | $▲$             | ✅            |
| $▲$ | $▽$ | $▲$             | ✅            |
| $▲$ | $▲$ | $▲$             | ✅            |

These statements are called "tautologies."

Other affirmations (or combination of affirmations) never evaluate to $▲$ no matter the values of the variables inside it, leaving no remaining valid cases to consider.

| $a$ | $b$ | $a \land \lnot a$ | $\therefore$ |
| --- | --- | ---------------- | ------------ |
| $▽$ | $▽$ | $▽$              | ❌            |
| $▽$ | $▲$ | $▽$              | ❌            |
| $▲$ | $▽$ | $▽$              | ❌           |
| $▲$ | $▲$ | $▽$              | ❌            |

These statements are called "contradictions."
