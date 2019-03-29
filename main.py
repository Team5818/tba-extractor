import csv
from argparse import ArgumentParser
from pathlib import Path

from tba_extractor.csv_data import CsvData
from tba_extractor.dataset import datasets

import sys


def arg_output(val):
    if val == '-':
        return lambda: sys.stdout
    p = Path(val)
    return lambda: p.open(mode='w')


def main():
    parser = ArgumentParser('tba-extractor', description='Extracts CSV data from TBA.')

    sub_parsers = parser.add_subparsers(
        title='Data Sets',
        help='Data set to extract.'
    )

    for k in sorted(datasets.keys()):
        p = sub_parsers.add_parser(k)
        p.set_defaults(dataset=datasets[k])
        for option in datasets[k].options:
            p.add_argument(option.flag,
                           help=option.help,
                           required=option.required,
                           action=option.action)

    parser.add_argument('--output', type=arg_output, default='-',
                        help='The file to output to, STDOUT by default. Use `-` to represent STDOUT.')

    args = parser.parse_args()
    if not hasattr(args, 'dataset'):
        parser.print_help()
        sys.exit(1)
    data: CsvData = args.dataset.func(args)
    with args.output() as fp:
        data.save_to(csv.writer(fp, dialect='unix'))


if __name__ == '__main__':
    main()
