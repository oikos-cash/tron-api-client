// use chrono::{Duration, Utc};
// use lazy_static::lazy_static;
// use tokio::sync::{Mutex, MutexGuard};

use tron_api_client::Client;

// mod data;

// use data::*;

fn get_client() -> Client {
    let client = Client::new("https://api.shasta.trongrid.io".into());
    client
}

#[tokio::test]
async fn node_info() {
    let client = get_client();

    let info = client
        .node_info()
        .await
        .expect("Error fetching node info");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_num() {
    let client = get_client();

    let info = client
        .get_block_by_num(10)
        .await
        .expect("Error fetching node info");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_id() {
    let client = get_client();

    let info = client
        .get_block_by_id("000000000000000a4efe701d7a03ff578104c6c1995ab70e713c30318b266e90")
        .await
        .expect("Error fetching node info");
    dbg!(info);
}




/*
#[tokio::test]
async fn search() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let cases = vec![
        ("name", SearchBy::Name(&PEII.series_name)),
        ("IMDb ID", SearchBy::IMDbID(&PEII.imdb_id)),
        ("Zap2it ID", SearchBy::Zap2itID(&PEII.zap2it_id)),
        ("slug", SearchBy::Slug(&PEII.slug)),
    ];

    for (case_name, search_by) in cases.into_iter() {
        let series = client
            .search(search_by)
            .await
            .unwrap()
            .into_iter()
            .find(|s| *s == *PEII);

        if series.is_none() {
            panic!("Expected series missing from {} search results", case_name)
        }
    }
}

#[tokio::test]
async fn series() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let peii = client.series(PEII.id).await.expect("Error fetching series");

    assert_eq!(peii, *PEII);
}
*/


/*
#[tokio::test]
async fn series_filter() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let keys = SeriesFilterKeys::new()
        .id()
        .series_name()
        .first_aired()
        .network()
        .slug()
        .status()
        .imdb_id()
        .zap2it_id();

    let series = client
        .series_filter(PEII.id, &keys)
        .await
        .expect("Error fetching filtered series");

    assert_eq!(series, *PEII);
}
*/


/*
#[tokio::test]
async fn updated() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let params = UpdatedParams::new(Utc::now() - Duration::days(1));

    let updates = client.updated(&params).await;

    if updates.is_err() {
        panic!("Error fetching updated series: {:?}", updates.unwrap_err());
    }
}
*/

/*
#[tokio::test]
async fn movie() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let movie = client.movie(TSR.id).await.expect("Error fetching movie");

    assert_eq!(movie, *TSR);

    let genre = movie.genres.iter().find(|g| *g == &*DRAMA);
    if genre.is_none() {
        panic!("Expected genre missing from movie genre list");
    }

    let translation = movie.translations.iter().find(|t| *t == &*TSR_ENG);
    if translation.is_none() {
        panic!("Expected translation missing from movie translation list");
    }

    let release_date = movie.release_dates.iter().find(|r| *r == &*RELEASE);
    if release_date.is_none() {
        panic!("Expected release date missing from movie release dates");
    }

    let remote_id = movie.remoteids.iter().find(|r| *r == &*TSR_IMDB);
    if remote_id.is_none() {
        panic!("Expected remote id missing from movie remote id list");
    }

    let actor = movie.people.actors.iter().find(|a| *a == &*ANDY);
    if actor.is_none() {
        panic!("Expected actor missing from movie actor list");
    }
}
*/

/*
#[tokio::test]
async fn movie_updates() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let since = Utc::now() - Duration::days(1);

    let movie_updates = client.movie_updates(since).await;

    if movie_updates.is_err() {
        panic!( "Error fetching movie updates: {:?}", movie_updates.unwrap_err());
    }
}
*/

