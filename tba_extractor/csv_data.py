from typing import List, Any


class CsvData:
    @staticmethod
    def new(data: List[List[Any]]):
        return CsvData(list(map(CsvRow, data)))

    def __init__(self, rows: List['CsvRow']):
        self.rows = rows

    def save_to(self, writer):
        for row in self.rows:
            writer.writerow(row.cols)


class CsvRow:
    def __init__(self, cols: List[Any]):
        self.cols = cols
