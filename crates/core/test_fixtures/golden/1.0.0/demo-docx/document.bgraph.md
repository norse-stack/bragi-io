```bgraph
{"schema":"1.0.0","kind":"document","bragi_version":"0.5.0","source":{"format":"docx","sha256":"269329fc7ae54b3f289b3ac52efde387edc2e566ef9a48d637e841022c7e0eab"},"flow_type":"Free","config_hash":"none","bgraph_sha256":"97203352142b94cf1b2e5b5b43b7040b513bde15defc57ef9877d80cacc81e2a"}
```

```bgraph-metadata
{"title":"DOCX Demo","author":"Kovid Goyal","description":"Demonstration of DOCX support in calibre","language":null,"created":"2013-06-05T07:56:00Z","docx":{"application":"Microsoft Office Word","app_version":"12.0000","pages":3,"words":1518,"characters":8657,"lines":72,"paragraphs":20,"company":null,"manager":null,"template":"Normal","total_time":8540,"doc_security":0,"last_modified_by":"kovid","revision":"79","modified":"2013-06-20T06:14:00Z","extras":{"keywords":"calibre, docs, ebook, conversion"}}}
```

# Demonstration of DOCX support in calibre
```bgraph-section
{"id":"f3bb3672-a9bf-5dc6-b651-0530242bcf9b","node_type":"Section","location":{"semantic":{"path":"1","depth":1,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":0,"token_count":10,"style":null}
```

This document demonstrates the ability of the calibre DOCX Input plugin to convert the various typographic features in a Microsoft Word (2007 and newer) document. Convert this document to a modern ebook format, such as AZW3 for Kindles or EPUB for other ebook readers, to see it in action.
```bgraph-paragraph
{"id":"e10b0982-57b0-513a-a483-85fd4672eb4d","node_type":"Paragraph","location":{"semantic":{"path":"1.1","depth":2,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":1,"token_count":72,"style":null}
```

There is support for images, tables, lists, footnotes, endnotes, links, dropcaps and various types of text and paragraph level formatting.
```bgraph-paragraph
{"id":"c0eaf184-f070-58e0-84b1-1fbe94c0da7c","node_type":"Paragraph","location":{"semantic":{"path":"1.2","depth":2,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":2,"token_count":34,"style":null}
```

To see the DOCX conversion in action, simply add this file to calibre using the **“Add Books”** button and then click “**Convert”.**  Set the output format in the top right corner of the conversion dialog to EPUB or AZW3 and click **“OK”**.
```bgraph-paragraph
{"id":"1d789a4f-ac7c-5d93-9459-fb08cb0d1e11","node_type":"Paragraph","location":{"semantic":{"path":"1.3","depth":2,"breadcrumbs":["DOCX Demo","Demonstration of DOCX support in calibre"]},"physical":null},"text_order":3,"token_count":63,"style":null}
```

# Text Formatting
```bgraph-section
{"id":"0f5f653f-7775-5928-b20e-05c3f5ee15e7","node_type":"Section","location":{"semantic":{"path":"2","depth":1,"breadcrumbs":["DOCX Demo","Text Formatting"]},"physical":null},"text_order":4,"token_count":3,"style":null}
```

## Inline formatting
```bgraph-section
{"id":"085c40a0-472d-5715-9c88-b9f2b38e61f3","node_type":"Section","location":{"semantic":{"path":"2.1","depth":2,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":5,"token_count":4,"style":null}
```

Here, we demonstrate various types of inline text formatting and the use of embedded fonts.
```bgraph-paragraph
{"id":"f39f0d9b-1f01-53a5-9b3a-3aa171904a01","node_type":"Paragraph","location":{"semantic":{"path":"2.1.1","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":6,"token_count":22,"style":null}
```

Here is some **bold,** *italic,* ***bold-italic,*** underlined and struck out  text. Then, we have a superscript and a subscript. Now we see some red, green and blue text. Some text with a yellow highlight. Some text in a box. Some text in inverse video.
```bgraph-paragraph
{"id":"238aa39f-a025-55f5-ba88-09dac0aa19fb","node_type":"Paragraph","location":{"semantic":{"path":"2.1.2","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":7,"token_count":63,"style":null}
```

