pub trait RequestSerializable {
    fn serialize(&self) -> String;
}
