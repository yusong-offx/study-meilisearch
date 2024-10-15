mod test;

#[tokio::main]
async fn main() {
    println!("Start!");

    // test::insert_test_data().await;
    test::search_data().await;
    println!("Finish!")
}
