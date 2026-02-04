use std::{any::Any, collections::BTreeMap, rc::Rc};

#[derive(Default, Clone)]
pub struct Map(BTreeMap<String, Rc<dyn Value>>);

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn set<V: Value>(&mut self, key: &str, value: V) -> &mut Self {
        self.0.insert(key.to_string(), Rc::new(value));
        self
    }

    pub fn get_as<V: Value>(&self, key: &str) -> Option<&V> {
        match self.0.get(key) {
            None => None,
            Some(v) => (v.as_ref() as &dyn Any).downcast_ref::<V>(),
        }
    }

    pub fn merge(&mut self, other: Self) -> &mut Self {
        for (key, value) in other.0 {
            self.0.insert(key, value);
        }

        self
    }
}

impl std::ops::Deref for Map {
    type Target = BTreeMap<String, Rc<dyn Value>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_map();

        for (key, value) in &self.0 {
            s.entry(key, value);
        }

        s.finish()
    }
}

pub trait Value: std::any::Any + std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: std::any::Any + std::fmt::Debug> Value for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<T: Value> AsRef<T> for dyn Value {
    fn as_ref(&self) -> &T {
        self.as_any().downcast_ref().unwrap()
    }
}

impl<T: Value> AsMut<T> for dyn Value {
    fn as_mut(&mut self) -> &mut T {
        self.as_any_mut().downcast_mut().unwrap()
    }
}
