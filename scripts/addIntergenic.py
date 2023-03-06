#!/usr/bin/env python3
# -*- coding: utf-8 -*-


import argparse
import logging


def read_output(filename):
    """
    Reading the GFF file - already changing the
    :param filename: GFF file name
    :return: data --> [[chr, start, end, tags(with feature type]]
    """
    data = dict()
    with open(filename) as file:
        for line in file.readlines():
            lsplit = line.split()
            if len(lsplit) > 1:
                node = int(lsplit[0])
                data[node] = lsplit[1].replace("\n","")
    return data

def add_write(data, file_out):
    """
    Writing the bed file
    :param data: the gff file
    :param filename: the output filename
    :return:
    """
    with open(file_out, "w") as file:
        for key, value in data.items():
            if len(value) == 0:
                print(str(key) + "\t" + "intergenic", file = file)
            else:
                print(str(key) + "\t" + ",".join([str(x) for x in value]), file = file)


if __name__ == "__main__":
    # Logging
    logger = logging.getLogger('simple_example')
    logging.basicConfig(format='%(asctime)s %(message)s')
    logger.setLevel(logging.INFO)
    logger.info("Running addIntergenic.py")


    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input", help="gfa_annotate output file", required=True)
    parser.add_argument("-o", "--output", help="Output file", required=True)
    args = parser.parse_args()


    logger.info("Read file")
    gff = read_output(args.input)

    logger.info("Write output")
    add_write(gff, args.output)