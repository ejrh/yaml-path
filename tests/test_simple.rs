use yaml_rust::{Yaml, YamlLoader};
use yaml_path::Path;

#[test]
fn test_simple() {
    let docs = YamlLoader::load_from_str("hello: there").unwrap();
    let doc = &docs[0];
    let path = Path::new("hello").unwrap();
    let results = path.get_all(&doc).unwrap();
    assert_eq!(results.len(), 1);
    let mut results = results.into_iter();
    let first = results.next().unwrap();
    assert_eq!(*first, Yaml::String(String::from("there")));
}
