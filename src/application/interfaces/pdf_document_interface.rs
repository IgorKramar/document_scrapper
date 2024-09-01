pub trait PdfDocumentInterface {
    /// Создает новый PDF-документ
    fn create_document(&mut self, title: &str);

    /// Добавляет текст на страницу PDF
    fn add_text(&mut self, text: &str, font_size: u32, x: f64, y: f64);

    /// Сохраняет PDF-документ в файл
    fn save_document(&mut self, output_path: &str) -> Result<(), Box<dyn std::error::Error>>;
}
