use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};
use std::{ops::Deref, rc::Rc};
use yew::{hook, use_memo, use_state, UseStateHandle};

// ========================// UseLocalStorageHandle //======================== //

pub struct UseLocalStorageHandle<T> {
    inner: UseStateHandle<T>,
    key: Rc<String>,
}

impl<T> UseLocalStorageHandle<T> {
    pub fn set(&self, value: T)
    where
        T: Serialize + Clone,
    {
        if LocalStorage::set(&*self.key, value.clone()).is_ok() {
            self.inner.set(value);
        }
    }
}

impl<T> Clone for UseLocalStorageHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            key: self.key.clone(),
        }
    }
}

impl<T> Deref for UseLocalStorageHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> PartialEq for UseLocalStorageHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

// ========================// use_local_storage //======================== //

#[hook]
pub fn use_local_storage<T>(key: &str) -> UseLocalStorageHandle<T>
where
    T: DeserializeOwned + Default + 'static,
{
    let inner: UseStateHandle<T> = use_state(|| LocalStorage::get(key).unwrap_or_default());
    let key = use_memo(|_| key.to_owned(), ());

    UseLocalStorageHandle { inner, key }
}
