```bgraph
{"schema":"1.0.0","kind":"document","blazegraph_version":"0.5.0","source":{"format":"markdown","filename":"","sha256":"6dd3058665c1709db0c87617a8d0610662e3344fce5844d2e19d7a9ab0f34234"},"flow_type":"Free","config_hash":"none","graph_sha256":"19fc78b3944641bdccab819127e47ae70a5f06ee3eca96f4b58a66e16be22d57"}
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
{"id":"a1953fb0-063f-52b8-90a6-115dd0b59a23","node_type":"Paragraph","location":{"semantic":{"path":"1","depth":1,"breadcrumbs":[]},"physical":null},"text_order":0,"token_count":72,"style":null}
```

There is support for images, tables, lists, footnotes, endnotes, links,
dropcaps and various types of text and paragraph level formatting.
```bgraph-paragraph
{"id":"f3c88d61-ba88-508b-8e62-1bffd9d42791","node_type":"Paragraph","location":{"semantic":{"path":"2","depth":1,"breadcrumbs":[]},"physical":null},"text_order":1,"token_count":34,"style":null}
```

To see the DOCX conversion in action, simply add this file to calibre
using the **“Add Books”** button and then click “**Convert”.** Set the
output format in the top right corner of the conversion dialog to EPUB
or AZW3 and click **“OK”**.
```bgraph-paragraph
{"id":"d0a0f871-9c5f-5390-8984-9f6354f0f905","node_type":"Paragraph","location":{"semantic":{"path":"3","depth":1,"breadcrumbs":[]},"physical":null},"text_order":2,"token_count":62,"style":null}
```

# Text Formatting
```bgraph-section
{"id":"0f5f653f-7775-5928-b20e-05c3f5ee15e7","node_type":"Section","location":{"semantic":{"path":"4","depth":1,"breadcrumbs":["Text Formatting"]},"physical":null},"text_order":3,"token_count":3,"style":null}
```

## Inline formatting
```bgraph-section
{"id":"085c40a0-472d-5715-9c88-b9f2b38e61f3","node_type":"Section","location":{"semantic":{"path":"4.1","depth":2,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":4,"token_count":4,"style":null}
```

Here, we demonstrate various types of inline text formatting and the use
of embedded fonts.
```bgraph-paragraph
{"id":"ea0b82e6-f529-5fd7-8482-9558e08f8897","node_type":"Paragraph","location":{"semantic":{"path":"4.1.1","depth":3,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":5,"token_count":22,"style":null}
```

Here is some **bold,** *italic,* ***bold-italic,*** underlined and
~~struck out~~ text. Then, we have a superscript and a
subscript. Now we see some red, green and blue text. Some
text with a yellow highlight. Some text in a
box. Some text in inverse video.
```bgraph-paragraph
{"id":"bd171d87-8ed3-5e9f-90c2-c48f60802572","node_type":"Paragraph","location":{"semantic":{"path":"4.1.2","depth":3,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":6,"token_count":64,"style":null}
```

A paragraph with styled text: *subtle emphasis* followed by **strong
text** and ***intense emphasis***. This paragraph uses document wide
styles for styling rather than inline text properties as demonstrated in
the previous paragraph — calibre can handle both with equal ease.
```bgraph-paragraph
{"id":"55654ee4-885a-56de-bd78-5d89197b4e14","node_type":"Paragraph","location":{"semantic":{"path":"4.1.3","depth":3,"breadcrumbs":["Text Formatting","Inline formatting"]},"physical":null},"text_order":7,"token_count":69,"style":null}
```

## Fun with fonts
```bgraph-section
{"id":"b87bdc0d-0ca6-53b4-bb95-63afcf60d588","node_type":"Section","location":{"semantic":{"path":"4.2","depth":2,"breadcrumbs":["Text Formatting","Fun with fonts"]},"physical":null},"text_order":8,"token_count":3,"style":null}
```

This document has embedded the Ubuntu font family. The body text is in
the Ubuntu typeface, here is some text in the Ubuntu Mono typeface,
notice how every letter has the same width, even i and m. Every embedded
font will automatically be embedded in the output ebook during
conversion.
```bgraph-paragraph
{"id":"06a280fc-1165-56cd-8912-74207d504340","node_type":"Paragraph","location":{"semantic":{"path":"4.2.1","depth":3,"breadcrumbs":["Text Formatting","Fun with fonts"]},"physical":null},"text_order":9,"token_count":71,"style":null}
```

