```bgraph
{"schema":"1.0.0","kind":"document","bragi_version":"0.5.0","source":{"format":"docx","sha256":"269329fc7ae54b3f289b3ac52efde387edc2e566ef9a48d637e841022c7e0eab"},"flow_type":"Free","config_hash":"none","bgraph_sha256":"4769de3840d492ec30780e5c1660f4dbd4d84a502ab87adf1be7e8df35eeb25b"}
```

```bgraph-metadata
{"title":"DOCX Demo","author":"Kovid Goyal","description":"Demonstration of DOCX support in calibre","language":null,"created":"2013-06-05T07:56:00Z","docx":{"application":"Microsoft Office Word","app_version":"12.0000","pages":3,"words":1518,"characters":8657,"lines":72,"paragraphs":20,"company":null,"manager":null,"template":"Normal","total_time":8540,"doc_security":0,"last_modified_by":"kovid","revision":"79","modified":"2013-06-20T06:14:00Z","extras":{"keywords":"calibre, docs, ebook, conversion"}}}
```

# Demonstration of DOCX support in calibre
```bgraph-section
{"id":"1b77c690-e668-5320-90b6-9267dbcb07cb","node_type":"Section","location":{"semantic":{"path":"1","depth":1,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":0,"token_count":10,"style":null}
```

This document demonstrates the ability of the calibre DOCX Input plugin to convert the various typographic features in a Microsoft Word (2007 and newer) document. Convert this document to a modern ebook format, such as AZW3 for Kindles or EPUB for other ebook readers, to see it in action.
```bgraph-paragraph
{"id":"d616752f-cb95-568b-a905-ae92b88ffa00","node_type":"Paragraph","location":{"semantic":{"path":"1.1","depth":2,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":1,"token_count":72,"style":null}
```

There is support for images, tables, lists, footnotes, endnotes, links, dropcaps and various types of text and paragraph level formatting.
```bgraph-paragraph
{"id":"555fb4d1-f258-545b-98b7-b886368e701e","node_type":"Paragraph","location":{"semantic":{"path":"1.2","depth":2,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":2,"token_count":34,"style":null}
```

To see the DOCX conversion in action, simply add this file to calibre using the **“Add Books”** button and then click “**Convert”.**  Set the output format in the top right corner of the conversion dialog to EPUB or AZW3 and click **“OK”**.
```bgraph-paragraph
{"id":"2330d96c-26a8-56f9-afdf-7dcf8c557d76","node_type":"Paragraph","location":{"semantic":{"path":"1.3","depth":2,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":3,"token_count":63,"style":null}
```

# Text Formatting
```bgraph-section
{"id":"9791ea30-e13f-5bac-839b-b1dfd508f912","node_type":"Section","location":{"semantic":{"path":"2","depth":1,"breadcrumbs":["DOCX Demo","Text Formatting"]},"physical":null},"text_order":4,"token_count":3,"style":null}
```

## Inline formatting
```bgraph-section
{"id":"a1376a19-efaf-5fbf-90a0-b713515558e7","node_type":"Section","location":{"semantic":{"path":"2.1","depth":2,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":5,"token_count":4,"style":null}
```

Here, we demonstrate various types of inline text formatting and the use of embedded fonts.
```bgraph-paragraph
{"id":"862e2325-193d-5c26-a3ca-c0077a963db6","node_type":"Paragraph","location":{"semantic":{"path":"2.1.1","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":6,"token_count":22,"style":null}
```

Here is some **bold,** *italic,* ***bold-italic,*** underlined and struck out  text. Then, we have a superscript and a subscript. Now we see some red, green and blue text. Some text with a yellow highlight. Some text in a box. Some text in inverse video.
```bgraph-paragraph
{"id":"6a76c626-360d-5f9f-ae68-1d2df0fe6c2e","node_type":"Paragraph","location":{"semantic":{"path":"2.1.2","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":7,"token_count":63,"style":null}
```

