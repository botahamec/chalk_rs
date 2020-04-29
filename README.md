A crate for terminal colors and styles

```rust
use chalk_rs::prelude::*;
fn main() {
    let mut chalk = BasicChalk::new();
    chalk.red().println(&"This text is red");
    chalk.bold().println(&"Now it's red AND bold");
}
```

That's an example of basic color. There are three types of color in chalk:
Basic, Ansi, and RGB.

```rust
use chalk_rs::prelude::*;

fn main() {
    let mut ansi = AnsiChalk::new();
    ansi.ansi(56).println(&"Purple-ish");
    let mut rgb = RgbChalk::new();
    rgb.rgb(25, 125, 63).println(&"This color is ugly");
}
```

RGB chalk is able to use ANSI and Basic color. ANSI chalk is able to use basic
colors. However, ANSI chalk cannot use RGB and Basic chalk can't use RGB
or ANSI.

```rust
use chalk_rs::prelude::*;

fn main() {
    let mut rgb = RgbChalk::new();
    rgb.ansi(56).println(&"Purple-ish");
    rgb.red().println(&"red");
}
```