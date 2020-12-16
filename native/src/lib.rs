use neon::prelude::*;
mod lexer;
mod token;
mod minify;
fn minify(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value();
    let lex = lexer::Lexer::new(&input);
    let mut minifier = minify::Minifier::new(lex);

    let string = minifier.generate_string();
    Ok(cx.string(string))
}

register_module!(mut cx, {
    cx.export_function("minify", minify)
});
