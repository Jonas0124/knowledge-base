use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct UserContext {

    pub id: String,

    pub username: String,

    pub email: String,
}

thread_local! {
    static USER_CONTEXT: RefCell<Option<UserContext>> = RefCell::new(None);
}