mod host_filter;
mod match_all_filter;

pub use self::match_all_filter::MatchAllFilter;
pub use self::host_filter::HostFilter;

use flowgger::record::Record;

pub trait CloneBoxedFilter {
    fn clone_boxed<'a>(&self) -> Box<Filter + Send + 'a>
    where
        Self: 'a;
}

impl<T: Filter + Clone + Send> CloneBoxedFilter for T {
    fn clone_boxed<'a>(&self) -> Box<Filter + Send + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<Filter> {
    fn clone(&self) -> Box<Filter> {
        self.clone_boxed()
    }
}

pub trait Filter: CloneBoxedFilter {
    fn condition(&self, record: &Record) -> bool;
    fn action(&self, record: &mut Record) -> ();
}
