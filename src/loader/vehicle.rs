use super::BaseLoader;
use crate::entity::{Joint, Link};
use osmpbfreader::{NodeId, OsmObj, OsmPbfReader, Way};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

pub struct LoaderForVehicle {
    links: Vec<Link>,
}

impl LoaderForVehicle {
    pub fn new() -> Self {
        LoaderForVehicle { links: vec![] }
    }
}

impl BaseLoader for LoaderForVehicle {
    fn load(&mut self, path: &Path) {
        let mut pbf = OsmPbfReader::new(File::open(path).unwrap());
        let target_highways = ["motorway", "trunk", "primary", "secondary", "tertiary"];

        let mut joints: HashMap<NodeId, Rc<Joint>> = HashMap::new();
        let mut way_buffer: Vec<Way> = Vec::new();
        let mut needed_node_ids: HashSet<NodeId> = HashSet::new();
        for obj in pbf.par_iter().map(Result::unwrap) {
            match obj {
                OsmObj::Way(way) => {
                    if let Some(highway) = way.tags.get("highway") {
                        if target_highways.contains(&highway.as_str()) {
                            for node_id in &way.nodes {
                                needed_node_ids.insert(*node_id);
                            }
                            way_buffer.push(way);
                        }
                    }
                }
                _ => {}
            }
        }

        for obj in pbf.par_iter().map(Result::unwrap) {
            match obj {
                OsmObj::Node(node) => {
                    if needed_node_ids.contains(&node.id) {
                        joints.insert(node.id, Rc::new(Joint::new(node.lat(), node.lon())));
                    }
                }
                _ => {}
            }
        }

        for way in way_buffer {
            self.links.push(Link::new(
                way.tags.get("lanes").and_then(|s| {
                    let s: &str = s.as_str();
                    s.parse().ok()
                }),
                way.tags.get("width").and_then(|s| {
                    let s: &str = s.as_str();
                    s.parse().ok()
                }),
                way.nodes
                    .iter()
                    .filter_map(|node_id| joints.get(node_id).cloned())
                    .collect(),
            ));
        }

        println!("loaded links: {}", self.links.len());
    }
}