A paragraph with styled text: subtle emphasis  followed by strong text and intense emphasis. This paragraph uses document wide styles for styling rather than inline text properties as demonstrated in the previous paragraph — calibre can handle both with equal ease.
```bgraph-paragraph
{"id":"ffe2bad4-1176-5d19-ae03-d8e77e4b030e","node_type":"Paragraph","location":{"semantic":{"path":"2.1.3","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Inline formatting"]},"physical":null},"text_order":8,"token_count":66,"style":null}
```

## Fun with fonts
```bgraph-section
{"id":"b87bdc0d-0ca6-53b4-bb95-63afcf60d588","node_type":"Section","location":{"semantic":{"path":"2.2","depth":2,"breadcrumbs":["DOCX Demo","Text Formatting","Fun with fonts"]},"physical":null},"text_order":9,"token_count":3,"style":null}
```

This document has embedded the Ubuntu font family. The body text is in the Ubuntu typeface, here is some text in the Ubuntu Mono typeface, notice how every letter has the same width, even i and m. Every embedded font will automatically be embedded in the output ebook during conversion.
```bgraph-paragraph
{"id":"23cb343d-8d94-5ae3-b5de-fa1c7fb357fb","node_type":"Paragraph","location":{"semantic":{"path":"2.2.1","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","Fun with fonts"]},"physical":null},"text_order":10,"token_count":71,"style":null}
```

## **Paragraph level formatting**
```bgraph-section
{"id":"5e5fa8f9-e8b3-5618-985d-fae1c39af013","node_type":"Section","location":{"semantic":{"path":"2.3","depth":2,"breadcrumbs":["DOCX Demo","Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":11,"token_count":7,"style":null}
```

You can do crazy things with paragraphs, if the urge strikes you. For instance this paragraph is right aligned and has a right border. It has also been given a light gray background.
```bgraph-paragraph
{"id":"b17b2df3-0c71-5361-805d-4f4226b722fb","node_type":"Paragraph","location":{"semantic":{"path":"2.3.1","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":12,"token_count":45,"style":null}
```

For the lovers of poetry amongst you, paragraphs with hanging indents, like this often come in handy. You can use hanging indents to ensure that a line of poetry retains its individual identity as a line even when the screen is  too narrow to display it as a single line. Not only does this paragraph have a hanging indent, it is also has an extra top margin, setting it apart from the preceding paragraph.
```bgraph-paragraph
{"id":"955eb278-fd65-5e55-afe4-803667f6536a","node_type":"Paragraph","location":{"semantic":{"path":"2.3.2","depth":3,"breadcrumbs":["DOCX Demo","Text Formatting","**Paragraph level formatting**"]},"physical":null},"text_order":13,"token_count":101,"style":null}
```

# Tables
```bgraph-section
{"id":"34132214-1e4d-5a32-8481-11fc9d2c6a76","node_type":"Section","location":{"semantic":{"path":"3","depth":1,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":14,"token_count":1,"style":null}
```

| ITEM        | NEEDED   |
|-------------|----------|
| Books       | 1        |
| Pens        | 3        |
| Pencils     | 2        |
| Highlighter | 2 colors |
| Scissors    | 1 pair   |
```bgraph-table
{"id":"ca3bbc3e-a037-5db1-9021-926abfd3d559","node_type":"Table","location":{"semantic":{"path":"3.1","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":15,"token_count":47,"style":null}
```

Tables in Word can vary from the extremely simple to the extremely complex. calibre tries to do its best when converting tables. While you may run into trouble with the occasional table, the vast majority of common cases should be converted very well, as demonstrated in this section. Note that for optimum results, when creating tables in Word, you should set their widths using percentages, rather than absolute units.  To the left of this paragraph is a floating two column table with a nice green border and header row.
```bgraph-paragraph
{"id":"c0d15a03-a494-56d5-9f36-603cd9e71a13","node_type":"Paragraph","location":{"semantic":{"path":"3.2","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":16,"token_count":130,"style":null}
```

