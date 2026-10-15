//! The description rules of R07, one behaviour per test. Inputs are synthetic.

mod common;

use deadloader_html::{ImageRef, Link, MAX_INPUT_BYTES, Sanitized, sanitize};

fn html(input: &str) -> String {
    let out = sanitize(input);
    common::assert_safe(input, &out);
    out.html
}

fn link(url: &str, host: &str) -> Link {
    Link {
        url: url.into(),
        host: host.into(),
        internal_mod_id: None,
    }
}

// --- tags -------------------------------------------------------------------------------------------------------

#[test]
fn plain_text_and_line_breaks_pass_through() {
    assert_eq!(html("one<br>two"), "one<br>two");
    assert_eq!(html("one<br/>two<BR>three"), "one<br>two<br>three");
}

#[test]
fn allowed_tags_are_kept() {
    let input = "<p><b>b</b><strong>s</strong><i>i</i><em>e</em><u>u</u><s>s</s><del>d</del><code>c</code></p>\
                 <h1>1</h1><h2>2</h2><h3>3</h3><h4>4</h4><h5>5</h5><h6>6</h6><hr>\
                 <ul><li>a</li></ul><ol><li>b</li></ol><blockquote>q</blockquote><div><span>x</span></div>";
    assert_eq!(html(input), input);
}

#[test]
fn tag_names_are_lowercased() {
    assert_eq!(html("<B>x</B><H2>y</H2>"), "<b>x</b><h2>y</h2>");
}

#[test]
fn tables_are_kept_with_spans() {
    assert_eq!(
        html(
            "<table><thead><tr><th colspan=\"2\">h</th></tr></thead><tr><td rowspan=\"3\">a</td></tr></table>"
        ),
        "<table><thead><tr><th colspan=\"2\">h</th></tr></thead><tbody><tr><td rowspan=\"3\">a</td></tr></tbody></table>"
    );
}

#[test]
fn spans_must_be_sane_numbers() {
    assert_eq!(
        html(
            "<table><tr><td colspan=\"x\">a</td><td colspan=\"0\">b</td><td colspan=\"99999\">c</td></tr></table>"
        ),
        "<table><tbody><tr><td>a</td><td>b</td><td>c</td></tr></tbody></table>"
    );
}

#[test]
fn ordered_list_start_is_kept_when_numeric() {
    assert_eq!(
        html("<ol start=\"3\"><li>a</li></ol>"),
        "<ol start=\"3\"><li>a</li></ol>"
    );
    assert_eq!(
        html("<ol start=\"-2\"><li>a</li></ol>"),
        "<ol start=\"-2\"><li>a</li></ol>"
    );
    assert_eq!(
        html("<ol start=\"3;x\"><li>a</li></ol>"),
        "<ol><li>a</li></ol>"
    );
}

#[test]
fn preformatted_text_keeps_its_leading_newline() {
    // The parser drops one newline right after <pre>; the output must survive the webview's parse the same way.
    assert_eq!(html("<pre>\n\n  code\n</pre>"), "<pre>\n\n  code\n</pre>");
}

#[test]
fn unknown_tags_are_dropped_keeping_their_text() {
    assert_eq!(
        html("<font color=\"red\">a</font><time>b</time><name>c</name><center>d</center>"),
        "abcd"
    );
}

#[test]
fn dangerous_elements_are_removed_with_their_content() {
    for tag in [
        "script", "style", "object", "svg", "math", "form", "button", "textarea", "select",
        "template", "noscript", "title", "audio", "video", "canvas",
    ] {
        assert_eq!(
            html(&format!("a<{tag}>hidden</{tag}>b")),
            "ab",
            "<{tag}> must go with its content"
        );
    }
    assert_eq!(html("a<input value=\"hidden\">b"), "ab");
    assert_eq!(html("a<embed src=\"https://x.example/a.swf\">b"), "ab");
}

#[test]
fn comments_doctype_and_head_elements_are_dropped() {
    assert_eq!(
        html(
            "<!DOCTYPE html><!-- note -->a<meta charset=\"utf-8\"><link rel=\"x\" href=\"https://a.com\"><base href=\"https://a.com/\">b"
        ),
        "ab"
    );
}

#[test]
fn text_is_escaped() {
    assert_eq!(
        html("1 &lt; 2 &amp;&amp; 3 &gt; 2 \"q\" 'a'"),
        "1 &lt; 2 &amp;&amp; 3 &gt; 2 \"q\" 'a'"
    );
    assert_eq!(html("a &nbsp;b"), "a \u{a0}b");
}

