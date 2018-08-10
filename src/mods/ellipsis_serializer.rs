use serde;
use std::any;

pub fn serialize<S>(_: impl any::Any, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_unit_struct("…")
}