A paragraph with styled text: subtle emphasis  followed by strong text and intense emphasis. This paragraph uses document wide styles for styling rather than inline text properties as demonstrated in the previous paragraph — calibre can handle both with equal ease.
```bgraph-paragraph
{"id":"7b67e0a2-96cd-5877-8c97-fdf0d7e07fdd","node_type":"Paragraph","location":{"semantic":{"path":"2.1.3","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":8,"token_count":66,"style":null}
```

## Fun with fonts
```bgraph-section
{"id":"17b6e7a8-cc86-5718-a9e9-db94e35c0065","node_type":"Section","location":{"semantic":{"path":"2.2","depth":2,"breadcrumbs":["DOCX Demo","Text Formatting","Fun with fonts"]},"physical":null},"text_order":9,"token_count":3,"style":null}
```

This document has embedded the Ubuntu font family. The body text is in the Ubuntu typeface, here is some text in the Ubuntu Mono typeface, notice how every letter has the same width, even i and m. Every embedded font will automatically be embedded in the output ebook during conversion.
```bgraph-paragraph
{"id":"86a454ba-dee8-5c24-8bd5-3e14cd998f8a","node_type":"Paragraph","location":{"semantic":{"path":"2.2.1","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Fun with fonts"]},"physical":null},"text_order":10,"token_count":71,"style":null}
```

## **Paragraph level formatting**
```bgraph-section
{"id":"20c7528e-f907-5032-8de9-6b52988b1d00","node_type":"Section","location":{"semantic":{"path":"2.3","depth":2,"breadcrumbs":["DOCX Demo","Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":11,"token_count":7,"style":null}
```

You can do crazy things with paragraphs, if the urge strikes you. For instance this paragraph is right aligned and has a right border. It has also been given a light gray background.
```bgraph-paragraph
{"id":"032f89b2-012f-5b60-a4d0-42877f64fe3c","node_type":"Paragraph","location":{"semantic":{"path":"2.3.1","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":12,"token_count":45,"style":null}
```

For the lovers of poetry amongst you, paragraphs with hanging indents, like this often come in handy. You can use hanging indents to ensure that a line of poetry retains its individual identity as a line even when the screen is  too narrow to display it as a single line. Not only does this paragraph have a hanging indent, it is also has an extra top margin, setting it apart from the preceding paragraph.
```bgraph-paragraph
{"id":"63c4d074-0f8b-5818-99da-03fdcbac2680","node_type":"Paragraph","location":{"semantic":{"path":"2.3.2","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":13,"token_count":101,"style":null}
```

# Tables
```bgraph-section
{"id":"4352f116-f515-5ce5-bb31-7434597007e6","node_type":"Section","location":{"semantic":{"path":"3","depth":1,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":14,"token_count":1,"style":null}
```

| ITEM        | NEEDED   |
|-------------|----------|
| Books       | 1        |
| Pens        | 3        |
| Pencils     | 2        |
| Highlighter | 2 colors |
| Scissors    | 1 pair   |
```bgraph-table
{"id":"6228e2ae-9bf1-565d-b2cf-e5880a92e2f5","node_type":"Table","location":{"semantic":{"path":"3.1","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":15,"token_count":47,"style":null}
```

Tables in Word can vary from the extremely simple to the extremely complex. calibre tries to do its best when converting tables. While you may run into trouble with the occasional table, the vast majority of common cases should be converted very well, as demonstrated in this section. Note that for optimum results, when creating tables in Word, you should set their widths using percentages, rather than absolute units.  To the left of this paragraph is a floating two column table with a nice green border and header row.
```bgraph-paragraph
{"id":"1b295809-7e20-5955-9459-16831b3de11c","node_type":"Paragraph","location":{"semantic":{"path":"3.2","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":16,"token_count":130,"style":null}
```

Now let’s look at a fancier table—one with alternating row colors and partial borders. This table is stretched out to take 100% of the available width.
```bgraph-paragraph
{"id":"f849f31d-2773-5d57-834a-c55bd33b6064","node_type":"Paragraph","location":{"semantic":{"path":"3.3","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":17,"token_count":38,"style":null}
```

