use serde_json::Value;
use std::io::{self, Read, Write};
/// This function reads from stdin and tries to parse it as a JSON object. If it cannot, it works
/// just like the `cat` command and echoes the same input to stdout. If it can, it calls
/// `json_processor` that will do some work that can then print to stdout with some different
/// formatting
///
/// If the `json_processor` function returns `Some(str)`, `str` will be outputed to stdout.
///
/// Example:
/// -------
///
/// ```
/// cat_processing_json(|v| {
///     println!("It's a json! {:?}", v);
///     Ok(None)
/// })
/// ```
pub fn cat_processing_json(
    json_processor: impl Fn(&Value) -> anyhow::Result<Option<String>>,
) -> anyhow::Result<()> {
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = io::stdin().read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        match serde_json::from_slice::<Value>(&buffer[..bytes_read])
            .map_err(anyhow::Error::new)
            .and_then(|v| {
                if v.is_object() {
                    return json_processor(&v);
                }
                Ok(None)
            }) {
            Ok(sb) => {
                if let Some(b) = sb {
                    io::stdout().write_all(b.as_bytes())?;
                }
            }
            Err(err) => {
                write!(io::stderr(), "Error: {err}")?;
                io::stdout().write_all(&buffer[..bytes_read])?;
            }
        }
    }

    Ok(())
}
