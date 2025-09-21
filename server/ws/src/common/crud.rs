use crate::{db::{DbErr, Trx}, error::ModelError};


pub trait CrudService {
    type Item;
    type NewItem;
    type PatchItem;
    type SessionData;
    type Lister;
    type Id;

    fn add_item<'r>(&'r self, item: &'r Self::NewItem, session: &'r Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>> + 'r;
    fn get_items<'r>(&'r self, lister: &'r Self::Lister, session: Option<&'r Self::SessionData>) -> impl Future<Output = Result<Vec<Self::Item>, ModelError>>;
    fn get_item<'r>(&'r self, id: Self::Id, session: Option<&'r Self::SessionData>) -> impl Future<Output = Result<Self::Item, ModelError>>;
    fn update_item<'r>(&'r self, id: Self::Id, item: &'r Self::PatchItem, session: &'r Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>>;
    fn delete_item<'r>(&'r self, id: Self::Id, session: &'r Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>>;
}

pub trait CrudQueries<'t> {
    type Item;
    type NewItem;
    type PatchItem;
    type Lister;
    type Id: Send + Sync + Unpin;

    /// Insert a new item
    fn add_item(&self, item: &Self::NewItem, transaction: &mut Trx) -> impl Future<Output = Result<Self::Item, DbErr>>;
    /// Retrieve all items visible to the optional session.
    fn get_items(&self, lister: &Self::Lister, transaction: &mut Trx) -> impl Future<Output = Result<Vec<Self::Item>, DbErr>>;
    /// Retrieve a single item by id if visible to the optional session.
    fn get_item(&self, id: Self::Id, access: Option<&'t str>, transaction: &mut Trx) -> impl Future<Output = Result<Self::Item, DbErr>>;
    /// Update an item. Access is validated by the service layer.
    fn update_item(&self, id: Self::Id, item: &Self::PatchItem, transaction: &mut Trx) -> impl Future<Output = Result<Self::Item, DbErr>>;
    /// Delete an item. Access is validated by the service layer.
    fn delete_item(&self, id: Self::Id, transaction: &mut Trx) -> impl Future<Output = Result<Self::Item, DbErr>>;
}

#[derive(Clone, Debug)]
pub struct QueryPager {
    pub limit: u32,
    pub offset: Option<i64>,
}


pub struct QueryFilter<'a, Id: Clone = i64, F = ()> {
    pub access: Option<&'a str>,
    pub filter: Option<F>,
    pub ids: Option<&'a Vec<Id>>,
    pub search: Option<&'a str>,
}

pub struct QueryLister<'a, Id: Clone = i64, F = ()> {
    pub filter: Option<QueryFilter<'a, Id, F>>,
    pub pager: QueryPager,
}

pub struct ListFilter<Id: Clone = i64, F = ()> {
    pub filter: Option<F>,
    pub ids: Option<Vec<Id>>,
    pub search: Option<String>,
}

pub struct ListParams<Id: Clone = i64, F = ()> {
    pub filter: Option<ListFilter<Id, F>>,
    pub pager: Option<QueryPager>,
}

impl<'a, Id: Clone, F> From<&'a ListParams<Id, F>> for QueryLister<'a, Id, F> {
    fn from(lister: &'a ListParams<Id, F>) -> Self {
        Self {
            pager: match &lister.pager {
                Some(p) => p.clone(),
                None => QueryPager {
                    limit: 100,
                    offset: None,
                },
            },
            filter: match &lister.filter {
                Some(f) => Some(QueryFilter {
                    access: None,
                    filter: None,
                    ids: f.ids.as_ref(),
                    search: f.search.as_deref(),
                }),
                None => None,
            },
        }
    }
}