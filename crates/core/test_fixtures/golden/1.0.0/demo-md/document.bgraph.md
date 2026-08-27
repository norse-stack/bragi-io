```bgraph
{"schema":"1.1.0","kind":"document","bragi_version":"0.6.0","source":{"format":"markdown","sha256":"6dd3058665c1709db0c87617a8d0610662e3344fce5844d2e19d7a9ab0f34234"},"flow_type":"Free","config_hash":"none","bgraph_sha256":"d8f5db3a873b492c8a8b8f148ceeacb50f681ee0238a084f16477922630255e2"}
```

```bgraph-metadata
{"title":null,"author":null,"description":null,"language":null,"created":null}
```

This document demonstrates the ability of the calibre DOCX Input plugin
to convert the various typographic features in a Microsoft Word (2007
and newer) document. Convert this document to a modern ebook format,
such as AZW3 for Kindles or EPUB for other ebook readers, to see it in
action.
```bgraph-paragraph
{"id":"7974a4ae-1c7a-5332-959b-3d69035588e9","node_type":"Paragraph","location":{"semantic":{"path":"1","depth":1,"breadcrumbs":[]},"physical":null},"text_order":0,"token_count":72,"style":null}
```

There is support for images, tables, lists, footnotes, endnotes, links,
dropcaps and various types of text and paragraph level formatting.
```bgraph-paragraph
{"id":"40ba8d91-9440-5609-95cb-0db6622bf8f6","node_type":"Paragraph","location":{"semantic":{"path":"2","depth":1,"breadcrumbs":[]},"physical":null},"text_order":1,"token_count":34,"style":null}
```

To see the DOCX conversion in action, simply add this file to calibre
using the **“Add Books”** button and then click “**Convert”.** Set the
output format in the top right corner of the conversion dialog to EPUB
or AZW3 and click **“OK”**.
```bgraph-paragraph
{"id":"d23f0c61-560e-5418-beae-4311cbe2afe0","node_type":"Paragraph","location":{"semantic":{"path":"3","depth":1,"breadcrumbs":[]},"physical":null},"text_order":2,"token_count":62,"style":null}
```

# Text Formatting
```bgraph-section
{"id":"9791ea30-e13f-5bac-839b-b1dfd508f912","node_type":"Section","location":{"semantic":{"path":"4","depth":1,"breadcrumbs":["Text Formatting"]},"physical":null},"text_order":3,"token_count":3,"style":null}
```

## Inline formatting
```bgraph-section
{"id":"a1376a19-efaf-5fbf-90a0-b713515558e7","node_type":"Section","location":{"semantic":{"path":"4.1","depth":2,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":4,"token_count":4,"style":null}
```

Here, we demonstrate various types of inline text formatting and the use
of embedded fonts.
```bgraph-paragraph
{"id":"9d9174b7-e129-529f-aaf9-7e3a95793170","node_type":"Paragraph","location":{"semantic":{"path":"4.1.1","depth":3,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":5,"token_count":22,"style":null}
```

Here is some **bold,** *italic,* ***bold-italic,*** underlined and
~~struck out~~ text. Then, we have a superscript and a
subscript. Now we see some red, green and blue text. Some
text with a yellow highlight. Some text in a
box. Some text in inverse video.
```bgraph-paragraph
{"id":"6214cd67-7f2a-5bc5-b534-208598d38872","node_type":"Paragraph","location":{"semantic":{"path":"4.1.2","depth":3,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":6,"token_count":64,"style":null}
```

A paragraph with styled text: *subtle emphasis* followed by **strong
text** and ***intense emphasis***. This paragraph uses document wide
styles for styling rather than inline text properties as demonstrated in
the previous paragraph — calibre can handle both with equal ease.
```bgraph-paragraph
{"id":"250f1bc1-0a84-5b4c-8483-78dbd625531b","node_type":"Paragraph","location":{"semantic":{"path":"4.1.3","depth":3,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":7,"token_count":69,"style":null}
```