| City or Town | Point A | Point B | Point C | Point D | Point E |
|--------------|---------|---------|---------|---------|---------|
| Point A      | —       |         |         |         |         |
| Point B      | 87      | —       |         |         |         |
| Point C      | 64      | 56      | —       |         |         |
| Point D      | 37      | 32      | 91      | —       |         |
| Point E      | 93      | 35      | 54      | 43      | —       |
```bgraph-table
{"id":"2f495def-7d03-5ba4-89f0-5d79bc8cdc53","node_type":"Table","location":{"semantic":{"path":"3.4","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":18,"token_count":119,"style":null}
```

Next, we see a table with special formatting in various locations. Notice how the formatting for the header row and sub header rows is preserved.
```bgraph-paragraph
{"id":"35239a35-9fc5-5b3e-be48-1ad1f0f01bfe","node_type":"Paragraph","location":{"semantic":{"path":"3.5","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":19,"token_count":36,"style":null}
```

| College          | New students | Graduating students | Change |
|------------------|--------------|---------------------|--------|
| Undergraduate    |              |                     |        |
| Cedar University | 110          | 103                 | +7     |
| Oak Institute    | 202          | 210                 | -8     |
| Graduate         |              |                     |        |
| Cedar University | 24           | 20                  | +4     |
| Elm College      | 43           | 53                  | -10    |
| Total            | 998          | 908                 | 90     |
```bgraph-table
{"id":"75ce7d2d-0a6c-5d36-9500-95c18f265771","node_type":"Table","location":{"semantic":{"path":"3.6","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":20,"token_count":150,"style":null}
```

Source: Fictitious data, for illustration purposes only
```bgraph-paragraph
{"id":"31d93911-387f-5487-8f1d-80448d633390","node_type":"Paragraph","location":{"semantic":{"path":"3.7","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":21,"token_count":13,"style":null}
```

Next, we have something a little more complex, a nested table, i.e. a table inside another table. Additionally, the inner table has some of its cells merged. The table is displayed horizontally centered.
```bgraph-paragraph
{"id":"e52a5e42-e476-552b-aa09-2fd966bf464d","node_type":"Paragraph","location":{"semantic":{"path":"3.8","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":22,"token_count":50,"style":null}
```

| One                                                            |     |
|----------------------------------------------------------------|-----|
| Three                                                          | Two |
| Four                                                           |     |
| To the left is a table inside a table, with some cells merged. |     |
```bgraph-table
{"id":"d3d63734-77bc-5128-a317-12c1422b1a2c","node_type":"Table","location":{"semantic":{"path":"3.9","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":23,"token_count":91,"style":null}
```

We end with a fancy calendar, note how much of the original formatting is preserved. Note that this table will only display correctly on relatively wide screens. In general, very wide tables or tables whose cells have fixed width requirements don’t fare well in ebooks.
```bgraph-paragraph
{"id":"bb80b929-2842-5c3a-900a-7ea328463d12","node_type":"Paragraph","location":{"semantic":{"path":"3.10","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":24,"token_count":67,"style":null}
```

| December 2007 |     |     |     |     |     |     |     |     |     |     |     |     |
|---------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| Sun           |     | Mon |     | Tue |     | Wed |     | Thu |     | Fri |     | Sat |
|               |     |     |     |     |     |     |     |     |     |     | 1   |     |
|               |     |     |     |     |     |     |     |     |     |     |     |     |
| 2             |     | 3   |     | 4   |     | 5   |     | 6   |     | 7   |     | 8   |
|               |     |     |     |     |     |     |     |     |     |     |     |     |
| 9             |     | 10  |     | 11  |     | 12  |     | 13  |     | 14  |     | 15  |
|               |     |     |     |     |     |     |     |     |     |     |     |     |
| 16            |     | 17  |     | 18  |     | 19  |     | 20  |     | 21  |     | 22  |
|               |     |     |     |     |     |     |     |     |     |     |     |     |
| 23            |     | 24  |     | 25  |     | 26  |     | 27  |     | 28  |     | 29  |
|               |     |     |     |     |     |     |     |     |     |     |     |     |
| 30            |     | 31  |     |     |     |     |     |     |     |     |     |     |
```bgraph-table
{"id":"29bd1755-a487-58ce-957a-0c76ca229d44","node_type":"Table","location":{"semantic":{"path":"3.11","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":25,"token_count":314,"style":null}
```

