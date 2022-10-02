use anstyle::{Color, RgbColor};

/**
## Prints a text and leaves a line break with the passed-in Style.

#### <b>current_style</b>:
    - The current style, the one that will be applied AFTER the text is rendered, used to reverse the previous ANSI escape. Optional, ignored if None() is passed.
#### <b>text_style</b>:
    - The style the text will be applied when printed, replaces the previous ASNI escape, if any.
#### <b>text</b>:
    - The text that will be rendered.

## Examples:

```rs
    // Generate red, underlined style.
    let text_style = anstyle::Style::new()
        .fg_color(Some(Color::Rgb(RgbColor::from((255, 0, 0)))))
        .underline();
    // The default conosle color.
    let default_style =
        anstyle::Style::new().fg_color(Some(Color::Rgb(RgbColor::from((192, 192, 192)))));

    // Example function that returns String
    let my_error_text = get_error();

    // This function.
    print_ansi(Some(default_style),text_style,my_error_text);

    //! Expected Output: Text that is red and underlined, and, after it ends, the normal style to be set again.

```

*/
pub fn println_ansi(
    current_style: &Option<anstyle::Style>,
    text_style: &anstyle::Style,
    text: &String,
) {
    // Render ANSI to console.
    ansi_print(current_style, text_style, text, true);
}
/**
## Prints a text and does not leave a line break with the passed-in Style.

#### <b>current_style</b>:
    - The current style, the one that will be applied AFTER the text is rendered, used to reverse the previous ANSI escape. Optional, ignored if None() is passed.
#### <b>text_style</b>:
    - The style the text will be applied when printed, replaces the previous ASNI escape, if any.
#### <b>text</b>:
    - The text that will be rendered.

## Examples:

```rs
    // Generate red, underlined style.
    let text_style = anstyle::Style::new()
        .fg_color(Some(Color::Rgb(RgbColor::from((255, 0, 0)))))
        .underline();
    // The default conosle color.
    let default_style =
        anstyle::Style::new().fg_color(Some(Color::Rgb(RgbColor::from((192, 192, 192)))));

    // Example function that returns String
    let my_error_text = get_error();

    // This function.
    print_ansi(Some(default_style),text_style,my_error_text);

    //! Expected Output: Text that is red and underlined, and, after it ends, the normal style to be set again.

```

*/
pub fn print_ansi(
    current_style: &Option<anstyle::Style>,
    text_style: &anstyle::Style,
    text: &String,
) {
    // Render ANSI to console.
    ansi_print(current_style, text_style, text, false);
}

fn ansi_print(
    current_style: &Option<anstyle::Style>,
    text_style: &anstyle::Style,
    text: &String,
    println: bool,
) {
    if println {
        print!("{}", text_style.render());
        println!("{}", text);

        match current_style {
            Some(style) => println!("{}", style.render()),
            _ => return,
        };
        return;
    }

    // use print! macro instead of println! macro.

    print!("{}", text_style.render());
    print!("{}", text);

    match current_style {
        Some(style) => print!("{}", style.render()),
        _ => return,
    };
}
/**
    Quickly initialize a Color variable.

*/
pub fn initialize_color(colors: (u8, u8, u8)) -> Color {
    Color::from(initialize_rgb(colors))
}

/**
    Quickly initialize a RGB Color variable.

*/
pub fn initialize_rgb(colors: (u8, u8, u8)) -> RgbColor {
    RgbColor(colors.0, colors.1, colors.2)
}