Now let’s look at a fancier table—one with alternating row colors and partial borders. This table is stretched out to take 100% of the available width.
```bgraph-paragraph
{"id":"7de10ee5-8f15-509a-a185-50413cb556cc","node_type":"Paragraph","location":{"semantic":{"path":"3.3","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":17,"token_count":38,"style":null}
```

| City or Town | Point A | Point B | Point C | Point D | Point E |
|--------------|---------|---------|---------|---------|---------|
| Point A      | —       |         |         |         |         |
| Point B      | 87      | —       |         |         |         |
| Point C      | 64      | 56      | —       |         |         |
| Point D      | 37      | 32      | 91      | —       |         |
| Point E      | 93      | 35      | 54      | 43      | —       |
```bgraph-table
{"id":"d7777cdc-2adf-507c-8b86-d2834bc6a300","node_type":"Table","location":{"semantic":{"path":"3.4","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":18,"token_count":119,"style":null}
```

Next, we see a table with special formatting in various locations. Notice how the formatting for the header row and sub header rows is preserved.
```bgraph-paragraph
{"id":"1df93a2f-f53e-5cf2-ba08-51d64024fa50","node_type":"Paragraph","location":{"semantic":{"path":"3.5","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":19,"token_count":36,"style":null}
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
{"id":"0dce4de6-5945-5a33-9c76-c0b48e186630","node_type":"Table","location":{"semantic":{"path":"3.6","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":20,"token_count":150,"style":null}
```

Source: Fictitious data, for illustration purposes only
```bgraph-paragraph
{"id":"d0b48850-a02f-5557-abb8-2d58812fb16b","node_type":"Paragraph","location":{"semantic":{"path":"3.7","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":21,"token_count":13,"style":null}
```

Next, we have something a little more complex, a nested table, i.e. a table inside another table. Additionally, the inner table has some of its cells merged. The table is displayed horizontally centered.
```bgraph-paragraph
{"id":"b70d3e96-51f6-5c68-bedf-7e16931b0a48","node_type":"Paragraph","location":{"semantic":{"path":"3.8","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":22,"token_count":50,"style":null}
```

| One                                                            |     |
|----------------------------------------------------------------|-----|
| Three                                                          | Two |
| Four                                                           |     |
| To the left is a table inside a table, with some cells merged. |     |
```bgraph-table
{"id":"27b12ade-72d0-59b9-bf64-95b539fd83c5","node_type":"Table","location":{"semantic":{"path":"3.9","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":23,"token_count":91,"style":null}
```

We end with a fancy calendar, note how much of the original formatting is preserved. Note that this table will only display correctly on relatively wide screens. In general, very wide tables or tables whose cells have fixed width requirements don’t fare well in ebooks.
```bgraph-paragraph
{"id":"95355bdd-1f11-5f62-baba-b7f3366e614b","node_type":"Paragraph","location":{"semantic":{"path":"3.10","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":24,"token_count":67,"style":null}
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
{"id":"d65093b8-5367-52de-b846-3155e3b4878e","node_type":"Table","location":{"semantic":{"path":"3.11","depth":2,"breadcrumbs":["DOCX Demo","Tables"]},"physical":null},"text_order":25,"token_count":314,"style":null}
```

# Structural Elements
```bgraph-section
{"id":"afa86083-24a9-55ed-9bf6-909bc29ac44d","node_type":"Section","location":{"semantic":{"path":"4","depth":1,"breadcrumbs":["DOCX Demo","Structural Elements"]},"physical":null},"text_order":26,"token_count":4,"style":null}
```

Miscellaneous structural elements you can add to your document, like footnotes, endnotes, dropcaps and the like.
```bgraph-paragraph
{"id":"f1a5ae4c-86d5-5024-bff8-88dde0650327","node_type":"Paragraph","location":{"semantic":{"path":"4.1","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements"]},"physical":null},"text_order":27,"token_count":28,"style":null}
```

## Footnotes & Endnotes
```bgraph-section
{"id":"ff7c246a-e32b-542b-8084-551d99e193e2","node_type":"Section","location":{"semantic":{"path":"4.2","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":28,"token_count":5,"style":null}
```

