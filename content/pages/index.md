All my cool projects!

```base
filters:
  and:
    - file.inFolder("content/pages/projects")
views:
  - type: kanban-view
    name: Kanban
    order:
      - file.name
      - title
      - description
      - tags
    columnOrders:
      file.file:
        - content/pages/projects/aio/index.md
        - content/pages/projects/building-blocks/index.md
        - content/pages/projects/novel/index.md
        - content/pages/projects/overtone/index.md
        - content/pages/projects/partitune/index.md
        - content/pages/projects/sol/index.md
        - content/pages/projects/ui-composer/index.md
      note.status:
        - To Do
        - In Progress
    cardOrders:
      file.file: {}
      note.status:
        Uncategorized: []
        To Do:
          - content/pages/projects/sol/index.md
          - content/pages/projects/ui-composer/index.md
          - content/pages/projects/aio/index.md
          - content/pages/projects/partitune/index.md
          - content/pages/projects/novel/index.md
          - content/pages/projects/overtone/index.md
    columnColors:
      file.file: {}
      note.status:
        In Progress: yellow
    cardTitleProperty: note.title
    groupByProperty: note.status
  - type: table
    name: Table
    order:
      - file.name
      - title
      - description
      - tags
      - dependencies
  - type: cards
    name: Cards
    order:
      - file.name
      - title
      - description
      - tags
    columnOrders:
      file.file:
        - content/pages/projects/aio/index.md
        - content/pages/projects/building-blocks/index.md
        - content/pages/projects/novel/index.md
        - content/pages/projects/overtone/index.md
        - content/pages/projects/partitune/index.md
        - content/pages/projects/sol/index.md
        - content/pages/projects/ui-composer/index.md
      note.status:
        - To Do
        - In Progress
    cardOrders:
      file.file: {}
      note.status:
        Uncategorized: []
        To Do:
          - content/pages/projects/sol/index.md
          - content/pages/projects/ui-composer/index.md
          - content/pages/projects/aio/index.md
          - content/pages/projects/partitune/index.md
          - content/pages/projects/novel/index.md
          - content/pages/projects/overtone/index.md
    columnColors:
      file.file: {}
      note.status:
        In Progress: yellow
    cardTitleProperty: note.title
    groupByProperty: note.status
    cardSize: 330

```