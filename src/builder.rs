use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

use num_traits::{Bounded, ToPrimitive, FromPrimitive};
use stable_deref_trait::StableDeref;

use arena_set::{Error, ArenaSet, StadiumSet};

/// Flexible builder for [`ArenaSet`].
///
/// ```
/// use std::sync::Arc;
///
/// let b1 = shawshank::Builder::<String>::new();
/// let mut p1 = b1.hash().unwrap();
/// assert_eq!(p1.intern("hello"), Ok(0));
/// assert_eq!(p1.resolve(0), Ok("hello"));
///
/// let b2 = shawshank::Builder::<Arc<String>>::new();
/// let mut p2 = b2.stadium_set_hash().unwrap();
/// assert_eq!(p2.intern("hello"), Ok(0));
/// let s: &String = p2.resolve(0).unwrap();
/// assert_eq!(s.as_str(), "hello");
/// ```
///
/// [`ArenaSet`]: struct.ArenaSet.html
pub struct Builder<O, I = usize> {
    _o: PhantomData<O>,
    _i: PhantomData<I>,
}

impl<O, I> Builder<O, I> {
    pub fn new() -> Self {
        Builder {
            _o: PhantomData,
            _i: PhantomData,
        }
    }
}

/// Create a [`Builder`] where the ID is `usize`.
/// [`Builder`]: struct.Builder.html
pub fn builder<O>() -> Builder<O> {
    Builder::<O>::new()
}

impl<O, I> Builder<O, I>
where O: StableDeref,
      I: Bounded + ToPrimitive + FromPrimitive
{
    /// Create an empty [`ArenaSet`] that uses a `HashMap`.
    /// [`ArenaSet`]: struct.ArenaSet.html
    pub fn hash(&self) -> Result<ArenaSet<O, I, HashMap<&'static O::Target, I>>, Error>
        where O::Target: Eq + Hash {
        ArenaSet::new()
    }

    /// Create an empty [`ArenaSet`] that uses a `BTreeMap`.
    /// [`ArenaSet`]: struct.ArenaSet.html
    pub fn btree(&self) -> Result<ArenaSet<O, I, BTreeMap<&'static O::Target, I>>, Error>
        where O::Target: Eq + Ord {
        ArenaSet::new()
    }
}

impl<O, I> Builder<O, I>
where O: StableDeref,
      O::Target: 'static + StableDeref,
      < O::Target as Deref >::Target: 'static,
      I: Bounded + ToPrimitive + FromPrimitive
{
    /// Create an empty [`StadiumSet`] that uses a `HashMap`.
    /// [`StadiumSet`]: struct.StadiumSet.html
    pub fn stadium_set_hash(&self) -> Result<StadiumSet<O, O::Target, I, HashMap<&'static < O::Target as Deref >::Target, I>>, Error>
        where < O::Target as Deref >::Target: Eq + Hash {
        ArenaSet::new().map(|p| StadiumSet(p))
    }

    /// Create an empty [`StadiumSet`] that uses a `BTreeMap`.
    /// [`StadiumSet`]: struct.StadiumSet.html
    pub fn stadium_set_btree(&self) -> Result<StadiumSet<O, O::Target, I, BTreeMap<&'static < O::Target as Deref >::Target, I>>, Error>
        where < O::Target as Deref >::Target: Eq + Ord {
        ArenaSet::new().map(|p| StadiumSet(p))
    }
}