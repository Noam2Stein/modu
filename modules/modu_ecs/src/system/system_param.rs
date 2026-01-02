pub trait SystemParam: Sized + Send + Sync + 'static {
    type WithLifetime<'a>;
}
