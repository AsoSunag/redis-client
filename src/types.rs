#[derive(Debug, Clone)]
pub enum SenderType {
    Simple,
    Pipe(usize)
}

#[derive(Debug, Clone)]
pub enum PubSubType {
	Simple,
	Channel(String),
	Pattern(String)
}

