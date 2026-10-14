//! Known script-injection and loading vectors: none may leave anything executable, loadable or navigable.

mod common;

use deadloader_html::sanitize;

const CORPUS: &[&str] = &[
    // Scripts and handlers.
    "<script>alert(1)</script>",
    "<SCRIPT SRC=https://evil.example/x.js></SCRIPT>",
    "<scr<script>ipt>alert(1)</script>",
    "<img src=x onerror=alert(1)>",
    "<img src=\"https://i.imgur.com/a.png\" onerror=\"alert(1)\" onload=\"alert(2)\">",
    "<b onclick=\"alert(1)\" onmouseover=alert(1)>x</b>",
    "<body onload=alert(1)>x",
    "<div/onclick=alert(1)>x</div>",
    "<p\nonclick=alert(1)>x</p>",
    "<a href=\"https://ok.example/\" onclick=\"alert(1)\" onfocus=alert(1) autofocus>x</a>",
    "<details open ontoggle=alert(1)>x</details>",
    "<video><source onerror=alert(1)></video>",
    "<audio src=x onerror=alert(1)>",
    "<marquee onstart=alert(1)>x</marquee>",
    // URL schemes, plain, mixed case, whitespace, control characters, entities.
    "<a href=\"javascript:alert(1)\">x</a>",
    "<a href=\"JaVaScRiPt:alert(1)\">x</a>",
    "<a href=\"  javascript:alert(1)\">x</a>",
    "<a href=\"\u{1}javascript:alert(1)\">x</a>",
    "<a href=\"java\tscript:alert(1)\">x</a>",
    "<a href=\"java&#x09;script:alert(1)\">x</a>",
    "<a href=\"java&NewLine;script:alert(1)\">x</a>",
    "<a href=\"&#106;&#97;&#118;&#97;&#115;&#99;&#114;&#105;&#112;&#116;&#58;alert(1)\">x</a>",
    "<a href=\"&#x6A;avascript&colon;alert(1)\">x</a>",
    "<a href=\"javascript&#58;alert(1)\">x</a>",
    "<a href=\"vbscript:msgbox(1)\">x</a>",
    "<a href=\"VBScript:msgbox(1)\">x</a>",
    "<a href=\"data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==\">x</a>",
    "<a href=\"DATA:text/html,<script>alert(1)</script>\">x</a>",
    "<img src=\"data:image/svg+xml;base64,PHN2Zy8+\">",
    "<img src=\"javascript:alert(1)\">",
    "<img src=\"jav&#x0A;ascript:alert(1)\">",
    "<a href=\"file:///C:/Windows/System32/calc.exe\">x</a>",
    "<a href=\"https://gamebanana.com/linkfilter?url=javascript%3Aalert(1)\">x</a>",
    "<a href=\"https://gamebanana.com/linkfilter?url=data%3Atext%2Fhtml%2C%3Cscript%3E\">x</a>",
    "<a href=\"https://ok.example/&quot; onclick=&quot;alert(1)\">x</a>",
    "<a href='https://ok.example/\"><script>alert(1)</script>'>x</a>",
    // Foreign content.
    "<svg onload=alert(1)>",
    "<svg><script>alert(1)</script></svg>",
    "<svg><a xlink:href=\"javascript:alert(1)\"><text>x</text></a></svg>",
    "<svg><animate attributeName=href values=javascript:alert(1) /></svg>",
    "<svg><foreignObject><img src=x onerror=alert(1)></foreignObject></svg>",
    "<math><mtext><table><mglyph><style><img src=x onerror=alert(1)>",
    "<math><mi xlink:href=\"javascript:alert(1)\">x</mi></math>",
    "<math><annotation-xml encoding=\"text/html\"><img src=x onerror=alert(1)></annotation-xml></math>",
    "<svg></p><style><a id=\"</style><img src=x onerror=alert(1)>\">",
    // Frames, objects, forms.
    "<iframe srcdoc=\"<script>alert(1)</script>\"></iframe>",
    "<iframe src=\"javascript:alert(1)\"></iframe>",
    "<iframe src=\"https://www.youtube.com/embed/abc\" srcdoc=\"<script>alert(1)</script>\" onload=alert(1)></iframe>",
    "<object data=\"javascript:alert(1)\"></object>",
    "<embed src=\"javascript:alert(1)\">",
    "<form action=\"javascript:alert(1)\"><input type=submit></form>",
    "<button formaction=\"javascript:alert(1)\">x</button>",
    "<input type=\"image\" src=\"https://evil.example/a.png\">",
    "<isindex action=javascript:alert(1) type=image>",
    // Document-level elements.
    "<base href=\"https://evil.example/\"><a href=\"/x\">x</a>",
    "<meta http-equiv=\"refresh\" content=\"0;url=javascript:alert(1)\">",
    "<meta http-equiv=\"Content-Security-Policy\" content=\"default-src *\">",
    "<link rel=\"stylesheet\" href=\"https://evil.example/x.css\">",
    "<link rel=\"import\" href=\"https://evil.example/x.html\">",
    // Styles.
    "<div style=\"background:url(javascript:alert(1))\">x</div>",
    "<div style=\"width: expression(alert(1))\">x</div>",
    "<span style=\"background-image:url('https://evil.example/track.png')\">x</span>",
    "<style>@import url(https://evil.example/x.css);</style>",
    "<style>*{background:url(https://evil.example/t.png)}</style>x",
    // Comments, CDATA, templates, raw-text tricks.
    "<!-- <script>alert(1)</script> -->",
    "<!--><img src=x onerror=alert(1)>-->",
    "<!--[if gte IE 4]><script>alert(1)</script><![endif]-->",
    "<![CDATA[<script>alert(1)</script>]]>",
    "<template><img src=x onerror=alert(1)></template>",
    "<noscript><p title=\"</noscript><img src=x onerror=alert(1)>\"></noscript>",
    "<textarea><img src=x onerror=alert(1)></textarea>",
    "<title><img src=x onerror=alert(1)></title>",
    "<xmp><img src=x onerror=alert(1)></xmp>",
    "<noembed><img src=x onerror=alert(1)></noembed>",
    "<plaintext><img src=x onerror=alert(1)>",
    "<select><template><img src=x onerror=alert(1)></template></select>",
    // Nested and malformed markup.
    "<a href=\"https://ok.example/\"><a href=\"javascript:alert(1)\">x</a></a>",
    "<a href=\"https://ok.example/\">a<table><a href=\"https://b.example/\">b</a></table></a>",
    "<b><p>x</b>y</p><i><b>z</i></b>",
    "<table><td><img src=x onerror=alert(1)></td><script>alert(1)</script></table>",
    "<div><div><div><span class=\"GreenColor\" onclick=alert(1)>x",
    "<a href=\"https://ok.example/\"",
    "<img src=\"https://ok.example/a.png",
    "<<script>script>alert(1)<</script>/script>",
    "<a href=https://ok.example/ class=GreenColor data-dl-link=99>x</a>",
    "<span class=\"dl-image\" data-dl-image=\"0\"></span>",
    "\0<scr\0ipt>alert(1)</scr\0ipt>",
    "<IMG SRC=JaVaScRiPt:alert('XSS')>",
    "<IMG SRC=`javascript:alert(\"RSnake says, 'XSS'\")`>",
    "<IMG \"\"\"><SCRIPT>alert(\"XSS\")</SCRIPT>\">",
    "<IMG SRC=\"jav&#x0D;ascript:alert('XSS');\">",
    "<img src=\"https://ok.example/a.png\" srcset=\"https://evil.example/b.png 2x\">",
];

