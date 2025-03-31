import datetime
import json
import os
import random
from google.cloud import pubsub_v1

os.environ['GOOGLE_APPLICATION_CREDENTIALS'] = './gen_credentials.json'

publisher = pubsub_v1.PublisherClient()
topic_path = publisher.topic_path("pcd-2-455310", "transactions-topic")


def random_dt():
    ts = random.randrange(1742468881, 1743332881)
    return datetime.datetime.fromtimestamp(ts).isoformat() + 'Z'

def generate_transaction():
    return json.dumps({
        "card_id": f"card_{random.randint(1000, 9999)}",
        "amount": random.randint(1, 5000),
        "location": random.choice(["NY", "CA", "TX", "FL"]),
        "timestamp": random_dt()
    })

def fraud_transaction():
    return json.dumps({
        "card_id": f"card_{random.randint(1000, 9999)}",
        "amount": 441,
        "location": random.choice(["NY", "CA", "TX", "FL"]),
        "timestamp": random_dt()
    })


def main():
    pubbed = publisher.publish(topic_path, fraud_transaction().encode("utf-8"))
    print(pubbed.result())
    # for _ in range(10):
    #     a = publisher.publish(topic_path, data=generate_transaction().encode("utf-8"))
    #     print(a.result())


if __name__ == '__main__':
    main()
