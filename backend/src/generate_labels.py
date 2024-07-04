import pandas as pd
import numpy as np
import tensorflow as tf
from transformers import BertTokenizer
import joblib

# Load BERT tokenizer
tokenizer = BertTokenizer.from_pretrained('bert-base-uncased')

# Load the saved model
model = tf.saved_model.load('bert_finance_model')

# Load the label encoder
label_encoder = joblib.load('label_encoder.joblib')

def preprocess_data(text, tokenizer, max_length=128):
    # Tokenize and encode text data
    inputs = tokenizer(
        text,
        truncation=True,
        padding='max_length',
        max_length=max_length,
        return_tensors='tf'
    )
    return {
        'input_ids': tf.convert_to_tensor(inputs['input_ids']),
        'attention_mask': tf.convert_to_tensor(inputs['attention_mask'])
    }

def predict_labels(csv_file):
    # Read the new CSV file
    df_new = pd.read_csv(csv_file)
    print(len(df_new))
    # Assume 'narration' column exists in the CSV
    X_new = df_new['Narration'].tolist()

    # Preprocess text data
    max_length = 128  # Ensure this matches the expected input length
    X_new_encodings = [preprocess_data(text, tokenizer, max_length) for text in X_new]

    # Get the concrete function for inference from the SavedModel
    infer = model.signatures["serving_default"]

    # Make predictions
    y_preds = [infer(input_ids=data['input_ids'], attention_mask=data['attention_mask'])['dense'] for data in X_new_encodings]

    # Decode predicted labels
    y_pred_classes = [np.argmax(y_pred, axis=-1).reshape(-1) for y_pred in y_preds]  # Reshape to 1D array
    predicted_labels = label_encoder.inverse_transform(np.concatenate(y_pred_classes))  # Concatenate and inverse transform

    # Add predicted labels to the dataframe
    df_new['predicted_label'] = predicted_labels

    return df_new


# Example usage:
csv_file_path = 'new_data.csv'
predicted_df = predict_labels(csv_file_path)
print(predicted_df.head())  # Display the predicted labels
