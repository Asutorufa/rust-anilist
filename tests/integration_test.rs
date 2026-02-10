use rust_anilist::Client;
use rust_anilist::models::CharacterRole;

#[tokio::test]
#[ignore]
async fn test_get_anime_real_request() {
    let client = Client::default();
    let anime = client.get_anime(1).await.expect("Failed to fetch anime");

    println!("Anime: {}", anime.title.english());

    // Verify studios
    if let Some(studios) = &anime.studios {
        println!("Studios count: {}", studios.len());
        if !studios.is_empty() {
            if let Some(url) = &studios[0].url {
                println!("Studio[0] URL: {}", url);
            } else {
                println!("Studio[0] has no URL");
            }
        }
    } else {
        println!("Studios is None");
    }
    assert!(anime.studios.is_some(), "Studios should be populated");

    // Verify characters
    if let Some(characters) = &anime.characters {
        println!("Characters count: {}", characters.len());
        for (i, c) in characters.iter().enumerate().take(5) {
             println!("Character[{}]: {} (Role: {:?})", i, c.name.full().unwrap_or_default(), c.role);
        }
    } else {
         println!("Characters is None");
         panic!("Characters should be populated");
    }

    // Verify staff
    if let Some(staff) = &anime.staff {
        println!("Staff count: {}", staff.len());
        assert!(!staff.is_empty(), "Staff list should not be empty");
        for (i, p) in staff.iter().enumerate().take(5) {
            println!("Staff[{}]: {} (ID: {})", i, p.name.full().unwrap_or_default(), p.id);
        }
    } else {
        println!("Staff is None");
        panic!("Staff should be populated");
    }
}
