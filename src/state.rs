use std::any::Any;
use std::sync::Arc;


///Trait required to be implmented for the struct expected to be the shared state.
#[derive(Clone,Debug)]
pub struct State{
    inner: Arc<dyn Any + Send + Sync+ 'static>
}
impl State{
    pub fn new()->Self{

        State{inner:Arc::new(())}
    }
    pub fn set<T: 'static + Send + Sync>(&mut self, obj:T){
        let inner = Arc::new(obj);
        self.inner= inner;
    }
    pub fn inner<T:'static + std::marker::Sync+ std::marker::Send>(&self) -> Option<Arc<T>>{
        let inner = self.inner.clone();
        match inner.downcast::<T>() {
            Ok(i) => {
                let ret = i.clone();
                return Some(ret)},
            Err(_) => None,
        }
    }
}