Footnotes and endnotes are automatically recognized and both are converted to endnotes, with backlinks for maximum ease of use in ebook devices.
```bgraph-paragraph
{"id":"151074aa-1adc-5275-9f5c-fae5bcbf75ab","node_type":"Paragraph","location":{"semantic":{"path":"4.2.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Footnotes & Endnotes"]},"physical":null},"text_order":29,"token_count":36,"style":null}
```

## Dropcaps
```bgraph-section
{"id":"528500d5-dccf-52c5-82d0-db5ba7e34602","node_type":"Section","location":{"semantic":{"path":"4.3","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Dropcaps"]},"physical":null},"text_order":30,"token_count":2,"style":null}
```

D
```bgraph-paragraph
{"id":"e2230c7d-72af-5df5-a757-c84ddaba95bb","node_type":"Paragraph","location":{"semantic":{"path":"4.3.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Dropcaps"]},"physical":null},"text_order":31,"token_count":1,"style":null}
```

rop caps are used to emphasize the leading paragraph at the start of a section. In Word it is possible to specify how many lines of text a drop-cap should use. Because of limitations in ebook technology, this is not possible when converting.  Instead, the converted drop cap will use font size and line height to simulate the effect as well as possible. While not as good as the original, the result is usually tolerable. This paragraph has a “D” dropcap set to occupy three lines of text with a font size of 58.5 pts. Depending on the screen width and capabilities of the device you view the book on, this dropcap can look anything from perfect to ugly.
```bgraph-paragraph
{"id":"e5534b2e-325e-5d07-9586-b3943a2484bd","node_type":"Paragraph","location":{"semantic":{"path":"4.3.2","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Dropcaps"]},"physical":null},"text_order":32,"token_count":164,"style":null}
```

## Links
```bgraph-section
{"id":"ad99285b-bb17-5b16-b7ba-8ec73dd4aaed","node_type":"Section","location":{"semantic":{"path":"4.4","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Links"]},"physical":null},"text_order":33,"token_count":1,"style":null}
```

Two kinds of links are possible, those that refer to an external website and those that refer to locations inside the document itself. Both are supported by calibre. For example, here is a link pointing to the calibre download page. Then we have a link that points back to the section on paragraph level formatting in this document.
```bgraph-paragraph
{"id":"28e518fd-b17b-5ae6-80e7-628ae8608684","node_type":"Paragraph","location":{"semantic":{"path":"4.4.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Links"]},"physical":null},"text_order":34,"token_count":83,"internal_refs":[{"text":"paragraph level formatting","target":{"kind":"named","name":"_Paragraph_level_formatting"}}],"external_refs":[{"text":"calibre download page","target":{"kind":"uri","url":"http://calibre-ebook.com/download"}}],"style":null}
```

## Table of Contents
```bgraph-section
{"id":"a7a90df2-29eb-5bd9-a122-92de3b0c001c","node_type":"Section","location":{"semantic":{"path":"4.5","depth":2,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":35,"token_count":4,"style":null}
```

There are two approaches that calibre takes when generating a Table of Contents. The first is if the Word document has a Table of Contents itself. Provided that the Table of Contents uses hyperlinks, calibre will automatically use it. The levels of the Table of Contents are identified by their left indent, so if you want the ebook to have a multi-level Table of Contents, make sure you create a properly indented Table of Contents in Word.
```bgraph-paragraph
{"id":"46cf3d31-87a9-566e-ac38-c2011c8fa59f","node_type":"Paragraph","location":{"semantic":{"path":"4.5.1","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":36,"token_count":110,"style":null}
```

If no Table of Contents is found in the document, then a table of contents is automatically generated from the headings in the document. A heading is identified as something that has the Heading 1 or Heading 2, etc. style applied to it. These headings are turned into a Table of Contents with Heading 1 being the topmost level, Heading 2 the second level and so on.
```bgraph-paragraph
{"id":"4879d4b6-d9b6-58d5-9101-6867afd67300","node_type":"Paragraph","location":{"semantic":{"path":"4.5.2","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":37,"token_count":91,"style":null}
```

