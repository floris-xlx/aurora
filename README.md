# Aurora

Aurora is a powerful document processing engine that transforms a wide range of documents, images, and financial statements into a structured JSON format. It automatically recognizes document types and outputs clean, machine-readable data.

## Features

- 🧠 **Intelligent Parsing**: Supports various document formats, including CSV, PDF, MT940, and proprietary bank formats.
- 🔄 **Data Normalization**: Converts unstructured data into a consistent JSON format.
- 📊 **Bank Statement Coverage**: Optimized for major Dutch and Belgian banks, with ongoing support for new formats.
- 📈 **Scalable and Fast**: Designed to handle large document batches with high accuracy and performance.

## Document Foundry

Aurora supports and optimizes the following document formats and financial institutions:

### ✅ Supported Banks and Formats

| Bank/Service            | Formats/Types                          |
|-------------------------|----------------------------------------|
| **ABN AMRO**            | Business, Personal (CSV, PDF, MT940, TXT250, XLX250) |
| **Bunq**                | Business                               |
| **ING**                 | Personal, Business                     |
| **Invoice2go**          |                                        |
| **Swedbank (SE)**       |                                        |
| **Shopify Orders**      |                                        |
| **Revolut**             | Personal, Business                     |
| **Knab**                |                                        |
| **Stripe**              | Receipts, Invoices                     |
| **BeoBank (BE)**        |                                        |
| **BNP Paribas Fortis (BE)** |                                    |

### 📌 Upcoming Coverage

We are continuously expanding our support for new banks and document types, including:

- Rabobank (NL)
- Triodos Bank (NL)
- KBC Bank (BE)
- Argenta (BE)
- Deutsche Bank (BE)
- Volksbank (NL)
- ASN Bank (NL)

## Usage

Aurora can be integrated with your existing infrastructure to automatically parse and transform documents. Detailed API documentation and SDK support are available for seamless integration.

```bash
# Clone the repository
git clone https://github.com/floris-xlx/aurora.git
cd aurora

# Copy example env
cp .env.example .env

# Build Aurora from source
cargo build --release

# Run Aurora
cp target/release/aurora aurora && ./aurora
```

## Contributing

We welcome contributions! If you'd like to add support for a new document type or bank, feel free to open an issue or submit a pull request.

## License

Aurora is licensed under the MIT License.

