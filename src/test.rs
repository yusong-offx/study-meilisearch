use meilisearch_sdk::{client::*, search::Selectors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: String,
    title: String,
}

pub async fn insert_test_data() {
    let client = Client::new("http://10.42.0.10:7700", Some("masterKey")).unwrap();

    let result = client
        .index("movies")
        .add_documents(
            &[
                Movie {
                    id: "1".to_string(),
                    title: "Carol".to_string(),
                },
                Movie {
                    id: "2".to_string(),
                    title: "Wonder Woman".to_string(),
                },
                Movie {
                    id: "3".to_string(),
                    title: "Life of Pi".to_string(),
                },
                Movie {
                    id: "4".to_string(),
                    title: "Mad Max: Fury Road".to_string(),
                },
                Movie {
                    id: "5".to_string(),
                    title: "Moana".to_string(),
                },
                Movie {
                    id: "6".to_string(),
                    title: "Philadelphia".to_string(),
                },
            ],
            Some("id"),
        )
        .await
        .unwrap();
    println!("{:#?}", result);
}

pub async fn search_data() {
    let client = Client::new("http://10.42.0.10:7700", Some("masterKey")).unwrap();

    let result = client
        .index("movies")
        .set_filterable_attributes(["id", "title"])
        .await
        .unwrap()
        .wait_for_completion(&client, None, None)
        .await
        .unwrap();

    let result = client
        .index("movies")
        .search()
        .with_query("m")
        .with_filter(r#"id = "m""#)
        .with_attributes_to_highlight(Selectors::Some(&["*"]))
        .execute::<Movie>()
        .await
        .unwrap();
    println!("{:#?}", result);
}