// --- attributes and classes ---------------------------------------------------------------------------------------

#[test]
fn foreign_attributes_are_dropped() {
    assert_eq!(
        html(
            "<p id=\"x\" dir=\"rtl\" tabindex=\"1\" title=\"t\" data-darkreader-inline-color=\"\" data-path-to-node=\"3\">x</p>"
        ),
        "<p>x</p>"
    );
}

#[test]
fn gamebanana_classes_map_to_ours() {
    assert_eq!(
        html(
            "<span class=\"GreenColor\">g</span><span class=\"RedColor SelectedElement\">r</span>"
        ),
        "<span class=\"dl-green\">g</span><span class=\"dl-red\">r</span>"
    );
    assert_eq!(
        html("<b class=\"GreenColor\">g</b>"),
        "<b class=\"dl-green\">g</b>"
    );
}

#[test]
fn spoiler_becomes_our_collapsed_element() {
    assert_eq!(
        html("<span class=\"Spoiler\">secret</span>"),
        "<span class=\"dl-spoiler\">secret</span>"
    );
    assert_eq!(
        html("<div class=\"Spoiler GreenColor\">s</div>"),
        "<div class=\"dl-green dl-spoiler\">s</div>"
    );
}

#[test]
fn other_classes_are_dropped_including_ours() {
    assert_eq!(
        html("<span class=\"markdown-heading octicon dl-green dl-image greencolor\">x</span>"),
        "<span>x</span>"
    );
    assert_eq!(
        html("<span class=\"GreenColor GreenColor\">x</span>"),
        "<span class=\"dl-green\">x</span>"
    );
}

// --- links --------------------------------------------------------------------------------------------------------

#[test]
fn links_become_inert_references() {
    let out = sanitize(
        "see <a href=\"https://github.com/someone/repo\" target=\"_blank\" rel=\"x\">repo</a>",
    );
    assert_eq!(out.html, "see <a data-dl-link=\"0\">repo</a>");
    assert_eq!(
        out.links,
        vec![link("https://github.com/someone/repo", "github.com")]
    );
}

#[test]
fn links_are_numbered_in_document_order() {
    let out = sanitize(
        "<a href=\"https://a.example.com/\">a</a><a href=\"https://b.example.com/\">b</a><a href=\"https://a.example.com/\">a</a>",
    );
    assert_eq!(
        out.html,
        "<a data-dl-link=\"0\">a</a><a data-dl-link=\"1\">b</a><a data-dl-link=\"2\">a</a>"
    );
    assert_eq!(out.links.len(), 3);
    assert_eq!(out.links[1].url, "https://b.example.com/");
}

#[test]
fn protocol_relative_links_become_https() {
    let out = sanitize("<a href=\"//x.com/someone\">x</a>");
    assert_eq!(out.links, vec![link("https://x.com/someone", "x.com")]);
}

#[test]
fn http_links_are_allowed() {
    let out = sanitize("<a href=\"http://example.com/a\">x</a>");
    assert_eq!(out.links, vec![link("http://example.com/a", "example.com")]);
}

#[test]
fn links_without_a_usable_url_keep_only_their_text() {
    for href in [
        "",
        "#top",
        "/mods/1",
        "mods/1",
        "?a=1",
        "mailto:a@example.com",
        "ftp://example.com/",
        "steam://run/1422450",
        "file:///C:/Windows",
    ] {
        let out = sanitize(&format!("<a href=\"{href}\">t</a>"));
        assert_eq!(out.html, "t", "href {href:?}");
        assert!(out.links.is_empty(), "href {href:?}");
    }
    assert_eq!(html("<a>t</a><a name=\"x\">u</a>"), "tu");
}

#[test]
fn urls_are_normalised_and_hosts_lowercased() {
    let out = sanitize("<a href=\"  HTTPS://GitHub.COM/A%20B c  \">x</a>");
    assert_eq!(
        out.links,
        vec![link("https://github.com/A%20B%20c", "github.com")]
    );
}

#[test]
fn international_hosts_are_reported_in_punycode() {
    let out = sanitize("<a href=\"https://bücher.example/\">x</a>");
    assert_eq!(
        out.links,
        vec![link(
            "https://xn--bcher-kva.example/",
            "xn--bcher-kva.example"
        )]
    );
}

#[test]
fn credentials_in_urls_are_stripped() {
    let out = sanitize("<a href=\"https://gamebanana.com:secret@evil.example/\">x</a>");
    assert_eq!(
        out.links,
        vec![link("https://evil.example/", "evil.example")]
    );
}