# Structural Elements
```bgraph-section
{"id":"e24ef65c-3327-5d83-bfde-b007b60ee163","node_type":"Section","location":{"semantic":{"path":"4","depth":1,"breadcrumbs":["DOCX Demo","Structural Elements"]},"physical":null},"text_order":26,"token_count":4,"style":null}
```

Miscellaneous structural elements you can add to your document, like footnotes, endnotes, dropcaps and the like.
```bgraph-paragraph
{"id":"fcf4f3d5-cd0f-5b31-9bf4-f7a01b82b89a","node_type":"Paragraph","location":{"semantic":{"path":"4.1","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements"]},"physical":null},"text_order":27,"token_count":28,"style":null}
```

## Footnotes & Endnotes
```bgraph-section
{"id":"384683b3-869f-5cfc-b582-783c264ebc7b","node_type":"Section","location":{"semantic":{"path":"4.2","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":28,"token_count":5,"style":null}
```

Footnotes and endnotes are automatically recognized and both are converted to endnotes, with backlinks for maximum ease of use in ebook devices.
```bgraph-paragraph
{"id":"18ee6cfd-0cf6-5d72-9f17-82c0ccaf8cf0","node_type":"Paragraph","location":{"semantic":{"path":"4.2.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":29,"token_count":36,"style":null}
```

## Dropcaps
```bgraph-section
{"id":"3e967b5a-e427-586d-8408-e84e06f9ae19","node_type":"Section","location":{"semantic":{"path":"4.3","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Dropcaps"]},"physical":null},"text_order":30,"token_count":2,"style":null}
```

D
```bgraph-paragraph
{"id":"b530c26f-4a9f-5c26-b2b0-0505b6bca781","node_type":"Paragraph","location":{"semantic":{"path":"4.3.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Dropcaps"]},"physical":null},"text_order":31,"token_count":1,"style":null}
```

rop caps are used to emphasize the leading paragraph at the start of a section. In Word it is possible to specify how many lines of text a drop-cap should use. Because of limitations in ebook technology, this is not possible when converting.  Instead, the converted drop cap will use font size and line height to simulate the effect as well as possible. While not as good as the original, the result is usually tolerable. This paragraph has a “D” dropcap set to occupy three lines of text with a font size of 58.5 pts. Depending on the screen width and capabilities of the device you view the book on, this dropcap can look anything from perfect to ugly.
```bgraph-paragraph
{"id":"cabd787b-e899-59f8-85e6-244efac18196","node_type":"Paragraph","location":{"semantic":{"path":"4.3.2","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Dropcaps"]},"physical":null},"text_order":32,"token_count":164,"style":null}
```

## Links
```bgraph-section
{"id":"3ad64674-e4ce-5230-9915-d042a24be0d4","node_type":"Section","location":{"semantic":{"path":"4.4","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Links"]},"physical":null},"text_order":33,"token_count":1,"style":null}
```

Two kinds of links are possible, those that refer to an external website and those that refer to locations inside the document itself. Both are supported by calibre. For example, here is a link pointing to the calibre download page. Then we have a link that points back to the section on paragraph level formatting in this document.
```bgraph-paragraph
{"id":"e1856abe-6dfd-5c33-af95-c4f2d05ea2e0","node_type":"Paragraph","location":{"semantic":{"path":"4.4.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Links"]},"physical":null},"text_order":34,"token_count":83,"internal_refs":[{"text":"paragraph level formatting","target":{"kind":"named","name":"_Paragraph_level_formatting"}}],"external_refs":[{"text":"calibre download page","target":{"kind":"uri","url":"http://calibre-ebook.com/download"}}],"style":null}
```

## Table of Contents
```bgraph-section
{"id":"4f716b29-e9e6-5130-b219-921e919a9855","node_type":"Section","location":{"semantic":{"path":"4.5","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":35,"token_count":4,"style":null}
```