## Fun with fonts
```bgraph-section
{"id":"17b6e7a8-cc86-5718-a9e9-db94e35c0065","node_type":"Section","location":{"semantic":{"path":"4.2","depth":2,"breadcrumbs":["Text Formatting","Fun with fonts"]},"physical":null},"text_order":8,"token_count":3,"style":null}
```

This document has embedded the Ubuntu font family. The body text is in
the Ubuntu typeface, here is some text in the Ubuntu Mono typeface,
notice how every letter has the same width, even i and m. Every embedded
font will automatically be embedded in the output ebook during
conversion.
```bgraph-paragraph
{"id":"b75058e7-0f37-5db5-b90c-becd27304af2","node_type":"Paragraph","location":{"semantic":{"path":"4.2.1","depth":3,"breadcrumbs":["Text Formatting","Fun with fonts"]},"physical":null},"text_order":9,"token_count":71,"style":null}
```

## **Paragraph level formatting**
```bgraph-section
{"id":"20c7528e-f907-5032-8de9-6b52988b1d00","node_type":"Section","location":{"semantic":{"path":"4.3","depth":2,"breadcrumbs":["Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":10,"token_count":7,"style":null}
```

You can do crazy things with paragraphs, if the urge strikes you. For
instance this paragraph is right aligned and has a right border. It has
also been given a light gray background.
```bgraph-paragraph
{"id":"9e027f50-1bbc-58ad-b289-fef6168c58f8","node_type":"Paragraph","location":{"semantic":{"path":"4.3.1","depth":3,"breadcrumbs":["Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":11,"token_count":45,"style":null}
```

For the lovers of poetry amongst you, paragraphs with hanging indents,
like this often come in handy. You can use hanging indents to ensure
that a line of poetry retains its individual identity as a line even
when the screen is too narrow to display it as a single line. Not only
does this paragraph have a hanging indent, it is also has an extra top
margin, setting it apart from the preceding paragraph.
```bgraph-paragraph
{"id":"160e06c3-3e6a-55d9-a50b-26990dd6a54f","node_type":"Paragraph","location":{"semantic":{"path":"4.3.2","depth":3,"breadcrumbs":["Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":12,"token_count":101,"style":null}
```

# Tables
```bgraph-section
{"id":"4352f116-f515-5ce5-bb31-7434597007e6","node_type":"Section","location":{"semantic":{"path":"5","depth":1,"breadcrumbs":["Tables"]},"physical":null},"text_order":13,"token_count":1,"style":null}
```

| ITEM        | NEEDED   |
|-------------|----------|
| Books       | 1        |
| Pens        | 3        |
| Pencils     | 2        |
| Highlighter | 2 colors |
| Scissors    | 1 pair   |
```bgraph-table
{"id":"6228e2ae-9bf1-565d-b2cf-e5880a92e2f5","node_type":"Table","location":{"semantic":{"path":"5.1","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":14,"token_count":47,"style":null}
```

Tables in Word can vary from the extremely simple to the extremely
complex. calibre tries to do its best when converting tables. While you
may run into trouble with the occasional table, the vast majority of
common cases should be converted very well, as demonstrated in this
section. Note that for optimum results, when creating tables in Word,
you should set their widths using percentages, rather than absolute
units. To the left of this paragraph is a floating two column table with
a nice green border and header row.
```bgraph-paragraph
{"id":"03b6ae2e-9dec-523e-982f-5396ceff286b","node_type":"Paragraph","location":{"semantic":{"path":"5.2","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":15,"token_count":130,"style":null}
```

Now let’s look at a fancier table—one with alternating row colors and
partial borders. This table is stretched out to take 100% of the
available width.
```bgraph-paragraph
{"id":"caa477cd-7a8a-5d77-ba66-2874506476c4","node_type":"Paragraph","location":{"semantic":{"path":"5.3","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":16,"token_count":38,"style":null}
```

