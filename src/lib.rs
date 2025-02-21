//! # Aurora
//!
//! Aurora is a powerful document processing engine that transforms a wide range of documents, images, and financial statements into a structured JSON format. It automatically recognizes document types and outputs clean, machine-readable data.
//!
//! ## Features
//!
//! - 🧠 **Intelligent Parsing**: Supports various document formats, including CSV, PDF, MT940, and proprietary bank formats.
//! - 🔄 **Data Normalization**: Converts unstructured data into a consistent JSON format.
//! - 📊 **Bank Statement Coverage**: Optimized for major Dutch and Belgian banks, with ongoing support for new formats.
//! - 📈 **Scalable and Fast**: Designed to handle large document batches with high accuracy and performance.
//!
//! ## API Example
//! ```bash
//! POST /
//!
//! {
//!   "file_url": "https://any-image-get-url"
//! }
//! ```
//!
//! ## Example response (`Revolut personal CSV`)
//! This is with the `f64` casting
//! ```json
//! [
//!   {
//!     "data": {
//!         "type": "TRANSFER",
//!         "balance": 100,
//!         "amount": 50,
//!         "completed_date": "2024-01-01 01:24:30",
//!         "currency": "EUR",
//!         "fee": 0.0,
//!         "description": "To EUR Pocket",
//!         "state": "COMPLETED",
//!         "started_date": "2024-01-01 01:24:30",
//!         "product": "Savings"
//!     },
//!     "document_provider": "Revolut"
//!   },
//!   {
//!     "data": {
//!         "type": "TRANSFER",
//!         "product": "Savings",
//!         "currency": "EUR",
//!         "state": "COMPLETED",
//!         "amount": 50,
//!         "started_date": "2024-01-01 09:22:37",
//!         "description": "To EUR Pocket",
//!         "balance": 500,
//!         "fee": 0.0,
//!         "completed_date": "2024-01-01 09:22:37"
//!     },
//!     "document_provider": "Revolut"
//!   },
//!   {
//!     "data": {
//!         "balance": 1000,
//!         "product": "Savings",
//!         "type": "TRANSFER",
//!         "description": "To EUR Pocket",
//!         "started_date": "2024-01-04 13:48:07",
//!         "fee": 0.0,
//!         "currency": "EUR",
//!         "state": "COMPLETED",
//!         "amount": 1.0,
//!         "completed_date": "2024-01-04 13:48:07"
//!     },
//!     "document_provider": "Revolut"
//!   }
//! ]
//! ```
//!
//! ## Document Foundry
//!
//! Aurora supports and optimizes the following document formats and financial institutions:
//!
//! ### ✅ Supported Banks and Formats
//!❌⏳
//! | Bank/Service            | Target    | Formats/Types         |  Status        |
//! |-------------------------|-----------|-----------------------|----------------|
//! | **ABN AMRO**            | Business  | CSV                   | ❌               |
//! | **ABN AMRO**            | Business  | PDF                   | ❌               |
//! | **ABN AMRO**            | Business  | MT940                 | ❌               |
//! | **ABN AMRO**            | Business  | TXT250                | ❌               |
//! | **ABN AMRO**            | Business  | XLX250                | ❌               |
//! | **ABN AMRO**            | Personal  | CSV                   | ❌               |
//! | **ABN AMRO**            | Personal  | PDF                   | ❌               |
//! | **ABN AMRO**            | Personal  | MT940                 | ❌               |
//! | **ABN AMRO**            | Personal  | TXT250                | ❌               |
//! | **ABN AMRO**            | Personal  | XLX250                | ❌               |
//! | **Bunq**                | Business  | PDF                   | ❌               |
//! | **ING**                 | Personal  |                       | ❌               |
//! | **ING**                 | Business  |                       | ❌               |
//! | **Invoice2go**          |           |                       | ❌               |
//! | **Swedbank (SE)**       |           |                       | ❌               |
//! | **Shopify Orders**      |           |                       | ❌               |
//! | **Revolut**             | Personal  | CSV                   | ✅               |
//! | **Revolut**             | Personal  | PDF                   | ❌               |
//! | **Revolut**             | Business  |                       | ❌               |
//! | **Knab**                |           |                       | ❌               |
//! | **Stripe**              | Receipts  |                       | ❌               |
//! | **Stripe**              | Invoices  |                       | ❌               |
//! | **BeoBank (BE)**        |           |                       | ❌               |
//! | **BNP Paribas Fortis (BE)** |       |                       | ❌               |
//! | **Nationwide (UK)**     |           |                       | ❌               |
//! | **Halifax (UK)**        |           |                       | ❌               |
//! | **Sparkasse (DE)**      |           |                       | ❌               |
//!
//! ### 📌 Upcoming Coverage
//!
//! We are continuously expanding our support for new banks and document types, including:
//!
//! - Rabobank (NL)
//! - Triodos Bank (NL)
//! - KBC Bank (BE)
//! - Argenta (BE)
//! - Deutsche Bank (BE)
//! - Volksbank (NL)
//! - ASN Bank (NL)
//!
//! ## Usage
//!
//! Aurora can be integrated with your existing infrastructure to automatically parse and transform documents. Detailed API documentation and SDK support are available for seamless integration.
//!
//! ## Environment variables
//!
//! To run this project, you will need to add the following environment variables to your .env file
//!
//! - `AURORA_SCRIPT_DIR` - Optional, if you are extending Aurora with custom implementations but don't want to alter the source code
//! - `POSTGRES_CONNECTION_STRING` - any postgres string will work
//! - `AURORA_API_PORT` - Defaults to 7777, exposes the `Actix-Web` Rest api
//!
//! ## Build Aurora (Ubuntu 24.xx)
//!
//! ### Leptonica and Tesseract dependencies
//! On Ubuntu and derivatives the additional dependencies can be installed by running:
//! ```sudo apt-get install libleptonica-dev libtesseract-dev clang```
//!
//! On Fedora 30 the additional dependencies can be installed by running:
//! ```sudo dnf install leptonica-devel tesseract-devel clang```
//!
//! On Termux 2019 (Android, Android on Chromebooks) the additional dependencies can be installed by running:
//! ```pkg install libclang leptonica-dev tesseract-dev```
//!
//! ## Build Aurora (Windows 10)
//! I don't know if it works on windows 11, i'd presume so but i can't say
//!
//! On Windows, this library uses Microsoft's `vcpkg` to provide tesseract.
//!
//! Please install `vcpkg` and set up user wide integration or `vcpkg` crate won't be able to find a library.
//!
//! To install tesseract
//! ```
//! REM from the vcpkg directory
//!
//! REM 32 bit
//! .\vcpkg install tesseract:x86-windows
//!
//! REM 64 bit
//! .\vcpkg install tesseract:x64-windows
//! ```
//!
//! `vcpkg` allows building either dynamically or statically linked application
//!
//! if you prefer dynamic linking
//! ```
//! SET VCPKGRS_DYNAMIC=true
//! ```
//!
//! for statically linked libraries
//!
//! ```
//! SET RUSTFLAGS=-Ctarget-feature=+crt-static
//! ```
//! To run the tests please download the English trained data to this directory and set
//!
//! ```
//! SET TESSDATA_PREFIX=.
//! ```
//! If you prefer to compile tesseract yourself (Because, for example, you could not get vcpkg to build using clang-cl.exe), you can set these environment variables: TESSERACT_INCLUDE_PATHS, TESSERACT_LINK_PATHS and TESSERACT_LINK_LIBS.
//!
//! For example:
//! ```
//! set TESSERACT_INCLUDE_PATHS=D:\tesseract\build\include
//! set TESSERACT_LINK_PATHS=D:\tesseract\build\lib
//! set TESSERACT_LINK_LIBS=tesseract41
//! ```
//!
//! ## Run Aurora
//! ```bash
//! # Clone the repository
//! git clone https://github.com/floris-xlx/aurora.git
//! cd aurora
//!
//! # Copy example env
//! cp .env.example .env
//!
//! # Build Aurora from source
//! cargo build --release
//!
//! # Run Aurora
//! cp target/release/aurora aurora && ./aurora
//! ```
//!
//! ## Contributing
//!
//! We welcome contributions! If you'd like to add support for a new document type or bank, feel free to open an issue or submit a pull request.
//!
//! ## License
//!
//! Aurora is licensed under the MIT License.
//!
//! ## Support
//!
//! For support, email floris@xylex.ai or submit an issue here on github

pub mod api;
pub mod config;
pub mod db;
pub mod foundry;
pub mod llm;
pub mod ocr;
pub mod parser;
pub mod utils;

use mime_guess::from_path;
use serde::Deserialize;
use std::path::Path;
use tracing::{error, info};

// crate imports
use crate::utils::file_extension::get_file_extension;

#[derive(Deserialize, Debug, Clone)]
pub struct Parser {
    pub file_url: String,
    pub file_extension: String,
    pub file_mime: String,
}

impl Parser {
    pub fn new(file_url: String) -> Self {
        info!("Creating a new Parser instance for file URL: {}", file_url);

        let file_extension: String = get_file_extension(&file_url);
        if file_extension.is_empty() {
            error!("Failed to extract file extension from URL: {}", file_url);
        } else {
            info!("Extracted file extension: {}", file_extension);
        }

        let path: &Path = Path::new(&file_url);
        let file_mime: String = from_path(path).first_or_octet_stream().to_string();

        info!("Determined MIME type: {}", file_mime);

        Parser {
            file_url,
            file_extension,
            file_mime,
        }
    }
}
