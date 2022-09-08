# gfa_annotate
Overlay gene annotation from a bed format to the genome graph. The bed format has a maximum of four columns 

If you want to convert gff to bed file, use the gff2bed.py script (s. below). 


## **Example input BED format** (tab-separated):  

| TAIR10 | 100 | 200 | gene |
|--------|-----|-----|------|
**OR**

| TAIR10 | 250 | 350 | AT1234 |
|--------|-----|-----|--------|

## Output 

| Col | Type        | Description  |
|-----|-------------|--------------|
| 1   | int         | Node id      |
| 2*  | String list | Type         |
| 3   | int         | Length (opt) |

*If the fraction parameter is set, the fraction value in added after the entry. If the same entry has two different fractions, entry is represented twice with different fractions. Must be here because it is specific to the annotation and not the node.




## gff2bed.py information    

**Requirements:** 
- Python3
- argparse
- logging

##TODO
- [x] Add a column for the fraction covered   
- [x] Script: Include 1- to 0- based switch
- [ ] More tests, not working
- [ ] README stuff, easy explain
- [ ] Check how much fraction in real data set 
- [x] Simplify pipeline to have only one column
- [ ] Add python script to covert from gff to bed
- [ ] Fraction and length
