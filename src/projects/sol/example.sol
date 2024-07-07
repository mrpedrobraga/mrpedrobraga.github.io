impl Item
  name = "Hamburger"
  durability = None

  action when_used(user: Character, target: Character)
    target.stats.heal(10HP)
  end
end
