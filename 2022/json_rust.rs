use serde::{Deserialize,Serialize};
use reqwest::blocking::get;
// cerner_2tothe5th_2022
// Learning more rust, this is from this turtorial 
// https://ectobit.com/blog/parsing-json-in-rust/
fn main() {
  let res = get("https://www.songsterr.com/a/ra/songs.json?pattern=Lemon Demon").unwrap();
  let songs = res.json::<Response>().unwrap();
  for song in songs {
      println!("Song: {} Artist: {:?}", song.title, song.artist.name);
  }
}
pub type Response = Vec<Song>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub artist: Artist,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name_without_the_prefix: String,
    pub use_the_prefix: bool,
    pub name: String,
}%
