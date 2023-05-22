use miette::SourceCode;

#[test]
fn substr_span() {
    // Make the file
    let contents = String::from("hello !there!");
    let source = axoasset::SourceFile::new("file.md", contents);

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
    let source = axoasset::SourceFile::new("file.md", contents);

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let there_span = source.span_for_substr("there");
    assert_eq!(there_span, None);
}

#[cfg(feature = "json-serde")]
#[test]
fn json_valid() {
    #[derive(serde::Deserialize, PartialEq, Eq, Debug)]
    struct MyType {
        hello: String,
        goodbye: bool,
    }

    // Make the file
    let contents = String::from(r##"{ "hello": "there", "goodbye": true }"##);
    let source = axoasset::SourceFile::new("file.js", contents);

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let val = source.deserialize_json::<MyType>().unwrap();
    assert_eq!(
        val,
        MyType {
            hello: "there".to_string(),
            goodbye: true
        }
    );
}

#[cfg(feature = "json-serde")]
#[test]
fn json_invalid() {
    #[derive(serde::Deserialize, PartialEq, Eq, Debug)]
    struct MyType {
        hello: String,
        goodbye: bool,
    }

    // Make the file
    let contents = String::from(r##"{ "hello": "there", "goodbye": true, }"##);
    let source = axoasset::SourceFile::new("file.js", contents);

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let res = source.deserialize_json::<MyType>();
    assert!(res.is_err());
}

#[cfg(feature = "toml-serde")]
#[test]
fn toml_valid() {
    #[derive(serde::Deserialize, PartialEq, Eq, Debug)]
    struct MyType {
        hello: String,
        goodbye: bool,
    }

    // Make the file
    let contents = String::from(
        r##"
hello = "there"
goodbye = true
"##,
    );
    let source = axoasset::SourceFile::new("file.toml", contents);

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let val = source.deserialize_toml::<MyType>().unwrap();
    assert_eq!(
        val,
        MyType {
            hello: "there".to_string(),
            goodbye: true
        }
    );
}

#[cfg(feature = "toml-serde")]
#[test]
fn toml_invalid() {
    #[derive(serde::Deserialize, PartialEq, Eq, Debug)]
    struct MyType {
        hello: String,
        goodbye: bool,
    }

    // Make the file
    let contents = String::from(
        r##"
hello = "there"
goodbye = 
"##,
    );
    let source = axoasset::SourceFile::new("file.toml", contents);

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let res = source.deserialize_toml::<MyType>();
    assert!(res.is_err());
}

#[cfg(feature = "toml-edit")]
#[test]
fn toml_edit_valid() {
    // Make the file
    let contents = String::from(
        r##"
hello = "there"
goodbye = true
"##,
    );
    let source = axoasset::SourceFile::new("file.toml", contents);

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let val = source.deserialize_toml_edit().unwrap();
    assert_eq!(val["hello"].as_str().unwrap(), "there");
    assert_eq!(val["goodbye"].as_bool().unwrap(), true);
}

#[cfg(feature = "toml-edit")]
#[test]
fn toml_edit_invalid() {
    // Make the file
    let contents = String::from(
        r##"
hello = "there"
goodbye = 
"##,
    );
    let source = axoasset::SourceFile::new("file.toml", contents);

    // Get the span for a non-substring (string literal isn't pointing into the String)
    let res = source.deserialize_toml_edit();
    assert!(res.is_err());
}
