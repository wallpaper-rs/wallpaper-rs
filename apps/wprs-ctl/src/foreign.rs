use crate::told::{told, tr};

const SERDE: &[&str] = &[
    told("missing field `%1`"),
    told("duplicate field `%1`"),
    told("unknown field `%1`, expected one of %2"),
    told("unknown field `%1`, there are no fields"),
    told("unknown field `%1`, expected %2"),
    told("unknown variant `%1`, expected one of %2"),
    told("unknown variant `%1`, expected %2"),
    told("invalid type: %1, expected %2"),
    told("invalid value: %1, expected %2"),
    told("invalid length %1, expected %2"),
    told("data did not match any variant of the untagged enum %1"),
    told("expected value"),
    told("expected `,` or `}`"),
    told("expected `,` or `]`"),
    told("expected `:`"),
    told("expected ident"),
    told("expected string"),
    told("key must be a string"),
    told("trailing characters"),
    told("trailing comma"),
    told("EOF while parsing a value"),
    told("EOF while parsing an object"),
    told("EOF while parsing a list"),
    told("EOF while parsing a string"),
    told("invalid escape"),
    told("invalid number"),
    told("number out of range"),
    told("control character (\\u0000-\\u001F) found while parsing a string"),
];
const PLACE: &str = told(" at line %1 column %2");
const KINDS: &[&str] = &[
    told("No such file or directory"),
    told("Permission denied"),
    told("File exists"),
    told("Not a directory"),
    told("Is a directory"),
    told("Directory not empty"),
    told("No space left on device"),
    told("Read-only file system"),
    told("Connection refused"),
    told("Broken pipe"),
    told("Invalid argument"),
    told("Too many open files"),
];
const WHOLE: &[&str] = &[
    told("cannot make an empty path absolute"),
    told("failed to fill whole buffer"),
    told("stream did not contain valid UTF-8"),
];
const OS: &str = told(" (os error %1)");

fn arguments(template: &str, text: &str) -> Option<(Vec<String>, usize, usize)> {
    let mut parts = template.split('%');
    let first = parts.next()?;
    let start = text.find(first)?;
    let mut at = start + first.len();
    let mut caught = Vec::new();
    for part in parts {
        let literal = part.trim_start_matches(|c: char| c.is_ascii_digit());
        let rest = &text[at..];
        let stop = match literal.is_empty() {
            true => {
                [rest.find(" at line "), rest.find('\n')].into_iter().flatten().min().unwrap_or(rest.len())
            }
            false => rest.find(literal)?,
        };
        caught.push(rest[..stop].to_owned());
        at += stop + literal.len();
    }
    Some((caught, start, at))
}

type Say<'a> = &'a dyn Fn(&str) -> String;

fn filled(say: Say, template: &str, args: &[String]) -> String {
    args.iter().enumerate().rev().fold(say(template), |out, (i, a)| out.replace(&format!("%{}", i + 1), a))
}

fn replaced(say: Say, text: &str, templates: &[&str]) -> String {
    let mut order = templates.to_vec();
    order.sort_by_key(|t| std::cmp::Reverse(t.len()));
    for template in order {
        if let Some((caught, start, end)) = arguments(template, text) {
            let rest = replaced(say, &text[end..], templates);
            return format!("{}{}{}", &text[..start], filled(say, template, &caught), rest);
        }
    }
    text.to_owned()
}

pub fn said(text: &str) -> String {
    said_with(&tr, text)
}

pub(crate) fn said_with(say: Say, text: &str) -> String {
    let text = replaced(say, text, SERDE);
    let text = replaced(say, &text, &[PLACE]);
    let text = replaced(say, &text, &[OS]);
    KINDS.iter().chain(WHOLE).fold(text, |text, known| text.replace(known, &say(known)))
}

#[cfg(test)]
mod tests {
    use super::said_with;
    use crate::told::speaking;

    #[test]
    fn serde_and_system_errors_come_out_in_spanish() {
        let es = speaking("es");
        let said = |text: &str| said_with(&es, text);
        assert_eq!(
            said("missing field `id` at line 1 column 7"),
            "falta el campo `id` en la línea 1, columna 7"
        );
        assert_eq!(
            said("unknown variant `x`, expected one of `set`, `add` at line 1 column 3"),
            "variante desconocida `x`; las válidas son `set`, `add` en la línea 1, columna 3"
        );
        assert_eq!(
            said("invalid type: string \"a\", expected u32"),
            "tipo no válido: string \"a\"; se esperaba u32"
        );
        assert_eq!(
            said("No such file or directory (os error 2)"),
            "No existe el archivo o la carpeta (error del sistema 2)"
        );
        assert_eq!(said("cannot make an empty path absolute"), "no se puede completar una ruta vacía");
        assert_eq!(said("something else entirely"), "something else entirely");
    }

    #[test]
    fn english_stays_when_there_is_no_translation() {
        let none = |text: &str| text.to_owned();
        assert_eq!(said_with(&none, "missing field `id`"), "missing field `id`");
    }
}
