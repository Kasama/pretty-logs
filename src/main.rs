mod cat;
mod color;

use clap::Parser;
use colored_json::{ColoredFormatter, CompactFormatter};
use colorized::Color;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

use self::cat::cat_processing_json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(
        short = 'c',
        long = "colors",
        default_value = "false",
        help = "Allow colored output"
    )]
    colors: bool,

    #[arg(
        help = "Handlebars-like template to use to parse inputs. If left empty, the program will try its best to guess [WIP] the structure of logs"
    )]
    template: Option<String>,
}

fn paint(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let color = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .map(color::from)
        .ok_or_else(|| RenderError::new("Param 0 is required to be a quoted string"))?;
    let val = h
        .param(1)
        .ok_or_else(|| RenderError::new("Param 1 is required to exist"))?;
    write!(out, "{}", val.value().render().color(color))?;
    Ok(())
}

fn rest(
    _: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let f = ColoredFormatter::with_styler(CompactFormatter {}, colored_json::Styler::default());
    write!(out, "{}", f.to_colored_json_auto(ctx.data())?)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    // best effort to enable ansi support on windows, but ignore any failure
    let _ = colored_json::enable_ansi_support();

    cat_processing_json(|v| {
        let tmpl_string = match &opts.template {
            Some(x) => x,
            None => "",
        };

        if tmpl_string.is_empty() {
            let f =
                ColoredFormatter::with_styler(CompactFormatter {}, colored_json::Styler::default());
            println!("{}", f.to_colored_json_auto(v)?);
            return Ok(None);
        }

        let mut tmpl = handlebars::Handlebars::new();

        tmpl.register_helper("paint", Box::new(paint));
        tmpl.register_helper("rest", Box::new(rest));

        println!("{}", tmpl.render_template(tmpl_string, v)?);
        Ok(None)
    })
}