## **Paragraph level formatting**
```bgraph-section
{"id":"5e5fa8f9-e8b3-5618-985d-fae1c39af013","node_type":"Section","location":{"semantic":{"path":"4.3","depth":2,"breadcrumbs":["Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":10,"token_count":7,"style":null}
```

You can do crazy things with paragraphs, if the urge strikes you. For
instance this paragraph is right aligned and has a right border. It has
also been given a light gray background.
```bgraph-paragraph
{"id":"b3219a7d-018e-55ca-88d6-e0fa30dba2cc","node_type":"Paragraph","location":{"semantic":{"path":"4.3.1","depth":3,"breadcrumbs":["Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":11,"token_count":45,"style":null}
```

For the lovers of poetry amongst you, paragraphs with hanging indents,
like this often come in handy. You can use hanging indents to ensure
that a line of poetry retains its individual identity as a line even
when the screen is too narrow to display it as a single line. Not only
does this paragraph have a hanging indent, it is also has an extra top
margin, setting it apart from the preceding paragraph.
```bgraph-paragraph
{"id":"7e822a83-3b13-51e8-814a-c4e4692ceeca","node_type":"Paragraph","location":{"semantic":{"path":"4.3.2","depth":3,"breadcrumbs":["Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":12,"token_count":101,"style":null}
```

# Tables
```bgraph-section
{"id":"34132214-1e4d-5a32-8481-11fc9d2c6a76","node_type":"Section","location":{"semantic":{"path":"5","depth":1,"breadcrumbs":["Tables"]},"physical":null},"text_order":13,"token_count":1,"style":null}
```

| ITEM        | NEEDED   |
|-------------|----------|
| Books       | 1        |
| Pens        | 3        |
| Pencils     | 2        |
| Highlighter | 2 colors |
| Scissors    | 1 pair   |
```bgraph-table
{"id":"ca3bbc3e-a037-5db1-9021-926abfd3d559","node_type":"Table","location":{"semantic":{"path":"5.1","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":14,"token_count":47,"style":null}
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
{"id":"b194918f-a18d-54cd-80be-c30328ccccab","node_type":"Paragraph","location":{"semantic":{"path":"5.2","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":15,"token_count":130,"style":null}
```

Now let’s look at a fancier table—one with alternating row colors and
partial borders. This table is stretched out to take 100% of the
available width.
```bgraph-paragraph
{"id":"d590c1c1-c9c9-53b8-8713-4a7b0d3d80c7","node_type":"Paragraph","location":{"semantic":{"path":"5.3","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":16,"token_count":38,"style":null}
```

| City or Town | Point A | Point B | Point C | Point D | Point E |
|--------------|:-------:|:-------:|:-------:|:-------:|:-------:|
| Point A      | —       |         |         |         |         |
| Point B      | 87      | —       |         |         |         |
| Point C      | 64      | 56      | —       |         |         |
| Point D      | 37      | 32      | 91      | —       |         |
| Point E      | 93      | 35      | 54      | 43      | —       |
```bgraph-table
{"id":"1777738f-8af9-50f3-bb68-2fe896cbfcd9","node_type":"Table","location":{"semantic":{"path":"5.4","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":17,"token_count":119,"style":null}
```

Next, we see a table with special formatting in various locations.
Notice how the formatting for the header row and sub header rows is
preserved.
```bgraph-paragraph
{"id":"ae250868-3ef0-5e51-a706-694f1ce3b8da","node_type":"Paragraph","location":{"semantic":{"path":"5.5","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":18,"token_count":36,"style":null}
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
{"id":"29a026fc-00d7-5de3-b70c-aa3a73d537fc","node_type":"Table","location":{"semantic":{"path":"5.6","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":19,"token_count":157,"style":null}
```

*Source:* Fictitious data, for illustration purposes only
```bgraph-paragraph
{"id":"4eaae8a3-6261-5874-83bf-b2e3bae69272","node_type":"Paragraph","location":{"semantic":{"path":"5.7","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":20,"token_count":14,"style":null}
```

Next, we have something a little more complex, a nested table, i.e. a
table inside another table. Additionally, the inner table has some of
its cells merged. The table is displayed horizontally centered.
```bgraph-paragraph
{"id":"47f4021f-080d-5fab-9194-5c75ce8e87ae","node_type":"Paragraph","location":{"semantic":{"path":"5.8","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":21,"token_count":50,"style":null}
```

