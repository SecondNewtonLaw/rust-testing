pub mod utils;

use anstyle::{Color, RgbColor, Style};
use clearscreen::ClearScreen;

use crate::utils::initialize_color;

#[tokio::main]
async fn main() {
    print_license();
}
/**
Print the license that the program was written with.
*/
fn print_license() {
    // ! Made (basically) to test IO.

    // The default style.
    let default_style =
        anstyle::Style::new().fg_color(Some(Color::Rgb(RgbColor::from((192, 192, 192)))));

    let license: String = match std::fs::read_to_string("LICENSE") {
        Ok(x) => x,
        Err(e) => {
            println!("{}, IO Error. Proceeding anyways...", e);
            String::from("ERR")
        }
    };
    // ! Check license IO Operation for error.
    if license == "ERR" {
        return;
    }

    let style_of_license = anstyle::Style::new()
        .fg_color(Some(Color::Rgb(RgbColor::from((255, 0, 0)))))
        .underline();

    utils::println_ansi(&Some(default_style), &style_of_license, &license);

    println!(
        "Do you accept the license? {}(Y/N){}> ",
        style_of_license.render(),
        default_style.render()
    );
    let mut buff: String = String::new();
    let read_size = match std::io::stdin().read_line(&mut buff) {
        Ok(num) => num,
        _ => {
            println!("The Standard Input Handle returned an error...");
            0
        }
    };

    if read_size == 0 {
        return;
    }
    let trimmed_buf = buff.trim().to_lowercase();
    let accepted = trimmed_buf == "y" || trimmed_buf == "yes";

    if accepted != true {
        println!(
            "The license {}must{} be accepted to run this program.",
            Style::new()
                .fg_color(Some(initialize_color((150, 0, 150))))
                .italic()
                .render(),
            default_style.render()
        );
    } else {
        ClearScreen::default()
            .clear()
            .expect("Could not clear the console...");
    }
}