| City or Town | Point A | Point B | Point C | Point D | Point E |
|--------------|:-------:|:-------:|:-------:|:-------:|:-------:|
| Point A      | —       |         |         |         |         |
| Point B      | 87      | —       |         |         |         |
| Point C      | 64      | 56      | —       |         |         |
| Point D      | 37      | 32      | 91      | —       |         |
| Point E      | 93      | 35      | 54      | 43      | —       |
```bgraph-table
{"id":"b1d24618-bf88-51a8-bc0e-ee4f9faf11a1","node_type":"Table","location":{"semantic":{"path":"5.4","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":17,"token_count":119,"style":null}
```

Next, we see a table with special formatting in various locations.
Notice how the formatting for the header row and sub header rows is
preserved.
```bgraph-paragraph
{"id":"282c0d02-807d-5c17-8558-b3ed7a33c76f","node_type":"Paragraph","location":{"semantic":{"path":"5.5","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":18,"token_count":36,"style":null}
```

| College          | New students    | Graduating students | Change |
|------------------|-----------------|---------------------|--------|
|                  | *Undergraduate* |                     |        |
| Cedar University | 110             | 103                 | +7     |
| Oak Institute    | 202             | 210                 | -8     |
|                  | *Graduate*      |                     |        |
| Cedar University | 24              | 20                  | +4     |
| Elm College      | 43              | 53                  | -10    |
| Total            | 998             | 908                 | 90     |
```bgraph-table
{"id":"441abbfc-1613-589c-9ed3-c97d8ca9c915","node_type":"Table","location":{"semantic":{"path":"5.6","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":19,"token_count":157,"style":null}
```

*Source:* Fictitious data, for illustration purposes only
```bgraph-paragraph
{"id":"726a6850-1681-5aa9-b98d-50789259363a","node_type":"Paragraph","location":{"semantic":{"path":"5.7","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":20,"token_count":14,"style":null}
```

Next, we have something a little more complex, a nested table, i.e. a
table inside another table. Additionally, the inner table has some of
its cells merged. The table is displayed horizontally centered.
```bgraph-paragraph
{"id":"7a64ad4d-06cc-58e0-9486-1cacaa72fe4d","node_type":"Paragraph","location":{"semantic":{"path":"5.8","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":21,"token_count":50,"style":null}
```

We end with a fancy calendar, note how much of the original formatting
is preserved. Note that this table will only display correctly on
relatively wide screens. In general, very wide tables or tables whose
cells have fixed width requirements don’t fare well in ebooks.
```bgraph-paragraph
{"id":"baeeb752-f052-5788-8bde-75c2ebf16b49","node_type":"Paragraph","location":{"semantic":{"path":"5.9","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":22,"token_count":67,"style":null}
```

# Structural Elements
```bgraph-section
{"id":"e24ef65c-3327-5d83-bfde-b007b60ee163","node_type":"Section","location":{"semantic":{"path":"6","depth":1,"breadcrumbs":["Structural Elements"]},"physical":null},"text_order":23,"token_count":4,"style":null}
```

Miscellaneous structural elements you can add to your document, like
footnotes, endnotes, dropcaps and the like.
```bgraph-paragraph
{"id":"0dfe5c4b-7906-5c3b-ae64-0f0548ce58d0","node_type":"Paragraph","location":{"semantic":{"path":"6.1","depth":2,"breadcrumbs":["Structural Elements"]},"physical":null},"text_order":24,"token_count":28,"style":null}
```

## Footnotes & Endnotes
```bgraph-section
{"id":"384683b3-869f-5cfc-b582-783c264ebc7b","node_type":"Section","location":{"semantic":{"path":"6.2","depth":2,"breadcrumbs":["Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":25,"token_count":5,"style":null}
```

Footnotes[^1] and endnotes[^2] are automatically recognized and both are
converted to endnotes, with backlinks for maximum ease of use in ebook
devices.
```bgraph-paragraph
{"id":"05df48f6-65ca-5834-a66d-cbe30760bbc8","node_type":"Paragraph","location":{"semantic":{"path":"6.2.1","depth":3,"breadcrumbs":["Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":26,"token_count":38,"style":null}
```

