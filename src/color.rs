use colorized::Colors;

pub fn from<T: AsRef<str>>(name: T) -> Colors {
    match name.as_ref() {
        "black" => Colors::BlackFg,
        "red" => Colors::RedFg,
        "green" => Colors::GreenFg,
        "yellow" => Colors::YellowFg,
        "blue" => Colors::BlueFg,
        "magenta" => Colors::MagentaFg,
        "cyan" => Colors::CyanFg,
        "white" => Colors::WhiteFg,
        "brightblack" => Colors::BrightBlackFg,
        "brightred" => Colors::BrightRedFg,
        "brightgreen" => Colors::BrightGreenFg,
        "brightyellow" => Colors::BrightYellowFg,
        "brightblue" => Colors::BrightBlueFg,
        "brightmagenta" => Colors::BrightMagentaFg,
        "brightcyan" => Colors::BrightCyanFg,
        "brightwhite" => Colors::BrightWhiteFg,
        _ => Colors::Reset,
    }
}
