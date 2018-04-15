mod add_field_action;

pub use self::add_field_action::AddFieldAction;

use flowgger::record::Record;

pub trait CloneBoxedAction {
    fn clone_boxed<'a>(&self) -> Box<Action + Send + 'a>
    where
        Self: 'a;
}

impl<T: Action + Clone + Send> CloneBoxedAction for T {
    fn clone_boxed<'a>(&self) -> Box<Action + Send + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<Action> {
    fn clone(&self) -> Box<Action> {
        self.clone_boxed()
    }
}

pub trait Action: CloneBoxedAction {
    fn apply(&self, record: &mut Record) -> ();
}