## Dropcaps
```bgraph-section
{"id":"3e967b5a-e427-586d-8408-e84e06f9ae19","node_type":"Section","location":{"semantic":{"path":"6.3","depth":2,"breadcrumbs":["Structural Elements","Dropcaps"]},"physical":null},"text_order":27,"token_count":2,"style":null}
```

Drop caps are used to emphasize the leading paragraph at the start of a
section. In Word it is possible to specify how many lines of text a
drop-cap should use. Because of limitations in ebook technology, this is
not possible when converting. Instead, the converted drop cap will use
font size and line height to simulate the effect as well as possible.
While not as good as the original, the result is usually tolerable. This
paragraph has a “D” dropcap set to occupy three lines of text with a
font size of 58.5 pts. Depending on the screen width and capabilities of
the device you view the book on, this dropcap can look anything from
perfect to ugly.
```bgraph-paragraph
{"id":"0e0faa14-3e1c-507b-bcd1-0d10dbf936b2","node_type":"Paragraph","location":{"semantic":{"path":"6.3.1","depth":3,"breadcrumbs":["Structural Elements","Dropcaps"]},"physical":null},"text_order":28,"token_count":164,"style":null}
```

## Links
```bgraph-section
{"id":"3ad64674-e4ce-5230-9915-d042a24be0d4","node_type":"Section","location":{"semantic":{"path":"6.4","depth":2,"breadcrumbs":["Structural Elements","Links"]},"physical":null},"text_order":29,"token_count":1,"style":null}
```

