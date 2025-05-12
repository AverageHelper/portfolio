use chrono::NaiveDate;
use markdown::{Constructs, ParseOptions, mdast::Node};
use regex_static::lazy_regex;
use serde::Deserialize;
use std::{cmp::Ordering, env, fs, io::ErrorKind, path::Path};

fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();

	// Construct a ways.gmi index file:
	let ways = "src/content/ways";
	let original_ways = fs::read_dir(ways).unwrap().flat_map(|entry| entry.ok());
	let mut ways_meta: Vec<WaysMetaWithSlug> = original_ways
		.flat_map(|entry| match fs::read(entry.path()) {
			Err(_) => None,
			Ok(file) => Some((entry, file)),
		})
		.map(|(entry, file)| (entry.file_name(), WaysMeta::from(file)))
		.flat_map(
			|(file_name, file)| match file_name.to_string_lossy().strip_suffix(".md") {
				None => None,
				Some(slug) => Some((slug.to_string(), file)),
			},
		)
		.map(WaysMetaWithSlug::from)
		.collect();
	ways_meta.sort();

	let articles_list = ways_meta
		.iter()
		.map(|w| format!("{w}"))
		.collect::<Vec<_>>()
		.join("\n");

	let ways_root = format!(
		"# Ways

{articles_list}

=> / Return home
"
	);
	let ways_gmi = Path::new(&out_dir).join("ways.gmi");
	fs::write(ways_gmi, ways_root).unwrap();

	// Transform src/content/ways/*.md into ./ways/*.gmi files
	let ways_container = Path::new(&out_dir).join("ways");
	match fs::create_dir(&ways_container) {
		Ok(()) => {}
		Err(err) if err.kind() == ErrorKind::AlreadyExists => {}
		Err(err) => panic!("{err}"),
	}

	let original_ways = fs::read_dir(ways).unwrap().flat_map(|entry| entry.ok());
	for original_md in original_ways {
		if original_md.file_type().unwrap().is_dir() {
			// Skip subdirectories
			continue;
		}

		let file_name_raw = original_md.file_name();
		let file_name = file_name_raw.to_str().unwrap();
		let slug = file_name.strip_suffix(".md").unwrap();
		let content_bytes = fs::read(original_md.path()).unwrap();
		let markdown_text = String::from_utf8(content_bytes.to_vec()).unwrap();

		// Convert file to Gemtext
		let content = gemtext_from_markdown(&markdown_text);
		let final_name = format!("{slug}.gmi");
		let final_content = format!(
			"{content}
-----

=> https://creativecommons.org/publicdomain/zero/1.0 Ways by Average Helper is marked with CC0 1.0
=> /ways Return to Ways
"
		);
		let dest_path = Path::new(&ways_container).join(final_name);
		fs::write(dest_path, final_content).unwrap();
	}

	// Create a route function that embeds all of these files and serves them at appropriate slugs
	let map = ways_meta
		.iter()
		.map(|meta_with_slug| &meta_with_slug.0)
		.map(|slug| {
			format!(
				r#"		"/ways/{}" => Some(include_str!("./ways/{}.gmi")),"#,
				slug, slug
			)
		})
		.collect::<Vec<_>>()
		.join("\n");

	let dest_path = Path::new(&out_dir).join("ways.rs");
	let ways_from_slug_fn = format!(
		r#"/// Returns the Ways document with the given slug (including the leading `/ways/` prefix).
fn ways_from_slug(slug: &str) -> Option<&'static str> {{
	match slug {{
{map}
		_ => None,
	}}
}}
"#
	);
	fs::write(&dest_path, ways_from_slug_fn).unwrap();

	// Rebuild if either Ways or this script change
	println!("cargo::rerun-if-changed=src/content/ways");
	println!("cargo::rerun-if-changed=build.rs");
}

type Lazy<T> = regex_static::once_cell::sync::Lazy<T>;
pub type Regex = Lazy<regex::Regex>;

const FRONTMATTER: Regex = lazy_regex!(r#"(?m)^---[\S\s\r\n]+title: "(.+)"[\S\s\r\n]+?---"#);

fn gemtext_from_markdown(markdown_text: &str) -> String {
	// Replace the frontmatter with only the title meta
	let rep = if let Some(captures) = FRONTMATTER.captures(markdown_text) {
		let title = captures.get(1).expect("Valid capture index");
		format!("# {}", title.as_str())
	} else {
		String::new()
	};

	let remaining_markdown = FRONTMATTER.replace(markdown_text, rep);
	md2gemtext::convert(&remaining_markdown)
}

fn markdown_ast(markdown_text: &str) -> Node {
	let options = ParseOptions {
		constructs: Constructs {
			frontmatter: true,
			..Default::default()
		},
		..Default::default()
	};

	markdown::to_mdast(markdown_text, &options).expect("Valid markdown")
}

#[derive(Deserialize, PartialEq, Eq)]
struct WaysMeta {
	title: String,
	description: String,
	date: NaiveDate,
}

impl From<Vec<u8>> for WaysMeta {
	fn from(data: Vec<u8>) -> Self {
		let markdown_text = String::from_utf8(data).unwrap();
		let ast = markdown_ast(&markdown_text);
		if let Node::Root(root) = ast {
			if let Some(Node::Yaml(yaml)) = root.children.first() {
				return serde_yml::from_str(&yaml.value).expect("Valid YAML");
			}
		}
		panic!("Malformed or missing Markdown frontmatter");
	}
}

impl Ord for WaysMeta {
	fn cmp(&self, other: &Self) -> Ordering {
		other.date.cmp(&self.date)
	}
}

impl PartialOrd for WaysMeta {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(PartialEq, Eq)]
struct WaysMetaWithSlug(String, WaysMeta);

impl std::fmt::Display for WaysMetaWithSlug {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let slug = &self.0;
		let file = &self.1;
		let date = file.date.format("%b %e, %Y");
		write!(f, "=> /ways/{} {} ({})", slug, file.title, date)
	}
}

impl From<(String, WaysMeta)> for WaysMetaWithSlug {
	fn from(value: (String, WaysMeta)) -> Self {
		Self(value.0, value.1)
	}
}

impl Ord for WaysMetaWithSlug {
	fn cmp(&self, other: &Self) -> Ordering {
		self.1.cmp(&other.1)
	}
}

impl PartialOrd for WaysMetaWithSlug {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