You can see the Table of Contents created by calibre by clicking the Table of Contents button in whatever viewer you are using to view the converted ebook.
```bgraph-paragraph
{"id":"1de148db-64ca-5ed5-8ca6-8143e1ef6ceb","node_type":"Paragraph","location":{"semantic":{"path":"4.5.3","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":38,"token_count":39,"style":null}
```

Demonstration of DOCX support in calibre	1
```bgraph-paragraph
{"id":"ff612e73-95a9-535d-b356-5135fde44cf3","node_type":"Paragraph","location":{"semantic":{"path":"4.5.4","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":39,"token_count":10,"internal_refs":[{"text":"Demonstration of DOCX support in calibre\t1","target":{"kind":"named","name":"_Toc359077851"}}],"style":null}
```

Text Formatting	2
```bgraph-paragraph
{"id":"814ac85c-a48f-5343-89bd-6e16855cd9a1","node_type":"Paragraph","location":{"semantic":{"path":"4.5.5","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":40,"token_count":4,"internal_refs":[{"text":"Text Formatting\t2","target":{"kind":"named","name":"_Toc359077852"}}],"style":null}
```

Inline formatting	2
```bgraph-paragraph
{"id":"0e2b658c-6760-5644-9c6e-e39b1a5efd6d","node_type":"Paragraph","location":{"semantic":{"path":"4.5.6","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":41,"token_count":4,"internal_refs":[{"text":"Inline formatting\t2","target":{"kind":"named","name":"_Toc359077853"}}],"style":null}
```

Fun with fonts	2
```bgraph-paragraph
{"id":"148dc0a2-1487-5000-9866-683be89215c6","node_type":"Paragraph","location":{"semantic":{"path":"4.5.7","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":42,"token_count":4,"internal_refs":[{"text":"Fun with fonts\t2","target":{"kind":"named","name":"_Toc359077854"}}],"style":null}
```

Paragraph level formatting	2
```bgraph-paragraph
{"id":"74c5604a-ed5f-5433-8089-6ec9016370bf","node_type":"Paragraph","location":{"semantic":{"path":"4.5.8","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":43,"token_count":7,"internal_refs":[{"text":"Paragraph level formatting\t2","target":{"kind":"named","name":"_Toc359077855"}}],"style":null}
```

Tables	3
```bgraph-paragraph
{"id":"b3082674-a72b-5600-9462-c0f7584ee33b","node_type":"Paragraph","location":{"semantic":{"path":"4.5.9","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":44,"token_count":2,"internal_refs":[{"text":"Tables\t3","target":{"kind":"named","name":"_Toc359077856"}}],"style":null}
```

Structural Elements	5
```bgraph-paragraph
{"id":"0aa95ecf-6a45-575d-bf1b-05d2e645ddcb","node_type":"Paragraph","location":{"semantic":{"path":"4.5.10","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":45,"token_count":5,"internal_refs":[{"text":"Structural Elements\t5","target":{"kind":"named","name":"_Toc359077857"}}],"style":null}
```

Footnotes & Endnotes	5
```bgraph-paragraph
{"id":"319a6f4f-0e50-5112-afb1-058b1b34b8dc","node_type":"Paragraph","location":{"semantic":{"path":"4.5.11","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":46,"token_count":5,"internal_refs":[{"text":"Footnotes & Endnotes\t5","target":{"kind":"named","name":"_Toc359077858"}}],"style":null}
```

Dropcaps	5
```bgraph-paragraph
{"id":"61107fce-cee4-59bc-9926-f736633e8de1","node_type":"Paragraph","location":{"semantic":{"path":"4.5.12","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":47,"token_count":2,"internal_refs":[{"text":"Dropcaps\t5","target":{"kind":"named","name":"_Toc359077859"}}],"style":null}
```

Links	5
```bgraph-paragraph
{"id":"abeb8eaf-b9e1-5084-a6c2-a93ab0bfcc79","node_type":"Paragraph","location":{"semantic":{"path":"4.5.13","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":48,"token_count":1,"internal_refs":[{"text":"Links\t5","target":{"kind":"named","name":"_Toc359077860"}}],"style":null}
```

