[![Rust build](https://github.com/ejrh/yaml-path/actions/workflows/rust-build.yml/badge.svg)](https://github.com/ejrh/yaml-path/actions/workflows/rust-build.yml)
[![Rust tests](https://github.com/ejrh/yaml-path/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/ejrh/yaml-path/actions/workflows/rust-tests.yml)
[![Rust Clippy](https://github.com/ejrh/yaml-path/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/ejrh/yaml-path/actions/workflows/rust-clippy.yml)

`yaml-path` is a Rust implementation of the [YAML Path specification][yaml-path-spec].
It builds on the YAML implementation provided by the Pure-rust [yaml-rust][yaml-rust-crate] crate.

Example usage
---

```
    let docs = YamlLoader::load_from_str("hello: there").unwrap();
    let first_doc = &docs[0];
    let processor = Processor::new(&first_doc);
    let path = Path::new("hello").unwrap();
    let results = processor.get_all(&path).unwrap();
    assert_eq!(results.len(), 1);
    let mut results = results.into_iter();
    let first = results.next().unwrap();
    assert_eq!(*first, Yaml::String(String::from("there")));
```

Compatibility
---

`yaml-path` aims to follow the Python reference implementation of YAML Path.  In particular, it aims
to support the same path format with the same behaviour.

The library API will differ, of course, particularly as Rust imposes particular ownership and mutability
constraints on data.

Internal details may echo the Python implementation in places, but will not necessarily aim to do so.

As of this writing, `yaml-path` is in a very nascent state and supports barely any of YAML Path.  The journey is just beginning. `:)`

[yaml-path-spec]: https://github.com/wwkimball/yamlpath/wiki
[yaml-rust-crate]: https://crates.io/crates/yaml-rust
