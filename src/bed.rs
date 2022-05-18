use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use gfaR_wrapper::NNode;
use linked_hash_set::LinkedHashSet;
use log::{info};

pub struct BedFile {
    pub size: usize,
    pub data: HashMap<String, Vec<BedEntry>>
}

/// Bed entry
/// chrom (or scaffold) name are stored in the BedFile
pub struct BedEntry {
    pub start: u32,
    pub end: u32,
    pub tag: BTreeMap<String, Vec<String>>, // Tag->Entries
}




impl BedFile {

    pub fn new() -> Self{
        Self{
            size: 0,
            data: HashMap::new(),
        }
    }


    /// Reading a bed file
    ///
    /// **Comment**
    /// Description of the BED file format is written in README
    pub fn read_bed(filename: &str) -> Self{
        info!("Reading BED file");
        let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
        let reader = BufReader::new(file);

        // Fasta_entry -> Vec<Bed_Entry>
        let mut result = HashMap::new();
        for line in reader.lines() {
            let line_data = line.unwrap();
            //let p: Vec<&str> = l.split("\t").collect();
            let col: Vec<&str> = line_data.split("\t").collect();
            //let ko3: HashSet<String> = p2.nth(4).unwrap().split(del).map(|s| s.to_string()).collect();
            // If you are not empty
            if col.len() > 3{

                let tag_data: BTreeMap<String, Vec<String>> =  col[3].split(";").map(|x | {
                    let split_tags: Vec<&str> = x.split("=").collect();
                    let split_entries: Vec<String> = split_tags[1].split(",").map(|x| x.to_string()).collect();

                    return (split_tags[0].to_string(), split_entries);
                }).into_iter().collect();
                result.entry(col[0].to_string()).or_insert(Vec::new()).push(BedEntry{start: col[1].parse().unwrap(), end: col[2].parse().unwrap(), tag: tag_data })

            }


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

/// Node to feature
///
/// **Comment**
/// This is the resulting data
pub struct Node2Feature {
    pub hs: HashMap<u32, Vec<LinkedHashSet<String>>>,
}


impl Node2Feature {

    pub fn from_nodes(index: &TagIndex, nnodes:  &HashMap<u32, NNode>) -> Self {

        let g2: HashMap<u32, Vec<LinkedHashSet<String>>>  = nnodes.iter().map(|x| {
            let g3: Vec<LinkedHashSet<String>>  = vec![LinkedHashSet::new(); index.tags.len()];
            let gg = (x.0.clone(), g3);
            return gg
        }).collect();
        Self{
            hs: g2,
        }
    }
}

pub struct TagIndex {
    pub tags: HashMap<String, usize>,
}

impl TagIndex {
    pub fn new(bedfile: &BedFile) -> Self{
        let mut index = HashSet::new();

        // Make index
        for (_name, data) in bedfile.data.iter(){
            for entry in data{
                for (key, _value) in entry.tag.iter(){
                    index.insert(key);
                }
            }
        }

        // Sort the index
        let mut sorted: Vec<String> = index.into_iter().cloned().collect();
        sorted.sort();
        println!("{:?}", sorted);
        let tag2pos: HashMap<String, usize> = sorted.iter().enumerate().map(|(key, value)| {
            return (value.clone(), key.clone());
        }).collect();

        Self{
            tags: tag2pos
        }
    }
}


/// Get the total size of the HashMap
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

