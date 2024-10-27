from collections import defaultdict
import csv
from main import compute_statistics


def test_main():
    data = defaultdict(list)
    with open("data.csv", "r", encoding="utf-8") as csvfile:
        reader = csv.DictReader(csvfile)
        headers = reader.fieldnames
        for row in reader:
            for header in headers:
                data[header].append(row[header])

    stats = compute_statistics(data)
    assert len(stats.items()) > 0
    assert stats['Age']['median'] == 25.0
    assert stats['G']['median'] == 37.0