#[test]
fn local_and_ip_hosts_are_refused() {
    for href in [
        "http://localhost/",
        "http://localhost:8080/x",
        "http://app.localhost/",
        "http://printer.local/",
        "http://router/",
        "http://127.0.0.1/",
        "http://192.168.0.1/",
        "http://[::1]/",
        "http://2130706433/",
        "http://0x7f.1/",
    ] {
        let out = sanitize(&format!("<a href=\"{href}\">t</a>"));
        assert!(out.links.is_empty(), "href {href:?} gave {:?}", out.links);
    }
}

#[test]
fn overlong_urls_are_refused() {
    let out = sanitize(&format!(
        "<a href=\"https://example.com/{}\">t</a>",
        "a".repeat(5000)
    ));
    assert_eq!(out.html, "t");
    assert!(out.links.is_empty());
}

#[test]
fn gamebanana_mod_links_are_marked_internal() {
    let out = sanitize(
        "<a href=\"https://gamebanana.com/mods/637275\">a</a>\
         <a href=\"https://www.gamebanana.com/mods/42/\">b</a>\
         <a href=\"https://gamebanana.com/mods/download/601444#FileInfo_1\">c</a>\
         <a href=\"https://gamebanana.com/members/3961199\">d</a>\
         <a href=\"https://gamebanana.com.evil.example/mods/1\">e</a>\
         <a href=\"https://gamebanana.com/mods/0\">f</a>",
    );
    let ids: Vec<_> = out.links.iter().map(|l| l.internal_mod_id).collect();
    assert_eq!(ids, vec![Some(637275), Some(42), None, None, None, None]);
}

#[test]
fn gamebanana_link_filter_is_unwrapped_to_its_target() {
    let out = sanitize(
        "<a href=\"https://gamebanana.com/linkfilter?url=https%3A%2F%2Fko-fi.com%2Fsomeone&amp;follow=true\">k</a>",
    );
    assert_eq!(
        out.links,
        vec![link("https://ko-fi.com/someone", "ko-fi.com")]
    );
}

#[test]
fn link_filter_with_a_bad_target_is_dropped() {
    for target in ["javascript%3Aalert(1)", "", "%2Fmods%2F1"] {
        let out = sanitize(&format!(
            "<a href=\"https://gamebanana.com/linkfilter?url={target}\">k</a>"
        ));
        assert_eq!(out.html, "k", "target {target:?}");
        assert!(out.links.is_empty(), "target {target:?}");
    }
}

#[test]
fn nested_links_do_not_nest_references() {
    let out = sanitize(
        "<a href=\"https://a.example.com/\"><span><a href=\"https://b.example.com/\">x</a></span></a>",
    );
    common::assert_safe("nested", &out);
    assert!(!out.html.contains("<a data-dl-link=\"0\"><span><a"));
}

#[test]
fn link_classes_are_kept() {
    assert_eq!(
        html("<a class=\"RedColor\" href=\"https://a.example.com/\">x</a>"),
        "<a class=\"dl-red\" data-dl-link=\"0\">x</a>"
    );
}

// --- images -------------------------------------------------------------------------------------------------------

#[test]
fn images_become_placeholders() {
    let out = sanitize(
        "<img src=\"https://i.imgur.com/AbC.png\" alt=\" a cat \" class=\"emote-inline\" loading=\"lazy\" width=\"5\">",
    );
    assert_eq!(
        out.html,
        "<span class=\"dl-image\" data-dl-image=\"0\"></span>"
    );
    assert_eq!(
        out.images,
        vec![ImageRef {
            url: "https://i.imgur.com/AbC.png".into(),
            host: "i.imgur.com".into(),
            alt: Some("a cat".into()),
        }]
    );
}

#[test]
fn image_hosts_are_lowercase_and_protocol_relative_is_https() {
    let out = sanitize(
        "<img src=\"//Files.GameBanana.com/img/ss/a.jpg\" alt=\"\"><img src=\"http://CDN.7tv.app/e/2x.webp\">",
    );
    assert_eq!(
        out.images,
        vec![
            ImageRef {
                url: "https://files.gamebanana.com/img/ss/a.jpg".into(),
                host: "files.gamebanana.com".into(),
                alt: None,
            },
            ImageRef {
                url: "http://cdn.7tv.app/e/2x.webp".into(),
                host: "cdn.7tv.app".into(),
                alt: None,
            },
        ]
    );
}

