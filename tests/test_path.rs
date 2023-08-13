use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

use test_case::test_case;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use yaml_path::Path;

fn get_test_data() -> Result<Vec<Yaml>, Box<dyn Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let mut path = PathBuf::from(manifest_dir);
    path.push("tests/test_path.yaml");
    let yaml_str = read_to_string(path)?;
    Ok(YamlLoader::load_from_str(&yaml_str)?)
}

#[test_case("/squads/alpha", &["1.1"], true, None)]
#[test_case("squads.bravo", &["2.2"], true, None)]
#[test_case("/array_of_hashes/1", &["step: 2\nname: two"], true, None)]

// #[test_case("aliases[&aliasAnchorOne]", &["Anchored Scalar Value"], true, None)]
// #[test_case("aliases[&newAlias]", &["Not in the original data"], false, Some("Not in the original data"))]
// #[test_case("aliases[0]", &["Anchored Scalar Value"], true, None)]
// #[test_case("aliases.0", &["Anchored Scalar Value"], true, None; "aliases.0")]
// #[test_case("(array_of_hashes.name)+(rollback_hashes.on_condition.failure.name)", &["[\"one\", \"two\", \"three\", \"four\"]"], true, None)]
// #[test_case("/array_of_hashes/name", &["one", "two"], true, None)]
// #[test_case("aliases[1:2]", &["[Hey, Number Two!]"], true, None)]
// #[test_case("aliases[1:1]", &["[Hey, Number Two!]"], true, None)]
// #[test_case("squads[bravo:charlie]", &["2.2", "3.3"], true, None)]
// #[test_case("/&arrayOfHashes/1/step", &["2"], true, None)]
// #[test_case("&arrayOfHashes[step=1].name", &["one"], true, None)]
// #[test_case("squads[.!=\"\"][.=1.1]", &["1.1"], true, None)]
// #[test_case("squads[.!=\"\"][.>1.1][.<3.3]", &["2.2"], true, None)]
// #[test_case("aliases[.^Hey]", &["Hey, Number Two!"], true, None)]
// #[test_case("aliases[.$Value]", &["Anchored Scalar Value"], true, None)]
// #[test_case("aliases[.%Value]", &["Anchored Scalar Value"], true, None; "alises[.%value]")]
// #[test_case("&arrayOfHashes[step>1].name", &["two"], true, None)]
// #[test_case("&arrayOfHashes[step<2].name", &["one"], true, None)]
// #[test_case("squads[.>charlie]", &["4.4"], true, None)]
// #[test_case("squads[.>=charlie]", &["3.3, 4.4"], true, None)]
// #[test_case("squads[.<bravo]", &["1.1"], true, None)]
// #[test_case("squads[.<=bravo]", &["1.1, 2.2"], true, None)]
// #[test_case(r"squads[.=~/^\w{6,}$/]", &["3.3"], true, None)]
// #[test_case("squads[alpha=1.1]", &["1.1"], true, None)]
// #[test_case("(&arrayOfHashes.step)+(/rollback_hashes/on_condition/failure/step)-(disabled_steps)", &["[1, 4]"], true, None)]
// #[test_case("(&arrayOfHashes.step)+((/rollback_hashes/on_condition/failure/step)-(disabled_steps))", &["[1, 2, 4]"], true, None)]
// #[test_case("(disabled_steps)+(&arrayOfHashes.step)", &["[2, 3, 1, 2]"], true, None)]
// #[test_case("(&arrayOfHashes.step)+(disabled_steps)[1]", &["2"], true, None)]
// #[test_case("((&arrayOfHashes.step)[1])[0]", &["2"], true, None)]
// #[test_case("does.not.previously.exist[7]", &["Huzzah!"], false, Some("Huzzah!"))]
// #[test_case("/number_keys/1", &["one"], true, None)]
// #[test_case("**.[.^Hey]", &["Hey, Number Two!"], true, None)]
// #[test_case("/**/Hey*", &["Hey, Number Two!"], true, None; "/**/Hey")]
// #[test_case("lots_of_names.**.name", &["Name 1-1", "Name 2-1", "Name 3-1", "Name 4-1", "Name 4-2", "Name 4-3", "Name 4-4"], true, None)]
// #[test_case("/array_of_hashes/**", &["1", "one", "2", "two"], true, None)]
// #[test_case("products_hash.*[dimensions.weight==4].(availability.start.date)+(availability.stop.date)", &["2020-08-01", "2020-09-25", "2020-01-01", "2020-09-25"], true, None)]
// #[test_case("products_array[dimensions.weight==4].product", &["doohickey", "widget"], true, None)]
// #[test_case("(products_hash.*.dimensions.weight)[max()][parent(2)].dimensions.weight", &["10"], true, None)]
// #[test_case("/Locations/*/*", &["ny", "bstn"], true, None)]
// #[test_case("/AoH_Locations/*/*/*", &["nyc", "bo"], true, None)]
// #[test_case("/Weird_AoH_Locations/*/*/*", &["nyc", "bstn"], true, None)]
// #[test_case("/Set_Locations/*/*", &["New York", "Boston"], true, None)]
fn test_get_nodes(yamlpath: &str, results: &[&str], _mustexist: bool, _default: Option<&str>) -> Result<(), Box<dyn Error>> {
    let yamlpath = Path::new(yamlpath)?;

    let data = get_test_data()?;
    let first = &data[0];

    let mut count = 0;
    for (idx, node) in yamlpath.get_all(&first)?.into_iter().enumerate() {
        let mut node_as_str = String::new();
        let mut emitter = YamlEmitter::new(&mut node_as_str);
        emitter.dump(node)?;
        node_as_str.drain(0..4);
        assert_eq!(node_as_str, results[idx]);
        count += 1;
    }
    assert_eq!(count, results.len());
    Ok(())
}
