---
title: Sol
description: Programming language and toolchain for creating game assets.
tags:
  - assets
  - compiler
  - game-development
---

## Here Comes The Sol

Sol is a programming language (and a toolchain) for defining game assets with code!

```sol
-- iron_sword.sol
using game::ItemKind;

name = "Iron Sword"
durability = 5
description = "A simple yet well-balanced blade. It's rusty."

attack_power = 5

on_throw = action
    * You throw your father's sword on the ground...
end
```