Table of Contents	5
```bgraph-paragraph
{"id":"fcdf342f-2199-5ca4-9155-9440eac3f9ee","node_type":"Paragraph","location":{"semantic":{"path":"4.5.14","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":49,"token_count":4,"internal_refs":[{"text":"Table of Contents\t5","target":{"kind":"named","name":"_Toc359077861"}}],"style":null}
```

Images	7
```bgraph-paragraph
{"id":"e6a27d0f-5be9-549f-8090-a041636dfef4","node_type":"Paragraph","location":{"semantic":{"path":"4.5.15","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":50,"token_count":2,"internal_refs":[{"text":"Images\t7","target":{"kind":"named","name":"_Toc359077862"}}],"style":null}
```

Lists	8
```bgraph-paragraph
{"id":"764c0f13-aff4-5421-847e-e6df19f83613","node_type":"Paragraph","location":{"semantic":{"path":"4.5.16","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":51,"token_count":1,"internal_refs":[{"text":"Lists\t8","target":{"kind":"named","name":"_Toc359077863"}}],"style":null}
```

Bulleted List	8
```bgraph-paragraph
{"id":"81ec1b40-3734-5bc6-a938-94df57406b3b","node_type":"Paragraph","location":{"semantic":{"path":"4.5.17","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":52,"token_count":3,"internal_refs":[{"text":"Bulleted List\t8","target":{"kind":"named","name":"_Toc359077864"}}],"style":null}
```

Numbered List	8
```bgraph-paragraph
{"id":"961196f6-671e-595a-9f1f-f375c8d7d631","node_type":"Paragraph","location":{"semantic":{"path":"4.5.18","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":53,"token_count":3,"internal_refs":[{"text":"Numbered List\t8","target":{"kind":"named","name":"_Toc359077865"}}],"style":null}
```

Multi-level Lists	8
```bgraph-paragraph
{"id":"dbee3a0a-5f73-5dde-9c7c-d4c1b215e2b1","node_type":"Paragraph","location":{"semantic":{"path":"4.5.19","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":54,"token_count":4,"internal_refs":[{"text":"Multi-level Lists\t8","target":{"kind":"named","name":"_Toc359077866"}}],"style":null}
```

Continued Lists	8
```bgraph-paragraph
{"id":"d37774b0-b7cb-542e-acec-8a56711f302f","node_type":"Paragraph","location":{"semantic":{"path":"4.5.20","depth":3,"breadcrumbs":["DOCX Demo","Structural Elements","Table of Contents"]},"physical":null},"text_order":55,"token_count":4,"internal_refs":[{"text":"Continued Lists\t8","target":{"kind":"named","name":"_Toc359077867"}}],"style":null}
```

# Images
```bgraph-section
{"id":"2c06f8b0-ca62-597a-b870-cd1932bf6904","node_type":"Section","location":{"semantic":{"path":"5","depth":1,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":56,"token_count":1,"style":null}
```

Images can be of three main types. Inline images are images that are part of the normal text flow, like this image of a green dot . Inline images do not cause breaks in the text and are usually small in size. The next category of image is a floating image, one that “floats “ on the page and is surrounded by text. Word supports more types of floating images than are possible with current ebook technology, so the conversion maps floating images to simple left and right floats, as you can see with the left and right arrow images on the sides of this paragraph.
```bgraph-paragraph
{"id":"122793b9-5f97-558a-bb67-1d2ef4ed972a","node_type":"Paragraph","location":{"semantic":{"path":"5.1","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":57,"token_count":141,"style":null}
```

The final type of image is a “block” image, one that becomes a paragraph on its own and has no text on either side. Below is a centered green dot.
```bgraph-paragraph
{"id":"dbcaebee-9383-5502-97ba-21655ee61c3a","node_type":"Paragraph","location":{"semantic":{"path":"5.2","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":58,"token_count":37,"style":null}
```

