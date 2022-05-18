# gfa_annotate
Overlay gene annotation from a gff format to the genome graph. Input format will change to bed.   


## Input (bed format)
| Col | Type   | Description |
|-----|--------|-------------|
| 1   | String | seqid       |
| 2   | int    | start       |
| 3   | int    | stop        |
| 4   | String | Tags        |  


####Tags (examples, semi-colon separated)
- ID=Custom_identifier (1,2,3)
- T=Type (Gene, mRNA)
- G=Gene_name
- OG=Orthogroup
- More tags can be added with "=" (key, value pair)

**Example** (tab-separated):  

| TAIR10 | 100 | 200 | ID=1;T=gene;G=AT3G43160;C=Biosynthesis |
|--------|-----|-----|----------------------------------------|


## Output 

| Col | Type         | Description |
|-----|--------------|-------------|
| 1   | int          | Node id     |
| 2   | String list  | Type        |
| 3   | String list  | Genes       |


##TODO
- [ ] Add a column for the fraction covered   
- [ ] Add guide on how to get GO terms
- [ ] Tests 
- [ ] Script: Include 1- to 0- based switch 