There are two approaches that calibre takes when generating a Table of Contents. The first is if the Word document has a Table of Contents itself. Provided that the Table of Contents uses hyperlinks, calibre will automatically use it. The levels of the Table of Contents are identified by their left indent, so if you want the ebook to have a multi-level Table of Contents, make sure you create a properly indented Table of Contents in Word.
```bgraph-paragraph
{"id":"bad090b5-1c9a-5cd3-afb7-cbfb96eafd80","node_type":"Paragraph","location":{"semantic":{"path":"4.5.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":36,"token_count":110,"style":null}
```

If no Table of Contents is found in the document, then a table of contents is automatically generated from the headings in the document. A heading is identified as something that has the Heading 1 or Heading 2, etc. style applied to it. These headings are turned into a Table of Contents with Heading 1 being the topmost level, Heading 2 the second level and so on.
```bgraph-paragraph
{"id":"cc811320-d877-52cf-8dfc-4f206310e1f2","node_type":"Paragraph","location":{"semantic":{"path":"4.5.2","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":37,"token_count":91,"style":null}
```

You can see the Table of Contents created by calibre by clicking the Table of Contents button in whatever viewer you are using to view the converted ebook.
```bgraph-paragraph
{"id":"f20a881e-0f21-58f6-af17-0bf4c9acaef5","node_type":"Paragraph","location":{"semantic":{"path":"4.5.3","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":38,"token_count":39,"style":null}
```

Demonstration of DOCX support in calibre	1
```bgraph-paragraph
{"id":"30c4ceac-fbad-5793-bda2-c672bf4aedb3","node_type":"Paragraph","location":{"semantic":{"path":"4.5.4","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":39,"token_count":10,"internal_refs":[{"text":"Demonstration of DOCX support in calibre\t1","target":{"kind":"named","name":"_Toc359077851"}}],"style":null}
```

Text Formatting	2
```bgraph-paragraph
{"id":"1b32df21-56c1-5d7c-90f6-d2db699b720c","node_type":"Paragraph","location":{"semantic":{"path":"4.5.5","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":40,"token_count":4,"internal_refs":[{"text":"Text Formatting\t2","target":{"kind":"named","name":"_Toc359077852"}}],"style":null}
```

Inline formatting	2
```bgraph-paragraph
{"id":"04a6c2ec-ef39-5bc3-ac21-62dc3c1b2d3b","node_type":"Paragraph","location":{"semantic":{"path":"4.5.6","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":41,"token_count":4,"internal_refs":[{"text":"Inline formatting\t2","target":{"kind":"named","name":"_Toc359077853"}}],"style":null}
```

Fun with fonts	2
```bgraph-paragraph
{"id":"45c34068-9b56-50b3-ad76-0f470e5871b9","node_type":"Paragraph","location":{"semantic":{"path":"4.5.7","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":42,"token_count":4,"internal_refs":[{"text":"Fun with fonts\t2","target":{"kind":"named","name":"_Toc359077854"}}],"style":null}
```

Paragraph level formatting	2
```bgraph-paragraph
{"id":"55eb7136-4ea1-5160-9052-6a5b237731ce","node_type":"Paragraph","location":{"semantic":{"path":"4.5.8","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":43,"token_count":7,"internal_refs":[{"text":"Paragraph level formatting\t2","target":{"kind":"named","name":"_Toc359077855"}}],"style":null}
```

Tables	3
```bgraph-paragraph
{"id":"1796e8d6-c8b0-5ad6-beac-4b612dd80942","node_type":"Paragraph","location":{"semantic":{"path":"4.5.9","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":44,"token_count":2,"internal_refs":[{"text":"Tables\t3","target":{"kind":"named","name":"_Toc359077856"}}],"style":null}
```

Structural Elements	5
```bgraph-paragraph
{"id":"cf6c8180-b358-57d1-9805-a3eeb5e45fba","node_type":"Paragraph","location":{"semantic":{"path":"4.5.10","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":45,"token_count":5,"internal_refs":[{"text":"Structural Elements\t5","target":{"kind":"named","name":"_Toc359077857"}}],"style":null}
```

Footnotes & Endnotes	5
```bgraph-paragraph
{"id":"09a1eda6-eed2-5ebc-b7e9-abf27134261b","node_type":"Paragraph","location":{"semantic":{"path":"4.5.11","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":46,"token_count":5,"internal_refs":[{"text":"Footnotes & Endnotes\t5","target":{"kind":"named","name":"_Toc359077858"}}],"style":null}
```