We end with a fancy calendar, note how much of the original formatting
is preserved. Note that this table will only display correctly on
relatively wide screens. In general, very wide tables or tables whose
cells have fixed width requirements don’t fare well in ebooks.
```bgraph-paragraph
{"id":"e75f2349-8fb5-5fff-bc32-095422d85d9b","node_type":"Paragraph","location":{"semantic":{"path":"5.9","depth":2,"breadcrumbs":["Tables"]},"physical":null},"text_order":22,"token_count":67,"style":null}
```

# Structural Elements
```bgraph-section
{"id":"afa86083-24a9-55ed-9bf6-909bc29ac44d","node_type":"Section","location":{"semantic":{"path":"6","depth":1,"breadcrumbs":["Structural Elements"]},"physical":null},"text_order":23,"token_count":4,"style":null}
```

Miscellaneous structural elements you can add to your document, like
footnotes, endnotes, dropcaps and the like.
```bgraph-paragraph
{"id":"5968b220-8222-5d53-b1e8-05237942f7ce","node_type":"Paragraph","location":{"semantic":{"path":"6.1","depth":2,"breadcrumbs":["Structural Elements"]},"physical":null},"text_order":24,"token_count":28,"style":null}
```

## Footnotes & Endnotes
```bgraph-section
{"id":"ff7c246a-e32b-542b-8084-551d99e193e2","node_type":"Section","location":{"semantic":{"path":"6.2","depth":2,"breadcrumbs":["Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":25,"token_count":5,"style":null}
```

Footnotes[^1] and endnotes[^2] are automatically recognized and both are
converted to endnotes, with backlinks for maximum ease of use in ebook
devices.
```bgraph-paragraph
{"id":"66e29083-b2d4-501a-b3eb-6a611efe4663","node_type":"Paragraph","location":{"semantic":{"path":"6.2.1","depth":3,"breadcrumbs":["Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":26,"token_count":38,"style":null}
```

## Dropcaps
```bgraph-section
{"id":"528500d5-dccf-52c5-82d0-db5ba7e34602","node_type":"Section","location":{"semantic":{"path":"6.3","depth":2,"breadcrumbs":["Structural Elements","Dropcaps"]},"physical":null},"text_order":27,"token_count":2,"style":null}
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
{"id":"f87f2c61-0ef9-531c-a5b8-8fe77ce0d08e","node_type":"Paragraph","location":{"semantic":{"path":"6.3.1","depth":3,"breadcrumbs":["Structural Elements","Dropcaps"]},"physical":null},"text_order":28,"token_count":164,"style":null}
```

## Links
```bgraph-section
{"id":"ad99285b-bb17-5b16-b7ba-8ec73dd4aaed","node_type":"Section","location":{"semantic":{"path":"6.4","depth":2,"breadcrumbs":["Structural Elements","Links"]},"physical":null},"text_order":29,"token_count":1,"style":null}
```

