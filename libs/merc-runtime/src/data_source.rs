pub trait DataSource {
    type Item;
    type Err;

    fn read(&self) -> Result<Self::Item, Self::Err>;
    fn write(&self, item: Self::Item) -> Result<(), Self::Err>;
}
