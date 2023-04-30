use crate::error::MResult;
use async_trait::async_trait;

/// Трейт, используемый в типах данных Майнкрафта, как источник чтения данных для кодирования в типы данных.
/// Его главная цель - создать общий интерфейс и для буфера с уже закешированными данными, которые сохранены в `Vec<u8>`
/// и для TCP стрима, данные из которого еще не получены
#[async_trait]
pub trait Buffer {
    /// Получение следующего байта
    /// В Векторе байт `Vec<u8>` удаляется и возвращается первый элемент,
    /// а в TCP стриме читается один байт
    async fn next_byte(&mut self) -> MResult<u8>;
}

#[async_trait]
impl Buffer for Vec<u8> {
    async fn next_byte(&mut self) -> MResult<u8> {
        Ok(self.remove(0))
    }
}
