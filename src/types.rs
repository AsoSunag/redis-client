#[derive(Debug, Clone)]
pub enum SenderType {
    Simple,
    Pipe(usize)
}
