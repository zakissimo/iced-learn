# iced-learn notes

Quick-reference patterns and norms. Built as I go.

## Layout vocabulary

The whole iced layout system is three widgets:

| Widget      | What it does                       | Holds                                       |
|-------------|------------------------------------|---------------------------------------------|
| `column`    | stacks children **vertically**     | many, `.spacing(n)` between                 |
| `row`       | stacks children **horizontally**   | many, `.spacing(n)` between                 |
| `container` | wraps **one** child in a frame     | `.padding`, `.max_width`, `.center_x`, etc. |

Bonus: `scrollable` is also "wrap one child" — same shape as `container`.

There is no vertical row. `column` = vertical, `row` = horizontal.

## Standard view tree

The shape almost every iced view takes:

```
container        ← outer frame: padding, max_width, center on screen
└── column       ← page (big spacing between sections)
    ├── widget                              section: title
    ├── row (small spacing)                 section: input bar
    │   ├── widget
    │   └── widget
    └── scrollable
        └── column (small spacing)          section: list
            ├── row    ← one item
            ├── row
            └── row
```

Sections aren't a widget. They emerge from the tree shape.

## Centering

```rust
let rectangle = container(text("hi"))
    .width(400)
    .height(300)
    .style(container::bordered_box);

container(rectangle).center(Fill).into()
```

- `.center(Fill)` = "I fill my parent, and I put my single child in the middle (both axes)."
- Need two containers because `.center(Fill)` forces width/height to `Fill` — conflicts with `.width(400)`.
- Containers are invisible by default. `.style(container::bordered_box)` (border) or `container::rounded_box` (rounded fill) to see them.

## Padding vs spacing

Easy to mix up. They are never the same thing.

|                 | `padding`                                       | `spacing`                       |
|-----------------|-------------------------------------------------|---------------------------------|
| Where           | inside `container` / `column` / `row`           | inside `column` / `row`         |
| What it controls | distance from the **edge** to the (single) child | gap **between** siblings        |
| Mental picture  | wall thickness                                  | gap between books on a shelf    |

No `margin` in iced. For "space around a widget," put it inside a parent with `padding`, or use `spacing` on the parent.

### Padding API forms

```rust
container(child).padding(20)             // all four sides
container(child).padding([10, 20])       // [vertical, horizontal]
container(child).padding([10, 20, 10, 20]) // [top, right, bottom, left]  (CSS order)
```

`column` and `row` also have `.padding(...)` — same semantics.

### Practical values

| Level             | Use                                              | Value             |
|-------------------|--------------------------------------------------|-------------------|
| Page padding      | outer container, breathing room from window edge | `.padding(20-24)` |
| Section padding   | card / sub-container                             | `.padding(12-16)` |
| Tight padding     | inside a button, badge                           | `.padding(4-8)`   |
| Outer spacing     | column wrapping the whole page (between sections) | `.spacing(20)`    |
| Inner spacing     | row or column inside a section (between siblings) | `.spacing(8)`     |

Rule: bigger gap between unrelated sections, smaller gap between related siblings.

## Sizing: Shrink vs Fill

Every widget has a width and a height set to a `Length`. The choice determines whether it hugs its content or expands.

| Length              | What it does                              | Default for                          |
|---------------------|-------------------------------------------|--------------------------------------|
| `Shrink`            | hug content                               | most widgets, including `container`  |
| `Fill`              | expand to fill parent                     | `text_input`, `scrollable`           |
| `Fixed(n)` / `.width(480)` | exact pixels                       | rare — only when truly fixed         |
| `FillPortion(n)`    | take `n` shares of remaining space        | dividing a row into ratios           |

Mental rule: **if a styled widget isn't covering what you expect, it almost always needs `.width(Fill)` / `.height(Fill)`.** Same trick fixes "my background only covers the top," "my button is too small," "my modal doesn't span."

`.center(Fill)` is a shortcut for `.width(Fill).height(Fill)` plus center-alignment of the single child.

## Pushing widgets apart in a row

A `row` is only as wide as its children. To push siblings to opposite ends, insert something that wants to grow.

```rust
use iced::widget::space;

row![
    text("left"),
    space::horizontal(),    // flexible spacer eats remaining width
    text("right"),
]
```

| API                             | What it gives you                          |
|---------------------------------|--------------------------------------------|
| `space::horizontal()`           | `Space` with `width(Fill)` — flex spacer   |
| `space::vertical()`             | `Space` with `height(Fill)` — same on Y    |
| `space()`                       | empty `Space`, no size (rarely useful)     |
| `Space::new().width(Length::Fill)` | manual, for asymmetric cases            |

Three ways to make a row spread its children:
1. Put `space::horizontal()` between them (most common — keeps both ends compact).
2. Give one child `.width(Fill)` so it stretches (e.g. `text_input` already does this by default — that's why your input + button row already pushed Add to the right).
3. `FillPortion(n)` on multiple children for ratios (e.g. 1:2:1).

## Window sizes

Default size by app shape:

| App type                       | Size       | Aspect       |
|--------------------------------|------------|--------------|
| Dialog / utility               | 400 × 300  | 4:3          |
| Single-column app (todo, calc) | 480 × 640  | 3:4 (taller) |
| General small app              | 600 × 400  | 3:2          |
| Editor / multi-pane            | 900 × 600  | 3:2          |
| Full IDE-ish                   | 1280 × 800 | 16:10        |

Rules of thumb:
- **Width follows content shape.** Lists/forms → tall. Editors/dashboards → wide.
- **3:2 is the neutral aspect ratio.** Use it when in doubt.
- **min_size:** ~320×240. Prevents collapse on resize.
- **max content width inside the window:** ~600–700px via `container.max_width(640)`. Long lines hurt readability past that.

## Niri auto-floating

niri auto-floats windows that advertise themselves as fixed-size over Wayland. To trigger this from iced:

```rust
iced::application(...)
    .window_size((480.0, 640.0))
    .resizable(false)              // sends min_size==max_size, niri reads as "fixed"
    .run()
```

For a resizable window that should still start floating at a chosen size, use a niri window-rule instead. Find the app-id with `niri msg windows`, then in `~/.config/niri/config.kdl`:

```kdl
window-rule {
    match app-id="todos"
    open-floating true
    default-column-width { fixed 640; }
    default-window-height { fixed 480; }
}
```

Rule of thumb:
- **Exact size, never resizable** → set in iced (`resizable(false)` + `window_size`).
- **Resizable, sensible starting size** → niri window-rule.
