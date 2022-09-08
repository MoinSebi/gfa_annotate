
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::process;
use clap::{App, AppSettings, Arg};
use gfaR_wrapper::{NGfa, NNode};
use log::{error, info};
use crate::bed::{BedFile, Node2Feature};


pub mod bed;

fn main() {
    let matches = App::new("gfa_annotate")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Overlap annotation and genome graphs")
        .setting(AppSettings::ArgRequiredElseHelp)
        .help_heading("Input")
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
        .help_heading("Additional features")
        .arg(Arg::new("fraction")
                 .short('f')
                 .long("fraction")
                 .about("Report fraction of how much is covered"))
        .arg(Arg::new("length")
            .short('l')
            .long("length")
            .about("Report length in the table"))
        .help_heading("Output")
        .arg(Arg::new("output")
            .short('o')
            .required(true)
            .takes_value(true)
            .about("Output file"))
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
    let u = bed_intersection(& graph, bedfile, &gfa2pos_btree, matches.is_present("fraction"));
    writer_v2(u,  &graph.nodes, matches.value_of("output").unwrap(), len);

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
pub fn bed_intersection<'a>(graph: &'a NGfa, bed: BedFile, path2pos: &'a HashMap<String, BTreeMap<u32, u32>>, fraction: bool) -> Node2Feature{

    //let mut k: HashMap<&'a u32, Vec<BTreeMap<String, String>>> = HashMap::new();
    let mut result = Node2Feature::new(graph);

    for (name, data)  in bed.data.iter(){
        if path2pos.contains_key(name){
            let index = path2pos.get(name).unwrap();
            for entry in data{
                let interval: Vec<_> = index.range(entry.start+1..entry.end).collect();

                // This is the node coming after the end of the entry - this might also be covered
                let bigger = index.range(entry.end..).next().unwrap();


                // This is the specific case of you are within a node
                if interval.len() == 0{
                    let entry_len = entry.end - entry.start;
                    let to_bigger = entry_len as f64/ graph.nodes.get(bigger.1).unwrap().len as f64;
                    let mut tag = entry.tag.clone();
                    if fraction{
                        tag = tag + &";".to_string() + &to_bigger.to_string();
                    }
                    result.data.entry(*bigger.1).or_insert(HashSet::new()).insert(tag);


                } else {
                    let entry_len = (entry.end) - (interval.last().unwrap().0);
                    // Do this
                    if entry_len != 0{

                        let to_bigger = entry_len as f64/ graph.nodes.get(bigger.1).unwrap().len as f64;
                        let mut tag = entry.tag.clone();
                        if fraction{
                            tag  = tag + &";".to_string() + &format!("{:.1$}", to_bigger, 2);
                        }
                        result.data.entry(*bigger.1).or_insert(HashSet::new()).insert(tag);

                    }
                    let from_smallest = (interval.first().unwrap().0) - entry.start;
                    let to_smallest = from_smallest as f64/ graph.nodes.get(interval.first().unwrap().1).unwrap().len as f64;
                    let mut tag = entry.tag.clone() ;
                    if fraction{
                        tag = tag + ";" + &format!("{:.1$}", to_smallest, 2);
                    }
                    result.data.entry(*interval.first().unwrap().1).or_insert(HashSet::new()).insert(tag);




                }
                for hit in interval.iter().skip(1){
                    let mut tag = entry.tag.clone() ;
                    if fraction {
                        tag += &";1.00".to_string();
                    }
                    result.data.entry(*hit.1).or_insert(HashSet::new()).insert(tag);
                }
            }
        }
    }

    return result
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
pub fn writer_v2(data: Node2Feature, nodes: &HashMap<u32, NNode>, output: &str, len: bool){
    let f = File::create(output).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    if len{
        for (node_id, feature) in data.data.iter(){
            write!(f, "{}\t{}\t{}\n", node_id, nodes.get(node_id).unwrap().len, feature.into_iter().cloned().collect::<Vec<String>>().join(",")).expect("Can't write file");

        }
    } else {
        for (node_id, feature) in data.data.iter(){
            write!(f, "{}\t{}\n", node_id, feature.into_iter().cloned().collect::<Vec<String>>().join(",")).expect("Can't write file");
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
        btree.insert(position+1, 0);
        result.insert(path.name.clone(), btree);
    }

    return result
}