Dropcaps	5
```bgraph-paragraph
{"id":"0c7af44b-2dc1-537a-9379-9b686e56592a","node_type":"Paragraph","location":{"semantic":{"path":"4.5.12","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":47,"token_count":2,"internal_refs":[{"text":"Dropcaps\t5","target":{"kind":"named","name":"_Toc359077859"}}],"style":null}
```

Links	5
```bgraph-paragraph
{"id":"640eaad4-0bea-52d4-9f7d-d5cd6fb33fbb","node_type":"Paragraph","location":{"semantic":{"path":"4.5.13","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":48,"token_count":1,"internal_refs":[{"text":"Links\t5","target":{"kind":"named","name":"_Toc359077860"}}],"style":null}
```

Table of Contents	5
```bgraph-paragraph
{"id":"ed8eedd4-4d35-5ac3-baf4-8d7228b69901","node_type":"Paragraph","location":{"semantic":{"path":"4.5.14","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":49,"token_count":4,"internal_refs":[{"text":"Table of Contents\t5","target":{"kind":"named","name":"_Toc359077861"}}],"style":null}
```

Images	7
```bgraph-paragraph
{"id":"9a994225-8d89-5fe3-9a2c-04efcd0d893b","node_type":"Paragraph","location":{"semantic":{"path":"4.5.15","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":50,"token_count":2,"internal_refs":[{"text":"Images\t7","target":{"kind":"named","name":"_Toc359077862"}}],"style":null}
```

Lists	8
```bgraph-paragraph
{"id":"8c3f5b35-68f9-5560-9ccf-52e2616085b4","node_type":"Paragraph","location":{"semantic":{"path":"4.5.16","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":51,"token_count":1,"internal_refs":[{"text":"Lists\t8","target":{"kind":"named","name":"_Toc359077863"}}],"style":null}
```

Bulleted List	8
```bgraph-paragraph
{"id":"ed4329ce-0fca-504b-b06e-f51c3355a820","node_type":"Paragraph","location":{"semantic":{"path":"4.5.17","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":52,"token_count":3,"internal_refs":[{"text":"Bulleted List\t8","target":{"kind":"named","name":"_Toc359077864"}}],"style":null}
```

Numbered List	8
```bgraph-paragraph
{"id":"abcd2f3a-c7cd-53c8-9553-42f405faf995","node_type":"Paragraph","location":{"semantic":{"path":"4.5.18","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":53,"token_count":3,"internal_refs":[{"text":"Numbered List\t8","target":{"kind":"named","name":"_Toc359077865"}}],"style":null}
```

Multi-level Lists	8
```bgraph-paragraph
{"id":"1262e2e9-6584-5499-b34b-7d90387babdd","node_type":"Paragraph","location":{"semantic":{"path":"4.5.19","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":54,"token_count":4,"internal_refs":[{"text":"Multi-level Lists\t8","target":{"kind":"named","name":"_Toc359077866"}}],"style":null}
```

Continued Lists	8
```bgraph-paragraph
{"id":"e82d9410-67ae-514b-b486-44ee418ce7e8","node_type":"Paragraph","location":{"semantic":{"path":"4.5.20","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":55,"token_count":4,"internal_refs":[{"text":"Continued Lists\t8","target":{"kind":"named","name":"_Toc359077867"}}],"style":null}
```

# Images
```bgraph-section
{"id":"13de0fea-ee97-5205-a360-cbd4ab59a1b2","node_type":"Section","location":{"semantic":{"path":"5","depth":1,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":56,"token_count":1,"style":null}
```

Images can be of three main types. Inline images are images that are part of the normal text flow, like this image of a green dot . Inline images do not cause breaks in the text and are usually small in size. The next category of image is a floating image, one that “floats “ on the page and is surrounded by text. Word supports more types of floating images than are possible with current ebook technology, so the conversion maps floating images to simple left and right floats, as you can see with the left and right arrow images on the sides of this paragraph.
```bgraph-paragraph
{"id":"998f8e13-073c-5fe3-a52e-f17f26d94a9b","node_type":"Paragraph","location":{"semantic":{"path":"5.1","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":57,"token_count":141,"style":null}
```

