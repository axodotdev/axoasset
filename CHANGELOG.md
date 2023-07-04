# Changelog

## v0.4.0 - 2023-07-04

###  🎁 Features

- **Don't use OpenSSL - [Gankra], [pr56]**

### 🛠️  Fixes

- **Don't reject spans that cover the last char - [Gankra], [pr55]**

[pr55]: https://github.com/axodotdev/axoasset/pull/55
[pr56]: https://github.com/axodotdev/axoasset/pull/56

## v0.3.0 - 2023-05-23

### 🎁 Features

- **SourceFile::deserialize_toml_edit (behind new toml-edit feature) - [Gankra], [pr52]**

  Just a convenience to read a SourceFile as toml-edit and map the error spans to the right format.

### 🛠️ Fixes

- **Separate compression into cargo features - [shadows-withal], [pr47]**

  The APIs for processing tarballs/zips are now behind "compression-tar" and "compression-zip",
  with a convenience "compression" feature that covers both.

- **LocalAsset API cleanup - [shadows-withal], [pr48]**

  Some breaking cleanups to APIs to make them more ergonomic longterm

  - Many APIs that previously took Strings now take `AsRef<Utf8Path>`
  - write_new_{all} now just takes a path to the file, instead of folder_path + name 

- **update github CI - [striezel], [pr50]**

  Updating several old Github CI actions to more modern/maintained versions, thanks a ton!

* **fix typos - [striezel], [pr51]**

  Thanks!!

[pr47]: https://github.com/axodotdev/axoasset/pull/47
[pr48]: https://github.com/axodotdev/axoasset/pull/48
[pr50]: https://github.com/axodotdev/axoasset/pull/50
[pr51]: https://github.com/axodotdev/axoasset/pull/51
[pr52]: https://github.com/axodotdev/axoasset/pull/52

## v0.2.0 - 2023-04-27

### 🎁 Features

- **✨ New `LocalAsset` functionality! - [shadows-withal], [pr38], [pr46]**

  We've added a lot more functions to `LocalAsset`:

  - `write_new_all`, to write a file and its parent directories
  - `create_dir`, which creates, well, a new directory
  - `create_dir_all`, which creates a directory and its parent directories
  - `remove_file`, which deletes a file
  - `remove_dir`, which deletes an empty directory
  - `remove_dir_all`, which deletes a directory and its contents
  - `tar_{gz,xz,zstd}_dir`, which are three separate functions that create a tar archive with the
    specified compression algorithm, either Gzip, Xzip, or Zstd
  - `zip_dir`, which creates a zip archive

- **✨ New feature: `SourceFile::span_for_substr` - [Gankra], [pr35]**

  This function enables the ability to get spans even when using a tool that
  doesn't support them as long as it returns actual substrings pointing into
  the original SourceFile's inner String.

### 🛠️ Fixes

- **Simply SourceFile::new and new_empty - [Gankra], [pr43]**

  SourceFile::new and new_empty no longer return Results and simply use the origin_path
  as the file name, making them appropriate for synthetic/test inputs that don't map
  to actual files.

[pr35]: https://github.com/axodotdev/axoasset/pull/35
[pr43]: https://github.com/axodotdev/axoasset/pull/43
[pr38]: https://github.com/axodotdev/axoasset/pull/38
[pr46]: https://github.com/axodotdev/axoasset/pull/46


## v0.1.1 - 2023-04-06

### 🛠️  Fixes

- **Fix compilation errors for features and add tests - [Gankra]/[ashleygwilliams], [pr33]**

[pr33]: https://github.com/axodotdev/axoasset/pull/33

## v0.1.0 - 2023-04-06

### 🎁 Features

- **✨ New type: `SourceFile` - [Gankra],  [pr25]**

  `SourceFile` is a new asset type which is a readonly String version of
  `Asset` wrapped in an `Arc`. The purpose of this type is to be cheap to
  clone and pass around everywhere so that errors can refer to it (using the
  miette `#[source_code]` and `#[label]` attributes). The `Arc` ensures this
  is cheap at minimal overhead. The String ensures the contents make sense to
  display.