Centered images like this are useful for large pictures that should be a focus of attention.
```bgraph-paragraph
{"id":"e541fcb7-aa6e-5a77-b5f9-cf0d6bf196de","node_type":"Paragraph","location":{"semantic":{"path":"5.3","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":59,"token_count":23,"style":null}
```

Generally, it is not possible to translate the exact positioning of images from a Word document to an ebook. That is because in Word, image positioning is specified in absolute units from the page boundaries.  There is no analogous technology in ebooks, so the conversion will usually end up placing the image either centered or floating close to the point in the text where it was inserted, not necessarily where it appears on the page in Word.
```bgraph-paragraph
{"id":"a56839ba-b2bf-5134-ad3f-05ff692ae5ee","node_type":"Paragraph","location":{"semantic":{"path":"5.4","depth":2,"breadcrumbs":["DOCX Demo","Images"]},"physical":null},"text_order":60,"token_count":111,"style":null}
```

# Lists
```bgraph-section
{"id":"a0dce2f1-f627-5d13-a251-66228e08c7bc","node_type":"Section","location":{"semantic":{"path":"6","depth":1,"breadcrumbs":["DOCX Demo","Lists"]},"physical":null},"text_order":61,"token_count":1,"style":null}
```

All types of lists are supported by the conversion, with the exception of lists that use fancy bullets, these get converted to regular bullets.
```bgraph-paragraph
{"id":"447f213a-a669-5c11-b466-5b736acc18a9","node_type":"Paragraph","location":{"semantic":{"path":"6.1","depth":2,"breadcrumbs":["DOCX Demo","Lists"]},"physical":null},"text_order":62,"token_count":35,"style":null}
```

## Bulleted List
```bgraph-section
{"id":"1ad1745e-0eaf-5444-9b21-71f325bcf963","node_type":"Section","location":{"semantic":{"path":"6.2","depth":2,"breadcrumbs":["DOCX Demo","Lists","Bulleted List"]},"physical":null},"text_order":63,"token_count":3,"style":null}
```

One
```bgraph-paragraph
{"id":"66a76022-b74d-5183-9549-345e8bfdc174","node_type":"Paragraph","location":{"semantic":{"path":"6.2.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Bulleted List"]},"physical":null},"text_order":64,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"44b99755-e57e-56a1-8c6d-271d37d367d4","node_type":"Paragraph","location":{"semantic":{"path":"6.2.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Bulleted List"]},"physical":null},"text_order":65,"token_count":1,"style":null}
```

## Numbered List
```bgraph-section
{"id":"417e9a7f-9c95-55eb-90e9-b0374dea2bf5","node_type":"Section","location":{"semantic":{"path":"6.3","depth":2,"breadcrumbs":["DOCX Demo","Lists","Numbered List"]},"physical":null},"text_order":66,"token_count":3,"style":null}
```

One, with a very long line to demonstrate that the hanging indent for the list is working correctly
```bgraph-paragraph
{"id":"08e6079a-855d-53ea-8a84-71a2a81f4c24","node_type":"Paragraph","location":{"semantic":{"path":"6.3.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Numbered List"]},"physical":null},"text_order":67,"token_count":24,"style":null}
```

Two
```bgraph-paragraph
{"id":"854786eb-3479-5f76-947b-300535f9cc1a","node_type":"Paragraph","location":{"semantic":{"path":"6.3.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Numbered List"]},"physical":null},"text_order":68,"token_count":1,"style":null}
```

## Multi-level Lists
```bgraph-section
{"id":"f08a1e4f-731e-520c-a08e-51c2cde267e1","node_type":"Section","location":{"semantic":{"path":"6.4","depth":2,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":69,"token_count":4,"style":null}
```

One
```bgraph-paragraph
{"id":"e927d4cf-1bb1-5c64-8a38-c114d221d900","node_type":"Paragraph","location":{"semantic":{"path":"6.4.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":70,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"e29c2cd4-d4a6-5daa-b08f-49cd3433ca5e","node_type":"Paragraph","location":{"semantic":{"path":"6.4.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":71,"token_count":1,"style":null}
```

