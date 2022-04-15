
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::process;
use clap::{App, AppSettings, Arg};
use gfaR_wrapper::NGfa;
use log::{error, info};
use crate::bed::{BedFile};

mod bed;

fn main() {
    let matches = App::new("gfa_annotate")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Overlay annotation to a genome graph")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("gfa")
            .short('g')
            .required(true)
            .about("input gfa")
            .takes_value(true))
        .arg(Arg::new("bed")
            .short('b')
            .required(true)
            .about("bed file")
            .takes_value(true))
        .arg(Arg::new("gff")
            .long("gff")
            .takes_value(true)
            .about("GFF input"))
        .arg(Arg::new("delimiter")
            .about("Delimiter between features (in bed file)")
            .takes_value(true)
            .long("delimiter"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .required(true)
            .takes_value(true)
            .about("Output file"))
        .get_matches();

    let gfa = matches.value_of("gfa").unwrap();
    let bed = matches.value_of("bed").unwrap();
    let gff ;
    if matches.is_present("gff"){
        gff = matches.value_of("gff").unwrap();
    } else {
        gff = "nothing";
    }

    if !Path::new(gfa).exists(){
        error!("No gfa file");
        process::exit(0x0100);

    }
    if !Path::new(bed).exists(){
        error!("No bed file");
        process::exit(0x0100);

    } else if !Path::new(gff).exists() {
        process::exit(0x0100);
    } else {
        let _bed = BedFile::read_bed(bed, ',');

    }


    // Running the graph
    info!("Read the gfa file");
    let mut graph = NGfa::new();
    graph.from_graph(gfa);
    let gfa2pos_btree = test(&graph);

    // Bed file
    info!("Read the gff/bed file");
    let bed = BedFile::read_gff(bed);

    // For each genome
    let mut k: HashMap<&u32, (HashSet<String>, HashSet<String>)> = HashMap::new();
    for x in graph.nodes.iter(){
        k.insert(x.0, (HashSet::new(), HashSet::new()));
    }

    for x in bed.jojo.iter() {
        if gfa2pos_btree.contains_key(x.0){
            for y in x.1{
                for ö in gfa2pos_btree.get(x.0).unwrap().range(y.start..y.end){
                    k.get_mut(ö.1).unwrap().0.insert(y.kind.clone());
                    k.get_mut(ö.1).unwrap().1.extend(y.gene.clone())
                }
            }
        }
    }
    writer(&k, matches.value_of("output").unwrap());




}


/// Write a output file
///
/// Nodes   KIND    GENE
pub fn writer(input: &HashMap<&u32, (HashSet<String>, HashSet<String>)>, output: &str){
    let f = File::create(output).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (k1,k2) in input.iter(){
        let o: Vec<String> = k2.0.iter().cloned().collect();
        let o2: Vec<String> = k2.1.iter().cloned().collect();
        write!(f, "{}\t{}\t{}\n", k1, o.join(","), o2.join(",")).expect("Can not write file");
    }
}

pub fn test(graph: &NGfa) -> HashMap<String, BTreeMap<u32, u32>>{
    let mut  hs: HashMap<String, BTreeMap<u32, u32>> = HashMap::new();

    for x in graph.paths.iter(){
        let mut h = BTreeMap::new();
        let mut lenn = 0;
        for y in x.nodes.iter(){
            h.insert(lenn + graph.nodes.get(y).unwrap().len as u32, y.clone());
            lenn += graph.nodes.get(y).unwrap().len as u32
        }
        hs.insert(x.name.clone(), h);
    }
    return hs
}
