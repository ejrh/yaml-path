use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

use test_case::test_case;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use yaml_path::{Path, PathError, Processor};

fn get_test_data() -> Result<Vec<Yaml>, Box<dyn Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let mut path = PathBuf::from(manifest_dir);
    path.push("tests/test_processor.yaml");
    let yaml_str = read_to_string(path)?;
    Ok(YamlLoader::load_from_str(&yaml_str)?)
}

#[test_case("/number_keys/1", &[&"one"], true, None)]
    //     ("aliases[&aliasAnchorOne]", ["Anchored Scalar Value"], True, None),
    //     ("aliases[&newAlias]", ["Not in the original data"], False, "Not in the original data"),
    //     ("aliases[0]", ["Anchored Scalar Value"], True, None),
    //     ("aliases[0]", ["Anchored Scalar Value"], True, None),
    //     ("aliases[0]", ["Anchored Scalar Value"], True, None),
    //     ("aliases[0]", ["Anchored Scalar Value"], True, None),
    //     ("aliases.0", ["Anchored Scalar Value"], True, None),
    //     ("(array_of_hashes.name)+(rollback_hashes.on_condition.failure.name)", [["one", "two", "three", "four"]], True, None),
    //     ("/array_of_hashes/name", ["one", "two"], True, None),
    //     ("aliases[1:2]", [["Hey, Number Two!"]], True, None),
    //     ("aliases[1:1]", [["Hey, Number Two!"]], True, None),
    //     ("squads[bravo:charlie]", [2.2, 3.3], True, None),
    //     ("/&arrayOfHashes/1/step", [2], True, None),
    //     ("&arrayOfHashes[step=1].name", ["one"], True, None),
    //     ("squads[.!=""][.=1.1]", [1.1], True, None),
    //     ("squads[.!=""][.>1.1][.<3.3]", [2.2], True, None),
    //     ("aliases[.^Hey]", ["Hey, Number Two!"], True, None),
    //     ("aliases[.$Value]", ["Anchored Scalar Value"], True, None),
    //     ("aliases[.%Value]", ["Anchored Scalar Value"], True, None),
    //     ("&arrayOfHashes[step>1].name", ["two"], True, None),
    //     ("&arrayOfHashes[step<2].name", ["one"], True, None),
    //     ("squads[.>charlie]", [4.4], True, None),
    //     ("squads[.>=charlie]", [3.3, 4.4], True, None),
    //     ("squads[.<bravo]", [1.1], True, None),
    //     ("squads[.<=bravo]", [1.1, 2.2], True, None),
    //     (r"squads[.=~/^\w{6,}$/]", [3.3], True, None),
    //     ("squads[alpha=1.1]", [1.1], True, None),
    //     ("(&arrayOfHashes.step)+(/rollback_hashes/on_condition/failure/step)-(disabled_steps)", [[1, 4]], True, None),
    //     ("(&arrayOfHashes.step)+((/rollback_hashes/on_condition/failure/step)-(disabled_steps))", [[1, 2, 4]], True, None),
    //     ("(disabled_steps)+(&arrayOfHashes.step)", [[2, 3, 1, 2]], True, None),
    //     ("(&arrayOfHashes.step)+(disabled_steps)[1]", [2], True, None),
    //     ("((&arrayOfHashes.step)[1])[0]", [2], True, None),
    //     ("does.not.previously.exist[7]", ["Huzzah!"], False, "Huzzah!"),
    //     ("/number_keys/1", ["one"], True, None),
    //     ("**.[.^Hey]", ["Hey, Number Two!"], True, None),
    //     ("/**/Hey*", ["Hey, Number Two!"], True, None),
    //     ("lots_of_names.**.name", ["Name 1-1", "Name 2-1", "Name 3-1", "Name 4-1", "Name 4-2", "Name 4-3", "Name 4-4"], True, None),
    //     ("/array_of_hashes/**", [1, "one", 2, "two"], True, None),
    //     ("products_hash.*[dimensions.weight==4].(availability.start.date)+(availability.stop.date)", [[AnchoredDate(2020, 8, 1), AnchoredDate(2020, 9, 25)], [AnchoredDate(2020, 1, 1), AnchoredDate(2020, 1, 1)]], True, None),
    //     ("products_array[dimensions.weight==4].product", ["doohickey", "widget"], True, None),
    //     ("(products_hash.*.dimensions.weight)[max()][parent(2)].dimensions.weight", [10], True, None),
    //     ("/Locations/*/*", ["ny", "bstn"], True, None),
    //     ("/AoH_Locations/*/*/*", ["nyc", "bo"], True, None),
    //     ("/Weird_AoH_Locations/*/*/*", ["nyc", "bstn"], True, None),
    //     ("/Set_Locations/*/*", ["New York", "Boston"], True, None),
