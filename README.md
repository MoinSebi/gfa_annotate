# gfa_annotate
Overlay gene annotation from a gff format to the genome graph. Input format will change to bed.   


## Input ([gff format](https://en.wikipedia.org/wiki/General_feature_format#:~:text=In%20bioinformatics%2C%20the%20general%20feature,DNA%2C%20RNA%20and%20protein%20sequences.)) 
| Col | Type      | Description                        |
|-----|-----------|------------------------------------|
| 1   | String    | seqid                              |
| 2   | String    | source                             |
| 3   | String    | type                               |
| 4   | String    | start                              |
| 4   | String    | end                                |
| 9   | String    | attributes (here are the gene ids) |

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