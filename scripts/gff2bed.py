"""
This is not part of the tool and was only added for my own project.
SV


"""


import argparse
import logging


def read_gff(filename):
    """
    Reading the GFF file - already changing the
    :param filename: GFF file name
    :return: data --> [[chr, start, end, tags(with feature type]]
    """
    data = []
    with open(filename) as file:
        for line in file.readlines():
            lsplit = line.split()

            if len(lsplit) > 7:
                chr = lsplit[0]
                t = lsplit[2]
                start = int(lsplit[3])
                end = int(lsplit[4])
                tagg = lsplit[8]
                data.append([chr, start-1, end, "T=" + t, tagg])
            elif len(lsplit) > 0:
                chr = lsplit[0]
                t = lsplit[2]
                start = int(lsplit[3])
                end = int(lsplit[4])
                data.append([chr, start-1, end, "T=" + t])

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
            file.write("\t".join([str(y) for y in x[:3]]) + "\t" + x[3] + ";" + x[4] + "\n")


if __name__ == "__main__":
    logger = logging.getLogger('simple_example')
    logging.basicConfig(format='%(asctime)s %(message)s')
    logger.setLevel(logging.INFO)
    logger.info("Running gff2bed")


    parser = argparse.ArgumentParser()
    parser.add_argument("-g", "--gff", help="Gff file", required=True)
    parser.add_argument("-o", "--output", help="Output file", required=True)
    args = parser.parse_args()


    logger.info("Read GFF")
    gff = read_gff(args.gff)

    logger.info("Write output BED")
    write_bed(gff, args.output)