Two kinds of links are possible, those that refer to an external website
and those that refer to locations inside the document itself. Both are
supported by calibre. For example, here is a link pointing to the
[calibre download page](http://calibre-ebook.com/download). Then we have
a link that points back to the section on [paragraph level
formatting](#paragraph-level-formatting) in this document.
```bgraph-paragraph
{"id":"5192ec3a-1e9c-53a4-b204-647ac57cfa40","node_type":"Paragraph","location":{"semantic":{"path":"6.4.1","depth":3,"breadcrumbs":["Structural Elements","Links"]},"physical":null},"text_order":30,"token_count":100,"style":null}
```

## Table of Contents
```bgraph-section
{"id":"a7a90df2-29eb-5bd9-a122-92de3b0c001c","node_type":"Section","location":{"semantic":{"path":"6.5","depth":2,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":31,"token_count":4,"style":null}
```

There are two approaches that calibre takes when generating a Table of
Contents. The first is if the Word document has a Table of Contents
itself. Provided that the Table of Contents uses hyperlinks, calibre
will automatically use it. The levels of the Table of Contents are
identified by their left indent, so if you want the ebook to have a
multi-level Table of Contents, make sure you create a properly indented
Table of Contents in Word.
```bgraph-paragraph
{"id":"cff091e2-7cdd-504b-92be-d5bfbb11786b","node_type":"Paragraph","location":{"semantic":{"path":"6.5.1","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":32,"token_count":110,"style":null}
```

If no Table of Contents is found in the document, then a table of
contents is automatically generated from the headings in the document. A
heading is identified as something that has the Heading 1 or Heading 2,
etc. style applied to it. These headings are turned into a Table of
Contents with Heading 1 being the topmost level, Heading 2 the second
level and so on.
```bgraph-paragraph
{"id":"86fa10d4-53e7-51a7-aa01-3c38050d2cc9","node_type":"Paragraph","location":{"semantic":{"path":"6.5.2","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":33,"token_count":91,"style":null}
```

You can see the Table of Contents created by calibre by clicking the
Table of Contents button in whatever viewer you are using to view the
converted ebook.
```bgraph-paragraph
{"id":"3976627b-05e5-5b7c-8d45-b442899a8ddc","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":34,"token_count":38,"style":null}
```

[Demonstration of DOCX support in calibre [1](#OLE_LINK1)](#OLE_LINK1)
```bgraph-paragraph
{"id":"21d7a919-50d1-5ff9-bbe4-ecaa916904c3","node_type":"Paragraph","location":{"semantic":{"path":"6.5.4","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":35,"token_count":17,"style":null}
```

[Text Formatting [2](#text-formatting)](#text-formatting)
```bgraph-paragraph
{"id":"b5f9d197-2c0a-56d4-93d3-c40da19b66bb","node_type":"Paragraph","location":{"semantic":{"path":"6.5.5","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":36,"token_count":14,"style":null}
```

[Inline formatting [2](#inline-formatting)](#inline-formatting)
```bgraph-paragraph
{"id":"4b37c2f9-d371-5216-b3b9-ea44b00c3712","node_type":"Paragraph","location":{"semantic":{"path":"6.5.6","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":37,"token_count":15,"style":null}
```

[Fun with fonts [2](#fun-with-fonts)](#fun-with-fonts)
```bgraph-paragraph
{"id":"4085c8bb-4edb-5e87-963b-854e889720a0","node_type":"Paragraph","location":{"semantic":{"path":"6.5.7","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":38,"token_count":13,"style":null}
```

[Paragraph level formatting
[2](#paragraph-level-formatting)](#paragraph-level-formatting)
```bgraph-paragraph
{"id":"e92c10dc-77fe-5cb5-aee5-2d3a97dc7745","node_type":"Paragraph","location":{"semantic":{"path":"6.5.8","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":39,"token_count":22,"style":null}
```

[Tables [3](#tables)](#tables)
```bgraph-paragraph
{"id":"dbd58711-f6e4-59be-b2f1-1f8c4e24bd71","node_type":"Paragraph","location":{"semantic":{"path":"6.5.9","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":40,"token_count":7,"style":null}
```

[Structural Elements [5](#structural-elements)](#structural-elements)
```bgraph-paragraph
{"id":"fff1c00c-5411-5a04-9737-5b3eb608e863","node_type":"Paragraph","location":{"semantic":{"path":"6.5.10","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":41,"token_count":17,"style":null}
```

[Footnotes & Endnotes [5](#footnotes-endnotes)](#footnotes-endnotes)
```bgraph-paragraph
{"id":"4f86a32f-36ae-5471-8874-5c0649b55d78","node_type":"Paragraph","location":{"semantic":{"path":"6.5.11","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":42,"token_count":17,"style":null}
```

[Dropcaps [5](#dropcaps)](#dropcaps)
```bgraph-paragraph
{"id":"58068559-2ea3-514d-8d3a-201bd42554ae","node_type":"Paragraph","location":{"semantic":{"path":"6.5.12","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":43,"token_count":9,"style":null}
```

[Links [5](#links)](#links)
```bgraph-paragraph
{"id":"954edb58-ba06-51af-81c5-6ee98f618c4a","node_type":"Paragraph","location":{"semantic":{"path":"6.5.13","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":44,"token_count":6,"style":null}
```

[Table of Contents [5](#table-of-contents)](#table-of-contents)
```bgraph-paragraph
{"id":"9199d685-f2bb-5fbb-8a77-37a4e500dcff","node_type":"Paragraph","location":{"semantic":{"path":"6.5.14","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":45,"token_count":15,"style":null}
```

[Images [7](#images)](#images)
```bgraph-paragraph
{"id":"61893104-5de6-5fa2-8cb0-4dd37a3b25c9","node_type":"Paragraph","location":{"semantic":{"path":"6.5.15","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":46,"token_count":7,"style":null}
```

[Lists [8](#lists)](#lists)
```bgraph-paragraph
{"id":"23d9769a-5fce-5a3a-a0d5-75ea2c4dc903","node_type":"Paragraph","location":{"semantic":{"path":"6.5.16","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":47,"token_count":6,"style":null}
```

[Bulleted List [8](#bulleted-list)](#bulleted-list)
```bgraph-paragraph
{"id":"ca813fd3-4711-574e-be7f-aeb6ae43a8b9","node_type":"Paragraph","location":{"semantic":{"path":"6.5.17","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":48,"token_count":12,"style":null}
```

[Numbered List [8](#numbered-list)](#numbered-list)
```bgraph-paragraph
{"id":"d22b34a9-3dbd-53ee-b7b1-901a814120f2","node_type":"Paragraph","location":{"semantic":{"path":"6.5.18","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":49,"token_count":12,"style":null}
```

[Multi-level Lists [8](#multi-level-lists)](#multi-level-lists)
```bgraph-paragraph
{"id":"11a3be75-77e9-5232-9b1e-e3faabcfa2d3","node_type":"Paragraph","location":{"semantic":{"path":"6.5.19","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":50,"token_count":15,"style":null}
```

[Continued Lists [8](#continued-lists)](#continued-lists)
```bgraph-paragraph
{"id":"ac591a3a-4a52-5523-96d2-4fb50fbaa82f","node_type":"Paragraph","location":{"semantic":{"path":"6.5.20","depth":3,"breadcrumbs":["Structural Elements","Table of Contents"]},"physical":null},"text_order":51,"token_count":14,"style":null}
```

# Images
```bgraph-section
{"id":"2c06f8b0-ca62-597a-b870-cd1932bf6904","node_type":"Section","location":{"semantic":{"path":"7","depth":1,"breadcrumbs":["Images"]},"physical":null},"text_order":52,"token_count":1,"style":null}
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
{"id":"9f5d489d-9f2d-520e-bbac-582f3d6c6d2d","node_type":"Paragraph","location":{"semantic":{"path":"7.1","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":53,"token_count":141,"style":null}
```

The final type of image is a “block” image, one that becomes a paragraph
on its own and has no text on either side. Below is a centered green
dot.
```bgraph-paragraph
{"id":"dab3afb1-4c1a-5443-97c6-6e1e4feef708","node_type":"Paragraph","location":{"semantic":{"path":"7.2","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":54,"token_count":37,"style":null}
```

Centered images like this are useful for large
pictures that should be a focus of attention.
```bgraph-paragraph
{"id":"9635619e-0f4b-5990-a824-090376137003","node_type":"Paragraph","location":{"semantic":{"path":"7.3","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":55,"token_count":23,"style":null}
```

Generally, it is not possible to translate the exact positioning of
images from a Word document to an ebook. That is because in Word, image
positioning is specified in absolute units from the page boundaries.
There is no analogous technology in ebooks, so the conversion will
usually end up placing the image either centered or floating close to
the point in the text where it was *inserted*, not necessarily where it
appears on the page in Word.
```bgraph-paragraph
{"id":"49f7aa04-1b54-5c2e-8fbe-11d7db5fe07a","node_type":"Paragraph","location":{"semantic":{"path":"7.4","depth":2,"breadcrumbs":["Images"]},"physical":null},"text_order":56,"token_count":111,"style":null}
```

# Lists
```bgraph-section
{"id":"a0dce2f1-f627-5d13-a251-66228e08c7bc","node_type":"Section","location":{"semantic":{"path":"8","depth":1,"breadcrumbs":["Lists"]},"physical":null},"text_order":57,"token_count":1,"style":null}
```

All types of lists are supported by the conversion, with the exception
of lists that use fancy bullets, these get converted to regular bullets.
```bgraph-paragraph
{"id":"e0c889e3-024f-5c37-ac78-9de2960d1f36","node_type":"Paragraph","location":{"semantic":{"path":"8.1","depth":2,"breadcrumbs":["Lists"]},"physical":null},"text_order":58,"token_count":35,"style":null}
```

## Bulleted List
```bgraph-section
{"id":"1ad1745e-0eaf-5444-9b21-71f325bcf963","node_type":"Section","location":{"semantic":{"path":"8.2","depth":2,"breadcrumbs":["Lists","Bulleted List"]},"physical":null},"text_order":59,"token_count":3,"style":null}
```

- One

- Two
```bgraph-list
{"id":"fe4562b7-46e0-58ba-b4d2-146a9a5ae88a","node_type":"List","location":{"semantic":{"path":"8.2.1","depth":3,"breadcrumbs":["Lists","Bulleted List"]},"physical":null},"text_order":60,"token_count":3,"style":null}
```

## Numbered List
```bgraph-section
{"id":"417e9a7f-9c95-55eb-90e9-b0374dea2bf5","node_type":"Section","location":{"semantic":{"path":"8.3","depth":2,"breadcrumbs":["Lists","Numbered List"]},"physical":null},"text_order":61,"token_count":3,"style":null}
```

1.  One, with a very long line to demonstrate that the hanging indent
    for the list is working correctly

2.  Two
```bgraph-list
{"id":"5bbb0203-484e-5d6b-aad6-b9afe1505bf1","node_type":"List","location":{"semantic":{"path":"8.3.1","depth":3,"breadcrumbs":["Lists","Numbered List"]},"physical":null},"text_order":62,"token_count":29,"style":null}
```

## Multi-level Lists
```bgraph-section
{"id":"f08a1e4f-731e-520c-a08e-51c2cde267e1","node_type":"Section","location":{"semantic":{"path":"8.4","depth":2,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":63,"token_count":4,"style":null}
```

1.  One

    1.  Two

        1.  Three

        2.  Four with a very long line to demonstrate that the hanging
            indent for the list is working correctly.

        3.  Five

2.  Six
```bgraph-list
{"id":"6e6c4031-ed15-595f-bbcb-3c47a1cafb4b","node_type":"List","location":{"semantic":{"path":"8.4.1","depth":3,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":64,"token_count":48,"style":null}
```

A Multi-level list with bullets:
```bgraph-paragraph
{"id":"f5debedd-5479-59da-b98c-55812f341eac","node_type":"Paragraph","location":{"semantic":{"path":"8.4.2","depth":3,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":65,"token_count":8,"style":null}
```

- One

  - Two

    - This bullet uses an image as the bullet item

      - Four

- Five
```bgraph-list
{"id":"e7c3b91b-0a0a-5774-8b1d-7db16281ed36","node_type":"List","location":{"semantic":{"path":"8.4.3","depth":3,"breadcrumbs":["Lists","Multi-level Lists"]},"physical":null},"text_order":66,"token_count":22,"style":null}
```

## Continued Lists
```bgraph-section
{"id":"bdba2cfb-7e3f-5c49-a514-3f1b0a9d163a","node_type":"Section","location":{"semantic":{"path":"8.5","depth":2,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":67,"token_count":3,"style":null}
```

1.  One

2.  Two
```bgraph-list
{"id":"6cf14f63-06f9-51a3-bc51-120a80fc305a","node_type":"List","location":{"semantic":{"path":"8.5.1","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":68,"token_count":4,"style":null}
```

An interruption in our regularly scheduled listing, for this essential
and very relevant public service announcement.
```bgraph-paragraph
{"id":"de6c5334-9812-5318-83f1-d6ebbf35c6a7","node_type":"Paragraph","location":{"semantic":{"path":"8.5.2","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":69,"token_count":29,"style":null}
```

3.  We now resume our normal programming

4.  Four
```bgraph-list
{"id":"ce654085-b70f-54eb-b751-7f4ef8f6d6c7","node_type":"List","location":{"semantic":{"path":"8.5.3","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":70,"token_count":12,"style":null}
```

[^1]: In paged media, footnotes are usually displayed at the bottom of
the text. However, in ebooks, a better paradigm is to make them
clickable endnotes that the user can browse at her pleasure. This
conversion is handled automatically by calibre.
```bgraph-paragraph
{"id":"63b21e15-77c2-5335-b7a4-74062ba2ebe7","node_type":"Paragraph","location":{"semantic":{"path":"8.5.4","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":71,"token_count":62,"style":null}
```

[^2]: Endnotes are typically used for longer notes, they remain endnotes
when converted into ebook form, except that they have an additional
backlink to make it easy to return to the current position after
reading the note.
```bgraph-paragraph
{"id":"9864ba83-d7a1-54a3-9bc6-87b233ad5605","node_type":"Paragraph","location":{"semantic":{"path":"8.5.5","depth":3,"breadcrumbs":["Lists","Continued Lists"]},"physical":null},"text_order":72,"token_count":55,"style":null}
```