The final type of image is a “block” image, one that becomes a paragraph on its own and has no text on either side. Below is a centered green dot.
```bgraph-paragraph
{"id":"badab6bd-e0d6-5f20-ad5f-11a1d19d7e08","node_type":"Paragraph","location":{"semantic":{"path":"5.2","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":58,"token_count":37,"style":null}
```

Centered images like this are useful for large pictures that should be a focus of attention.
```bgraph-paragraph
{"id":"d424318b-d8f1-58da-a98d-fd43833955e6","node_type":"Paragraph","location":{"semantic":{"path":"5.3","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":59,"token_count":23,"style":null}
```

Generally, it is not possible to translate the exact positioning of images from a Word document to an ebook. That is because in Word, image positioning is specified in absolute units from the page boundaries.  There is no analogous technology in ebooks, so the conversion will usually end up placing the image either centered or floating close to the point in the text where it was inserted, not necessarily where it appears on the page in Word.
```bgraph-paragraph
{"id":"e500992e-f40a-566b-88de-48dd760c96df","node_type":"Paragraph","location":{"semantic":{"path":"5.4","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":60,"token_count":111,"style":null}
```

# Lists
```bgraph-section
{"id":"3b7a356f-74d8-5c42-a340-b515b6311a2d","node_type":"Section","location":{"semantic":{"path":"6","depth":1,"breadcrumbs":["DOCX Demo","Lists"]},"physical":null},"text_order":61,"token_count":1,"style":null}
```

All types of lists are supported by the conversion, with the exception of lists that use fancy bullets, these get converted to regular bullets.
```bgraph-paragraph
{"id":"c3ad2ac5-82be-53e2-be6a-c1d60a087b76","node_type":"Paragraph","location":{"semantic":{"path":"6.1","depth":2,"breadcrumbs":["DOCX Demo","Lists"]},"physical":null},"text_order":62,"token_count":35,"style":null}
```

## Bulleted List
```bgraph-section
{"id":"dd14cf3a-b037-54fc-88b2-b4c3355113ea","node_type":"Section","location":{"semantic":{"path":"6.2","depth":2,"breadcrumbs":["DOCX Demo","Lists","Bulleted List"]},"physical":null},"text_order":63,"token_count":3,"style":null}
```

One
```bgraph-paragraph
{"id":"0f8e5763-2903-54b1-8614-00d649867754","node_type":"Paragraph","location":{"semantic":{"path":"6.2.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Bulleted List"]},"physical":null},"text_order":64,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"537e5f7a-4840-5050-83d8-09ea0a4e877b","node_type":"Paragraph","location":{"semantic":{"path":"6.2.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Bulleted List"]},"physical":null},"text_order":65,"token_count":1,"style":null}
```

## Numbered List
```bgraph-section
{"id":"b87e1f60-0e11-58ed-8601-8e5c8c59cc09","node_type":"Section","location":{"semantic":{"path":"6.3","depth":2,"breadcrumbs":["DOCX Demo","Lists","Numbered List"]},"physical":null},"text_order":66,"token_count":3,"style":null}
```

One, with a very long line to demonstrate that the hanging indent for the list is working correctly
```bgraph-paragraph
{"id":"233665aa-eb29-5098-b964-9ae60c22aed7","node_type":"Paragraph","location":{"semantic":{"path":"6.3.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Numbered List"]},"physical":null},"text_order":67,"token_count":24,"style":null}
```

Two
```bgraph-paragraph
{"id":"35c21d85-f4d6-5a4b-91bf-103d98626e54","node_type":"Paragraph","location":{"semantic":{"path":"6.3.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Numbered List"]},"physical":null},"text_order":68,"token_count":1,"style":null}
```

## Multi-level Lists
```bgraph-section
{"id":"c3707509-ad3f-5fa8-9ac3-2cf95167a2ed","node_type":"Section","location":{"semantic":{"path":"6.4","depth":2,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":69,"token_count":4,"style":null}
```

