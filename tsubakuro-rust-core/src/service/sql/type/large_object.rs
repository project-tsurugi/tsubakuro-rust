use crate::jogasaki::proto::sql::common::LargeObjectProvider;

pub(crate) trait TgLargeObjectReference {
    fn provider(&self) -> LargeObjectProvider;
    fn object_id(&self) -> u64;
}
