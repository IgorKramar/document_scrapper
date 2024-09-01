use crate::application::interfaces::pdf_document_interface::PdfDocumentInterface;
use crate::domain::models::parsed_content::ParsedContent;

pub struct PdfCreationService<D: PdfDocumentInterface> {
    document: D,
}

impl<D: PdfDocumentInterface> PdfCreationService<D> {
    /// Создает новый сервис для работы с PDF-документом
    pub fn new(document: D) -> Self {
        Self { document }
    }

    /// Создает PDF-документ с заданным контентом и сохраняет его
    pub fn create_pdf(
        &mut self,
        content: ParsedContent,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.document.create_document("Документация");

        let mut y_position = 700.0; // Начальная позиция по оси Y
        for text in content.content {
            self.document.add_text(&text, 12, 100.0, y_position);
            y_position -= 20.0; // Сдвиг вниз для каждой строки текста
        }

        self.document.save_document(output_path)?;

        Ok(())
    }
}
