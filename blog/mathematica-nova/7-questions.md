---
date: 2025-01-01
tags:
  - logic
  - algebra
title: M. Nova Chapter 07 — Questions
description: ...
---

In addition to affirmations, you can also do questions. While affirming $a$ means assuming it evaluates to $▲$ and constraining the inner variables of the affirmation, questioning means "proving" that the question evaluates to $▲$.

$$\therefore a?$$

The question, like an affirmation, has a column made of all the possible values its expression can evaluate to.

| $\therefore a?$ |
| --------------- |
| $▽$             |
| $▲$             |

Proving means appending the question column to the end of the table, and observing how the possible values of the question are constrained.

For example, $a \land b$ applies sufficient constraints such that in all remaining valid cases $a?$ only evaluates to $▲$.

| $a$ | $b$ | $a \land b$ | $\therefore a?$ |
| --- | --- | ----------- | --------------- |
| $▽$ | $▽$ | $▽$         |                 |
| $▽$ | $▲$ | $▽$         |                 |
| $▲$ | $▽$ | $▽$         |                 |
| $▲$ | $▲$ | $▲$         | $▲$             |

Appending a question after affirmations is how we represent the idea of "Do these statements imply this other statement?"

On the previous section I showed how $(a \lor b) \land \lnot b$ implies $a$. Here is how we might "question" if it really does imply $a$.

| $a$ | $b$ | $a \lor b$ | $\lnot b$ | $\therefore a?$ |
| :-: | :-: | :--------: | :------: | :-------------: |
| $▽$ | $▽$ |    $▽$     |          |                 |
| $▽$ | $▲$ |    $▲$     |   $▽$    |                 |
| $▲$ | $▽$ |    $▲$     |   $▲$    |       $▲$       |
| $▲$ | $▲$ |    $▲$     |   $▽$    |                 |

And in the one resulting case, you can see that $a$ does evaluate to $▲$. Meaning that yes!

Let us see how this looks like for the previous section's exercise, too, regarding Hypothetical Syllogism, and ask ourselves: "Does $(a \implies b) \land a$ imply $b$?"

| $a$ | $b$ | $a \implies b$ | $a$ | $\therefore b?$ |
| :-: | :-: | :------------: | :-: | :-------------: |
| $▽$ | $▽$ |      $▲$       | $▽$ |                 |
| $▽$ | $▲$ |      $▲$       | $▽$ |                 |
| $▲$ | $▽$ |      $▽$       |     |                 |
| $▲$ | $▲$ |      $▲$       | $▲$ |       $▲$       |

And we see that it does!

Here is a case of a failed proof.

| $a$ | $b$ | $a \implies b$ | $b$ | $\therefore a?$ |
| :-: | :-: | :------------: | :-: | :-------------: |
| $▽$ | $▽$ |      $▲$       | $▽$ |                 |
| $▽$ | $▲$ |      $▲$       | $▲$ |       $▽$       |
| $▲$ | $▽$ |      $▽$       |     |                 |
| $▲$ | $▲$ |      $▲$       | $▲$ |       $▲$       |

The question column has both $▲$ and $▽$, meaning that the expression can not be definitely constrained.

Tautologies can not help in proofs, since they perform no constraining.

| $a$ | $b$ | $a \lor \lnot a$ | $\therefore b?$ |
| :-: | :-: | :-------------: | :-------------: |
| $▽$ | $▽$ |       $▲$       |       $▽$       |
| $▽$ | $▲$ |       $▲$       |       $▲$       |
| $▲$ | $▽$ |       $▲$       |       $▽$       |
| $▲$ | $▲$ |       $▲$       |       $▲$       |

Contradictions can not help in proofs, since they completely eliminate all possible cases.

| $a$ | $b$ | $(a \implies b) \land (a \implies \lnot b)$ | $\therefore b?$ |
| :-: | :-: | :----------------------------------------: | :-------------: |
| $▽$ | $▽$ |                    $▽$                     |       $▽$       |
| $▽$ | $▲$ |                    $▽$                     |       $▲$       |
| $▲$ | $▽$ |                    $▽$                     |       $▽$       |
| $▲$ | $▲$ |                    $▽$                     |       $▲$       |

When used in question forms, however, tautologies can be proved without any affirmations.

| $a$ | $b$ | $\therefore a \lor \lnot a?$ |
| :-: | :-: | :-------------------------: |
| $▽$ | $▽$ |             $▲$             |
| $▽$ | $▲$ |             $▲$             |
| $▲$ | $▽$ |             $▲$             |
| $▲$ | $▲$ |             $▲$             |

Contradictions can _never_ be proved, no matter how many affirmations.

| $a$ | $b$ | $\therefore \lnot a \impliedby a?$ |
| :-: | :-: | :-------------------------------: |
| $▽$ | $▽$ |                $▽$                |
| $▽$ | $▲$ |                $▽$                |
| $▲$ | $▽$ |                $▽$                |
| $▲$ | $▲$ |                $▽$                |

