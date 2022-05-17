
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::process;
use clap::{App, AppSettings, Arg};
use gfaR_wrapper::NGfa;
use log::{error, info};
use crate::bed::{BedFile, out1, out_index};

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
            .long("output")
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


    if !Path::new(gfa).exists(){
        error!("No gfa file");
        process::exit(0x0100);

    }
    let mut bedfile = BedFile::new();
    if !Path::new(bed).exists(){
        error!("No bed file");

        process::exit(0x0100);
    } else {// Bed file
        info!("Read the gff/bed file");
        bedfile = BedFile::read_bed(bed, ',');
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
    writer_v2(u.1, u.0, matches.value_of("out").unwrap());

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
pub fn bed_intersection<'a>(graph: &'a NGfa, bed: & BedFile, path2pos: &'a HashMap<String, BTreeMap<u32, u32>>) -> (out1, out_index){
    let index = out_index::new(bed);
    let mut kk: out1 = out1::new(&index, &graph.nodes);
    //let mut k: HashMap<&'a u32, Vec<BTreeMap<String, String>>> = HashMap::new();



    for x in bed.data.iter() {
        //
        if path2pos.contains_key(x.0){
            for y in x.1{
                let mut op = path2pos.get(x.0).unwrap().range(y.start..y.end);


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


pub fn writer_v2(index: out_index, out11: out1, output: &str){
    let f = File::create(output).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in out11.hs.iter(){
        let k: Vec<String> = x.1.iter().map(|x|{
            let f = x.iter().cloned().collect::<Vec<String>>().join(",");
            return f
        }).into_iter().collect();
        let k2 = k.join("\t");

        write!(f, "{}\t{}\n", x.0, k2, );
    }
}

// /// Write a output file
// ///
// /// Nodes   KIND    GENE
// pub fn writer<'a>(input: &HashMap<&'a u32, Vec<BTreeMap<String, String>>>, output: &str){
//     let f = File::create(output).expect("Unable to create file");
//     let mut f = BufWriter::new(f);
//     for (k1,k2) in input.iter(){
//         let o: Vec<String> = k2.0.iter().cloned().collect();
//         let o2: Vec<String> = k2.1.iter().cloned().collect();
//         write!(f, "{}\t{}\t{}\n", k1, o.join(","), o2.join(",")).expect("Can not write file");
//     }
// }



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