Two kinds of links are possible, those that refer to an external website
and those that refer to locations inside the document itself. Both are
supported by calibre. For example, here is a link pointing to the
[calibre download page](http://calibre-ebook.com/download). Then we have
a link that points back to the section on [paragraph level
formatting](#paragraph-level-formatting) in this document.
```bgraph-paragraph
{"id":"40940df2-326e-50c3-8574-568c1e531ad7","node_type":"Paragraph","location":{"semantic":{"path":"6.4.1","depth":3,"breadcrumbs":["Structural Elements","Links"]},"physical":null},"text_order":30,"token_count":100,"style":null}
```

## Table of Contents
```bgraph-section
{"id":"4f716b29-e9e6-5130-b219-921e919a9855","node_type":"Section","location":{"semantic":{"path":"6.5","depth":2,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":31,"token_count":4,"style":null}
```

There are two approaches that calibre takes when generating a Table of
Contents. The first is if the Word document has a Table of Contents
itself. Provided that the Table of Contents uses hyperlinks, calibre
will automatically use it. The levels of the Table of Contents are
identified by their left indent, so if you want the ebook to have a
multi-level Table of Contents, make sure you create a properly indented
Table of Contents in Word.
```bgraph-paragraph
{"id":"a32d0849-bd2d-5d59-8d66-27f9fc46d66e","node_type":"Paragraph","location":{"semantic":{"path":"6.5.1","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":32,"token_count":110,"style":null}
```

If no Table of Contents is found in the document, then a table of
contents is automatically generated from the headings in the document. A
heading is identified as something that has the Heading 1 or Heading 2,
etc. style applied to it. These headings are turned into a Table of
Contents with Heading 1 being the topmost level, Heading 2 the second
level and so on.
```bgraph-paragraph
{"id":"51f73a3f-58f6-55d2-a56a-65cdb6887d12","node_type":"Paragraph","location":{"semantic":{"path":"6.5.2","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":33,"token_count":91,"style":null}
```

You can see the Table of Contents created by calibre by clicking the
Table of Contents button in whatever viewer you are using to view the
converted ebook.
```bgraph-paragraph
{"id":"64d2b185-f7ef-5ecc-8587-442e0695883e","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":34,"token_count":38,"style":null}
```

[Demonstration of DOCX support in calibre [1](#OLE_LINK1)](#OLE_LINK1)
```bgraph-paragraph
{"id":"6c56f0da-5aff-5ce2-bc99-8cdd0c89c2af","node_type":"Paragraph","location":{"semantic":{"path":"6.5.4","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":35,"token_count":17,"style":null}
```

[Text Formatting [2](#text-formatting)](#text-formatting)
```bgraph-paragraph
{"id":"ae9d8037-bbc4-5cf3-88f6-5b957948dbde","node_type":"Paragraph","location":{"semantic":{"path":"6.5.5","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":36,"token_count":14,"style":null}
```

[Inline formatting [2](#inline-formatting)](#inline-formatting)
```bgraph-paragraph
{"id":"b6112144-3fee-554b-84c3-45cee8e257b4","node_type":"Paragraph","location":{"semantic":{"path":"6.5.6","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":37,"token_count":15,"style":null}
```

[Fun with fonts [2](#fun-with-fonts)](#fun-with-fonts)
```bgraph-paragraph
{"id":"7b847263-1d21-5e43-bafb-e9c6c6f40683","node_type":"Paragraph","location":{"semantic":{"path":"6.5.7","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":38,"token_count":13,"style":null}
```

[Paragraph level formatting
[2](#paragraph-level-formatting)](#paragraph-level-formatting)
```bgraph-paragraph
{"id":"f052ba0b-84cb-5a99-9930-b58f576f5396","node_type":"Paragraph","location":{"semantic":{"path":"6.5.8","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":39,"token_count":22,"style":null}
```

[Tables [3](#tables)](#tables)
```bgraph-paragraph
{"id":"6e3a97d4-5c1a-5238-8c91-3a27fea22c1f","node_type":"Paragraph","location":{"semantic":{"path":"6.5.9","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":40,"token_count":7,"style":null}
```

[Structural Elements [5](#structural-elements)](#structural-elements)
```bgraph-paragraph
{"id":"b36d21b1-b3b8-51fa-8b24-0c274c26ede7","node_type":"Paragraph","location":{"semantic":{"path":"6.5.10","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":41,"token_count":17,"style":null}
```

[Footnotes & Endnotes [5](#footnotes-endnotes)](#footnotes-endnotes)
```bgraph-paragraph
{"id":"e750ca07-b113-59fb-ab03-007549c47901","node_type":"Paragraph","location":{"semantic":{"path":"6.5.11","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":42,"token_count":17,"style":null}
```

[Dropcaps [5](#dropcaps)](#dropcaps)
```bgraph-paragraph
{"id":"ace23cb5-a912-5d1d-a4a3-18ddc438bdb0","node_type":"Paragraph","location":{"semantic":{"path":"6.5.12","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":43,"token_count":9,"style":null}
```

[Links [5](#links)](#links)
```bgraph-paragraph
{"id":"c3e5a052-8bb6-51ad-be26-6c452dd37467","node_type":"Paragraph","location":{"semantic":{"path":"6.5.13","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":44,"token_count":6,"style":null}
```

[Table of Contents [5](#table-of-contents)](#table-of-contents)
```bgraph-paragraph
{"id":"5802a6a3-75e0-508b-a46d-4bfc493f83be","node_type":"Paragraph","location":{"semantic":{"path":"6.5.14","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":45,"token_count":15,"style":null}
```

[Images [7](#images)](#images)
```bgraph-paragraph
{"id":"57312ebb-c72d-5cbc-a123-4408742ee525","node_type":"Paragraph","location":{"semantic":{"path":"6.5.15","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":46,"token_count":7,"style":null}
```

[Lists [8](#lists)](#lists)
```bgraph-paragraph
{"id":"939d75f4-1f33-5054-85b3-6f5710da0c16","node_type":"Paragraph","location":{"semantic":{"path":"6.5.16","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":47,"token_count":6,"style":null}
```

[Bulleted List [8](#bulleted-list)](#bulleted-list)
```bgraph-paragraph
{"id":"eb85ecf5-2d4e-5294-a883-07c62bc663dd","node_type":"Paragraph","location":{"semantic":{"path":"6.5.17","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":48,"token_count":12,"style":null}
```

[Numbered List [8](#numbered-list)](#numbered-list)
```bgraph-paragraph
{"id":"94bd12e2-09aa-58b4-a66b-7bf526382dfa","node_type":"Paragraph","location":{"semantic":{"path":"6.5.18","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":49,"token_count":12,"style":null}
```

[Multi-level Lists [8](#multi-level-lists)](#multi-level-lists)
```bgraph-paragraph
{"id":"5f5a03f0-a0a8-5c1c-ae49-0cf0fef6a528","node_type":"Paragraph","location":{"semantic":{"path":"6.5.19","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":50,"token_count":15,"style":null}
```

[Continued Lists [8](#continued-lists)](#continued-lists)
```bgraph-paragraph
{"id":"5a7216df-1684-5486-ae05-e54a8e513689","node_type":"Paragraph","location":{"semantic":{"path":"6.5.20","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":51,"token_count":14,"style":null}
```

# Images
```bgraph-section
{"id":"13de0fea-ee97-5205-a360-cbd4ab59a1b2","node_type":"Section","location":{"semantic":{"path":"7","depth":1,"breadcrumbs":["Images"]},"physical":null},"text_order":52,"token_count":1,"style":null}
```

Images can be of three main types. Inline images are images that are
part of the normal text flow, like this image of a green dot
. Inline images do not cause breaks in the text
and are usually small in
size. The next category of image is a floating image, one
that “floats “ on the page and is surrounded by text. Word supports more
types of floating images than are possible with current ebook
technology, so the conversion maps floating images to simple left and
right floats, as you can see with the left and right arrow images on the
sides of this paragraph.
```bgraph-paragraph
{"id":"172fcd48-07a7-5b8b-9586-14c148c146f0","node_type":"Paragraph","location":{"semantic":{"path":"7.1","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":53,"token_count":141,"style":null}
```

The final type of image is a “block” image, one that becomes a paragraph
on its own and has no text on either side. Below is a centered green
dot.
```bgraph-paragraph
{"id":"2114e9d0-09c4-5374-a7f5-22d29cf41daa","node_type":"Paragraph","location":{"semantic":{"path":"7.2","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":54,"token_count":37,"style":null}
```

Centered images like this are useful for large
pictures that should be a focus of attention.
```bgraph-paragraph
{"id":"7a4762cd-2d8e-5cba-b158-cf920a6dfccf","node_type":"Paragraph","location":{"semantic":{"path":"7.3","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":55,"token_count":23,"style":null}
```

Generally, it is not possible to translate the exact positioning of
images from a Word document to an ebook. That is because in Word, image
positioning is specified in absolute units from the page boundaries.
There is no analogous technology in ebooks, so the conversion will
usually end up placing the image either centered or floating close to
the point in the text where it was *inserted*, not necessarily where it
appears on the page in Word.
```bgraph-paragraph
{"id":"082e07c7-efe4-5e11-a19f-595c31b3177c","node_type":"Paragraph","location":{"semantic":{"path":"7.4","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":56,"token_count":111,"style":null}
```

# Lists
```bgraph-section
{"id":"3b7a356f-74d8-5c42-a340-b515b6311a2d","node_type":"Section","location":{"semantic":{"path":"8","depth":1,"breadcrumbs":["Lists"]},"physical":null},"text_order":57,"token_count":1,"style":null}
```

All types of lists are supported by the conversion, with the exception
of lists that use fancy bullets, these get converted to regular bullets.
```bgraph-paragraph
{"id":"a26c9662-dcd5-54dc-a646-a26f9a85afbb","node_type":"Paragraph","location":{"semantic":{"path":"8.1","depth":2,"breadcrumbs":["Lists"]},"physical":null},"text_order":58,"token_count":35,"style":null}
```

## Bulleted List
```bgraph-section
{"id":"dd14cf3a-b037-54fc-88b2-b4c3355113ea","node_type":"Section","location":{"semantic":{"path":"8.2","depth":2,"breadcrumbs":["Lists","Bulleted List"]},"physical":null},"text_order":59,"token_count":3,"style":null}
```

- One

- Two
```bgraph-list
{"id":"2476b551-e6b6-5497-b7eb-dfe8bd7a3560","node_type":"List","location":{"semantic":{"path":"8.2.1","depth":3,"breadcrumbs":["Lists","Bulleted List"]},"physical":null},"text_order":60,"token_count":3,"style":null}
```

## Numbered List
```bgraph-section
{"id":"b87e1f60-0e11-58ed-8601-8e5c8c59cc09","node_type":"Section","location":{"semantic":{"path":"8.3","depth":2,"breadcrumbs":["Lists","Numbered List"]},"physical":null},"text_order":61,"token_count":3,"style":null}
```

1.  One, with a very long line to demonstrate that the hanging indent
    for the list is working correctly

2.  Two
```bgraph-list
{"id":"69c5e9b0-e574-5947-866a-21da4184b768","node_type":"List","location":{"semantic":{"path":"8.3.1","depth":3,"breadcrumbs":["Lists","Numbered List"]},"physical":null},"text_order":62,"token_count":29,"style":null}
```

## Multi-level Lists
```bgraph-section
{"id":"c3707509-ad3f-5fa8-9ac3-2cf95167a2ed","node_type":"Section","location":{"semantic":{"path":"8.4","depth":2,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":63,"token_count":4,"style":null}
```

1.  One

    1.  Two

        1.  Three

        2.  Four with a very long line to demonstrate that the hanging
            indent for the list is working correctly.

        3.  Five

2.  Six
```bgraph-list
{"id":"d01374f5-a37a-5d02-a65e-a379d6af6709","node_type":"List","location":{"semantic":{"path":"8.4.1","depth":3,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":64,"token_count":48,"style":null}
```

A Multi-level list with bullets:
```bgraph-paragraph
{"id":"3c331daa-b5af-56be-bedc-e63bd0fe7f15","node_type":"Paragraph","location":{"semantic":{"path":"8.4.2","depth":3,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":65,"token_count":8,"style":null}
```

- One

  - Two

    - This bullet uses an image as the bullet item

      - Four

- Five
```bgraph-list
{"id":"5fff43cf-7c12-5cc9-bcb9-a055e4d251aa","node_type":"List","location":{"semantic":{"path":"8.4.3","depth":3,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":66,"token_count":22,"style":null}
```

## Continued Lists
```bgraph-section
{"id":"b82ccb3c-559d-5733-a3b8-5e3f05d90a92","node_type":"Section","location":{"semantic":{"path":"8.5","depth":2,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":67,"token_count":3,"style":null}
```

1.  One

2.  Two
```bgraph-list
{"id":"745327af-9e95-50a5-9583-8748dc8517a3","node_type":"List","location":{"semantic":{"path":"8.5.1","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":68,"token_count":4,"style":null}
```

An interruption in our regularly scheduled listing, for this essential
and very relevant public service announcement.
```bgraph-paragraph
{"id":"7f59c549-2fe5-5c0f-9776-ba78d279b28d","node_type":"Paragraph","location":{"semantic":{"path":"8.5.2","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":69,"token_count":29,"style":null}
```

3.  We now resume our normal programming

4.  Four
```bgraph-list
{"id":"252170a8-83f8-5b22-94de-b84b12299861","node_type":"List","location":{"semantic":{"path":"8.5.3","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":70,"token_count":12,"style":null}
```

[^1]: In paged media, footnotes are usually displayed at the bottom of
the text. However, in ebooks, a better paradigm is to make them
clickable endnotes that the user can browse at her pleasure. This
conversion is handled automatically by calibre.
```bgraph-paragraph
{"id":"7b6586f5-989a-5b6e-b382-6f963cb008c4","node_type":"Paragraph","location":{"semantic":{"path":"8.5.4","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":71,"token_count":62,"style":null}
```

[^2]: Endnotes are typically used for longer notes, they remain endnotes
when converted into ebook form, except that they have an additional
backlink to make it easy to return to the current position after
reading the note.
```bgraph-paragraph
{"id":"0060f5af-d4b3-5a67-a4d4-dc2520caa2d0","node_type":"Paragraph","location":{"semantic":{"path":"8.5.5","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":72,"token_count":55,"style":null}
```
