"""
This is not part of the tool and was only added for my own project.
SV


"""


import argparse
import logging


def readgff(filename):
    data = []
    with open(filename) as file:
        for line in file.readlines():
            lsplit = line.split()
            if len(lsplit) > 7:
                tags = dict([(x.split("=")[0], x.split("=")[1].split(",")) for x in lsplit[8].split(";")])


                chr = lsplit[0]
                t = lsplit[2]
                start = lsplit[3]
                end = lsplit[4]
                tagg = lsplit[8]
                data.append([chr, start, end, "T=" + t, tagg])

    return data





def write_bed(data, filename):
    with open(filename, "w") as file:
        for x in data:
            file.write("\t".join(x[:3]) + "\t" + x[3] + ";" + x[4] + "\n")


if __name__ == "__main__":
    logger = logging.getLogger('simple_example')
    logging.basicConfig(format='%(asctime)s %(message)s')
    logger.setLevel(logging.INFO)
    logger.info("Running gff2bed")


    parser = argparse.ArgumentParser()
    parser.add_argument("-g", "--gff", help="Gff file")
    parser.add_argument("-o", "--output", help="Output file")
    args = parser.parse_args()


    logger.info("Read GFF")
    gff = readgff(args.gff)

    logger.info("Write output BED")
    write_bed(gff, args.output)