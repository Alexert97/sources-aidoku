#![no_std]
use aidoku::{Source, prelude::*};
use mangaworld_template::{Impl, MangaWorldTemplate, Params};

const BASE_URL: &str = "https://www.mangaworld.me";

struct MangaWorld;

impl Impl for MangaWorld {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
		}
	}
}

register_source!(MangaWorldTemplate<MangaWorld>, DeepLinkHandler);

#[cfg(test)]
mod tests {
	use super::*;
	use aidoku::alloc::{String, Vec};
	use aidoku_test::aidoku_test;

	#[aidoku_test]
	fn catalog_returns_entries() {
		let source = MangaWorldTemplate::<MangaWorld>::new();
		let result = source
			.get_search_manga_list(None, 1, Vec::new())
			.expect("catalog request should succeed");
		assert!(!result.entries.is_empty(), "catalog returned no manga");
		assert!(result.entries.iter().all(|m| !m.title.is_empty() && m.key.starts_with("/manga/")));
	}

	#[aidoku_test]
	fn search_details_chapters_and_reader() {
		let source = MangaWorldTemplate::<MangaWorld>::new();
		let result = source
			.get_search_manga_list(Some(String::from("One Piece")), 1, Vec::new())
			.expect("search request should succeed");
		let manga = result.entries.into_iter()
			.find(|m| m.title.eq_ignore_ascii_case("One Piece"))
			.expect("search should return One Piece");
		let mut manga = source.get_manga_update(manga, true, true)
			.expect("details and chapters request should succeed");
		assert!(!manga.title.is_empty());
		assert!(manga.cover.is_some(), "missing cover");
		let chapter = manga.chapters.take().expect("missing chapter list")
			.into_iter().next().expect("chapter list is empty");
		let pages = source.get_page_list(manga, chapter)
			.expect("reader request should succeed");
		assert!(!pages.is_empty(), "reader returned no pages");
	}
}
