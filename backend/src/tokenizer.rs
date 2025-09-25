use std::collections::HashMap;

/// Простой токенизатор для русского текста
/// В реальном проекте здесь должна быть интеграция с библиотекой токенизации
#[allow(dead_code)]
pub struct SimpleTokenizer {
    vocab: HashMap<String, u32>,
    max_length: usize,
}

impl SimpleTokenizer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut vocab = HashMap::new();
        
        // Базовые токены
        vocab.insert("[PAD]".to_string(), 0);
        vocab.insert("[UNK]".to_string(), 1);
        vocab.insert("[CLS]".to_string(), 2);
        vocab.insert("[SEP]".to_string(), 3);
        
        // Простой словарь на основе частых слов
        let common_words = [
            "и", "в", "не", "на", "с", "по", "для", "от", "до", "из", "к", "о", "у", "за", "при",
            "обслуживание", "банк", "карта", "кредит", "ипотека", "вклад", "мобильный", "приложение",
            "онлайн", "сайт", "терминал", "поддержка", "хорошо", "плохо", "быстро", "медленно",
            "понравилось", "непонравилось", "рекомендую", "удобно", "зависает", "работает"
        ];
        
        for (i, word) in common_words.iter().enumerate() {
            vocab.insert(word.to_string(), (i + 4) as u32);
        }
        
        Self {
            vocab,
            max_length: 512,
        }
    }
    
    /// Токенизирует текст и возвращает массив индексов токенов
    #[allow(dead_code)]
    pub fn tokenize(&self, text: &str) -> Vec<u32> {
        let mut tokens = vec![self.vocab["[CLS]"]];
        
        // Простая токенизация по словам и знакам препинания
        let words: Vec<&str> = text
            .split_whitespace()
            .flat_map(|word| {
                // Разделяем по знакам препинания
                word.split_inclusive(|c: char| c.is_ascii_punctuation())
                    .filter(|s| !s.is_empty())
            })
            .collect();
        
        for word in words.iter().take(self.max_length - 2) {
            let word_lower = word.to_lowercase();
            let token = self.vocab.get(&word_lower)
                .or_else(|| self.vocab.get(&word.to_string()))
                .unwrap_or(&self.vocab["[UNK]"]);
            tokens.push(*token);
        }
        
        tokens.push(self.vocab["[SEP]"]);
        
        // Дополняем до максимальной длины
        while tokens.len() < self.max_length {
            tokens.push(self.vocab["[PAD]"]);
        }
        
        tokens.truncate(self.max_length);
        tokens
    }
    
    /// Создает маску внимания (1 для реальных токенов, 0 для padding)
    #[allow(dead_code)]
    pub fn create_attention_mask(&self, tokens: &[u32]) -> Vec<u32> {
        tokens.iter()
            .map(|&token| if token == self.vocab["[PAD]"] { 0 } else { 1 })
            .collect()
    }
}

impl Default for SimpleTokenizer {
    fn default() -> Self {
        Self::new()
    }
}