#[test]
fn images_without_a_usable_url_disappear() {
    for src in [
        "",
        "a.png",
        "/img/a.png",
        "data:image/png;base64,AAAA",
        "http://127.0.0.1/a.png",
    ] {
        let out = sanitize(&format!("x<img src=\"{src}\" alt=\"alt\">y"));
        assert_eq!(out.html, "xy", "src {src:?}");
        assert!(out.images.is_empty(), "src {src:?}");
    }
}

#[test]
fn images_inside_links_keep_both_references() {
    let out = sanitize(
        "<a href=\"https://ko-fi.com/someone\"><img src=\"https://files.gamebanana.com/b.png\" alt=\"Buy\"></a>",
    );
    assert_eq!(
        out.html,
        "<a data-dl-link=\"0\"><span class=\"dl-image\" data-dl-image=\"0\"></span></a>"
    );
    assert_eq!(out.links.len(), 1);
    assert_eq!(out.images.len(), 1);
}

// --- embeds -------------------------------------------------------------------------------------------------------

#[test]
fn youtube_embeds_become_link_cards() {
    let out = sanitize(
        "a<iframe width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/w17H3wX_0Zw\" frameborder=\"0\" allowfullscreen=\"\"></iframe>b",
    );
    assert_eq!(
        out.html,
        "a<span class=\"dl-youtube\" data-dl-link=\"0\"></span>b"
    );
    assert_eq!(
        out.links,
        vec![link(
            "https://www.youtube.com/watch?v=w17H3wX_0Zw",
            "www.youtube.com"
        )]
    );
}

#[test]
fn youtube_hosts_and_protocol_relative_embeds_are_recognised() {
    for src in [
        "//www.youtube.com/embed/abc_DEF-123",
        "https://youtube.com/embed/abc_DEF-123?start=10",
        "https://www.youtube-nocookie.com/embed/abc_DEF-123",
        "http://m.youtube.com/embed/abc_DEF-123",
    ] {
        let out = sanitize(&format!("<iframe src=\"{src}\"></iframe>"));
        assert_eq!(
            out.links,
            vec![link(
                "https://www.youtube.com/watch?v=abc_DEF-123",
                "www.youtube.com"
            )],
            "src {src:?}"
        );
    }
}

#[test]
fn other_iframes_are_removed() {
    for src in [
        "https://evil.example/embed/abc",
        "https://www.youtube.com/watch?v=abc",
        "https://www.youtube.com/embed/",
        "https://www.youtube.com/embed/a%22b",
        "https://www.youtube.com.evil.example/embed/abc",
        "javascript:alert(1)",
    ] {
        let out = sanitize(&format!("a<iframe src=\"{src}\">fallback</iframe>b"));
        assert_eq!(out.html, "ab", "src {src:?}");
        assert!(out.links.is_empty(), "src {src:?}");
    }
}

// --- limits -------------------------------------------------------------------------------------------------------

#[test]
fn small_inputs_are_not_truncated() {
    assert!(!sanitize("x").truncated);
    assert_eq!(sanitize(""), Sanitized::default());
}

#[test]
fn oversized_inputs_are_truncated_safely() {
    let tail = "<a href=\"javascript:alert(1)\">x</a><script>alert(1)</script>";
    let input = format!("{}{tail}", "é".repeat(MAX_INPUT_BYTES / 2 - 3));
    let out = sanitize(&input);
    common::assert_safe("oversized", &out);
    assert!(out.truncated);
    assert!(out.links.is_empty());
    assert!(!out.html.contains("alert"));
    assert!(out.html.len() <= MAX_INPUT_BYTES);
}

#[test]
fn deep_nesting_stops_without_overflowing_the_stack() {
    let depth = 20_000;
    let input = format!("{}x{}", "<div>".repeat(depth), "</div>".repeat(depth));
    let out = sanitize(&input);
    common::assert_safe("deep", &out);
    assert!(out.truncated);
    let divs = out.html.matches("<div>").count();
    // The parser stops at its depth limit (512).
    assert!(divs > 0 && divs <= 512, "{divs} nested divs kept");
    assert_eq!(divs, out.html.matches("</div>").count());
}

#[test]
fn multiplying_formatting_stops_at_the_node_limit() {
    let input = format!(
        "<p>{}{}",
        (0..400).map(|i| format!("<b a{i}>")).collect::<String>(),
        "<p>x".repeat(400)
    );
    let out = sanitize(&input);
    common::assert_safe("multiplied", &out);
    assert!(out.truncated);
}
