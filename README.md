# gfa_annotate
Overlay gene annotation from a bed format to the genome graph.   

If you want to convert gff to bed file, use the gff2bed.py script (s. below)


## **Example input BED format** (tab-separated):  

| TAIR10 | 100 | 200 | ID=1;T=gene;G=AT3G43160;C=Biosynthesis |
|--------|-----|-----|----------------------------------------|


## Output 

| Col | Type         | Description |
|-----|--------------|-------------|
| 1   | int          | Node id     |
| 2   | String list  | Type        |
| 3   | String list  | TAGs        |


## gff2bed.py information    

Requirements: 
- Python3
- argparse
- logging

##TODO
- [x] Add a column for the fraction covered   
- [x] Script: Include 1- to 0- based switch
- [ ] More tests, not working
- [ ] README stuff, easy explain
- [ ] Check how much fraction in real data set 
- [ ] Think about multiple entries having the same gene
