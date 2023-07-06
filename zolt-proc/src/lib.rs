use proc_macro::TokenStream;

#[proc_macro]
pub fn msg_macros(tokens: TokenStream) -> TokenStream {
    let items: Vec<String> = tokens.into_iter().map(|item| item.to_string()).collect();
    let macro_name = items.get(0).expect("No macro name was supplied");
    let msg_type = items
        .get(2)
        .expect("No message type was specified")
        .trim_start_matches('"')
        .trim_end_matches('"');
    let msg_prefix = items.get(4).expect("No message prefix was specified");
    let msg_color = items.get(6).expect("No message color was specified");
    let target_macro = items
        .get(8)
        .expect("No target printing macro was specified");

    let gen_macro = |name, fmt_macro, action, suffix, fmt_note| {
        format!(
            r#"
            /// {action} a message suitable for {msg_type} information{suffix}.
            /// It takes the same arguments as [`{fmt_macro}!`].{}
            #[macro_export]
            macro_rules! {name} {{
                ($($arg:tt)*) => {{{{
                    let prefix = {{
                        use $crate::colored::Colorize;
                        {msg_prefix}.{msg_color}().bold()
                    }};
                    {fmt_macro}!("{{}}: {{}}", prefix, format!($($arg)*))
                }}}}
            }}
        "#,
            if fmt_note {
                format!(
                "\n///\n/// It outputs to [`std::io::{}`] by default. If you need more control than that, use [`{macro_name}fmt!`] instead.",
                if target_macro == "print" {
                    "Stdout"
                } else {
                    "Stderr"
                }
            )
            } else {
                "".to_owned()
            }
        )
    };
    let mut macros = gen_macro(format!("{macro_name}fmt"), "format", "Format", "", false);
    macros += &gen_macro(macro_name.to_owned(), target_macro, "Print", "", true);
    macros += &gen_macro(
        format!("{macro_name}ln"),
        &format!("{target_macro}ln"),
        "Print",
        ", with a newline at the end",
        true,
    );
    // panic!("{macros}");
    macros.parse().unwrap()
}
