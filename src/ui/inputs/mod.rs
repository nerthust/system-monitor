pub enum InputEvent<I> {
    /// An input event occurred.
    Input(I),
    /// An tick event occurred.
    Tick,
}
