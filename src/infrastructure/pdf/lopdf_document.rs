use crate::application::interfaces::pdf_document_interface::PdfDocumentInterface;
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};

/// Реализация интерфейса PdfDocumentInterface с использованием lopdf
pub struct LopdfDocument {
    document: Document,
    current_y: f64,
    font_id: lopdf::ObjectId,
    resources_id: lopdf::ObjectId,
    pages_id: lopdf::ObjectId,
    operations: Vec<Operation>,
}

impl LopdfDocument {
    /// Создает новый экземпляр LopdfDocument
    pub fn new() -> Self {
        let mut document = Document::with_version("1.5");
        let pages_id = document.new_object_id();
        let font_id = Self::create_font(&mut document);
        let resources_id = Self::create_resources(&mut document, font_id);

        Self {
            document,
            current_y: 700.0, // начальная позиция по Y
            font_id,
            resources_id,
            pages_id,
            operations: vec![],
        }
    }

    /// Создает шрифт и добавляет его в документ
    fn create_font(document: &mut Document) -> lopdf::ObjectId {
        document.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        })
    }

    /// Создает ресурсы для страницы (например, шрифт)
    fn create_resources(document: &mut Document, font_id: lopdf::ObjectId) -> lopdf::ObjectId {
        document.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        })
    }

    /// Добавляет страницу в документ
    fn add_page(&mut self, content_id: lopdf::ObjectId) -> lopdf::ObjectId {
        self.document.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => self.pages_id,
            "Contents" => content_id,
            "Resources" => self.resources_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        })
    }

    /// Создает содержимое страницы на основе операций
    fn create_content_stream(&mut self) -> lopdf::ObjectId {
        let content = Content {
            operations: self.operations.clone(),
        };
        self.document
            .add_object(Stream::new(dictionary! {}, content.encode().unwrap()))
    }

    /// Создает каталог и добавляет его в документ
    fn create_catalog(&mut self) -> lopdf::ObjectId {
        let content_id = self.create_content_stream();
        let page_id = self.add_page(content_id);

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        };
        self.document
            .objects
            .insert(self.pages_id, Object::Dictionary(pages));

        self.document.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => self.pages_id,
        })
    }

    /// Сохраняет документ в файл
    fn finalize_document(&mut self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let catalog_id = self.create_catalog();
        self.document.trailer.set("Root", catalog_id);
        self.document.compress();
        self.document.save(output_path)?;

        Ok(())
    }
}

impl PdfDocumentInterface for LopdfDocument {
    /// Создает новый PDF-документ с заданным заголовком
    fn create_document(&mut self, _title: &str) {
        // Заголовок можно использовать в метаданных или добавлять его на первую страницу
        // В данном примере он не используется напрямую
    }

    /// Добавляет текст на текущую страницу PDF
    fn add_text(&mut self, text: &str, font_size: u32, x: f64, y: f64) {
        self.operations.push(Operation::new("BT", vec![]));
        self.operations
            .push(Operation::new("Tf", vec!["F1".into(), font_size.into()]));
        self.operations
            .push(Operation::new("Td", vec![x.into(), y.into()]));
        self.operations
            .push(Operation::new("Tj", vec![Object::string_literal(text)]));
        self.operations.push(Operation::new("ET", vec![]));
    }

    /// Сохраняет PDF-документ в файл
    fn save_document(&mut self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.finalize_document(output_path)
    }
}
