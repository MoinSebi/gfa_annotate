# gfa_annotate
Overlay gene annotation from a gff format to the genome graph. Input format will change to bed.   


## Input (bed format)
| Col | Type   | Description |
|-----|--------|-------------|
| 1   | String | seqid       |
| 2   | int    | start       |
| 3   | int    | stop        |
| 4   | String | Tags        |  


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
- [x] Add a column for the fraction covered   
- [x] Script: Include 1- to 0- based switch
- [x] Add dummy start and end with not function (0 and max+1)
- [ ] More tests, not working
- [ ] README stuff, easy explain
- [ ] Check how much fraction in real data set 
- [ ] Think about multiple entries having the same gene
