use miette::SourceCode;

#[test]
fn substr_span() {
    // Make the file
    let contents = String::from("hello !there!");
    let source = axoasset::SourceFile::new("file.md", contents).unwrap();

    // Do some random parsing operation
    let mut parse = source.contents().split('!');
    let _ = parse.next();
    let there = parse.next().unwrap();

    // Get the span
    let there_span = source.span_for_substr(there).unwrap();

    // Assert the span is correct
    let span_bytes = source.read_span(&there_span, 0, 0).unwrap().data();
    assert_eq!(std::str::from_utf8(span_bytes).unwrap(), "there");
}

#[test]
fn substr_span_invalid() {
    // Make the file
    let contents = String::from("hello !there!");
    let source = axoasset::SourceFile::new("file.md", contents).unwrap();

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let there_span = source.span_for_substr("there");
    assert_eq!(there_span, None);
}
