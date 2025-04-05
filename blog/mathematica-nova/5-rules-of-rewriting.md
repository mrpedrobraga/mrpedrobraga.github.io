---
date: 2025-01-01
tags:
  - logic
  - algebra
title: M. Nova Chapter 05 — Rules of Rewriting
description: ...
---
Consider an expression, which has some value:

$$\lnot(▽)$$
Now consider this other expression:

$$▲$$

If you remember the evaluation for $\lnot$, you will notice that these two expressions have the same value. This fact is important for our first Rule of Rewriting.

**Expressions that have the same value are equivalent and, thus, can be rewritten as each other.**

"What do you mean by 'can be rewritten'? Surely, if I have a pencil and paper I can write whatever I want, so what do you mean by that?"

In the previous chapter, we saw the expression $\land(\lnot(▽), ▲)$, right? This expression has some _value_ that we want to find out, but we can't.

By "rewriting," I mean that, by observing $\lnot(▽)$ and $▲$ have the same value, we can generate a _new_ expression where the sub-expression $\lnot(▽)$ has been substituted for $▲$: 
$$\land(▲, ▲)$$

"Uh... okay, but why would we do that?"

Hey, have some patience! The reason why we would do that is because this new expression... drum roll... has the same value as $\land(\lnot(▽), ▲)$! If we can evaluate it, we can evaluate the original expression.

> [!TIP]
> Because $\lnot(▽)$ is equivalent to $▲$,
> $\land(\lnot(▽), ▲)$ is equivalent to $\land(▲, ▲)$.

And, hey, we _are_ able to evaluate $\land(▲, ▲)$ by using that evaluation table. The resulting value is $▲$. Retroactively, we know this is also the value of the first expression $\land(\lnot(▽), ▲)$.

A good rewriting trick to have up your sleeve is something called "Reduction" — rewriting an expression by looking at its evaluation table. For example, $\lnot(▽)$ evaluates to $▲$ in the table, right? In this case we can perform "Reduction" of $\lnot(▽)$ to $▲$.

Taking from the top, the whole process for evaluating $\land(\lnot(▽), ▲)$ becomes:

1. $\land(\lnot(▽), ▲)$
2. Reduction, RW $\lnot(▽)$ to $▲$ — $\land(▲, ▲)$  :: 
3. Reduction, RW $\land(▲, ▲)$ to $▲$ — $▲$.

More complicated expressions can be rewritten step by step. Without explicitly annotating, you should be able to observe each Reduction:

1. $\land(\lnot(▽), \land(▽, ▲))$
2. $\land(▲, \land(▽, ▲))$
3. $\land(▲, ▽)$
4. $▽$

An expression like $▽$ that can not be reduced any further is considered to be in "normal form."