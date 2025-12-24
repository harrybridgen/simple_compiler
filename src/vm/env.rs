use super::VM;
use crate::grammar::Type;

impl VM {
    pub(crate) fn lookup_var(&self, name: &str) -> Option<&Type> {
        self.find_immutable(name)
            .or_else(|| self.local_env.as_ref().and_then(|e| e.get(name)))
            .or_else(|| self.global_env.get(name))
    }

    pub(crate) fn find_immutable(&self, name: &str) -> Option<&Type> {
        self.immutable_stack.iter().rev().find_map(|s| s.get(name))
    }

    pub(crate) fn immutable_exists(&self, name: &str) -> bool {
        self.find_immutable(name).is_some()
    }

    pub(crate) fn ensure_mutable_binding(&self, name: &str) {
        if self.immutable_exists(name) {
            panic!("cannot assign to immutable variable `{name}`");
        }
    }
}
