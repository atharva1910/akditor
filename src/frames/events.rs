pub enum AKEvent {
    Quit,
    NewBuffer,
    FileExp,
    ListBuffer,
    StatusBar(Option<String>),
}
