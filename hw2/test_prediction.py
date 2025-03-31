import os
from datetime import datetime

os.environ['GOOGLE_APPLICATION_CREDENTIALS'] = './firebase_credentials.json'

from google.cloud import aiplatform, firestore
import json
import base64

# Set up Firestore and Vertex AI
db = firestore.Client()

def detect_fraud(transaction):
    # Format data for Vertex AI model
    instance = {
        "amount": str(transaction["amount"]),
        "location": transaction["location"],
        "timestamp": transaction["timestamp"],
        "card_id": transaction["card_id"]
    }

    # Call Vertex AI model
    endpoint = aiplatform.Endpoint(endpoint_name=f"projects/pcd-2-455310/locations/europe-west3/endpoints/4494922281604087808")
    prediction = endpoint.predict(instances=[instance])

    # Extract fraud score
    fraud_score = prediction.predictions[0]["scores"][1]

    if fraud_score > 0.8:
        # Store fraud alert in Firestore
        alert_ref = db.collection("fraud-alerts").document(transaction["card_id"])
        alert_ref.set({
            "transaction": transaction,
            "fraud_score": fraud_score
        })
        print(f"🚨 Fraud detected for card {transaction['card_id']} with score {fraud_score}")


if __name__ == '__main__':
    detect_fraud({
        'amount': 123456,
        'location': 'TX',
        'timestamp': datetime.now().isoformat(),
        'card_id': '4494922281604087808'
    })
