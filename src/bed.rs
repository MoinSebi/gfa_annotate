use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use gfaR_wrapper::NNode;
use linked_hash_set::LinkedHashSet;
use log::info;

pub struct BedFile {
    pub size: usize,
    pub data: HashMap<String, Vec<BedEntry>>
}

/// Bed entry
/// chrom (or scaffold) name are stored in the BedFile
pub struct BedEntry {
    pub start: u32,
    pub end: u32,
    pub tag: BTreeMap<String, String>,
}




impl BedFile {

    pub fn new() -> Self{
        Self{
            size: 0,
            data: HashMap::new(),
        }
    }


    /// Reading a bed file
    /// Format description in the README
    pub fn read_bed(filename: &str, del: char) -> Self{
        info!("Reading bed file");
        let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
        let reader = BufReader::new(file);

        // Fasta_entry -> Vec<Bed_Entry>
        let mut result = HashMap::new();
        for line in reader.lines() {
            let l = line.unwrap();
            //let p: Vec<&str> = l.split("\t").collect();
            let mut p2: Vec<&str> = l.split("\t").collect();
            //let ko3: HashSet<String> = p2.nth(4).unwrap().split(del).map(|s| s.to_string()).collect();
            let mut p3: BTreeMap<String, String>= BTreeMap::new();
            if p2.len() > 3{

                let mut p4: Vec<Vec<&str>>= p2[2].split(";").map(|x | x.split("=").collect()).collect();
                p3 = p2[2].split(";").map(|x | {
                    let o: Vec<&str> = x.split("=").collect();
                    let p = (o[0].to_string(), o[1].to_string());
                    return p
                }).into_iter().collect();
            }

            result.entry(p2[0].to_string()).or_insert(Vec::new()).push(BedEntry{start: p2[1].parse().unwrap(), end: p2[2].parse().unwrap(), tag: p3})

            //let ko3: HashSet<String> = p[4].split(del).map(|s| s.to_string()).collect();

            //result.entry(p2.nth(0).unwrap().to_string()).or_insert(Vec::new()).push(BedEntry { start: p2.nth(1).unwrap().parse().unwrap(), end: p2.nth(2).unwrap().parse().unwrap(), kind: p2.nth(3).unwrap().to_string(), gene: HashSet::new() });
        }
        // let mut count = 0;
        // for (_k,v) in result.iter(){
        //     count += v.len();
        // }



        Self{
            size: get_size(&result),
            data: result,

        }
    }

}


pub struct out1{
    pub hs: HashMap<u32, Vec<LinkedHashSet<String>>>,
}


impl out1 {
    pub fn new(index: &out_index,nnodes:  &HashMap<u32, NNode>) -> Self {

        let mut g2: HashMap<u32, Vec<LinkedHashSet<String>>>  = nnodes.iter().map(|x| {
            let g3: Vec<LinkedHashSet<String>> = index.tags.iter().map(|x| LinkedHashSet::new()).collect();
            let gg = (x.0.clone(), g3);
            return gg
        }).collect();
        Self{
            hs: g2,
        }
    }
}

pub struct out_index{
    pub tags: HashMap<String, usize>,
}

impl out_index {
    pub fn new(bedfile: &BedFile) -> Self{
        let mut index = HashSet::new();

        for (name, data) in bedfile.data.iter(){
            for entry in data{
                for (key, value) in entry.tag.iter(){
                    index.insert(key);
                }
            }
        }

        let mut h: Vec<String> = index.into_iter().cloned().collect();
        h.sort();
        println!("{:?}", h);
        let g: HashMap<String, usize> = h.iter().enumerate().map(|(key, value)| {
            return (value.clone(), key.clone());
        }).collect();

        Self{
            tags: g

        }
    }
}

impl out1 {

}


pub fn get_size(data: &HashMap<String, Vec<BedEntry>>) -> usize{
    let mut count = 0;
    for (_name, entry) in data.iter(){
        count += entry.len();
    }
    return count;
}







#[cfg(test)]
mod tests {
    use crate::bed;
    use std::time::{Duration, Instant};

    #[test]
    fn it_works() {
        let start = Instant::now();

        let g = bed::BedFile::read_gff("9888.gff");
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);

    }
}

