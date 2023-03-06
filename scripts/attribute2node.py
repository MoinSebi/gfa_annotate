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
                feature = [x.split(";")[0] for x in lsplit[1].split(",")]
                print(feature)
                for x in feature:
                    if x in data:
                        data[x].add(node)
                    else:
                        data[x] = set([node])
    return data

def write_output(data, filename):
    """
    Writing the bed file
    :param data: the gff file
    :param filename: the output filename
    :return:
    """
    with open(filename, "w") as file:
        for key, value in data.items():
            print(str(key) + "\t" + ",".join([str(x) for x in value]), file = file)


if __name__ == "__main__":
    # Logging
    logger = logging.getLogger('simple_example')
    logging.basicConfig(format='%(asctime)s %(message)s')
    logger.setLevel(logging.INFO)
    logger.info("Running gff2bed")


    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input", help="gfa_annotate output file", required=True)
    parser.add_argument("-o", "--output", help="Output file", required=True)
    args = parser.parse_args()


    logger.info("Read file")
    gff = read_output(args.input, True)

    logger.info("Write output")
    write_output(gff, args.output)