# This is a sample Python script.
import datetime
import random
import time

from gen_transactions import generate_transaction


# Press ⌃R to execute it or replace it with your code.
# Press Double ⇧ to search everywhere for classes, files, tool windows, actions, and settings.

def random_dt():
    ts = random.randrange(1742468881, 1743332881)
    return datetime.datetime.fromtimestamp(ts).isoformat() + 'Z'


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    bob = 123
    a = str(bob)
    c = 0
