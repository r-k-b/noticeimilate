extern crate chrono;
extern crate reqwest;
extern crate roxmltree;

use roxmltree::{Document, Node};
use std::str::ParseBoolError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let _response: String = reqwest::get("https://feeds.feedburner.com/Metafilter")?.text()?;
    let _example_feed = Feed {
        entries: Vec::new(),
    };
    Ok(())
}

#[derive(Debug)]
struct Feed {
    entries: Vec<FeedItem>,
}

/// `Timestamp` values are "seconds since the UNIX epoch"; if you're after milliseconds since then,
/// use `TimestampMillis`.
///
/// Cf <https://docs.rs/chrono/0.4.6/chrono/struct.DateTime.html#method.timestamp>
#[derive(Debug, PartialEq)]
struct Timestamp(pub i64);

#[derive(Debug, PartialEq)]
struct FeedItem {
    title: String,
    link: String,
    desc: String,
    guid: ItemGuid,
    publication_date: Timestamp,
}

#[derive(Debug, PartialEq)]
struct ItemGuid {
    is_permalink: bool,
    content: String,
}

#[derive(Debug)]
enum FeedItemError {
    DescriptionNodeMissing,
    GuidNodeEmpty,
    GuidNodeMissing,
    GuidNodeOmitsPermalink,
    GuidNodePermalinkInvalid(ParseBoolError),
    LinkNodeEmpty,
    LinkNodeMissing,
    PubDateNodeEmpty,
    PubDateNodeInvalid(chrono::ParseError),
    PubDateNodeMissing,
    TitleNodeEmpty,
    TitleNodeMissing,
}

#[derive(Debug)]
enum FeedError {
    XmlParseFailure(roxmltree::Error),
}

fn decode_feed(src: &str) -> Result<Vec<Result<FeedItem, FeedItemError>>, FeedError> {
    let doc: Document = match roxmltree::Document::parse(&src) {
        Ok(doc) => doc,
        Err(e) => return Err(FeedError::XmlParseFailure(e)),
    };

    //    let channel = doc.descendants();

    return Ok(doc
        .descendants()
        .map(|item_node| decode_feed_item(item_node))
        // fixme: don't eagerly evaluate the iterator here, leave that for later (How?)
        .collect());
}

fn decode_feed_item(item_node: Node) -> Result<FeedItem, FeedItemError> {
    let title = match item_node
        .descendants()
        .find(|n| n.tag_name().name() == "title")
    {
        None => return Err(FeedItemError::TitleNodeMissing),
        Some(node) => match node.text() {
            None => return Err(FeedItemError::TitleNodeEmpty),
            Some(text) => text,
        },
    };

    let link = match item_node
        .descendants()
        .find(|n| n.tag_name().name() == "link")
    {
        None => return Err(FeedItemError::LinkNodeMissing),
        Some(node) => match node.text() {
            None => return Err(FeedItemError::LinkNodeEmpty),
            Some(text) => text,
        },
    };

    let description = match item_node
        .descendants()
        .find(|n| n.tag_name().name() == "description")
    {
        None => return Err(FeedItemError::DescriptionNodeMissing),
        Some(node) => node.text().unwrap_or(""),
    };

    let guid: (&str, bool) = match item_node
        .descendants()
        .find(|n| n.tag_name().name() == "guid")
    {
        None => return Err(FeedItemError::GuidNodeMissing),
        Some(node) => (
            match node.text() {
                None => return Err(FeedItemError::GuidNodeEmpty),
                Some(text) => text,
            },
            match node.attribute("isPermaLink") {
                None => return Err(FeedItemError::GuidNodeOmitsPermalink),
                Some(contents) => match contents.parse::<bool>() {
                    Err(e) => return Err(FeedItemError::GuidNodePermalinkInvalid(e)),
                    Ok(b) => b,
                },
            },
        ),
    };

    let pub_date: Timestamp = match item_node
        .descendants()
        .find(|n| n.tag_name().name() == "pubDate")
    {
        None => return Err(FeedItemError::PubDateNodeMissing),
        Some(node) => match node.text() {
            None => return Err(FeedItemError::PubDateNodeEmpty),
            Some(text) => match chrono::DateTime::parse_from_rfc2822(text) {
                Err(err) => return Err(FeedItemError::PubDateNodeInvalid(err)),
                Ok(date) => Timestamp(date.timestamp()),
            },
        },
    };

    return Ok(FeedItem {
        title: String::from(title),
        link: String::from(link),
        desc: String::from(description),
        guid: ItemGuid {
            is_permalink: guid.1,
            content: String::from(guid.0),
        },
        publication_date: pub_date,
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
</rss>
"#;

        let expected = FeedItem {
            title: String::from("a Title"),
            link: String::from("http://example.com"),
            desc: String::from("Some Text"),
            guid: ItemGuid {
                is_permalink: false,
                content: String::from("tag:metafilter.com,2019:site.184490"),
            },
            publication_date: Timestamp(1575696199),
        };

        match decode_feed(sample) {
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