#[test]
fn corpus_leaves_nothing_active() {
    for input in CORPUS {
        let out = sanitize(input);
        common::assert_safe(input, &out);
        // Text cannot contain a raw '<' (it is escaped), so these are exactly the tags.
        let html = out.html.to_ascii_lowercase();
        let tags: Vec<&str> = html
            .split('<')
            .skip(1)
            .map(|t| t.split('>').next().unwrap_or_default())
            .collect();
        for tag in &tags {
            for needle in [
                "src=", "href=", "style=", "srcdoc", "script", "svg", "math", "iframe",
            ] {
                assert!(
                    !tag.contains(needle),
                    "{needle:?} in <{tag}> (input {input:?})"
                );
            }
            assert!(
                !tag.split(' ').skip(1).any(|a| a.starts_with("on")),
                "event handler in <{tag}> (input {input:?})"
            );
        }
        for link in &out.links {
            assert!(
                link.host.ends_with("ok.example")
                    || link.host == "b.example"
                    || link.host == "www.youtube.com",
                "unexpected link {link:?} from {input:?}"
            );
        }
        for image in &out.images {
            assert!(
                image.host == "i.imgur.com" || image.host == "ok.example",
                "unexpected image {image:?} from {input:?}"
            );
        }
    }
}

#[test]
fn script_like_text_is_shown_as_text() {
    let out = sanitize("<p>&lt;script&gt;alert(1)&lt;/script&gt;</p>");
    assert_eq!(out.html, "<p>&lt;script&gt;alert(1)&lt;/script&gt;</p>");
}

#[test]
fn attribute_breakouts_stay_inside_the_url() {
    let out = sanitize("<a href=\"https://ok.example/&quot; onclick=&quot;alert(1)\">x</a>");
    assert_eq!(out.html, "<a data-dl-link=\"0\">x</a>");
    assert_eq!(
        out.links[0].url,
        "https://ok.example/%22%20onclick=%22alert(1)"
    );
}

#[test]
fn a_link_forged_with_our_attributes_is_not_trusted() {
    let out = sanitize(
        "<a data-dl-link=\"7\" class=\"dl-image\">x</a><span data-dl-image=\"0\">y</span>",
    );
    assert_eq!(out.html, "x<span>y</span>");
    assert!(out.links.is_empty() && out.images.is_empty());
}

#[test]
fn no_attribute_survives_on_kept_tags() {
    let out = sanitize(
        "<p id=a lang=en dir=rtl hidden contenteditable draggable=true accesskey=x tabindex=0 popover \
         slot=s part=p is=x-y xmlns=x nonce=n>x</p>",
    );
    assert_eq!(out.html, "<p>x</p>");
}
