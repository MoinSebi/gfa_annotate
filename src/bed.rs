use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct BedFile {
    pub size: usize,
    pub jojo: HashMap<String, Vec<BedEntry>>
}


pub struct BedEntry {
    pub start: u32,
    pub end: u32,
    pub kind: String,
    pub gene: HashSet<String>,
}

impl BedFile {
    /// Read a bed file from file
    pub fn read_gff(filename: &str) -> Self{
        let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
        let reader = BufReader::new(file);

        let mut hhiohi = HashMap::new();
        for (_i, line) in reader.lines().enumerate() {
            let l = line.unwrap();
            let p: Vec<&str> = l.split("\t").collect();
            if p.len() > 4 {
                if p[2] == "gene"{
                    let ko: Vec<&str> = p[8].split(";").collect();
                    if ko.len() > 3{
                        let ko2: Vec<&str> = ko[3].split("=").collect();
                        let ko3: HashSet<String> = ko2.last().unwrap().split(",").map(|s| s.to_string()).collect();

                        let bb = BedEntry {start: p[3].parse().unwrap(), end: p[4].parse().unwrap(), kind: p[2].to_string(), gene: ko3};

                        hhiohi.entry(p[0].to_string()).or_insert(Vec::new()).push(bb);
                    } else {
                        let bb = BedEntry {start: p[3].parse().unwrap(), end: p[4].parse().unwrap(), kind: p[2].to_string(), gene: HashSet::new()};
                        hhiohi.entry(p[0].to_string()).or_insert(Vec::new()).push(bb);
                    }
                } else {
                    let bb = BedEntry {start: p[3].parse().unwrap(), end: p[4].parse().unwrap(), kind: p[2].to_string(), gene: HashSet::new()};
                    hhiohi.entry(p[0].to_string()).or_insert(Vec::new()).push(bb);
                }
            }
        }

        Self{
            size: 10,
            jojo: hhiohi,

        }
    }


    pub fn read_bed(filename: &str, del: char) -> Self{
        let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
        let reader = BufReader::new(file);

        // Fasta_entry -> Vec<Bed_Entry>
        let mut result = HashMap::new();
        for (_i, line) in reader.lines().enumerate() {
            let l = line.unwrap();
            let p: Vec<&str> = l.split("\t").collect();
            let ko3: HashSet<String> = p[4].split(del).map(|s| s.to_string()).collect();

            let bb = BedEntry { start: p[1].parse().unwrap(), end: p[2].parse().unwrap(), kind: p[3].to_string(), gene: ko3 };
            result.entry(p[0].to_string()).or_insert(Vec::new()).push(bb);
        }
        let mut count = 0;
        for (_k,v) in result.iter(){
            count += v.len();
        }

        Self{
            size: count,
            jojo: result,

        }
    }

}


