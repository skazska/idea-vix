use axum::{Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

pub async fn get_boards() -> Json<Vec<Board>> {
    let boards = vec![
        Board { id: 1, name: "Board 1".to_string(), description: Some("Description for Board 1".to_string()), icon: Some("icon1.png".to_string()) },
        Board { id: 2, name: "Board 2".to_string(), description: Some("Description for Board 2".to_string()), icon: Some("icon2.png".to_string()) },
    ];

    Json(boards)
}
