#!/usr/bin/env python3
# -*- coding: utf-8 -*-


import argparse
import logging


def read_gff(filename, a = None):
    """
    Reading the GFF file - already changing the
    :param filename: GFF file name
    :return: data --> [[chr, start, end, tags(with feature type]]
    """
    data = []
    with open(filename) as file:
        for line in file.readlines():
            lsplit = line.split()
            if len(lsplit) != 0:
                chr = lsplit[0]
                start = int(lsplit[3])
                end = int(lsplit[4])
                if a != None:
                    tagg = lsplit[8]
                    tag = tagg.split(";")
                    tt = [x.split(":")[0] for x in tag if x.startswith(a)]
                    if len(tt) != 0:
                        data.append([chr, start-1, end, tt[0]])

                else:
                    t = lsplit[2]
                    data.append([chr, start-1, end, t])


    return data





def write_bed(data, filename):
    """
    Writing the bed file
    :param data: the gff file
    :param filename: the output filenam
    :return:
    """
    with open(filename, "w") as file:
        for x in data:
            print(x)
            file.write("\t".join([str(y) for y in x[:3]]) + "\t" + x[3] + "\n")


if __name__ == "__main__":
    # Logging
    logger = logging.getLogger('simple_example')
    logging.basicConfig(format='%(asctime)s %(message)s')
    logger.setLevel(logging.INFO)
    logger.info("Running gff2bed")


    parser = argparse.ArgumentParser()
    parser.add_argument("-g", "--gff", help="Gff file", required=True)
    parser.add_argument("-t", "--type", help="Extract type [default: True]", action="store_true")
    parser.add_argument("-a", "--attribute", help = "Extract this attribute (except of type)")
    parser.add_argument("-o", "--output", help="Output file", required=True)
    args = parser.parse_args()


    logger.info("Read GFF")
    o = True
    gff = read_gff(args.gff, args.attribute)

    logger.info("Write output BED")
    write_bed(gff, args.output)