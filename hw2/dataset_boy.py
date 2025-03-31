import csv
import json

from gen_transactions import generate_transaction


def main():
    with open('dataset.csv', 'w', newline='') as csvfile:
        spamwriter = csv.DictWriter(csvfile, fieldnames=['card_id', 'amount', 'location', 'timestamp', 'fraud_label'])
        spamwriter.writeheader()
        for _ in range(1000):
            entry = json.loads(generate_transaction())
            spamwriter.writerow({**entry, 'fraud_label': 0})
        
        
if __name__ == '__main__':
    main()
