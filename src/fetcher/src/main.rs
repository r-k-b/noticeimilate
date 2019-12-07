extern crate reqwest;

use roxmltree::{Document, Node};
use std::time;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let _response: String = reqwest::get("https://feeds.feedburner.com/Metafilter")?.text()?;
    let _example_feed = Feed {
        entries: Vec::new(),
    };
    Ok(())
}

#[derive(Debug)]
struct Feed<'a> {
    entries: Vec<FeedItem<'a>>,
}

#[derive(Debug, PartialEq)]
struct FeedItem<'a> {
    title: String,
    link: &'a str,
    desc: &'a str,
    guid: ItemGuid<'a>,
    publication_date: time::SystemTime,
}

#[derive(Debug, PartialEq)]
struct ItemGuid<'a> {
    is_permalink: bool,
    content: &'a str,
}

#[derive(Debug)]
enum FeedItemError {
    TitleNodeEmpty,
    TitleNodeMissing,
}

#[derive(Debug)]
enum FeedError {
    XmlParseFailure(roxmltree::Error),
}

fn decode_feed<'a, I>(now: SystemTime, src: &str) -> Result<I, FeedError>
where
    I: IntoIterator<Item = Result<FeedItem<'a>, FeedItemError>>,
{
    let doc: Document = match roxmltree::Document::parse(&src) {
        Ok(doc) => doc,
        Err(e) => return Result::Err(FeedError::XmlParseFailure(e)),
    };

    //    let channel = doc.descendants();

    return Result::ok(
        doc.descendants()
            .map(|itemNode| decode_feed_item(now, itemNode)),
    );
}

fn decode_feed_item<'a>(now: SystemTime, item_node: Node) -> Result<FeedItem<'a>, FeedItemError> {
    let elem = match item_node
        .descendants()
        .find(|n| n.tag_name().name() == "title")
    {
        None => return Result::Err(FeedItemError::TitleNodeMissing),
        Some(node) => match node.text() {
            None => return Result::Err(FeedItemError::TitleNodeEmpty),
            Some(text) => text,
        },
    };

    return Result::Ok(FeedItem {
        title: String::from(elem),
        link: "",
        desc: "",
        guid: ItemGuid {
            is_permalink: false,
            content: "",
        },
        publication_date: now,
    });
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::borrow::Borrow;
    use std::time::SystemTime;

    #[test]
    fn test_decode_feed() {
        let sample = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:admin="http://webns.net/mvcb/"
     xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:wfw="http://wellformedweb.org/CommentAPI/" xmlns:feedburner="http://rssnamespace.org/feedburner/ext/1.0"
     version="2.0">
    <channel>
        <item>
            <title>a Title</title>
            <link>http://example.com</link>
            <description>Some Text</description>
            <guid isPermaLink="false">tag:metafilter.com,2019:site.184490</guid>
            <pubDate>Sat, 07 Dec 2019 05:23:19 GMT</pubDate>
            <category>catA</category>
            <category>catB</category>
            <dc:creator>Rumple</dc:creator>
            <wfw:commentRss>http://example.com/rss</wfw:commentRss>
        </item>
    </channel>
</rss>"#;
        let now = SystemTime::now();

        let expected = FeedItem {
            title: String::from("a Title"),
            link: "",
            desc: "",
            guid: ItemGuid {
                is_permalink: false,
                content: "",
            },
            publication_date: now,
        };

        match decode_feed::<Iterator>(now, sample) {
            Ok(items) => match items.first() {
                None => assert!(false, "no items were returned"),
                Some(item) => match item {
                    Ok(actual) => {
                        assert_eq!(actual, expected.borrow());
                    }
                    Err(err) => assert!(false, "the item failed parsing: {:?}", err),
                },
            },
            Err(err) => assert!(false, "Decoding failed: {:?}", err),
        }
    }
}
