import pdfplumber
import pandas as pd
from tqdm import tqdm

def extract_table_from_page(page):
    table = page.extract_table()
    return table if table else None

def process_pdf(pdf_path):
    tables = []
    with pdfplumber.open(pdf_path) as pdf:
        for page in tqdm(pdf.pages, desc="Processing pages"):
            table = extract_table_from_page(page)
            if table:
                tables.append(table)
    return tables

def main(pdf_path):
    tables = process_pdf(pdf_path)

    if not tables:
        print("No tables were extracted from the PDF.")
        return

    # Get the header from the first table
    header = tables[0][0]

    # Combine all data rows
    data = [row for table in tables for row in table[1:]]

    # Create DataFrame
    df = pd.DataFrame(data, columns=header)

    # Display the DataFrame
    print(df)

    # Save the DataFrame to a CSV file
    df.to_csv('output.csv', index=False)
    print("Data has been saved to output.csv")

if __name__ == "__main__":
    pdf_path = '/Users/anjalafghan/Downloads/anjal_data.pdf'
    main(pdf_path)