Three
```bgraph-paragraph
{"id":"91212635-f3b8-50c5-b2b4-5fa4ed33320a","node_type":"Paragraph","location":{"semantic":{"path":"6.4.3","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":72,"token_count":1,"style":null}
```

Four with a very long line to demonstrate that the hanging indent for the list is working correctly.
```bgraph-paragraph
{"id":"7c2f1543-63d3-5f17-8453-331bf71994f6","node_type":"Paragraph","location":{"semantic":{"path":"6.4.4","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":73,"token_count":25,"style":null}
```

Five
```bgraph-paragraph
{"id":"7eef819e-790b-5d0f-aae1-7c1e6f40f51b","node_type":"Paragraph","location":{"semantic":{"path":"6.4.5","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":74,"token_count":1,"style":null}
```

Six
```bgraph-paragraph
{"id":"bd7b635b-81ba-5b49-b419-ba21461f974e","node_type":"Paragraph","location":{"semantic":{"path":"6.4.6","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":75,"token_count":1,"style":null}
```

A Multi-level list with bullets:
```bgraph-paragraph
{"id":"f5debedd-5479-59da-b98c-55812f341eac","node_type":"Paragraph","location":{"semantic":{"path":"6.4.7","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":76,"token_count":8,"style":null}
```

One
```bgraph-paragraph
{"id":"68631d82-511e-5445-a34c-686d842033ba","node_type":"Paragraph","location":{"semantic":{"path":"6.4.8","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":77,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"254f7287-2605-5ab6-b6f2-0adaf58ed756","node_type":"Paragraph","location":{"semantic":{"path":"6.4.9","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":78,"token_count":1,"style":null}
```

This bullet uses an image as the bullet item
```bgraph-paragraph
{"id":"f39cfa08-fa72-583a-8c32-65f3a207fc5a","node_type":"Paragraph","location":{"semantic":{"path":"6.4.10","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":79,"token_count":11,"style":null}
```

Four
```bgraph-paragraph
{"id":"e9428156-64a9-5c1e-9b77-52ef8de29b37","node_type":"Paragraph","location":{"semantic":{"path":"6.4.11","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":80,"token_count":1,"style":null}
```

Five
```bgraph-paragraph
{"id":"5949446d-3275-580d-b8ec-32ff27bdaaf6","node_type":"Paragraph","location":{"semantic":{"path":"6.4.12","depth":3,"breadcrumbs":["DOCX Demo","Lists","Multi-level Lists"]},"physical":null},"text_order":81,"token_count":1,"style":null}
```

## Continued Lists
```bgraph-section
{"id":"bdba2cfb-7e3f-5c49-a514-3f1b0a9d163a","node_type":"Section","location":{"semantic":{"path":"6.5","depth":2,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":82,"token_count":3,"style":null}
```

One
```bgraph-paragraph
{"id":"48778ec0-f64f-52c0-9067-5423b62b884f","node_type":"Paragraph","location":{"semantic":{"path":"6.5.1","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":83,"token_count":1,"style":null}
```

Two
```bgraph-paragraph
{"id":"e682dbda-60b6-5eb7-b03f-b98467f39509","node_type":"Paragraph","location":{"semantic":{"path":"6.5.2","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":84,"token_count":1,"style":null}
```

An interruption in our regularly scheduled listing, for this essential and very relevant public service announcement.
```bgraph-paragraph
{"id":"5622c63c-4b39-5a87-a74a-2be1b80f8812","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":85,"token_count":29,"style":null}
```

We now resume our normal programming
```bgraph-paragraph
{"id":"4a576cac-8517-59fe-9b05-8a23d4072e65","node_type":"Paragraph","location":{"semantic":{"path":"6.5.4","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":86,"token_count":9,"style":null}
```

Four
```bgraph-paragraph
{"id":"4967f16e-598a-5358-a211-313662f08969","node_type":"Paragraph","location":{"semantic":{"path":"6.5.5","depth":3,"breadcrumbs":["DOCX Demo","Lists","Continued Lists"]},"physical":null},"text_order":87,"token_count":1,"style":null}
```
