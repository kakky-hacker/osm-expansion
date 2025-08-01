extern crate osmpbfreader;
use std::collections::HashMap;

fn main() {
    let filename = "./pbf/kanto-latest.osm.pbf";
    let path = std::path::Path::new(filename);
    let r = std::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);

    type TagMap = HashMap<String, Vec<String>>;
    let mut node_tags = TagMap::new();
    let mut way_tags = TagMap::new();
    let mut relation_tags = TagMap::new();

    for obj in pbf.par_iter().map(Result::unwrap) {
        match obj {
            osmpbfreader::OsmObj::Node(node) => {
                for (k, v) in node.tags.iter() {
                    (*node_tags.entry(k.to_string()).or_insert(vec![])).push(v.to_string());
                }
            }
            osmpbfreader::OsmObj::Way(way) => {
                for (k, v) in way.tags.iter() {
                    (*way_tags.entry(k.to_string()).or_insert(vec![])).push(v.to_string());
                }
            }
            osmpbfreader::OsmObj::Relation(rel) => {
                for (k, v) in rel.tags.iter() {
                    (*relation_tags.entry(k.to_string()).or_insert(vec![])).push(v.to_string());
                }
            }
        }
    }

    fn select_tags(tags: &TagMap, max_items: usize) -> Vec<(String, usize)> {
        let mut filtered_tags = tags
            .iter()
            .filter_map(|(k, v)| {
                if v.len() > max_items {
                    Some((k.clone(), v.len()))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, usize)>>();
        filtered_tags.sort_by(|(_k1, v1), (_k2, v2)| v2.cmp(v1));
        filtered_tags
    }

    let max_items = 3000;

    println!("--- node tags ---");
    println!("{:?}", select_tags(&node_tags, max_items));
    println!("");

    println!("--- way tags ---");
    println!("{:?}", select_tags(&way_tags, max_items));
    println!("");

    println!("--- relation tags ---");
    println!("{:?}", select_tags(&relation_tags, max_items));
    println!("");
}
