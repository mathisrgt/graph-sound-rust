mod fetchdata;
use fetchdata::fetch_data_and_return_percentage_variations;

mod soundplay;
use soundplay::play_audio;

async fn play(month: u32) {
    let percentage_variations = fetch_data_and_return_percentage_variations(month).await.unwrap();
    let mut index = 29;
    for variation in percentage_variations {
        index = (index as i64 + variation.percentage_variation).rem_euclid(63) as usize;
        play_audio(index).unwrap();
        println!("Date: {}, Variation: {}%, Index: {}", variation.date,variation.percentage_variation,index);
    }
}

#[tokio::main]
async fn main() {
    play(11).await;
    // let handle1 = tokio::spawn(play(3));
    // let handle2 = tokio::spawn(play(4));
    
    // tokio::try_join!(handle1, handle2).unwrap();
}