fn test_get_nodes(yamlpath: &str, results: &[&str], mustexist: bool, default: Option<&str>) -> Result<(), Box<dyn Error>> {
    let yamlpath = Path::new(&yamlpath)?;

    let data = get_test_data()?;
    let first = &data[0];

    let processor = Processor::new(&first);
    let mut count = 0;
    for (idx, node) in processor.get_all(&yamlpath)?.into_iter().enumerate() {
        let mut node_as_str = String::new();
        let mut emitter = YamlEmitter::new(&mut node_as_str);
        emitter.dump(node)?;
        node_as_str.drain(0..4);
        assert_eq!(node_as_str, results[idx]);
        count += 1;
    }
    assert_eq!(count, results.len());
    Ok(())

        // yaml = YAML()
        // processor = Processor(quiet_logger, yaml.load(YAMLDATA))
        // matchidx = 0
        // for node in processor.get_nodes(
        //         yamlpath, mustexist=mustexist, default_value=default
        // ):
        //     assert unwrap_node_coords(node) == results[matchidx]
        //     matchidx += 1
        // assert len(results) == matchidx
}



    // @pytest.mark.parametrize("yamlpath,results,mustexist,default", [
    //     ("aliases[&aliasAnchorOne]", ["Anchored Scalar Value"], True, None),
    //     ("aliases[&newAlias]", ["Not in the original data"], False, "Not in the original data"),
    //     ("aliases[0]", ["Anchored Scalar Value"], True, None),
    //     ("aliases.0", ["Anchored Scalar Value"], True, None),
    //     ("(array_of_hashes.name)+(rollback_hashes.on_condition.failure.name)", [["one", "two", "three", "four"]], True, None),
    //     ("/array_of_hashes/name", ["one", "two"], True, None),
    //     ("aliases[1:2]", [["Hey, Number Two!"]], True, None),
    //     ("aliases[1:1]", [["Hey, Number Two!"]], True, None),
    //     ("squads[bravo:charlie]", [2.2, 3.3], True, None),
    //     ("/&arrayOfHashes/1/step", [2], True, None),
    //     ("&arrayOfHashes[step=1].name", ["one"], True, None),
    //     ("squads[.!=""][.=1.1]", [1.1], True, None),
    //     ("squads[.!=""][.>1.1][.<3.3]", [2.2], True, None),
    //     ("aliases[.^Hey]", ["Hey, Number Two!"], True, None),
    //     ("aliases[.$Value]", ["Anchored Scalar Value"], True, None),
    //     ("aliases[.%Value]", ["Anchored Scalar Value"], True, None),
    //     ("&arrayOfHashes[step>1].name", ["two"], True, None),
    //     ("&arrayOfHashes[step<2].name", ["one"], True, None),
    //     ("squads[.>charlie]", [4.4], True, None),
    //     ("squads[.>=charlie]", [3.3, 4.4], True, None),
    //     ("squads[.<bravo]", [1.1], True, None),
    //     ("squads[.<=bravo]", [1.1, 2.2], True, None),
    //     (r"squads[.=~/^\w{6,}$/]", [3.3], True, None),
    //     ("squads[alpha=1.1]", [1.1], True, None),
    //     ("(&arrayOfHashes.step)+(/rollback_hashes/on_condition/failure/step)-(disabled_steps)", [[1, 4]], True, None),
    //     ("(&arrayOfHashes.step)+((/rollback_hashes/on_condition/failure/step)-(disabled_steps))", [[1, 2, 4]], True, None),
    //     ("(disabled_steps)+(&arrayOfHashes.step)", [[2, 3, 1, 2]], True, None),
    //     ("(&arrayOfHashes.step)+(disabled_steps)[1]", [2], True, None),
    //     ("((&arrayOfHashes.step)[1])[0]", [2], True, None),
    //     ("does.not.previously.exist[7]", ["Huzzah!"], False, "Huzzah!"),
    //     ("/number_keys/1", ["one"], True, None),
    //     ("**.[.^Hey]", ["Hey, Number Two!"], True, None),
    //     ("/**/Hey*", ["Hey, Number Two!"], True, None),
    //     ("lots_of_names.**.name", ["Name 1-1", "Name 2-1", "Name 3-1", "Name 4-1", "Name 4-2", "Name 4-3", "Name 4-4"], True, None),
    //     ("/array_of_hashes/**", [1, "one", 2, "two"], True, None),
    //     ("products_hash.*[dimensions.weight==4].(availability.start.date)+(availability.stop.date)", [[AnchoredDate(2020, 8, 1), AnchoredDate(2020, 9, 25)], [AnchoredDate(2020, 1, 1), AnchoredDate(2020, 1, 1)]], True, None),
    //     ("products_array[dimensions.weight==4].product", ["doohickey", "widget"], True, None),
    //     ("(products_hash.*.dimensions.weight)[max()][parent(2)].dimensions.weight", [10], True, None),
    //     ("/Locations/*/*", ["ny", "bstn"], True, None),
    //     ("/AoH_Locations/*/*/*", ["nyc", "bo"], True, None),
    //     ("/Weird_AoH_Locations/*/*/*", ["nyc", "bstn"], True, None),
    //     ("/Set_Locations/*/*", ["New York", "Boston"], True, None),
    // ])
