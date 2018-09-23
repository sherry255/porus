pub trait Handle: Copy {}

pub trait Pool {
    type Handle: Handle;
    type Elem;

    fn get(&self, handle: Self::Handle) -> &Self::Elem;
    fn get_mut(&mut self, handle: Self::Handle) -> &mut Self::Elem;
    fn add(&mut self, item: Self::Elem) -> Self::Handle;
    fn remove(&mut self, handle: Self::Handle) -> Self::Elem;
}

pub fn get<P: Pool>(pool: &P, handle: P::Handle) -> &P::Elem {
    Pool::get(pool, handle)
}

pub fn get_mut<P: Pool>(pool: &mut P, handle: P::Handle) -> &mut P::Elem {
    Pool::get_mut(pool, handle)
}

pub fn add<P: Pool>(pool: &mut P, item: P::Elem) -> P::Handle {
    Pool::add(pool, item)
}

pub fn remove<P: Pool>(pool: &mut P, handle: P::Handle) -> P::Elem {
    Pool::remove(pool, handle)
}