- **✨ New type: `Spanned` - [Gankra],  [pr25]**

  `Spanned<T>` is a new type which tries to behave like `Box<T>` in the sense
  that it's "as if" it's a `T` but with source span info embedded. If you want
  to remember that a value was decoded from an asset at bytes 100 to 200, you
  can wrap it in a `Spanned` without disrupting any of the code that uses it.
  Then if you determine that value caused a problem, you can call
  `Spanned::span(&value)` to extract the span and have miette include the
  asset context in the error message.

- **✨ New features: `serde_json` and `toml-rs` - [Gankra],  [pr25]**

  `json-serde` and `toml-serde` are new features which pull in dedicated
  support for `serde_json` and `toml-rs`. These features add `deserialize_json`
  and `deserialize_toml` methods to `SourceFile` which understand those crates'
  native error types and produce full pretty miette-y errors when deserializing,
  like this:

  ```
    × failed to read JSON
    ╰─▶ trailing comma at line 3 column 1
     ╭─[src/tests/res/bad-package.json:2:1]
   2 │     "name": null,
   3 │ }
     · ─
     ╰────
  ```

  (In this case serde_json itself points at the close brace and not the actual comma, we're just faithfully forwarding that.)

  `Spanned` has special integration with `toml-rs`, because it's actually a
  fork of that crate's [own magic `Spanned` type]. If you deserialize a struct
  that contains a `Spanned<T>` it will automagically fill in the span info
  for you. Ours further improves on this by putting in more effort to be totally
  transparent like `Box`.

- **✨ New function: `write_new` for `LocalAsset` - [ashleygwilliams], [pr28]**

  axoasset was first conceived to handle assets declared by end users for use
  in `oranda`, but quickly grew to encompass all fs/network calls. one of the
  things we often need to do is create a new file. This is only available on
  `LocalAsset` as, at least for the moment, that is the only place axoasset
  has permissions to create new assets.

- **make `RemoteAsset` an optional feature - [Gankra], [pr26]**

  A feature of `axoasset` is that it is agnostic to the origin of the asset:
  it can be local or remote. However, often, authors can be certain that they
  will only be using local assets. In this case, it reduces dependencies to
  not include the remote functionality. Previously this wasn't possible!

- **`miette-ify` errors - [Gankra], [pr24]**

  Previously we were using `thiserror` for error handling, but to be consistent
  across our toolchain, we've updated our errors to use `miette`. This has the
  added benefit of formalizing structures we were informally building into our
  error types (help/diagnostic text, forwarding the bare error as details, etc).


- **consistent `Asset` interface - [ashleygwilliams], [pr30]**

  With 3 asset types, `LocalAsset`, `RemoteAsset`, and `SourceFile`, it felt
  important to align their structures so they could be used nearly identically.
  Every type now has a:

     - `origin_path`: the original source of the file
     - `filename`: derived from the `origin_path` and, in the case of `RemoteAsset`s
        also the headers from the network response.
     - `contents`: the contents of the asset as bytes or a String depending on
        asset type

[pr24]: https://github.com/axodotdev/axoasset/pull/24
[pr25]: https://github.com/axodotdev/axoasset/pull/25
[pr26]: https://github.com/axodotdev/axoasset/pull/26
[pr28]: https://github.com/axodotdev/axoasset/pull/28
[pr30]: https://github.com/axodotdev/axoasset/pull/30

[own magic `Spanned` type]: https://docs.rs/toml/latest/toml/struct.Spanned.html

## v0.0.1 - 2023-02-14

Initial release.

[ashleygwilliams]: https://github.com/ashleygwilliams
[gankra]: https://github.com/gankra
[shadows-withal]: https://github.com/shadows-withal
[striezel]: https://github.com/striezel