One
```bgraph-paragraph
{"id":"2beea09d-1d14-5549-a81c-e039472a0916","node_type":"Paragraph","location":{"semantic":{"path":"6.4.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":70,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"7892613e-39fa-545c-90fc-728609daf807","node_type":"Paragraph","location":{"semantic":{"path":"6.4.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":71,"token_count":1,"style":null}
```

Three
```bgraph-paragraph
{"id":"13f334f8-e279-5551-898f-7a6f499322cb","node_type":"Paragraph","location":{"semantic":{"path":"6.4.3","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":72,"token_count":1,"style":null}
```

Four with a very long line to demonstrate that the hanging indent for the list is working correctly.
```bgraph-paragraph
{"id":"dd16da8f-6eb6-5ffa-93ad-6583e3975493","node_type":"Paragraph","location":{"semantic":{"path":"6.4.4","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":73,"token_count":25,"style":null}
```

Five
```bgraph-paragraph
{"id":"6a65d881-c599-5265-83b7-301ded47f213","node_type":"Paragraph","location":{"semantic":{"path":"6.4.5","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":74,"token_count":1,"style":null}
```

Six
```bgraph-paragraph
{"id":"282e3c0b-810d-53ed-aed0-8e714dea9c21","node_type":"Paragraph","location":{"semantic":{"path":"6.4.6","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":75,"token_count":1,"style":null}
```

A Multi-level list with bullets:
```bgraph-paragraph
{"id":"3c331daa-b5af-56be-bedc-e63bd0fe7f15","node_type":"Paragraph","location":{"semantic":{"path":"6.4.7","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":76,"token_count":8,"style":null}
```

One
```bgraph-paragraph
{"id":"b761b2bf-c805-54f9-bf0a-356816096996","node_type":"Paragraph","location":{"semantic":{"path":"6.4.8","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":77,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"87b70c2f-5a0b-502d-9673-7947b386e6eb","node_type":"Paragraph","location":{"semantic":{"path":"6.4.9","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":78,"token_count":1,"style":null}
```

This bullet uses an image as the bullet item
```bgraph-paragraph
{"id":"fe27b1a5-ab02-57b0-9637-2b9a7d9ee042","node_type":"Paragraph","location":{"semantic":{"path":"6.4.10","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":79,"token_count":11,"style":null}
```

Four
```bgraph-paragraph
{"id":"473f7d8d-8368-57fd-8c51-bf55e7d22b41","node_type":"Paragraph","location":{"semantic":{"path":"6.4.11","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":80,"token_count":1,"style":null}
```

Five
```bgraph-paragraph
{"id":"dc5099f0-f24c-5bad-b943-d287c3b5136b","node_type":"Paragraph","location":{"semantic":{"path":"6.4.12","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":81,"token_count":1,"style":null}
```

## Continued Lists
```bgraph-section
{"id":"b82ccb3c-559d-5733-a3b8-5e3f05d90a92","node_type":"Section","location":{"semantic":{"path":"6.5","depth":2,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":82,"token_count":3,"style":null}
```

One
```bgraph-paragraph
{"id":"97ce64dd-9fab-57a4-8fac-9090ad036538","node_type":"Paragraph","location":{"semantic":{"path":"6.5.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":83,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"8084f443-2e8d-5d2b-a983-efe6ab42ecaf","node_type":"Paragraph","location":{"semantic":{"path":"6.5.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":84,"token_count":1,"style":null}
```

An interruption in our regularly scheduled listing, for this essential and very relevant public service announcement.
```bgraph-paragraph
{"id":"a9b5c38a-9405-5009-81a3-6e19fe1f861b","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":85,"token_count":29,"style":null}
```

We now resume our normal programming
```bgraph-paragraph
{"id":"9cb65b99-2858-5646-bdc5-0c5d8b8bc02f","node_type":"Paragraph","location":{"semantic":{"path":"6.5.4","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":86,"token_count":9,"style":null}
```

Four
```bgraph-paragraph
{"id":"ef4ccd5c-d9a3-50a6-a882-aba756520373","node_type":"Paragraph","location":{"semantic":{"path":"6.5.5","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":87,"token_count":1,"style":null}
```
