
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::process;
use clap::{App, AppSettings, Arg};
use gfaR_wrapper::{NGfa, NNode};
use log::{error, info};
use crate::bed::{BedFile, Node2Feature, TagIndex};

pub mod bed;

fn main() {
    let matches = App::new("gfa_annotate")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Overlap annotation and genome graph")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("gfa")
            .short('g')
            .required(true)
            .about("input gfa")
            .takes_value(true))
        .arg(Arg::new("bed")
            .short('b')
            .about("bed file")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .required(true)
            .takes_value(true)
            .about("Output file"))
        .arg(Arg::new("length")
            .short('l')
            .long("length")
            .about("Report length in the table"))
        .get_matches();


    let gfa = matches.value_of("gfa").unwrap();
    let bed = matches.value_of("bed").unwrap();
    let len = matches.is_present("length");


    if !Path::new(gfa).exists(){
        error!("No gfa file");
        process::exit(0x0100);

    }
    let bedfile;
    if !Path::new(bed).exists(){
        error!("No bed file");

        process::exit(0x0100);
    } else {// Bed file
        info!("Read the gff/bed file");
        bedfile = BedFile::read_bed(bed);
    }


    // Running the graph
    info!("Read the gfa file");
    let mut graph = NGfa::new();
    graph.from_graph(gfa);
    let gfa2pos_btree = node2pos(&graph);
    // Bed file
    info!("Read the gff/bed file");

    // For each genome
    let u = bed_intersection(& graph, &bedfile, &gfa2pos_btree);
    writer_v2(u.1, u.0, &graph.nodes, matches.value_of("out").unwrap(), len);

}

pub fn node2length(nodes: &HashMap<u32, NNode>) -> HashMap<u32, usize>{
    let node_length: HashMap<u32, usize> = nodes.iter().map(|x| (x.0.clone(), x.1.len)).collect();
    return node_length
}

/// Intersecting the bed file and with positions in the graph
///
/// # Arguments:
/// * 'graph': NGfa data structure
/// * 'path2pos': {genome_id -> {pos -> node_id}}
///
/// # Output
/// - 'node2data'
///     - {u32 -> {u32 -> u32
pub fn bed_intersection<'a>(graph: &'a NGfa, bed: & BedFile, path2pos: &'a HashMap<String, BTreeMap<u32, u32>>) -> (Node2Feature, TagIndex){
    let index = TagIndex::new(bed);
    let mut kk: Node2Feature = Node2Feature::from_nodes(&index, &graph.nodes);
    //let mut k: HashMap<&'a u32, Vec<BTreeMap<String, String>>> = HashMap::new();



    for x in bed.data.iter() {
        //
        if path2pos.contains_key(x.0){
            for y in x.1{
                let op = path2pos.get(x.0).unwrap().range(y.start..y.end);


                for ö in op{
                    for jo in x.1.iter(){
                        for (k,v) in jo.tag.iter(){
                            for x in v.iter(){
                                kk.hs.get_mut(ö.1).unwrap()[index.tags[k]].insert_if_absent(x.clone());
                            }
                        }
                    }

                }
            }
        }
    }
    return (kk, index)
}

/// Writing output
///
/// **Comment**
/// Tabular output format
/// Node Tag1 Tag2 Tag3
///
/// **Arguments**
/// * index: Index structure for column name
/// * data: Containing node_id + tags
pub fn writer_v2(index: TagIndex, data: Node2Feature, nodes: &HashMap<u32, NNode>, output: &str, len: bool){
    let file = File::create(output).expect("Unable to create file");
    let mut file = BufWriter::new(file);

    // Index
    if len{
        let tags_name: String = index.tags.iter().map(|x| x.0.clone()).collect::<Vec<String>>().join(",");
        write!(file, "{}\t{}\t{}\n", "node", "length", tags_name).expect("Can not write output");
    } else {
        let tags_name: String = index.tags.iter().map(|x| x.0.clone()).collect::<Vec<String>>().join(",");
        write!(file, "{}\t{}\t{}\n", "node", "length", tags_name).expect("Can not write output");
    }


    for (key, value) in data.hs.iter(){
        let tags_vector: Vec<String> = value.iter().map(|x|{
            let f = x.iter().cloned().collect::<Vec<String>>().join(",");
            return f
        }).into_iter().collect();
        let tags = tags_vector.join("\t");

        if len{
            write!(file, "{}\t{}\t{}\n", key, nodes[key].len, tags).expect("Can not write output");

        }
       else{
           write!(file, "{}\t{}\n", key, tags).expect("Can not write output");
       }

    }
}




/// Position to node for each genome in the graph
///
/// # Arguments
/// * 'graph' - A NGfa data structure
///
/// # Output
/// {Genome_id ->  BtreeMap(position -> node)}
///
pub fn node2pos(graph: &NGfa) -> HashMap<String, BTreeMap<u32, u32>>{
    let mut result: HashMap<String, BTreeMap<u32, u32>> = HashMap::new();

    for path in graph.paths.iter(){
        let mut btree = BTreeMap::new();
        let mut position = 0;
        for node in path.nodes.iter(){
            // {"End"-position of the node -> node_id}
            btree.insert(position + graph.nodes.get(node).unwrap().len as u32, node.clone());
            // Update position
            position += graph.nodes.get(node).unwrap().len as u32
        }
        // Add btree to corresponding path
        result.insert(path.name.clone(), btree);
    }
    return result
}
