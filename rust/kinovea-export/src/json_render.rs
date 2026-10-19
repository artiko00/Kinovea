//! A JSON tree and its indented text, laid out the way `JsonTextWriter` lays out
//! `Formatting.Indented` (two spaces, `"name": value`, empty containers on one line).

/// Object members keep their order and may repeat a name, as the original writes them.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Object(Vec<(String, Json)>),
    Array(Vec<Json>),
    String(String),
    Float(f32),
    Int(i64),
}

impl Json {
    pub fn object(members: Vec<(&str, Json)>) -> Json {
        Json::Object(
            members
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v))
                .collect(),
        )
    }

    pub fn text(s: &str) -> Json {
        Json::String(s.to_owned())
    }
}

pub fn render(value: &Json) -> String {
    let mut out = String::new();
    write_value(value, 0, &mut out);
    out
}

fn write_value(value: &Json, depth: usize, out: &mut String) {
    match value {
        Json::Object(members) => {
            write_container('{', '}', members, depth, out, |(name, v), out| {
                out.push_str(&serde_json::to_string(name).unwrap_or_default());
                out.push_str(": ");
                write_value(v, depth + 1, out);
            })
        }
        Json::Array(items) => write_container('[', ']', items, depth, out, |v, out| {
            write_value(v, depth + 1, out)
        }),
        Json::String(s) => out.push_str(&serde_json::to_string(s).unwrap_or_default()),
        Json::Float(v) => write_float(*v, out),
        Json::Int(v) => out.push_str(&v.to_string()),
    }
}

fn write_container<T>(
    open: char,
    close: char,
    items: &[T],
    depth: usize,
    out: &mut String,
    mut item: impl FnMut(&T, &mut String),
) {
    out.push(open);
    if items.is_empty() {
        out.push(close);
        return;
    }
    for (i, element) in items.iter().enumerate() {
        out.push_str(if i == 0 { "\n" } else { ",\n" });
        out.push_str(&"  ".repeat(depth + 1));
        item(element, out);
    }
    out.push('\n');
    out.push_str(&"  ".repeat(depth));
    out.push(close);
}

/// Shortest round-trip text with a decimal point; non-finite values are quoted strings, the
/// default of `JsonTextWriter` (`FloatFormatHandling.String`).
fn write_float(v: f32, out: &mut String) {
    if v.is_nan() {
        out.push_str("\"NaN\"");
    } else if v.is_infinite() {
        out.push_str(if v > 0.0 {
            "\"Infinity\""
        } else {
            "\"-Infinity\""
        });
    } else {
        out.push_str(&format!("{v:?}"));
    }
}
