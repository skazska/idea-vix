use crate::error::ModelError;


pub trait CrudService {
    type Item;
    type NewItem;
    type PatchItem;
    type SessionData;
    type Lister;
    type Id;

    fn add_item(&self, item: Self::NewItem, session: &Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>>;
    fn get_items(&self, lister: Self::Lister, session: Option<&Self::SessionData>) -> impl Future<Output = Result<Vec<Self::Item>, ModelError>>;
    fn get_item(&self, id: Self::Id, session: Option<&Self::SessionData>) -> impl Future<Output = Result<Self::Item, ModelError>>;
    fn update_item(&self, id: Self::Id, item: Self::PatchItem, session: &Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>>;
    fn delete_item(&self, id: Self::Id, session: &Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>>;
}

pub trait CrudQueries<'a> {
    type Item;
    type NewItem;
    type PatchItem;
    type Lister;
    type Error;
    type Transaction;
    type Id: Send + Sync + Unpin;

    /// Insert a new item
    fn add_item(&self, item: &Self::NewItem, transaction: &mut Self::Transaction) -> impl Future<Output = Result<Self::Item, Self::Error>>;
    /// Retrieve all items visible to the optional session.
    fn get_items(&self, lister: &Self::Lister, transaction: &mut Self::Transaction) -> impl Future<Output = Result<Vec<Self::Item>, Self::Error>>;
    /// Retrieve a single item by id if visible to the optional session.
    fn get_item(&self, id: Self::Id, access: Option<&'a str>, transaction: &mut Self::Transaction) -> impl Future<Output = Result<Self::Item, Self::Error>>;
    /// Update an item. Access is validated by the service layer.
    fn update_item(&self, id: Self::Id, item: &Self::PatchItem, transaction: &mut Self::Transaction) -> impl Future<Output = Result<Self::Item, Self::Error>>;
    /// Delete an item. Access is validated by the service layer.
    fn delete_item(&self, id: Self::Id, transaction: &mut Self::Transaction) -> impl Future<Output = Result<Self::Item, Self::Error>>;
}

#[derive(Clone, Debug)]
pub struct QueryPager {
    pub limit: u32,
    pub offset: Option<i64>,
}


pub struct QueryFilter<Id = i64, F = ()> {
    pub access: Option<String>,
    pub filter: Option<F>,
    pub ids: Option<Vec<Id>>,
    pub search: Option<String>,
}

pub struct QueryLister<Id = i64, F = ()> {
    pub filter: Option<QueryFilter<Id, F>>,
    pub pager: QueryPager,
}

pub struct ListFilter<Id = i64, F = ()> {
    pub filter: Option<F>,
    pub ids: Option<Vec<Id>>,
    pub search: Option<String>,
}

pub struct ListParams<Id = i64, F = ()> {
    pub filter: Option<QueryFilter<Id, F>>,
    pub pager: Option<QueryPager>,
}

impl<'a, Id, F> From<ListParams<Id, F>> for QueryLister<Id, F> {
    fn from(lister: ListParams<Id, F>) -> Self {
        Self {
            pager: match lister.pager {
                Some(p) => p.clone(),
                None => QueryPager {
                    limit: 100,
                    offset: None,
                },
            },
            filter: match lister.filter {
                Some(f) => Some(QueryFilter {
                    access: None,
                    filter: f.filter,
                    ids: f.ids,
                    search: f.search,
                }),
                None => None,
            },
        }
    }
}