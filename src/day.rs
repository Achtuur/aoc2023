use anyhow::Result;
pub trait Day {
    fn read_input(&mut self) -> Result<()>;
    fn A(&mut self) -> Result<String>;
    fn B(&mut self) -> Result<String>;
}