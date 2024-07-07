import pandas as pd
import numpy as np
import os
import tensorflow as tf
from transformers import BertTokenizer, TFBertModel
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import LabelEncoder
from sklearn.metrics import classification_report
import psycopg2
from dotenv import load_dotenv
import joblib

# Load environment variables
load_dotenv()
POSTGRES_HOST = os.getenv('POSTGRES_HOST')
POSTGRES_PORT = os.getenv('POSTGRES_PORT')
POSTGRES_USER = os.getenv('POSTGRES_USER')
POSTGRES_PASSWORD = os.getenv('POSTGRES_PASSWORD')
POSTGRES_DB = os.getenv('POSTGRES_DB')

# Connect to the database
connection = psycopg2.connect(database=POSTGRES_DB, user=POSTGRES_USER, password=POSTGRES_PASSWORD, host=POSTGRES_HOST, port=POSTGRES_PORT)

# Fetch and preprocess data
df = pd.read_sql_query('SELECT narration, label FROM finance_data', connection)
X = df['narration']
y = df['label']

# Print label distribution
print(df['label'].value_counts())

# Encode categorical labels as integers
label_encoder = LabelEncoder()
y_encoded = label_encoder.fit_transform(y)

# Split data into train and test sets
X_train, X_test, y_train, y_test = train_test_split(X, y_encoded, test_size=0.2, random_state=42)

# Load BERT tokenizer
tokenizer = BertTokenizer.from_pretrained('bert-base-uncased')

# Tokenize and encode text data
max_length = 128  # Ensure this matches the expected input length
X_train_encodings = tokenizer(X_train.tolist(), truncation=True, padding='max_length', max_length=max_length, return_tensors='tf')
X_test_encodings = tokenizer(X_test.tolist(), truncation=True, padding='max_length', max_length=max_length, return_tensors='tf')

# Convert labels to numpy arrays
y_train = np.array(y_train)
y_test = np.array(y_test)

# Load pre-trained BERT model
bert_model = TFBertModel.from_pretrained('bert-base-uncased')

# Freeze BERT layers (optional)
for layer in bert_model.layers:
    layer.trainable = False

# Build the model architecture
input_ids = tf.keras.Input(shape=(max_length,), name='input_ids', dtype='int32')
attention_mask = tf.keras.Input(shape=(max_length,), name='attention_mask', dtype='int32')
bert_output = bert_model(input_ids, attention_mask=attention_mask)[1]  # Use pooled_output (CLS token) as features

# Use more neurons in the dense layer for multi-class classification
outputs = tf.keras.layers.Dense(len(label_encoder.classes_), activation='softmax')(bert_output)

model = tf.keras.Model(inputs=[input_ids, attention_mask], outputs=outputs)

# Compile the model with categorical_crossentropy loss
model.compile(optimizer='adam',
              loss='sparse_categorical_crossentropy',
              metrics=['accuracy'])

# Add sample weighting (if needed) for handling imbalanced classes
class_weights = dict(enumerate(len(y_train) / (len(label_encoder.classes_) * np.bincount(y_train))))

# Train the model
history = model.fit(
    {'input_ids': X_train_encodings['input_ids'], 'attention_mask': X_train_encodings['attention_mask']},
    y_train,
    epochs=10,  # Increase the number of epochs
    batch_size=16,
    validation_data=(
        {'input_ids': X_test_encodings['input_ids'], 'attention_mask': X_test_encodings['attention_mask']},
        y_test
    ),
    class_weight=class_weights  # Add class weights to the fit method
)

# Evaluate the model
y_pred = model.predict(
    {'input_ids': X_test_encodings['input_ids'], 'attention_mask': X_test_encodings['attention_mask']}
)
y_pred_classes = np.argmax(y_pred, axis=1)

# Specify the labels explicitly
labels = np.unique(y_encoded)

# Print classification report
print(classification_report(y_test, y_pred_classes, labels=labels, target_names=label_encoder.classes_, zero_division=0))

# Save the model architecture and weights in .keras format
tf.keras.models.save_model(model, 'bert_finance_model', save_format='tf')

# Optionally, save the label encoder as well
joblib.dump(label_encoder, 'label_encoder.joblib')
