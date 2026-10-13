```bgraph
{"schema":"1.2.0","kind":"document","bragi_version":"0.7.2","source":{"format":"pdf","sha256":"730f1a9f38d16689900969a3eeeb1b31e45eda1dcc1f37a043afa90aff76c867"},"flow_type":"Fixed","config_hash":"6daf2782c704bd37307850ae43bdcbbe5982a6a0ac08b1c5104de3e78738d5ce","bgraph_sha256":"8ec7010f61d6e3aab72b4d6722c028f7a09f0dbc66c1b77941b8789c150e4b9e"}
```

```bgraph-metadata
{"title":"From Local to Global: A GraphRAG Approach to Query-Focused Summarization","author":null,"description":null,"language":null,"created":"2025-02-20T01:38:02Z","pdf":{"version":"1.5","producer":"pdfTeX-1.40.25","creator_tool":"LaTeX with hyperref","publisher":null,"page_count":26,"encrypted":false,"has_marked_content":false,"modified":"2025-02-20T01:38:02Z","extras":{"Content-Type":"application/pdf","X-TIKA:versionCount":"0","access_permission:assemble_document":"true","access_permission:can_modify":"true","access_permission:can_print":"true","access_permission:can_print_faithful":"true","access_permission:extract_content":"true","access_permission:extract_for_accessibility":"true","access_permission:fill_in_form":"true","access_permission:modify_annotations":"true","dc:format":"application/pdf; version=1.5","pdf:docinfo:created":"2025-02-20T01:38:02Z","pdf:docinfo:creator_tool":"LaTeX with hyperref","pdf:docinfo:custom:PTEX.Fullbanner":"This is pdfTeX, Version 3.141592653-2.6-1.40.25 (TeX Live 2023) kpathsea version 6.3.5","pdf:docinfo:modified":"2025-02-20T01:38:02Z","pdf:docinfo:producer":"pdfTeX-1.40.25","pdf:docinfo:trapped":"False","pdf:eofOffsets":"6893854","pdf:hasCollection":"false","pdf:hasXFA":"false","pdf:hasXMP":"false","pdf:incrementalUpdateCount":"0"}}}
```

```bgraph-outline
{"sections":[{"title":"Introduction","order":0,"level":1},{"title":"Background","order":1,"level":1},{"title":"RAG Approaches and Systems","order":2,"level":2},{"title":"Using Knowledge Graphs with LLMs and RAG","order":3,"level":2},{"title":"Adaptive benchmarking for RAG Evaluation","order":4,"level":2},{"title":"RAG evaluation criteria","order":5,"level":2},{"title":"Methods","order":6,"level":1},{"title":"GraphRAG Workflow","order":7,"level":2},{"title":"Source Documents –&gt; Text Chunks","order":8,"level":3},{"title":"Text Chunks –&gt; Entities &amp; Relationships","order":9,"level":3},{"title":"Entities &amp; Relationships –&gt; Knowledge Graph","order":10,"level":3},{"title":"Knowledge Graph –&gt; Graph Communities","order":11,"level":3},{"title":"Graph Communities –&gt; Community Summaries","order":12,"level":3},{"title":"Community Summaries –&gt; Community Answers –&gt; Global Answer","order":13,"level":3},{"title":"Global Sensemaking Question Generation","order":14,"level":2},{"title":"Criteria for Evaluating Global Sensemaking","order":15,"level":2},{"title":"Analysis","order":16,"level":1},{"title":"Experiment 1","order":17,"level":2},{"title":"Datasets","order":18,"level":3},{"title":"Conditions","order":19,"level":3},{"title":"Configuration","order":20,"level":3},{"title":"Experiment 2","order":21,"level":2},{"title":"Results","order":22,"level":1},{"title":"Experiment 1","order":23,"level":2},{"title":"Experiment 2","order":24,"level":2},{"title":"Discussion","order":25,"level":1},{"title":"Limitations of evaluation approach","order":26,"level":2},{"title":"Future work","order":27,"level":2},{"title":"Conclusion","order":28,"level":1},{"title":"Entity and Relationship Extraction Approach","order":29,"level":1},{"title":"Entity Extraction","order":30,"level":2},{"title":"Self-Reflection","order":31,"level":2},{"title":"Example Community Detection","order":32,"level":1},{"title":"Context Window Selection","order":33,"level":1},{"title":"Example Answer Comparison","order":34,"level":1},{"title":"System Prompts","order":35,"level":1},{"title":"Element Instance Generation","order":36,"level":2},{"title":"Community Summary Generation","order":37,"level":2},{"title":"Community Answer Generation","order":38,"level":2},{"title":"Global Answer Generation","order":39,"level":2},{"title":"Evaluation Prompts","order":40,"level":1},{"title":"Relative Assessment Prompt","order":41,"level":2},{"title":"Relative Assessment Metrics","order":42,"level":2},{"title":"Statistical Analysis","order":43,"level":1}]}
```

# From Local to Global: A GraphRAG Approach to Query-Focused Summarization
```bgraph-section
{"id":"97488a75-0154-50d2-a24a-66b52e793154","node_type":"Section","location":{"semantic":{"path":"1","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":121.7,"y":100.1,"width":368.7,"height":36.500008}}},"text_order":0,"token_count":17,"style":null}
```

## Darren Edge 1† Ha Trinh 1† Newman Cheng 2 Joshua Bradley 2 Alex Chao 3
```bgraph-section
{"id":"0696d509-1b6e-50a5-97e6-09da4e414e2a","node_type":"Section","location":{"semantic":{"path":"1.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Darren Edge 1† Ha Trinh 1† Newman Cheng 2 Joshua Bradley 2 Alex Chao 3"]},"physical":{"page":1,"bounding_box":{"x":120.1,"y":180.6,"width":371.3,"height":9.999985}}},"text_order":1,"token_count":17,"style":null}
```

## Apurva Mody 3 Steven Truitt 2 Dasha Metropolitansky 1 Robert Osazuwa Ness 1
```bgraph-section
{"id":"92daf96a-3dd4-5244-adb7-c7a0e5964037","node_type":"Section","location":{"semantic":{"path":"1.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Apurva Mody 3 Steven Truitt 2 Dasha Metropolitansky 1 Robert Osazuwa Ness 1"]},"physical":{"page":1,"bounding_box":{"x":118.8,"y":208.8,"width":373.90002,"height":10.0}}},"text_order":2,"token_count":18,"style":null}
```

## Jonathan Larson 1
```bgraph-section
{"id":"850d42d5-39c1-5706-a365-f662ce2b79f7","node_type":"Section","location":{"semantic":{"path":"1.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Jonathan Larson 1"]},"physical":{"page":1,"bounding_box":{"x":267.7,"y":236.9,"width":76.099976,"height":10.100006}}},"text_order":3,"token_count":4,"style":null}
```

1 Microsoft Research 2 Microsoft Strategic Missions and Technologies 3 Microsoft Office of the CTO
```bgraph-paragraph
{"id":"8d61fbce-8a08-59ba-8955-7fd514bd2d75","node_type":"Paragraph","location":{"semantic":{"path":"1.3.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Jonathan Larson 1"]},"physical":{"page":1,"bounding_box":{"x":210.1,"y":259.6,"width":191.9,"height":33.30002}}},"text_order":4,"token_count":24,"style":null}
```

{ daedge,trinhha,newmancheng,joshbradley,achao,moapurva, steventruitt,dasham,robertness,jolarso } @microsoft.com
```bgraph-paragraph
{"id":"985f1dce-5b91-5acb-a5c0-bc52a3679b72","node_type":"Paragraph","location":{"semantic":{"path":"1.3.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Jonathan Larson 1"]},"physical":{"page":1,"bounding_box":{"x":162.3,"y":305.3,"width":287.40002,"height":20.200012}}},"text_order":5,"token_count":27,"style":null}
```

† These authors contributed equally to this work
```bgraph-paragraph
{"id":"4e58c586-20ca-59a2-a9e2-cdfe95b0cdbb","node_type":"Paragraph","location":{"semantic":{"path":"1.3.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Jonathan Larson 1"]},"physical":{"page":1,"bounding_box":{"x":221.0,"y":338.1,"width":170.1,"height":9.300018}}},"text_order":6,"token_count":12,"style":null}
```

## Abstract
```bgraph-section
{"id":"9007e608-7426-51d4-8be6-b5eaabe03ee8","node_type":"Section","location":{"semantic":{"path":"1.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Abstract"]},"physical":{"page":1,"bounding_box":{"x":283.8,"y":378.5,"width":44.5,"height":10.6}}},"text_order":7,"token_count":2,"style":null}
```

The use of retrieval-augmented generation (RAG) to retrieve relevant informa- tion from an external knowledge source enables large language models (LLMs) to answer questions over private and/or previously unseen document collections. However, RAG fails on global questions directed at an entire text corpus, such as “What are the main themes in the dataset?”, since this is inherently a query- focused summarization (QFS) task, rather than an explicit retrieval task. Prior QFS methods, meanwhile, do not scale to the quantities of text indexed by typ- ical RAG systems. To combine the strengths of these contrasting methods, we propose GraphRAG , a graph-based approach to question answering over private text corpora that scales with both the generality of user questions and the quantity of source text. Our approach uses an LLM to build a graph index in two stages: first, to derive an entity knowledge graph from the source documents, then to pre- generate community summaries for all groups of closely related entities. Given a question, each community summary is used to generate a partial response, before all partial responses are again summarized in a final response to the user. For a class of global sensemaking questions over datasets in the 1 million token range, we show that GraphRAG leads to substantial improvements over a conventional RAG baseline for both the comprehensiveness and diversity of generated answers.
```bgraph-paragraph
{"id":"1c74d0fb-885c-5fd6-97da-13a44f9727e3","node_type":"Paragraph","location":{"semantic":{"path":"1.4.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Abstract"]},"physical":{"page":1,"bounding_box":{"x":143.9,"y":404.6,"width":324.30002,"height":194.1}}},"text_order":8,"token_count":345,"style":null}
```

## 1 Introduction
```bgraph-section
{"id":"2e608627-25a9-5a26-8391-a3030d89cbde","node_type":"Section","location":{"semantic":{"path":"1.5","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":1,"bounding_box":{"x":108.0,"y":623.8,"width":82.8,"height":10.6}}},"text_order":9,"token_count":3,"style":null}
```

Retrieval augmented generation (RAG) ( Lewis et al. , 2020 ) is an established approach to using LLMs to answer queries based on data that is too large to contain in a language model’s context window , meaning the maximum number of tokens (units of text) that can be processed by the LLM at once ( Kuratov et al. , 2024 ; Liu et al. , 2023 ). In the canonical RAG setup, the system has access to a large external corpus of text records and retrieves a subset of records that are individually relevant to the query and collectively small enough to fit into the context window of the LLM. The LLM then
```bgraph-paragraph
{"id":"93728492-b55b-5b67-9a8a-e5ba5b933ab9","node_type":"Paragraph","location":{"semantic":{"path":"1.5.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":1,"bounding_box":{"x":108.0,"y":649.3,"width":396.0,"height":63.30005}}},"text_order":10,"token_count":142,"internal_refs":[{"text":"","source_page":1,"source_bbox":{"x":275.0,"y":648.2,"width":50.8,"height":10.9},"target":{"kind":"named","name":"cite.lewis2020retrieval","page":13,"point":{"x":83.0,"y":474.0}}},{"text":"","source_page":1,"source_bbox":{"x":330.2,"y":648.2,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.lewis2020retrieval","page":13,"point":{"x":83.0,"y":474.0}}},{"text":"","source_page":1,"source_bbox":{"x":142.8,"y":681.0,"width":55.5,"height":10.9},"target":{"kind":"named","name":"cite.kuratov2024search","page":13,"point":{"x":83.0,"y":371.0}}},{"text":"","source_page":1,"source_bbox":{"x":201.0,"y":681.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.kuratov2024search","page":13,"point":{"x":83.0,"y":371.0}}},{"text":"","source_page":1,"source_bbox":{"x":225.9,"y":681.0,"width":37.0,"height":10.9},"target":{"kind":"named","name":"cite.liu-etal:2023:tacl","page":13,"point":{"x":83.0,"y":515.0}}},{"text":"","source_page":1,"source_bbox":{"x":265.6,"y":681.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.liu-etal:2023:tacl","page":13,"point":{"x":83.0,"y":515.0}}}],"style":null}
```

Preprint. Under review.
```bgraph-paragraph
{"id":"e5b93e0c-0fa3-5047-bbc9-b121a30aeefd","node_type":"Paragraph","location":{"semantic":{"path":"1.5.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":1,"bounding_box":{"x":108.0,"y":733.9,"width":84.2,"height":7.7}}},"text_order":11,"token_count":5,"style":null}
```

arXiv:2404.16130v2 [cs.CL] 19 Feb 2025
```bgraph-margin
{"id":"b5e1d6b4-afb7-57dc-b318-e71dc3b9f91e","node_type":"Margin","location":{"semantic":{"path":"1.5.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":1,"bounding_box":{"x":32.0,"y":208.3,"width":0.0,"height":356.0}}},"text_order":12,"token_count":9,"style":null}
```

generates a response based on both the query and the retrieved records ( Baumel et al. , 2018 ; Dang , 2006 ; Laskar et al. , 2020 ; Yao et al. , 2017 ). This conventional approach, which we collectively call vector RAG , works well for queries that can be answered with information localized within a small set of records. However, vector RAG approaches do not support sensemaking queries, meaning queries that require global understanding of the entire dataset, such as ” What are the key trends in how scientific discoveries are influenced by interdisciplinary research over the past decade? ”
```bgraph-paragraph
{"id":"ac5ced73-660d-5fec-a113-f6a9e982025b","node_type":"Paragraph","location":{"semantic":{"path":"1.5.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":63.199997}}},"text_order":13,"token_count":145,"internal_refs":[{"text":"","source_page":2,"source_bbox":{"x":395.2,"y":74.2,"width":55.2,"height":10.9},"target":{"kind":"named","name":"cite.baumel2018query","page":12,"point":{"x":83.0,"y":207.0}}},{"text":"","source_page":2,"source_bbox":{"x":453.6,"y":74.2,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.baumel2018query","page":12,"point":{"x":83.0,"y":207.0}}},{"text":"","source_page":2,"source_bbox":{"x":478.9,"y":74.2,"width":23.6,"height":10.9},"target":{"kind":"named","name":"cite.dang2006duc","page":12,"point":{"x":83.0,"y":427.0}}},{"text":"","source_page":2,"source_bbox":{"x":107.0,"y":85.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.dang2006duc","page":12,"point":{"x":83.0,"y":427.0}}},{"text":"","source_page":2,"source_bbox":{"x":132.3,"y":85.1,"width":51.1,"height":10.9},"target":{"kind":"named","name":"cite.laskar2020query","page":13,"point":{"x":83.0,"y":421.0}}},{"text":"","source_page":2,"source_bbox":{"x":186.5,"y":85.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.laskar2020query","page":13,"point":{"x":83.0,"y":421.0}}},{"text":"","source_page":2,"source_bbox":{"x":211.7,"y":85.1,"width":39.6,"height":10.9},"target":{"kind":"named","name":"cite.yao2017recent","page":15,"point":{"x":83.0,"y":536.0}}},{"text":"","source_page":2,"source_bbox":{"x":254.4,"y":85.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yao2017recent","page":15,"point":{"x":83.0,"y":536.0}}}],"style":null}
```

Sensemaking tasks require reasoning over “ connections (which can be among people, places, and events) in order to anticipate their trajectories and act effectively ” ( Klein et al. , 2006 ). LLMs such as GPT ( Achiam et al. , 2023 ; Brown et al. , 2020 ), Llama ( Touvron et al. , 2023 ), and Gemini ( Anil et al. , 2023 ) excel at sensemaking in complex domains like scientific discovery ( Microsoft , 2023 ) and intelligence analysis ( Ranade and Joshi , 2023 ). Given a sensemaking query and a text with an implicit and interconnected set of concepts, an LLM can generate a summary that answers the query. The challenge, however, arises when the volume of data requires a RAG approach, since vector RAG approaches are unable to support sensemaking over an entire corpus.
```bgraph-paragraph
{"id":"2746ea50-9902-5221-b257-98f22d72f1e2","node_type":"Paragraph","location":{"semantic":{"path":"1.5.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":146.2,"width":396.0,"height":85.0}}},"text_order":14,"token_count":184,"internal_refs":[{"text":"","source_page":2,"source_bbox":{"x":378.4,"y":156.0,"width":46.3,"height":10.9},"target":{"kind":"named","name":"cite.klein2006amaking","page":13,"point":{"x":83.0,"y":310.0}}},{"text":"","source_page":2,"source_bbox":{"x":427.9,"y":156.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.klein2006amaking","page":13,"point":{"x":83.0,"y":310.0}}},{"text":"","source_page":2,"source_bbox":{"x":143.0,"y":166.9,"width":56.0,"height":10.6},"target":{"kind":"named","name":"cite.achiam2023gpt","page":11,"point":{"x":83.0,"y":688.0}}},{"text":"","source_page":2,"source_bbox":{"x":202.2,"y":166.9,"width":21.9,"height":10.6},"target":{"kind":"named","name":"cite.achiam2023gpt","page":11,"point":{"x":83.0,"y":688.0}}},{"text":"","source_page":2,"source_bbox":{"x":227.7,"y":166.9,"width":51.3,"height":10.6},"target":{"kind":"named","name":"cite.brown2020language","page":12,"point":{"x":83.0,"y":292.0}}},{"text":"","source_page":2,"source_bbox":{"x":282.2,"y":166.9,"width":21.9,"height":10.6},"target":{"kind":"named","name":"cite.brown2020language","page":12,"point":{"x":83.0,"y":292.0}}},{"text":"","source_page":2,"source_bbox":{"x":342.4,"y":166.9,"width":57.9,"height":10.6},"target":{"kind":"named","name":"cite.touvron2023llama","page":15,"point":{"x":83.0,"y":194.0}}},{"text":"","source_page":2,"source_bbox":{"x":403.6,"y":166.9,"width":21.9,"height":10.6},"target":{"kind":"named","name":"cite.touvron2023llama","page":15,"point":{"x":83.0,"y":194.0}}},{"text":"","source_page":2,"source_bbox":{"x":485.3,"y":166.9,"width":19.7,"height":10.6},"target":{"kind":"named","name":"cite.team2023gemini","page":12,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":2,"source_bbox":{"x":107.0,"y":177.8,"width":22.0,"height":10.9},"target":{"kind":"named","name":"cite.team2023gemini","page":12,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":2,"source_bbox":{"x":132.6,"y":177.8,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.team2023gemini","page":12,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":2,"source_bbox":{"x":434.9,"y":177.8,"width":41.3,"height":10.9},"target":{"kind":"named","name":"cite.ai4science2023impact","page":14,"point":{"x":83.0,"y":103.0}}},{"text":"","source_page":2,"source_bbox":{"x":479.8,"y":177.8,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.ai4science2023impact","page":14,"point":{"x":83.0,"y":103.0}}},{"text":"","source_page":2,"source_bbox":{"x":211.7,"y":188.7,"width":72.4,"height":10.9},"target":{"kind":"named","name":"cite.ranade2023fabula","page":14,"point":{"x":83.0,"y":468.0}}},{"text":"","source_page":2,"source_bbox":{"x":287.4,"y":188.7,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.ranade2023fabula","page":14,"point":{"x":83.0,"y":468.0}}}],"style":null}
```

In this paper, we present GraphRAG – a graph-based RAG approach that enables sensemaking over the entirety of a large text corpus. GraphRAG first uses an LLM to construct a knowledge graph, where nodes correspond to key entities in the corpus and edges represent relationships between those entities. Next, it partitions the graph into a hierarchy of communities of closely related entities, before using an LLM to generate community-level summaries. These summaries are generated in a bottom-up manner following the hierarchical structure of extracted communities, with summaries at higher levels of the hierarchy recursively incorporating lower-level summaries. Together, these community summaries provide global descriptions and insights over the corpus. Finally, GraphRAG answers queries through map-reduce processing of community summaries; in the map step, the summaries are used to provide partial answers to the query independently and in parallel, then in the reduce step, the partial answers are combined and used to generate a final global answer.
```bgraph-paragraph
{"id":"1975dbc8-729c-5d2c-a7ab-776d645b55af","node_type":"Paragraph","location":{"semantic":{"path":"1.5.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":238.8,"width":396.0,"height":117.90001}}},"text_order":15,"token_count":258,"style":null}
```

The GraphRAG method and its ability to perform global sensemaking over an entire corpus form the main contribution of this work. To demonstrate this ability, we developed a novel application of the LLM-as-a-judge technique ( Zheng et al. , 2024 ) suitable for questions targeting broad issues and themes where there is no ground-truth answer. This approach first uses one LLM to generate a diverse set of global sensemaking questions based on corpus-specific use cases, before using a second LLM to judge the answers of two different RAG systems using predefined criteria (defined in Section 3.3 ). We use this approach to compare GraphRAG to vector RAG on two representative real-world text datasets. Results show GraphRAG strongly outperforms vector RAG when using GPT-4 as the LLM.
```bgraph-paragraph
{"id":"bfe6a367-1e7a-5367-b262-54eafdd6b143","node_type":"Paragraph","location":{"semantic":{"path":"1.5.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":364.4,"width":396.0,"height":96.00003}}},"text_order":16,"token_count":189,"internal_refs":[{"text":"","source_page":2,"source_bbox":{"x":248.8,"y":385.2,"width":49.7,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}},{"text":"","source_page":2,"source_bbox":{"x":301.7,"y":385.2,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}},{"text":"","source_page":2,"source_bbox":{"x":117.5,"y":428.8,"width":47.1,"height":10.9},"target":{"kind":"named","name":"subsection.3.3","page":6,"point":{"x":108.0,"y":307.0}}}],"style":null}
```

GraphRAG is available as open-source software at https://github . com/microsoft/graphrag . In ad- dition, versions of the GraphRAG approach are also available as extensions to multiple open- source libraries, including LangChain ( LangChain , 2024 ), LlamaIndex ( LlamaIndex , 2024 ), Nebu- laGraph ( NebulaGraph , 2024 ), and Neo4J ( Neo4J , 2024 ).
```bgraph-paragraph
{"id":"fe96d0a4-844f-5234-9d55-445636d669a7","node_type":"Paragraph","location":{"semantic":{"path":"1.5.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":467.9,"width":396.0,"height":41.600006}}},"text_order":17,"token_count":81,"internal_refs":[{"text":"","source_page":2,"source_bbox":{"x":264.4,"y":488.8,"width":46.3,"height":10.9},"target":{"kind":"named","name":"cite.LangchainGraphRAG","page":13,"point":{"x":83.0,"y":401.0}}},{"text":"","source_page":2,"source_bbox":{"x":313.9,"y":488.8,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.LangchainGraphRAG","page":13,"point":{"x":83.0,"y":401.0}}},{"text":"","source_page":2,"source_bbox":{"x":396.5,"y":488.8,"width":50.0,"height":10.9},"target":{"kind":"named","name":"cite.LlamaIndexGraphRAG","page":13,"point":{"x":83.0,"y":546.0}}},{"text":"","source_page":2,"source_bbox":{"x":449.8,"y":488.8,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.LlamaIndexGraphRAG","page":13,"point":{"x":83.0,"y":546.0}}},{"text":"","source_page":2,"source_bbox":{"x":144.9,"y":499.7,"width":55.5,"height":10.9},"target":{"kind":"named","name":"cite.NebulaGraph","page":14,"point":{"x":83.0,"y":165.0}}},{"text":"","source_page":2,"source_bbox":{"x":203.4,"y":499.7,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.NebulaGraph","page":14,"point":{"x":83.0,"y":165.0}}},{"text":"","source_page":2,"source_bbox":{"x":279.7,"y":499.7,"width":27.4,"height":10.9},"target":{"kind":"named","name":"cite.Neo4jGraphRAG","page":14,"point":{"x":83.0,"y":196.0}}},{"text":"","source_page":2,"source_bbox":{"x":310.2,"y":499.7,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.Neo4jGraphRAG","page":14,"point":{"x":83.0,"y":196.0}}}],"external_refs":[{"text":"","source_page":2,"source_bbox":{"x":317.6,"y":467.0,"width":154.2,"height":10.9},"target":{"kind":"uri","url":"https://github.com/microsoft/graphrag"}},{"text":"","source_page":2,"source_bbox":{"x":317.6,"y":467.0,"width":154.2,"height":10.9},"target":{"kind":"uri","url":"https://github.com/microsoft/graphrag"}},{"text":"","source_page":2,"source_bbox":{"x":317.6,"y":467.0,"width":154.2,"height":10.9},"target":{"kind":"uri","url":"https://github.com/microsoft/graphrag"}}],"style":null}
```

## 2 Background
```bgraph-section
{"id":"1702f4f9-efef-539f-986c-879d13ce1876","node_type":"Section","location":{"semantic":{"path":"1.6","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":529.9,"width":80.8,"height":10.6}}},"text_order":18,"token_count":3,"style":null}
```

### 2.1 RAG Approaches and Systems
```bgraph-section
{"id":"8b49ddde-6ec4-5b87-9d33-e46ba52dcc05","node_type":"Section","location":{"semantic":{"path":"1.6.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":555.9,"width":152.29999,"height":8.700012}}},"text_order":19,"token_count":7,"style":null}
```

RAG generally refers to any system where a user query is used to retrieve relevant information from external data sources, whereupon this information is incorporated into the generation of a response to the query by an LLM (or other generative AI model, such as a multi-media model). The query and retrieved records populate a prompt template, which is then passed to the LLM ( Ram et al. , 2023 ). RAG is ideal when the total number of records in a data source is too large to include in a single prompt to the LLM, i.e. the amount of text in the data source exceeds the LLM’s context window.
```bgraph-paragraph
{"id":"aa48cc69-9dba-5a39-bf4e-aa67f300ec0c","node_type":"Paragraph","location":{"semantic":{"path":"1.6.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":576.9,"width":396.0,"height":63.200012}}},"text_order":20,"token_count":144,"internal_refs":[{"text":"","source_page":2,"source_bbox":{"x":430.8,"y":608.5,"width":43.2,"height":10.9},"target":{"kind":"named","name":"cite.ram2023context","page":14,"point":{"x":83.0,"y":426.0}}},{"text":"","source_page":2,"source_bbox":{"x":477.3,"y":608.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.ram2023context","page":14,"point":{"x":83.0,"y":426.0}}}],"style":null}
```

In canonical RAG approaches, the retrieval process returns a set number of records that are seman- tically similar to the query and the generated answer uses only the information in those retrieved records. A common approach to conventional RAG is to use text embeddings, retrieving records closest to the query in vector space where closeness corresponds to semantic similarity ( Gao et al. , 2023 ). While some RAG approaches may use alternative retrieval mechanisms, we collectively refer to the family of conventional approaches as vector RAG . GraphRAG contrasts with vector RAG in its ability to answer queries that require global sensemaking over the entire data corpus.
```bgraph-paragraph
{"id":"1dba16a3-c730-501d-aa9d-694490820497","node_type":"Paragraph","location":{"semantic":{"path":"1.6.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":108.0,"y":647.8,"width":396.0,"height":74.20001}}},"text_order":21,"token_count":163,"internal_refs":[{"text":"","source_page":2,"source_bbox":{"x":461.6,"y":679.5,"width":40.9,"height":10.9},"target":{"kind":"named","name":"cite.gao2023retrieval","page":12,"point":{"x":83.0,"y":625.0}}},{"text":"","source_page":2,"source_bbox":{"x":107.0,"y":690.4,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.gao2023retrieval","page":12,"point":{"x":83.0,"y":625.0}}}],"style":null}
```

2
```bgraph-paragraph
{"id":"a746eea8-d2ea-565f-a529-3582f399cac6","node_type":"Paragraph","location":{"semantic":{"path":"1.6.1.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":22,"token_count":1,"style":null}
```

GraphRAG builds upon prior work on advanced RAG strategies. GraphRAG leverages summaries over large sections of the data source as a form of ”self-memory” (described in Cheng et al. 2024 ), which are later used to answer queries as in Mao et al. 2020 ). These summaries are generated in parallel and iteratively aggregated into global summaries, similar to prior techniques ( Feng et al. , 2023 ; Gao et al. , 2023 ; Khattab et al. , 2022 ; Shao et al. , 2023 ; Su et al. , 2020 ; Trivedi et al. , 2022 ; Wang et al. , 2024 ). In particular, GraphRAG is similar to other approaches that use hierarchical indexing to create summaries (similar to Kim et al. 2023 ; Sarthi et al. 2024 ). GraphRAG contrasts with these approaches by generating a graph index from the source data, then applying graph-based community detection to create a thematic partitioning of the data.
```bgraph-paragraph
{"id":"b25de6a6-39ce-5c8e-a1ee-15663fed7244","node_type":"Paragraph","location":{"semantic":{"path":"1.6.1.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":95.899994}}},"text_order":23,"token_count":209,"internal_refs":[{"text":"","source_page":3,"source_bbox":{"x":426.3,"y":85.1,"width":50.3,"height":10.9},"target":{"kind":"named","name":"cite.cheng2024lift","page":12,"point":{"x":83.0,"y":334.0}}},{"text":"","source_page":3,"source_bbox":{"x":477.3,"y":85.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.cheng2024lift","page":12,"point":{"x":83.0,"y":334.0}}},{"text":"","source_page":3,"source_bbox":{"x":289.0,"y":96.0,"width":43.5,"height":10.9},"target":{"kind":"named","name":"cite.mao2020generation","page":13,"point":{"x":83.0,"y":648.0}}},{"text":"","source_page":3,"source_bbox":{"x":333.7,"y":96.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.mao2020generation","page":13,"point":{"x":83.0,"y":648.0}}},{"text":"","source_page":3,"source_bbox":{"x":457.3,"y":106.9,"width":45.3,"height":10.9},"target":{"kind":"named","name":"cite.feng2023retrieval","page":12,"point":{"x":83.0,"y":574.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":117.8,"width":21.9,"height":10.2},"target":{"kind":"named","name":"cite.feng2023retrieval","page":12,"point":{"x":83.0,"y":574.0}}},{"text":"","source_page":3,"source_bbox":{"x":132.5,"y":117.8,"width":41.0,"height":10.2},"target":{"kind":"named","name":"cite.gao2023retrieval","page":12,"point":{"x":83.0,"y":625.0}}},{"text":"","source_page":3,"source_bbox":{"x":176.8,"y":117.8,"width":21.9,"height":10.2},"target":{"kind":"named","name":"cite.gao2023retrieval","page":12,"point":{"x":83.0,"y":625.0}}},{"text":"","source_page":3,"source_bbox":{"x":202.3,"y":117.8,"width":56.0,"height":10.2},"target":{"kind":"named","name":"cite.khattab2022demonstrate","page":13,"point":{"x":83.0,"y":185.0}}},{"text":"","source_page":3,"source_bbox":{"x":261.6,"y":117.8,"width":21.9,"height":10.2},"target":{"kind":"named","name":"cite.khattab2022demonstrate","page":13,"point":{"x":83.0,"y":185.0}}},{"text":"","source_page":3,"source_bbox":{"x":287.1,"y":117.8,"width":44.4,"height":10.2},"target":{"kind":"named","name":"cite.shao2023enhancing","page":14,"point":{"x":83.0,"y":604.0}}},{"text":"","source_page":3,"source_bbox":{"x":334.8,"y":117.8,"width":21.9,"height":10.2},"target":{"kind":"named","name":"cite.shao2023enhancing","page":14,"point":{"x":83.0,"y":604.0}}},{"text":"","source_page":3,"source_bbox":{"x":360.2,"y":117.8,"width":35.0,"height":10.2},"target":{"kind":"named","name":"cite.su2020caire","page":15,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":3,"source_bbox":{"x":398.5,"y":117.8,"width":21.9,"height":10.2},"target":{"kind":"named","name":"cite.su2020caire","page":15,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":3,"source_bbox":{"x":424.0,"y":117.8,"width":53.0,"height":10.2},"target":{"kind":"named","name":"cite.trivedi2022interleaving","page":15,"point":{"x":83.0,"y":294.0}}},{"text":"","source_page":3,"source_bbox":{"x":480.3,"y":117.8,"width":21.9,"height":10.2},"target":{"kind":"named","name":"cite.trivedi2022interleaving","page":15,"point":{"x":83.0,"y":294.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":128.7,"width":48.8,"height":10.9},"target":{"kind":"named","name":"cite.wang2024feb4rag","page":15,"point":{"x":83.0,"y":365.0}}},{"text":"","source_page":3,"source_bbox":{"x":159.7,"y":128.7,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.wang2024feb4rag","page":15,"point":{"x":83.0,"y":365.0}}},{"text":"","source_page":3,"source_bbox":{"x":272.0,"y":139.6,"width":42.2,"height":10.9},"target":{"kind":"named","name":"cite.kim2023tree","page":13,"point":{"x":83.0,"y":279.0}}},{"text":"","source_page":3,"source_bbox":{"x":314.9,"y":139.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.kim2023tree","page":13,"point":{"x":83.0,"y":279.0}}},{"text":"","source_page":3,"source_bbox":{"x":340.4,"y":139.6,"width":48.3,"height":10.9},"target":{"kind":"named","name":"cite.sarthi2024raptor","page":14,"point":{"x":83.0,"y":552.0}}},{"text":"","source_page":3,"source_bbox":{"x":389.5,"y":139.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.sarthi2024raptor","page":14,"point":{"x":83.0,"y":552.0}}}],"style":null}
```

### 2.2 Using Knowledge Graphs with LLMs and RAG
```bgraph-section
{"id":"bb091309-4887-5306-978d-2d4161e69c02","node_type":"Section","location":{"semantic":{"path":"1.6.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.2 Using Knowledge Graphs with LLMs and RAG"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":188.8,"width":224.4,"height":8.699997}}},"text_order":24,"token_count":11,"style":null}
```

Approaches to knowledge graph extraction from natural language text corpora include rule- matching, statistical pattern recognition, clustering, and embeddings ( Etzioni et al. , 2004 ; Kim et al. , 2016 ; Mooney and Bunescu , 2005 ; Yates et al. , 2007 ). GraphRAG falls into a more recent body of research that use of LLMs for knowledge graph extraction ( Ban et al. , 2023 ; Melnyk et al. , 2022 ; OpenAI , 2023 ; Tan et al. , 2017 ; Trajanoska et al. , 2023 ; Yao et al. , 2023 ; Yates et al. , 2007 ; Zhang et al. , 2024a ). It also adds to a growing body of RAG approaches that use a knowledge graph as an index ( Gao et al. , 2023 ). Some techniques use subgraphs, elements of the graph, or properties of the graph structure directly in the prompt ( Baek et al. , 2023 ; He et al. , 2024 ; Zhang , 2023 ) or as factual grounding for generated outputs ( Kang et al. , 2023 ; Ranade and Joshi , 2023 ). Other techniques ( Wang et al. , 2023b ) use the knowledge graph to enhance retrieval, where at query time an LLM-based agent dynamically traverses a graph with nodes representing document elements (e.g., passages, tables) and edges encoding lexical and semantical similarity or structural relation- ships. GraphRAG contrasts with these approaches by focusing on a previously unexplored quality of graphs in this context: their inherent modularity ( Newman , 2006 ) and the ability to partition graphs into nested modular communities of closely related nodes (e.g., Louvain, Blondel et al. 2008 ; Lei- den, Traag et al. 2019 ). Specifically, GraphRAG recursively creates increasingly global summaries by using the LLM to create summaries spanning this community hierarchy.
```bgraph-paragraph
{"id":"f6970bee-af1f-5574-ba66-849a5fa64021","node_type":"Paragraph","location":{"semantic":{"path":"1.6.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.2 Using Knowledge Graphs with LLMs and RAG"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":209.7,"width":396.0,"height":183.3}}},"text_order":25,"token_count":402,"internal_refs":[{"text":"","source_page":3,"source_bbox":{"x":382.4,"y":219.6,"width":51.9,"height":10.9},"target":{"kind":"named","name":"cite.knowitall","page":12,"point":{"x":83.0,"y":521.0}}},{"text":"","source_page":3,"source_bbox":{"x":436.9,"y":219.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.knowitall","page":12,"point":{"x":83.0,"y":521.0}}},{"text":"","source_page":3,"source_bbox":{"x":461.7,"y":219.6,"width":40.8,"height":10.9},"target":{"kind":"named","name":"cite.kimprob","page":13,"point":{"x":83.0,"y":227.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":230.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.kimprob","page":13,"point":{"x":83.0,"y":227.0}}},{"text":"","source_page":3,"source_bbox":{"x":132.4,"y":230.5,"width":89.1,"height":10.9},"target":{"kind":"named","name":"cite.mooney","page":14,"point":{"x":83.0,"y":134.0}}},{"text":"","source_page":3,"source_bbox":{"x":224.7,"y":230.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.mooney","page":14,"point":{"x":83.0,"y":134.0}}},{"text":"","source_page":3,"source_bbox":{"x":250.1,"y":230.5,"width":45.9,"height":10.9},"target":{"kind":"named","name":"cite.yates-etal-2007-textrunner","page":15,"point":{"x":83.0,"y":595.0}}},{"text":"","source_page":3,"source_bbox":{"x":299.2,"y":230.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yates-etal-2007-textrunner","page":15,"point":{"x":83.0,"y":595.0}}},{"text":"","source_page":3,"source_bbox":{"x":350.3,"y":241.4,"width":41.1,"height":10.9},"target":{"kind":"named","name":"cite.ban2023query","page":12,"point":{"x":83.0,"y":145.0}}},{"text":"","source_page":3,"source_bbox":{"x":395.0,"y":241.4,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.ban2023query","page":12,"point":{"x":83.0,"y":145.0}}},{"text":"","source_page":3,"source_bbox":{"x":420.8,"y":241.4,"width":55.9,"height":10.9},"target":{"kind":"named","name":"cite.melnyk2022knowledge","page":13,"point":{"x":83.0,"y":710.0}}},{"text":"","source_page":3,"source_bbox":{"x":480.3,"y":241.4,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.melnyk2022knowledge","page":13,"point":{"x":83.0,"y":710.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":252.3,"width":34.1,"height":10.9},"target":{"kind":"named","name":"cite.openai2023chatgpt","page":14,"point":{"x":83.0,"y":322.0}}},{"text":"","source_page":3,"source_bbox":{"x":144.3,"y":252.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.openai2023chatgpt","page":14,"point":{"x":83.0,"y":322.0}}},{"text":"","source_page":3,"source_bbox":{"x":169.7,"y":252.3,"width":39.0,"height":10.9},"target":{"kind":"named","name":"cite.representationlearning","page":15,"point":{"x":83.0,"y":112.0}}},{"text":"","source_page":3,"source_bbox":{"x":212.0,"y":252.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.representationlearning","page":15,"point":{"x":83.0,"y":112.0}}},{"text":"","source_page":3,"source_bbox":{"x":237.4,"y":252.3,"width":68.2,"height":10.9},"target":{"kind":"named","name":"cite.Trajanoska2023EnhancingKG","page":15,"point":{"x":83.0,"y":264.0}}},{"text":"","source_page":3,"source_bbox":{"x":308.9,"y":252.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.Trajanoska2023EnhancingKG","page":15,"point":{"x":83.0,"y":264.0}}},{"text":"","source_page":3,"source_bbox":{"x":334.3,"y":252.3,"width":39.9,"height":10.9},"target":{"kind":"named","name":"cite.yao2023exploring","page":15,"point":{"x":83.0,"y":566.0}}},{"text":"","source_page":3,"source_bbox":{"x":377.5,"y":252.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yao2023exploring","page":15,"point":{"x":83.0,"y":566.0}}},{"text":"","source_page":3,"source_bbox":{"x":402.9,"y":252.3,"width":46.0,"height":10.9},"target":{"kind":"named","name":"cite.yates-etal-2007-textrunner","page":15,"point":{"x":83.0,"y":595.0}}},{"text":"","source_page":3,"source_bbox":{"x":452.1,"y":252.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yates-etal-2007-textrunner","page":15,"point":{"x":83.0,"y":595.0}}},{"text":"","source_page":3,"source_bbox":{"x":477.5,"y":252.3,"width":27.4,"height":10.9},"target":{"kind":"named","name":"cite.zhang2024causal","page":16,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":263.2,"width":22.1,"height":10.9},"target":{"kind":"named","name":"cite.zhang2024causal","page":16,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":3,"source_bbox":{"x":132.8,"y":263.2,"width":26.3,"height":10.9},"target":{"kind":"named","name":"cite.zhang2024causal","page":16,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":3,"source_bbox":{"x":147.9,"y":274.1,"width":41.7,"height":10.9},"target":{"kind":"named","name":"cite.gao2023retrieval","page":12,"point":{"x":83.0,"y":625.0}}},{"text":"","source_page":3,"source_bbox":{"x":193.3,"y":274.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.gao2023retrieval","page":12,"point":{"x":83.0,"y":625.0}}},{"text":"","source_page":3,"source_bbox":{"x":301.5,"y":285.0,"width":46.9,"height":10.9},"target":{"kind":"named","name":"cite.baek2023knowledge","page":12,"point":{"x":83.0,"y":114.0}}},{"text":"","source_page":3,"source_bbox":{"x":352.7,"y":285.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.baek2023knowledge","page":12,"point":{"x":83.0,"y":114.0}}},{"text":"","source_page":3,"source_bbox":{"x":379.2,"y":285.0,"width":38.1,"height":10.9},"target":{"kind":"named","name":"cite.he2024g","page":12,"point":{"x":83.0,"y":657.0}}},{"text":"","source_page":3,"source_bbox":{"x":421.6,"y":285.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.he2024g","page":12,"point":{"x":83.0,"y":657.0}}},{"text":"","source_page":3,"source_bbox":{"x":448.0,"y":285.0,"width":27.4,"height":10.9},"target":{"kind":"named","name":"cite.zhang2023graph","page":15,"point":{"x":83.0,"y":699.0}}},{"text":"","source_page":3,"source_bbox":{"x":479.8,"y":285.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.zhang2023graph","page":15,"point":{"x":83.0,"y":699.0}}},{"text":"","source_page":3,"source_bbox":{"x":296.6,"y":295.9,"width":46.7,"height":10.9},"target":{"kind":"named","name":"cite.kang2023knowledge","page":13,"point":{"x":83.0,"y":155.0}}},{"text":"","source_page":3,"source_bbox":{"x":347.0,"y":295.9,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.kang2023knowledge","page":13,"point":{"x":83.0,"y":155.0}}},{"text":"","source_page":3,"source_bbox":{"x":372.8,"y":295.9,"width":73.0,"height":10.9},"target":{"kind":"named","name":"cite.ranade2023fabula","page":14,"point":{"x":83.0,"y":468.0}}},{"text":"","source_page":3,"source_bbox":{"x":449.5,"y":295.9,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.ranade2023fabula","page":14,"point":{"x":83.0,"y":468.0}}},{"text":"","source_page":3,"source_bbox":{"x":155.6,"y":306.8,"width":47.2,"height":10.9},"target":{"kind":"named","name":"cite.wang2023knowledge","page":15,"point":{"x":83.0,"y":435.0}}},{"text":"","source_page":3,"source_bbox":{"x":206.0,"y":306.8,"width":26.9,"height":10.9},"target":{"kind":"named","name":"cite.wang2023knowledge","page":15,"point":{"x":83.0,"y":435.0}}},{"text":"","source_page":3,"source_bbox":{"x":303.8,"y":350.5,"width":37.7,"height":11.0},"target":{"kind":"named","name":"cite.newman2006modularity","page":14,"point":{"x":83.0,"y":227.0}}},{"text":"","source_page":3,"source_bbox":{"x":344.6,"y":350.5,"width":21.9,"height":11.0},"target":{"kind":"named","name":"cite.newman2006modularity","page":14,"point":{"x":83.0,"y":227.0}}},{"text":"","source_page":3,"source_bbox":{"x":403.7,"y":361.4,"width":56.1,"height":10.9},"target":{"kind":"named","name":"cite.blondel2008fast","page":12,"point":{"x":83.0,"y":249.0}}},{"text":"","source_page":3,"source_bbox":{"x":460.7,"y":361.4,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.blondel2008fast","page":12,"point":{"x":83.0,"y":249.0}}},{"text":"","source_page":3,"source_bbox":{"x":126.6,"y":372.3,"width":47.2,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}},{"text":"","source_page":3,"source_bbox":{"x":174.5,"y":372.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}}],"style":null}
```

### 2.3 Adaptive benchmarking for RAG Evaluation
```bgraph-section
{"id":"78661ef3-28ad-5b12-8c04-f32a61d6dddd","node_type":"Section","location":{"semantic":{"path":"1.6.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.3 Adaptive benchmarking for RAG Evaluation"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":410.6,"width":213.4,"height":8.700012}}},"text_order":26,"token_count":11,"style":null}
```

Many benchmark datasets for open-domain question answering exist, including HotPotQA ( Yang et al. , 2018 ), MultiHop-RAG ( Tang and Yang , 2024 ), and MT-Bench ( Zheng et al. , 2024 ). However, these benchmarks are oriented towards vector RAG performance, i.e., they evaluate performance on explicit fact retrieval. In this work, we propose an approach for generating a set of questions for evaluating global sensemaking over the entirety of the corpus. Our approach is related to LLM methods that use a corpus to generate questions whose answers would be summaries of the corpus, such as in Xu and Lapata ( 2021 ). However, in order to produce a fair evaluation, our method avoids generating the questions directly from the corpus itself (as an alternative implementation, one can use a subset of the corpus held out from subsequent graph extraction and answer evaluation steps).
```bgraph-paragraph
{"id":"1b369493-ffd0-59fa-a0b7-7bf046eaf199","node_type":"Paragraph","location":{"semantic":{"path":"1.6.3.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.3 Adaptive benchmarking for RAG Evaluation"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":431.5,"width":396.0,"height":96.0}}},"text_order":27,"token_count":211,"internal_refs":[{"text":"","source_page":3,"source_bbox":{"x":482.4,"y":430.4,"width":22.6,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":441.3,"width":21.1,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}},{"text":"","source_page":3,"source_bbox":{"x":130.9,"y":441.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}},{"text":"","source_page":3,"source_bbox":{"x":227.7,"y":441.3,"width":61.2,"height":10.9},"target":{"kind":"named","name":"cite.tang2024multihop","page":15,"point":{"x":83.0,"y":164.0}}},{"text":"","source_page":3,"source_bbox":{"x":291.7,"y":441.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.tang2024multihop","page":15,"point":{"x":83.0,"y":164.0}}},{"text":"","source_page":3,"source_bbox":{"x":384.7,"y":441.3,"width":48.8,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}},{"text":"","source_page":3,"source_bbox":{"x":436.3,"y":441.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}},{"text":"","source_page":3,"source_bbox":{"x":148.6,"y":495.9,"width":60.5,"height":10.9},"target":{"kind":"named","name":"cite.xu2021text","page":15,"point":{"x":83.0,"y":465.0}}},{"text":"","source_page":3,"source_bbox":{"x":212.9,"y":495.9,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.xu2021text","page":15,"point":{"x":83.0,"y":465.0}}}],"style":null}
```

Adaptive benchmarking refers to the process of dynamically generating evaluation benchmarks tai- lored to specific domains or use cases. Recent work has used LLMs for adaptive benchmarking to ensure relevance, diversity, and alignment with the target application or task ( Yuan et al. , 2024 ; Zhang et al. , 2024b ). In this work, we propose an adaptive benchmarking approach to generating global sensemaking queries for the LLM. Our approach builds on prior work in LLM-based persona generation, where the LLM is used to generate diverse and authentic sets of personas ( Kosinski , 2024 ; Salminen et al. , 2024 ; Shin et al. , 2024 ). Our adaptive benchmarking procedure uses persona generation to create queries that are representative of real-world RAG system usage. Specifically, our approach uses the LLM to infer the potential users would use the RAG system and their use cases, which guide the generation of corpus-specific sensemaking queries.
```bgraph-paragraph
{"id":"c87235da-b15b-504c-81cb-f3eafc6990b9","node_type":"Paragraph","location":{"semantic":{"path":"1.6.3.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.3 Adaptive benchmarking for RAG Evaluation"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":535.2,"width":396.0,"height":106.79999}}},"text_order":28,"token_count":229,"internal_refs":[{"text":"","source_page":3,"source_bbox":{"x":431.6,"y":555.9,"width":45.3,"height":10.9},"target":{"kind":"named","name":"cite.yuan2024s","page":15,"point":{"x":83.0,"y":658.0}}},{"text":"","source_page":3,"source_bbox":{"x":480.3,"y":555.9,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yuan2024s","page":15,"point":{"x":83.0,"y":658.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":566.8,"width":50.6,"height":10.9},"target":{"kind":"named","name":"cite.zhang2024darg","page":16,"point":{"x":83.0,"y":101.0}}},{"text":"","source_page":3,"source_bbox":{"x":161.2,"y":566.8,"width":26.9,"height":10.9},"target":{"kind":"named","name":"cite.zhang2024darg","page":16,"point":{"x":83.0,"y":101.0}}},{"text":"","source_page":3,"source_bbox":{"x":465.4,"y":588.6,"width":37.1,"height":10.9},"target":{"kind":"named","name":"cite.kosinski2024evaluating","page":13,"point":{"x":83.0,"y":340.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":599.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.kosinski2024evaluating","page":13,"point":{"x":83.0,"y":340.0}}},{"text":"","source_page":3,"source_bbox":{"x":132.2,"y":599.5,"width":61.6,"height":10.9},"target":{"kind":"named","name":"cite.salminen2024deus","page":14,"point":{"x":83.0,"y":499.0}}},{"text":"","source_page":3,"source_bbox":{"x":196.9,"y":599.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.salminen2024deus","page":14,"point":{"x":83.0,"y":499.0}}},{"text":"","source_page":3,"source_bbox":{"x":222.1,"y":599.5,"width":42.2,"height":10.9},"target":{"kind":"named","name":"cite.shin2024understanding","page":14,"point":{"x":83.0,"y":646.0}}},{"text":"","source_page":3,"source_bbox":{"x":267.4,"y":599.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.shin2024understanding","page":14,"point":{"x":83.0,"y":646.0}}}],"style":null}
```

### 2.4 RAG evaluation criteria
```bgraph-section
{"id":"2250e15b-c882-5788-9a1c-d6a66941925b","node_type":"Section","location":{"semantic":{"path":"1.6.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":659.7,"width":124.5,"height":8.700012}}},"text_order":29,"token_count":6,"style":null}
```

Our evaluation relies on the LLM to evaluate how well the RAG system answers the generated ques- tions. Prior work has shown LLMs to be good evaluators of natural language generation, includ- ing work where LLMs evaluations were competitive with human evaluations ( Wang et al. , 2023a ; Zheng et al. , 2024 ). Some prior work proposes criteria for having LLMs quantify the quality of
```bgraph-paragraph
{"id":"448d6dbf-0207-5833-bffb-01bdfa704994","node_type":"Paragraph","location":{"semantic":{"path":"1.6.4.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":3,"bounding_box":{"x":108.0,"y":680.6,"width":396.0,"height":41.400024}}},"text_order":30,"token_count":94,"internal_refs":[{"text":"","source_page":3,"source_bbox":{"x":424.2,"y":701.3,"width":48.1,"height":10.9},"target":{"kind":"named","name":"cite.wang2023chatgpt","page":15,"point":{"x":83.0,"y":335.0}}},{"text":"","source_page":3,"source_bbox":{"x":475.9,"y":701.3,"width":26.3,"height":10.9},"target":{"kind":"named","name":"cite.wang2023chatgpt","page":15,"point":{"x":83.0,"y":335.0}}},{"text":"","source_page":3,"source_bbox":{"x":107.0,"y":712.2,"width":51.3,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}},{"text":"","source_page":3,"source_bbox":{"x":162.2,"y":712.2,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}}],"style":null}
```

3
```bgraph-paragraph
{"id":"b095e14f-4be8-5be5-a851-1ef0326667f3","node_type":"Paragraph","location":{"semantic":{"path":"1.6.4.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":3,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":31,"token_count":1,"style":null}
```

Source Documents Global Answer text extraction query-focused and chunking summarization Text Chunks Community Answers domain-tailored query-focused summarization summarization Entities & Relationships Community Summaries domain-tailored domain-tailored summarization community summarization detection Knowledge Graph Graph Communities
```bgraph-paragraph
{"id":"8cf42301-ac56-53de-a785-ebceb59f5599","node_type":"Paragraph","location":{"semantic":{"path":"1.6.4.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":4,"bounding_box":{"x":144.8,"y":79.0,"width":316.39996,"height":143.7}}},"text_order":32,"token_count":70,"style":null}
```

Indexing Time Pipeline Stage Query Time
```bgraph-paragraph
{"id":"aea953b3-eef1-5b05-b686-29829f5a5026","node_type":"Paragraph","location":{"semantic":{"path":"1.6.4.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":4,"bounding_box":{"x":165.7,"y":259.4,"width":258.40002,"height":8.700012}}},"text_order":33,"token_count":8,"style":null}
```

Figure 1: Graph RAG pipeline using an LLM-derived graph index of source document text. This graph index spans nodes (e.g., entities), edges (e.g., relationships), and covariates (e.g., claims) that have been detected, extracted, and summarized by LLM prompts tailored to the domain of the dataset. Community detection (e.g., Leiden, Traag et al. , 2019 ) is used to partition the graph index into groups of elements (nodes, edges, covariates) that the LLM can summarize in parallel at both indexing time and query time. The “global answer” to a given query is produced using a final round of query-focused summarization over all community summaries reporting relevance to that query.
```bgraph-paragraph
{"id":"9ca551a4-7b71-56ff-a652-5aaa157cacb9","node_type":"Paragraph","location":{"semantic":{"path":"1.6.4.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":285.3,"width":396.0,"height":74.20001}}},"text_order":34,"token_count":166,"internal_refs":[{"text":"","source_page":4,"source_bbox":{"x":286.6,"y":317.0,"width":47.3,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}},{"text":"","source_page":4,"source_bbox":{"x":337.2,"y":317.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}}],"style":null}
```

generated texts such as “fluency” ( Wang et al. , 2023a ) Some of these criteria are generic to vector RAG systems and not relevant to global sensemaking, such as “context relevance”, “faithfulness”, and “answer relevance” (RAGAS, Es et al. 2023 ). Lacking a gold standard for evaluation, one can quantify relative performance for a given criterion by prompting the LLM to compare generations from two different competing models (LLM-as-a-judge, ( Zheng et al. , 2024 )). In this work, we de- sign criteria for evaluating RAG-generated answers to global sensemaking questions and evaluate our results using the comparative approach. We also validate results using statistics derived from LLM-extracted statements of verifiable facts, or “claims.”
```bgraph-paragraph
{"id":"70018d9a-83dd-5f01-ab83-be09f5ca7170","node_type":"Paragraph","location":{"semantic":{"path":"1.6.4.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":386.4,"width":396.0,"height":85.00003}}},"text_order":35,"token_count":182,"internal_refs":[{"text":"","source_page":4,"source_bbox":{"x":248.9,"y":385.3,"width":47.4,"height":10.9},"target":{"kind":"named","name":"cite.wang2023chatgpt","page":15,"point":{"x":83.0,"y":335.0}}},{"text":"","source_page":4,"source_bbox":{"x":299.6,"y":385.3,"width":26.3,"height":10.9},"target":{"kind":"named","name":"cite.wang2023chatgpt","page":15,"point":{"x":83.0,"y":335.0}}},{"text":"","source_page":4,"source_bbox":{"x":246.5,"y":407.1,"width":34.4,"height":10.9},"target":{"kind":"named","name":"cite.es2023ragas","page":12,"point":{"x":83.0,"y":490.0}}},{"text":"","source_page":4,"source_bbox":{"x":281.7,"y":407.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.es2023ragas","page":12,"point":{"x":83.0,"y":490.0}}},{"text":"","source_page":4,"source_bbox":{"x":337.0,"y":428.9,"width":49.6,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}},{"text":"","source_page":4,"source_bbox":{"x":389.8,"y":428.9,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.zheng2024judging","page":16,"point":{"x":83.0,"y":131.0}}}],"style":null}
```

## 3 Methods
```bgraph-section
{"id":"9adb920f-65ff-59b8-95e9-844f29273c22","node_type":"Section","location":{"semantic":{"path":"1.7","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":493.5,"width":62.4,"height":10.6}}},"text_order":36,"token_count":2,"style":null}
```

### 3.1 GraphRAG Workflow
```bgraph-section
{"id":"e165ca8c-d779-598f-835f-7e82d1ca6c34","node_type":"Section","location":{"semantic":{"path":"1.7.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":520.5,"width":116.5,"height":8.700012}}},"text_order":37,"token_count":5,"style":null}
```

Figure 1 illustrates the high-level data flow of the GraphRAG approach and pipeline. In this section, we describe the key design parameters, techniques, and implementation details for each step.
```bgraph-paragraph
{"id":"77619dd0-d9b4-517b-83ac-da8660c530d4","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":542.2,"width":396.0,"height":19.599976}}},"text_order":38,"token_count":48,"internal_refs":[{"text":"","source_page":4,"source_bbox":{"x":107.0,"y":541.1,"width":35.3,"height":10.9},"target":{"kind":"named","name":"figure.1","page":3,"point":{"x":148.0,"y":282.0}}}],"style":null}
```

#### 3.1.1 Source Documents → Text Chunks
```bgraph-section
{"id":"18e299e0-e98e-59d8-aec7-f0aa0e0689f0","node_type":"Section","location":{"semantic":{"path":"1.7.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.1 Source Documents → Text Chunks"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":579.5,"width":178.30002,"height":9.299988}}},"text_order":39,"token_count":8,"style":null}
```

To start, the documents in the corpus are split into text chunks. The LLM extracts information from each chunk for downstream processing. Selecting the size of the chunk is a fundamental design decision; longer text chunks require fewer LLM calls for such extraction (which reduces cost) but suffer from degraded recall of information that appears early in the chunk ( Kuratov et al. , 2024 ; Liu et al. , 2023 ). See Section A.1 for prompts and examples of the recall-precision trade-offs.
```bgraph-paragraph
{"id":"9b099ca9-321d-59d3-9e9a-1171e0ae6ab6","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.2.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.1 Source Documents → Text Chunks"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":600.5,"width":396.0,"height":52.299988}}},"text_order":40,"token_count":118,"internal_refs":[{"text":"","source_page":4,"source_bbox":{"x":405.1,"y":632.1,"width":56.1,"height":10.9},"target":{"kind":"named","name":"cite.kuratov2024search","page":13,"point":{"x":83.0,"y":371.0}}},{"text":"","source_page":4,"source_bbox":{"x":464.1,"y":632.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.kuratov2024search","page":13,"point":{"x":83.0,"y":371.0}}},{"text":"","source_page":4,"source_bbox":{"x":489.2,"y":632.1,"width":15.8,"height":10.9},"target":{"kind":"named","name":"cite.liu-etal:2023:tacl","page":13,"point":{"x":83.0,"y":515.0}}},{"text":"","source_page":4,"source_bbox":{"x":107.0,"y":643.0,"width":21.4,"height":10.9},"target":{"kind":"named","name":"cite.liu-etal:2023:tacl","page":13,"point":{"x":83.0,"y":515.0}}},{"text":"","source_page":4,"source_bbox":{"x":131.4,"y":643.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.liu-etal:2023:tacl","page":13,"point":{"x":83.0,"y":515.0}}},{"text":"","source_page":4,"source_bbox":{"x":177.1,"y":643.0,"width":49.0,"height":10.9},"target":{"kind":"named","name":"subsection.A.1","page":17,"point":{"x":108.0,"y":162.0}}}],"style":null}
```

#### 3.1.2 Text Chunks → Entities & Relationships
```bgraph-section
{"id":"223c1a18-ed70-5b94-bae4-d4801c80e746","node_type":"Section","location":{"semantic":{"path":"1.7.1.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":670.5,"width":202.5,"height":9.299988}}},"text_order":41,"token_count":10,"style":null}
```

In this step, the LLM is prompted to extract instances of important entities and the relationships between the entities from a given chunk. Additionally, the LLM generates short descriptions for the entities and relationships. To illustrate, suppose a chunk contained the following text:
```bgraph-paragraph
{"id":"c1498b48-fd20-59f8-855f-ccd4346f7bdb","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":4,"bounding_box":{"x":108.0,"y":691.5,"width":396.0,"height":30.5}}},"text_order":42,"token_count":68,"style":null}
```

4
```bgraph-paragraph
{"id":"5c1341b0-58ab-5b92-a023-474ecf85d442","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.2","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":4,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":43,"token_count":1,"style":null}
```

NeoChip’s (NC) shares surged in their first week of trading on the NewTech Ex- change. However, market analysts caution that the chipmaker’s public debut may not reflect trends for other technology IPOs. NeoChip, previously a private entity, was acquired by Quantum Systems in 2016. The innovative semiconductor firm specializes in low-power processors for wearables and IoT devices.
```bgraph-paragraph
{"id":"eb518e64-0e2c-51ec-9b1e-b74f9a454a54","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.3","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":143.9,"y":75.3,"width":324.3,"height":52.299995}}},"text_order":44,"token_count":94,"style":null}
```

The LLM is prompted such that it extracts the following:
```bgraph-paragraph
{"id":"75cab76c-d1af-5898-8bd9-daa1cded37c6","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.4","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":108.0,"y":141.4,"width":226.8,"height":8.7}}},"text_order":45,"token_count":14,"style":null}
```

• The entity NeoChip , with description “NeoChip is a publicly traded company specializing in low-power processors for wearables and IoT devices.” • The entity Quantum Systems , with description “Quantum Systems is a firm that previ- ously owned NeoChip.” • A relationship between NeoChip and Quantum Systems , with description “Quantum Sys- tems owned NeoChip from 2016 until NeoChip became publicly traded.”
```bgraph-paragraph
{"id":"9d08bc06-cd3d-5db6-81a8-a794935d526e","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.5","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":135.4,"y":162.6,"width":368.6,"height":74.399994}}},"text_order":46,"token_count":96,"style":null}
```

These prompts can be tailored to the domain of the document corpus by choosing domain appropriate few-shot exemplars for in-context learning ( Brown et al. , 2020 ). For example, while our default prompt extracts the broad class of “named entities” like people, places, and organizations and is generally applicable, domains with specialized knowledge (e.g., science, medicine, law) will benefit from few-shot exemplars specialized to those domains. The LLM can also be prompted to extract claims about detected entities. Claims are important factual statements about entities, such as dates, events, and interactions with other entities. As with entities and relationships, in-context learning exemplars can provide domain-specific guidance. Claim descriptions extracted from the example tetx chunk are as follows:
```bgraph-paragraph
{"id":"235bc7e1-b728-5c27-a7d7-3ee1a05cce41","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.6","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":108.0,"y":249.5,"width":396.0,"height":101.400024}}},"text_order":47,"token_count":198,"internal_refs":[{"text":"","source_page":5,"source_bbox":{"x":287.5,"y":259.3,"width":52.7,"height":10.9},"target":{"kind":"named","name":"cite.brown2020language","page":12,"point":{"x":83.0,"y":292.0}}},{"text":"","source_page":5,"source_bbox":{"x":344.2,"y":259.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.brown2020language","page":12,"point":{"x":83.0,"y":292.0}}}],"style":null}
```

• NeoChip’s shares surged during their first week of trading on the NewTech Exchange. • NeoChip debuted as a publicly listed company on the NewTech Exchange. • Quantum Systems acquired NeoChip in 2016 and held ownership until NeoChip went pub- lic.
```bgraph-paragraph
{"id":"05015720-d089-5f0c-af8f-e611cfbc02f8","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.7","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":135.4,"y":363.4,"width":368.6,"height":52.50003}}},"text_order":48,"token_count":62,"style":null}
```

See Appendix A for prompts and details on our implementation of entity and claim extraction.
```bgraph-paragraph
{"id":"1d62a219-b82a-50ec-a165-085295ae0cbe","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.3.8","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":108.0,"y":428.4,"width":375.90002,"height":8.700012}}},"text_order":49,"token_count":22,"internal_refs":[{"text":"","source_page":5,"source_bbox":{"x":123.9,"y":427.3,"width":51.0,"height":10.9},"target":{"kind":"named","name":"appendix.A","page":17,"point":{"x":108.0,"y":72.0}}}],"style":null}
```

#### 3.1.3 Entities & Relationships → Knowledge Graph
```bgraph-section
{"id":"e0e2c882-a7e6-5539-a07d-03f8684532a8","node_type":"Section","location":{"semantic":{"path":"1.7.1.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.3 Entities & Relationships → Knowledge Graph"]},"physical":{"page":5,"bounding_box":{"x":108.0,"y":452.6,"width":226.5,"height":9.300018}}},"text_order":50,"token_count":11,"style":null}
```

The use of an LLM to extract entities, relationships, and claims is a form of abstractive summariza- tion – these are meaningful summaries of concepts that, in the case of relationships and claims, may not be explicitly stated in the text. The entity/relationship/claim extraction processes creates mul- tiple instances of a single element because an element is typically detected and extracted multiple times across documents. In the final step of the knowledge graph extraction process, these instances of entities and rela- tionships become individual nodes and edges in the graph. Entity descriptions are aggregated and summarized for each node and edge. Relationships are aggregated into graph edges, where the num- ber of duplicates for a given relationship becomes edge weights. Claims are aggregated similarly. In this manuscript, our analysis uses exact string matching for entity matching – the task of reconcil- ing different extracted names for the same entity ( Barlaug and Gulla , 2021 ; Christen and Christen , 2012 ; Elmagarmid et al. , 2006 ). However, softer matching approaches can be used with minor ad- justments to prompts or code. Furthermore, GraphRAG is generally resilient to duplicate entities since duplicates are typically clustered together for summarization in subsequent steps.
```bgraph-paragraph
{"id":"b8a301dc-91b7-5817-bbd9-032ff611ecb4","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.4.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.3 Entities & Relationships → Knowledge Graph"]},"physical":{"page":5,"bounding_box":{"x":108.0,"y":472.6,"width":396.1,"height":161.50003}}},"text_order":51,"token_count":319,"internal_refs":[{"text":"","source_page":5,"source_bbox":{"x":308.4,"y":591.6,"width":75.7,"height":10.9},"target":{"kind":"named","name":"cite.barlaug2021neural","page":12,"point":{"x":83.0,"y":176.0}}},{"text":"","source_page":5,"source_bbox":{"x":387.4,"y":591.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.barlaug2021neural","page":12,"point":{"x":83.0,"y":176.0}}},{"text":"","source_page":5,"source_bbox":{"x":413.0,"y":591.6,"width":89.5,"height":10.9},"target":{"kind":"named","name":"cite.christen2012data","page":12,"point":{"x":83.0,"y":376.0}}},{"text":"","source_page":5,"source_bbox":{"x":107.0,"y":602.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.christen2012data","page":12,"point":{"x":83.0,"y":376.0}}},{"text":"","source_page":5,"source_bbox":{"x":132.5,"y":602.5,"width":73.7,"height":10.9},"target":{"kind":"named","name":"cite.elmagarmid2006duplicate","page":12,"point":{"x":83.0,"y":458.0}}},{"text":"","source_page":5,"source_bbox":{"x":209.5,"y":602.5,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.elmagarmid2006duplicate","page":12,"point":{"x":83.0,"y":458.0}}}],"style":null}
```

#### 3.1.4 Knowledge Graph → Graph Communities
```bgraph-section
{"id":"45256da9-010e-57b9-ab51-6ab27742b3dc","node_type":"Section","location":{"semantic":{"path":"1.7.1.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":5,"bounding_box":{"x":108.0,"y":649.6,"width":210.79999,"height":9.300049}}},"text_order":52,"token_count":10,"style":null}
```

Given the graph index created in the previous step, a variety of community detection algorithms may be used to partition the graph into communities of strongly connected nodes (e.g., see the surveys by Fortunato ( 2010 ) and Jin et al. ( 2021 )). In our pipeline, we use Leiden community detection ( Traag et al. , 2019 ) in a hierarchical manner, recursively detecting sub-communities within each detected community until reaching leaf communities that can no longer be partitioned.
```bgraph-paragraph
{"id":"3a7ddf27-df5a-505b-ad42-d0e140337d00","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.5.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":5,"bounding_box":{"x":108.0,"y":669.7,"width":396.0,"height":52.299988}}},"text_order":53,"token_count":115,"internal_refs":[{"text":"","source_page":5,"source_bbox":{"x":154.8,"y":690.4,"width":40.6,"height":10.9},"target":{"kind":"named","name":"cite.fortunato2010community","page":12,"point":{"x":83.0,"y":605.0}}},{"text":"","source_page":5,"source_bbox":{"x":200.6,"y":690.4,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.fortunato2010community","page":12,"point":{"x":83.0,"y":605.0}}},{"text":"","source_page":5,"source_bbox":{"x":245.9,"y":690.4,"width":38.2,"height":10.9},"target":{"kind":"named","name":"cite.jin2021survey","page":13,"point":{"x":83.0,"y":113.0}}},{"text":"","source_page":5,"source_bbox":{"x":289.2,"y":690.4,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.jin2021survey","page":13,"point":{"x":83.0,"y":113.0}}},{"text":"","source_page":5,"source_bbox":{"x":148.9,"y":701.3,"width":45.9,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}},{"text":"","source_page":5,"source_bbox":{"x":197.4,"y":701.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}}],"style":null}
```

5
```bgraph-paragraph
{"id":"46dcd2fe-05fc-5f19-96e6-e408b9a3e32b","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.5.2","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":5,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":54,"token_count":1,"style":null}
```

Each level of this hierarchy provides a community partition that covers the nodes of the graph in a mutually exclusive, collectively exhaustive way, enabling divide-and-conquer global summarization. An illustration of such hierarchical partitioning on an example dataset can be found in Appendix B .
```bgraph-paragraph
{"id":"6829ddbd-d422-56c7-8086-386784e21848","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.5.3","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":6,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":30.499992}}},"text_order":55,"token_count":72,"internal_refs":[{"text":"","source_page":6,"source_bbox":{"x":452.1,"y":96.0,"width":50.4,"height":10.9},"target":{"kind":"named","name":"appendix.B","page":17,"point":{"x":108.0,"y":691.0}}}],"style":null}
```

#### 3.1.5 Graph Communities Community Summaries
```bgraph-section
{"id":"761f11f0-f225-5d8f-8a18-f2ed03edf14f","node_type":"Section","location":{"semantic":{"path":"1.7.1.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":108.0,"y":121.1,"width":234.20001,"height":8.700005}}},"text_order":56,"token_count":9,"style":null}
```

→
```bgraph-paragraph
{"id":"49412c9d-6711-520e-85ac-4f6fbf2f50f4","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.6.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":228.1,"y":120.5,"width":10.0,"height":9.2}}},"text_order":57,"token_count":1,"style":null}
```

The next step creates report-like summaries of each community in the community hierarchy, using a method designed to scale to very large datasets. These summaries are independently useful as a way to understand the global structure and semantics of the dataset, and may themselves be used to make sense of a corpus in the absence of a specific query. For example, a user may scan through community summaries at one level looking for general themes of interest, then read linked reports at a lower level that provide additional details for each subtopic. Here, however, we focus on their utility as part of a graph-based index used for answering global queries. GraphRAG generates community summaries by adding various element summaries (for nodes, edges, and related claims) to a community summary template. Community summaries from lower- level communities are used to generate summaries for higher-level communities as follows:
```bgraph-paragraph
{"id":"aca733c9-b2e1-59c3-9a1e-7c538f103f6d","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.6.2","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":108.0,"y":140.2,"width":396.0,"height":112.399994}}},"text_order":58,"token_count":226,"style":null}
```

• Leaf-level communities . The element summaries of a leaf-level community are prioritized and then iteratively added to the LLM context window until the token limit is reached. The prioritization is as follows: for each community edge in decreasing order of combined source and target node degree (i.e., overall prominence), add descriptions of the source node, target node, the edge itself, and related claims. • Higher-level communities . If all element summaries fit within the token limit of the con- text window, proceed as for leaf-level communities and summarize all element summaries within the community. Otherwise, rank sub-communities in decreasing order of element summary tokens and iteratively substitute sub-community summaries (shorter) for their associated element summaries (longer) until they fit within the context window.
```bgraph-paragraph
{"id":"16fe5382-6960-5d6a-a598-63ffac92d188","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.6.3","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":135.4,"y":265.9,"width":368.6,"height":111.70001}}},"text_order":59,"token_count":205,"style":null}
```

#### 3.1.6 Community Summaries Community Answers Global Answer
```bgraph-section
{"id":"6187f243-71d9-53e2-967a-3c4cacaae4d2","node_type":"Section","location":{"semantic":{"path":"1.7.1.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries Community Answers Global Answer"]},"physical":{"page":6,"bounding_box":{"x":108.0,"y":392.9,"width":315.19998,"height":8.700012}}},"text_order":60,"token_count":13,"style":null}
```

→ →
```bgraph-paragraph
{"id":"49d165e6-ce4f-52b4-8d2e-a3d2ccb74ace","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.7.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries Community Answers Global Answer"]},"physical":{"page":6,"bounding_box":{"x":241.9,"y":392.3,"width":114.899994,"height":9.200012}}},"text_order":61,"token_count":2,"style":null}
```

Given a user query, the community summaries generated in the previous step can be used to generate a final answer in a multi-stage process. The hierarchical nature of the community structure also means that questions can be answered using the community summaries from different levels, raising the question of whether a particular level in the hierarchical community structure offers the best balance of summary detail and scope for general sensemaking questions (evaluated in section 4 ). For a given community level, the global answer to any user query is generated as follows:
```bgraph-paragraph
{"id":"dd0a5009-1b66-5ac2-a3aa-efd08648067c","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.7.2","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries Community Answers Global Answer"]},"physical":{"page":6,"bounding_box":{"x":108.0,"y":412.1,"width":396.0,"height":68.70001}}},"text_order":62,"token_count":141,"internal_refs":[{"text":"","source_page":6,"source_bbox":{"x":450.7,"y":454.6,"width":37.7,"height":10.9},"target":{"kind":"named","name":"section.4","page":7,"point":{"x":108.0,"y":222.0}}}],"style":null}
```

• Prepare community summaries . Community summaries are randomly shuffled and divided into chunks of pre-specified token size. This ensures relevant information is distributed across chunks, rather than concentrated (and potentially lost) in a single context window. • Map community answers . Intermediate answers are generated in parallel. The LLM is also asked to generate a score between 0-100 indicating how helpful the generated answer is in answering the target question. Answers with score 0 are filtered out. • Reduce to global answer . Intermediate community answers are sorted in descending order of helpfulness score and iteratively added into a new context window until the token limit is reached. This final context is used to generate the global answer returned to the user.
```bgraph-paragraph
{"id":"03336acf-d376-53f2-b0cd-a7a7a2c1f64d","node_type":"Paragraph","location":{"semantic":{"path":"1.7.1.7.3","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries Community Answers Global Answer"]},"physical":{"page":6,"bounding_box":{"x":135.4,"y":494.1,"width":368.6,"height":105.69998}}},"text_order":63,"token_count":190,"style":null}
```

### 3.2 Global Sensemaking Question Generation
```bgraph-section
{"id":"b0b4dc04-1a99-5bd0-bdb9-d95bba0ad2a2","node_type":"Section","location":{"semantic":{"path":"1.7.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.2 Global Sensemaking Question Generation"]},"physical":{"page":6,"bounding_box":{"x":108.0,"y":616.4,"width":200.9,"height":8.700012}}},"text_order":64,"token_count":10,"style":null}
```

To evaluate the effectiveness of RAG systems for global sensemaking tasks, we use an LLM to generate a set of corpus-specific questions designed to asses high-level understanding of a given corpus, without requiring retrieval of specific low-level facts. Instead, given a high-level description of a corpus and its purposes, the LLM is prompted to generate personas of hypothetical users of the RAG system. For each hypothetical user, the LLM is then prompted to specify tasks that this user would use the RAG system to complete. Finally, for each combination of user and task, the LLM is prompted to generate questions that require understanding of the entire corpus. Algorithm 1 describes the approach.
```bgraph-paragraph
{"id":"1d54ff36-ea11-5402-9389-4035219642be","node_type":"Paragraph","location":{"semantic":{"path":"1.7.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.2 Global Sensemaking Question Generation"]},"physical":{"page":6,"bounding_box":{"x":108.0,"y":636.9,"width":396.0,"height":85.099976}}},"text_order":65,"token_count":171,"style":null}
```

6
```bgraph-paragraph
{"id":"6888f1be-9a8b-5fbb-a57d-a662a81d05ef","node_type":"Paragraph","location":{"semantic":{"path":"1.7.2.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.2 Global Sensemaking Question Generation"]},"physical":{"page":6,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":66,"token_count":1,"style":null}
```

### Algorithm 1: Prompting Procedure for Question Generation
```bgraph-section
{"id":"829a33a8-a30f-5166-a97c-45cd46d6e2a8","node_type":"Section","location":{"semantic":{"path":"1.7.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":108.0,"y":86.5,"width":256.6,"height":8.7}}},"text_order":67,"token_count":14,"style":null}
```

1: Input: Description of a corpus, number of users K , number of tasks per user N , number of questions per (user, task) combination M . 2: Output: A set of K ∗ N ∗ M high-level questions requiring global understanding of the corpus. 3: procedure G ENERATE Q UESTIONS 4: Based on the corpus description, prompt the LLM to: 1. Describe personas of K potential users of the dataset. 2. For each user, identify N tasks relevant to the user. 3. Specific to each user & task pair, generate M high-level questions that: • Require understanding of the entire corpus. • Do not require retrieval of specific low-level facts. 5: Collect the generated questions to produce K ∗ N ∗ M test questions for the dataset. 6: end procedure
```bgraph-paragraph
{"id":"4c89a46d-b3a3-54b6-96a2-5655e188f312","node_type":"Paragraph","location":{"semantic":{"path":"1.7.3.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":113.0,"y":105.8,"width":391.0,"height":145.0}}},"text_order":68,"token_count":175,"style":null}
```

For our evaluation, we set K = M = N = 5 for a total of 125 test questions per dataset. Table 1 shows example questions for each of the two evaluation datasets.
```bgraph-paragraph
{"id":"a60e3a7f-6f5d-523b-978a-11e89bc96587","node_type":"Paragraph","location":{"semantic":{"path":"1.7.3.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":108.0,"y":276.8,"width":396.0,"height":19.700012}}},"text_order":69,"token_count":40,"internal_refs":[{"text":"","source_page":7,"source_bbox":{"x":473.3,"y":275.8,"width":31.7,"height":11.0},"target":{"kind":"named","name":"table.1","page":6,"point":{"x":145.0,"y":510.0}}}],"style":null}
```

### 3.3 Criteria for Evaluating Global Sensemaking
```bgraph-section
{"id":"780b7470-098f-5517-b743-610be26a2b38","node_type":"Section","location":{"semantic":{"path":"1.7.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":108.0,"y":314.3,"width":210.20001,"height":8.700012}}},"text_order":70,"token_count":11,"style":null}
```

Given the lack of gold standard answers to our activity-based sensemaking questions, we adopt the head-to-head comparison approach using an LLM evaluator that judges relative performance according to specific criteria. We designed three target criteria capturing qualities that are desirable for global sensemaking activities. Appendix F shows the prompts for our head-to-head measures computed using an LLM evaluator, summarized as:
```bgraph-paragraph
{"id":"8453fea9-c8d9-5a2b-8829-3b05f85a69df","node_type":"Paragraph","location":{"semantic":{"path":"1.7.4.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":108.0,"y":335.3,"width":396.0,"height":68.70001}}},"text_order":71,"token_count":105,"internal_refs":[{"text":"","source_page":7,"source_bbox":{"x":107.0,"y":383.3,"width":49.7,"height":10.9},"target":{"kind":"named","name":"appendix.F","page":24,"point":{"x":108.0,"y":72.0}}}],"style":null}
```

• Comprehensiveness . How much detail does the answer provide to cover all aspects and details of the question? • Diversity . How varied and rich is the answer in providing different perspectives and insights on the question? • Empowerment . How well does the answer help the reader understand and make informed judgments about the topic?
```bgraph-paragraph
{"id":"60fb6226-d580-53b0-a39b-2303a3c00f02","node_type":"Paragraph","location":{"semantic":{"path":"1.7.4.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":135.4,"y":416.7,"width":368.6,"height":75.0}}},"text_order":72,"token_count":81,"style":null}
```

Table 1: Examples of potential users, tasks, and questions generated by the LLM based on short descriptions of the target datasets. Questions target global understanding rather than specific details.
```bgraph-paragraph
{"id":"4408b5aa-adea-5e7b-93e6-9fb6d4a9946f","node_type":"Paragraph","location":{"semantic":{"path":"1.7.4.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":108.0,"y":513.8,"width":396.0,"height":19.600037}}},"text_order":73,"token_count":49,"style":null}
```

Dataset Example activity framing and generation of global sensemaking questions Podcast User : A tech journalist looking for insights and trends in the tech industry transcripts Task : Understanding how tech leaders view the role of policy and regulation Questions : 1. Which episodes deal primarily with tech policy and government regulation? 2. How do guests perceive the impact of privacy laws on technology development? 3. Do any guests discuss the balance between innovation and ethical considerations? 4. What are the suggested changes to current policies mentioned by the guests? 5. Are collaborations between tech companies and governments discussed and how? News User : Educator incorporating current affairs into curricula articles Task : Teaching about health and wellness Questions : 1. What current topics in health can be integrated into health education curricula? 2. How do news articles address the concepts of preventive medicine and wellness? 3. Are there examples of health articles that contradict each other, and if so, why? 4. What insights can be gleaned about public health priorities based on news coverage? 5. How can educators use the dataset to highlight the importance of health literacy?
```bgraph-paragraph
{"id":"25d3b0f3-ad01-5916-93e3-9288f6e2b1db","node_type":"Paragraph","location":{"semantic":{"path":"1.7.4.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":114.0,"y":544.6,"width":372.3,"height":173.00006}}},"text_order":74,"token_count":290,"style":null}
```

7
```bgraph-paragraph
{"id":"d749af5b-c28e-55a3-befc-66104f70d490","node_type":"Paragraph","location":{"semantic":{"path":"1.7.4.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":75,"token_count":1,"style":null}
```

Furthermore, we use a “control criterion” called Directness that answers “How specifically and clearly does the answer address the question?” . In plain terms, directness evaluates the concision of an answer in a generic sense that applies to any generated LLM summarization. We include it to behave as a reference against which we can judge the soundness of results for the other criteria. Since directness is effectively in opposition to comprehensiveness and diversity, we would not expect any method to win across all four criteria. In our evaluations, the LLM is provided with the question, the generated answers from two compet- ing systems, and prompted to compare the two answers according to the criterion before giving a final judgment of which answer is preferred. The LLM either indicates a winner; or, it returns a tie if they are fundamentally similar. To account for the inherent stochasticity of LLM generation, we run each comparison with multiple replicates and average the results across replicates and questions. An illustration of LLM assessment for answers to a sample question can be found in Appendix D .
```bgraph-paragraph
{"id":"53a35cb4-8969-52d0-9b89-32131b00e721","node_type":"Paragraph","location":{"semantic":{"path":"1.7.4.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":134.09999}}},"text_order":76,"token_count":274,"internal_refs":[{"text":"","source_page":8,"source_bbox":{"x":445.4,"y":199.7,"width":51.0,"height":10.9},"target":{"kind":"named","name":"appendix.D","page":19,"point":{"x":108.0,"y":72.0}}}],"style":null}
```

## 4 Analysis
```bgraph-section
{"id":"d2a4c482-d74b-5d9e-9195-38faa70d60b8","node_type":"Section","location":{"semantic":{"path":"1.8","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":230.5,"width":61.1,"height":10.6}}},"text_order":77,"token_count":2,"style":null}
```

### 4.1 Experiment 1
```bgraph-section
{"id":"76b7ea82-56ba-5266-aaa1-16daf6773ba2","node_type":"Section","location":{"semantic":{"path":"1.8.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":256.9,"width":80.2,"height":8.700012}}},"text_order":78,"token_count":4,"style":null}
```

#### 4.1.1 Datasets
```bgraph-section
{"id":"3f3476f0-2030-563c-a49b-d8e89f59aa2a","node_type":"Section","location":{"semantic":{"path":"1.8.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.1 Datasets"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":278.1,"width":65.9,"height":8.7}}},"text_order":79,"token_count":3,"style":null}
```

We selected two datasets in the one million token range, each representative of corpora that users may encounter in their real-world activities: Podcast transcripts . Public transcripts of Behind the Tech with Kevin Scott , a podcast featuring conversations between Microsoft CTO Kevin Scott and various thought leaders in science and tech- nology ( Scott , 2024 ). This corpus was divided into 1669 × 600-token text chunks, with 100-token overlaps between chunks ( ∼ 1 million tokens). News articles . A benchmark dataset comprised of news articles published from September 2013 to December 2023 in a range of categories, including entertainment, business, sports, technology, health, and science ( Tang and Yang , 2024 ). The corpus is divided into 3197 × 600-token text chunks, with 100-token overlaps between chunks ( ∼ 1.7 million tokens).
```bgraph-paragraph
{"id":"856032c7-a2a3-586a-b675-bbe69f8f8727","node_type":"Paragraph","location":{"semantic":{"path":"1.8.1.1.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.1 Datasets"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":298.1,"width":396.0,"height":117.80002}}},"text_order":80,"token_count":202,"internal_refs":[{"text":"","source_page":8,"source_bbox":{"x":143.2,"y":346.1,"width":22.5,"height":10.9},"target":{"kind":"named","name":"cite.BehindTheTech","page":14,"point":{"x":83.0,"y":584.0}}},{"text":"","source_page":8,"source_bbox":{"x":168.8,"y":346.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.BehindTheTech","page":14,"point":{"x":83.0,"y":584.0}}},{"text":"","source_page":8,"source_bbox":{"x":196.0,"y":395.2,"width":64.1,"height":10.9},"target":{"kind":"named","name":"cite.tang2024multihop","page":15,"point":{"x":83.0,"y":164.0}}},{"text":"","source_page":8,"source_bbox":{"x":264.3,"y":395.2,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.tang2024multihop","page":15,"point":{"x":83.0,"y":164.0}}}],"style":null}
```

#### 4.1.2 Conditions
```bgraph-section
{"id":"85efa101-535d-51be-bc2c-0f3461f2e316","node_type":"Section","location":{"semantic":{"path":"1.8.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":433.2,"width":76.4,"height":8.7}}},"text_order":81,"token_count":4,"style":null}
```

We compared six conditions including GraphRAG at four different graph community levels ( C0 , C1 , C2 , C3 ), a text summarization method that applies our map-reduce approach directly to source texts ( TS ), and a vector RAG “semantic search” approach ( SS ):
```bgraph-paragraph
{"id":"e1966e53-0926-5fbc-9d56-3eb3ea222461","node_type":"Paragraph","location":{"semantic":{"path":"1.8.1.2.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":453.1,"width":396.0,"height":30.600006}}},"text_order":82,"token_count":67,"style":null}
```

• CO . Uses root-level community summaries (fewest in number) to answer user queries. • C1 . Uses high-level community summaries to answer queries. These are sub-communities of C0, if present, otherwise C0 communities projected downwards. • C2 . Uses intermediate-level community summaries to answer queries. These are sub- communities of C1, if present, otherwise C1 communities projected downwards. • C3 . Uses low-level community summaries (greatest in number) to answer queries. These are sub-communities of C2, if present, otherwise C2 communities projected downwards. • TS . The same method as in Section 3.1.6 , except source texts (rather than community summaries) are shuffled and chunked for the map-reduce summarization stages. • SS . An implementation of vector RAG in which text chunks are retrieved and added to the available context window until the specified token limit is reached.
```bgraph-paragraph
{"id":"862c8e26-4acf-589f-87f3-b15835eb932e","node_type":"Paragraph","location":{"semantic":{"path":"1.8.1.2.2","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":135.4,"y":498.3,"width":368.6,"height":151.20001}}},"text_order":83,"token_count":223,"internal_refs":[{"text":"","source_page":8,"source_bbox":{"x":263.1,"y":600.3,"width":55.7,"height":11.0},"target":{"kind":"named","name":"subsubsection.3.1.6","page":5,"point":{"x":108.0,"y":386.0}}}],"style":null}
```

The size of the context window and the prompts used for answer generation are the same across all six conditions (except for minor modifications to reference styles to match the types of context information used). Conditions only differ in how the contents of the context window are created. The graph index supporting conditions C0 - C3 was created using our generic prompts for entity and relationship extraction, with entity types and few-shot examples tailored to the domain of the data.
```bgraph-paragraph
{"id":"cdde1c92-83ae-5c6f-9d54-73aaaee8ba58","node_type":"Paragraph","location":{"semantic":{"path":"1.8.1.2.3","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":108.0,"y":664.2,"width":396.0,"height":57.799988}}},"text_order":84,"token_count":122,"style":null}
```

8
```bgraph-paragraph
{"id":"9fc92cee-3837-59c4-8288-11e8dc6b6cc2","node_type":"Paragraph","location":{"semantic":{"path":"1.8.1.2.4","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":85,"token_count":1,"style":null}
```

#### 4.1.3 Configuration
```bgraph-section
{"id":"56da8542-0a53-5d61-8987-b830e05a5ff3","node_type":"Section","location":{"semantic":{"path":"1.8.1.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.3 Configuration"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":75.1,"width":89.7,"height":8.7}}},"text_order":86,"token_count":4,"style":null}
```

We used a fixed context window size of 8k tokens for generating community summaries, community answers, and global answers (explained in Appendix C ). Graph indexing with a 600 token window (explained in Section A.2 ) took 281 minutes for the Podcast dataset, running on a virtual machine (16GB RAM, Intel(R) Xeon(R) Platinum 8171M CPU @ 2.60GHz) and using a public OpenAI endpoint for gpt-4-turbo (2M TPM, 10k RPM). We implemented Leiden community detection using the graspologic library ( Chung et al. , 2019 ). The prompts used to generate the graph index and global answers can be found in Appendix E , while the prompts used to evaluate LLM responses against our criteria can be found in Appendix F . A full statistical analysis of the results presented in the next section can be found in Appendix G .
```bgraph-paragraph
{"id":"f868ded4-b5ae-5dcc-a933-40c855104e93","node_type":"Paragraph","location":{"semantic":{"path":"1.8.1.3.1","depth":5,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.3 Configuration"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":94.3,"width":396.0,"height":101.5}}},"text_order":87,"token_count":192,"internal_refs":[{"text":"","source_page":9,"source_bbox":{"x":278.6,"y":104.1,"width":50.6,"height":10.9},"target":{"kind":"named","name":"appendix.C","page":18,"point":{"x":108.0,"y":385.0}}},{"text":"","source_page":9,"source_bbox":{"x":162.5,"y":115.0,"width":49.5,"height":10.9},"target":{"kind":"named","name":"subsection.A.2","page":17,"point":{"x":108.0,"y":239.0}}},{"text":"","source_page":9,"source_bbox":{"x":421.4,"y":153.3,"width":52.0,"height":10.9},"target":{"kind":"named","name":"cite.chung2019graspy","page":12,"point":{"x":83.0,"y":396.0}}},{"text":"","source_page":9,"source_bbox":{"x":477.3,"y":153.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.chung2019graspy","page":12,"point":{"x":83.0,"y":396.0}}},{"text":"","source_page":9,"source_bbox":{"x":451.5,"y":164.2,"width":51.0,"height":10.9},"target":{"kind":"named","name":"appendix.E","page":20,"point":{"x":108.0,"y":72.0}}},{"text":"","source_page":9,"source_bbox":{"x":453.2,"y":175.1,"width":49.3,"height":10.9},"target":{"kind":"named","name":"appendix.F","page":24,"point":{"x":108.0,"y":72.0}}},{"text":"","source_page":9,"source_bbox":{"x":441.7,"y":186.0,"width":51.0,"height":10.9},"target":{"kind":"named","name":"appendix.G","page":25,"point":{"x":108.0,"y":72.0}}}],"style":null}
```

### 4.2 Experiment 2
```bgraph-section
{"id":"8f663247-b310-509b-884c-fbce6bae7eeb","node_type":"Section","location":{"semantic":{"path":"1.8.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":212.5,"width":80.2,"height":8.699997}}},"text_order":88,"token_count":4,"style":null}
```

To validate the comprehensiveness and diversity results from Experiment 1, we implemented claim- based measures of these qualities. We use the definition of a factual claim from Ni et al. ( 2024 ), which is “a statement that explicitly presents some verifiable facts.” For example, the sentence “California and New York implemented incentives for renewable energy adoption, highlighting the broader importance of sustainability in policy decisions” contains two factual claims: (1) California implemented incentives for renewable energy adoption, and (2) New York implemented incentives for renewable energy adoption. To extract factual claims, we used Claimify ( Metropolitansky and Larson , 2025 ), an LLM-based method that identifies sentences in an answer containing at least one factual claim, then decomposes these sentences into simple, self-contained factual claims. We applied Claimify to the answers generated under the conditions from Experiment 1. After removing duplicate claims from each answer, we extracted 47,075 unique claims, with an average of 31 claims per answer. We defined two metrics, with higher values indicating better performance:
```bgraph-paragraph
{"id":"dde61751-6e0e-5766-a13a-e096bb5622f9","node_type":"Paragraph","location":{"semantic":{"path":"1.8.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":233.0,"width":396.09998,"height":150.5}}},"text_order":89,"token_count":283,"internal_refs":[{"text":"","source_page":9,"source_bbox":{"x":437.4,"y":242.8,"width":35.3,"height":10.9},"target":{"kind":"named","name":"cite.afacta","page":14,"point":{"x":83.0,"y":258.0}}},{"text":"","source_page":9,"source_bbox":{"x":477.3,"y":242.8,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.afacta","page":14,"point":{"x":83.0,"y":258.0}}},{"text":"","source_page":9,"source_bbox":{"x":293.6,"y":313.7,"width":116.4,"height":11.0},"target":{"kind":"named","name":"cite.claimify","page":14,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":9,"source_bbox":{"x":413.9,"y":313.7,"width":21.9,"height":11.0},"target":{"kind":"named","name":"cite.claimify","page":14,"point":{"x":83.0,"y":72.0}}}],"style":null}
```

1. Comprehensiveness : Measured as the average number of claims extracted from the an- swers generated under each condition. 2. Diversity : Measured by clustering the claims for each answer and calculating the average number of clusters.
```bgraph-paragraph
{"id":"a5142bc9-9718-5ec9-b201-b36125b71b0e","node_type":"Paragraph","location":{"semantic":{"path":"1.8.2.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":131.4,"y":395.6,"width":372.6,"height":46.399994}}},"text_order":90,"token_count":56,"style":null}
```

For clustering, we followed the approach described by Padmakumar and He ( 2024 ), which involved using Scikit-learn’s implementation of agglomerative clustering ( Pedregosa et al. , 2011 ). Clusters were merged through “complete” linkage, meaning they were combined only if the maximum dis- tance between their farthest points was less than or equal to a predefined distance threshold. The distance metric used was 1 − ROUGE-L. Since the distance threshold influences the number of clusters, we report results across a range of thresholds.
```bgraph-paragraph
{"id":"e588d350-9def-57e2-ae3c-f92735334808","node_type":"Paragraph","location":{"semantic":{"path":"1.8.2.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":454.2,"width":396.0,"height":63.200012}}},"text_order":91,"token_count":131,"internal_refs":[{"text":"","source_page":9,"source_bbox":{"x":325.1,"y":453.1,"width":85.2,"height":10.9},"target":{"kind":"named","name":"cite.padmakumar","page":14,"point":{"x":83.0,"y":342.0}}},{"text":"","source_page":9,"source_bbox":{"x":414.0,"y":453.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.padmakumar","page":14,"point":{"x":83.0,"y":342.0}}},{"text":"","source_page":9,"source_bbox":{"x":370.3,"y":464.0,"width":65.8,"height":10.9},"target":{"kind":"named","name":"cite.perdregosa","page":14,"point":{"x":83.0,"y":373.0}}},{"text":"","source_page":9,"source_bbox":{"x":439.7,"y":464.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.perdregosa","page":14,"point":{"x":83.0,"y":373.0}}}],"style":null}
```

## 5 Results
```bgraph-section
{"id":"ea87ca3f-a1db-5f97-9d05-49d906bd8f33","node_type":"Section","location":{"semantic":{"path":"1.9","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":536.5,"width":55.1,"height":10.6}}},"text_order":92,"token_count":2,"style":null}
```

### 5.1 Experiment 1
```bgraph-section
{"id":"d7fef212-6906-5e88-9e83-34ffd941ccc6","node_type":"Section","location":{"semantic":{"path":"1.9.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":561.8,"width":80.2,"height":8.700012}}},"text_order":93,"token_count":4,"style":null}
```

The indexing process resulted in a graph consisting of 8,564 nodes and 20,691 edges for the Podcast dataset, and a larger graph of 15,754 nodes and 19,520 edges for the News dataset. Table 2 shows the number of community summaries at different levels of each graph community hierarchy. Global approaches vs. vector RAG . As shown in Figure 2 and Table 6 , global approaches signifi- cantly outperformed conventional vector RAG ( SS ) in both comprehensiveness and diversity criteria across datasets. Specifically, global approaches achieved comprehensiveness win rates between 72- 83% (p < .001) for Podcast transcripts and 72-80% (p < .001) for News articles, while diversity win rates ranged from 75-82% (p < .001) and 62-71% (p < .01) respectively. Our use of directness as a validity test confirmed that vector RAG produces the most direct responses across all comparisons. Empowerment . Empowerment comparisons showed mixed results for both global approaches ver- sus vector RAG ( SS ) and GraphRAG approaches versus source text summarization ( TS ). Using an LLM to analyze LLM reasoning for this measure indicated that the ability to provide specific exam-
```bgraph-paragraph
{"id":"37599214-e414-53f9-a767-9eece0c58bad","node_type":"Paragraph","location":{"semantic":{"path":"1.9.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":108.0,"y":582.3,"width":396.0,"height":139.70001}}},"text_order":94,"token_count":277,"internal_refs":[{"text":"","source_page":9,"source_bbox":{"x":445.8,"y":592.2,"width":31.7,"height":10.9},"target":{"kind":"named","name":"table.2","page":9,"point":{"x":142.0,"y":411.0}}},{"text":"","source_page":9,"source_bbox":{"x":312.2,"y":619.4,"width":35.6,"height":11.0},"target":{"kind":"named","name":"figure.2","page":9,"point":{"x":147.0,"y":333.0}}},{"text":"","source_page":9,"source_bbox":{"x":365.3,"y":619.4,"width":31.4,"height":11.0},"target":{"kind":"named","name":"table.6","page":25,"point":{"x":147.0,"y":97.0}}}],"style":null}
```

9
```bgraph-paragraph
{"id":"0004038e-9c9e-5d04-b060-b468affb7671","node_type":"Paragraph","location":{"semantic":{"path":"1.9.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":303.5,"y":743.2,"width":5.0,"height":8.7}}},"text_order":95,"token_count":1,"style":null}
```

### Podcast transcripts
```bgraph-section
{"id":"850ea9bc-b08b-5452-8fa3-793f13b41e0c","node_type":"Section","location":{"semantic":{"path":"1.9.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","Podcast transcripts"]},"physical":{"page":10,"bounding_box":{"x":265.0,"y":72.0,"width":82.0,"height":8.7}}},"text_order":96,"token_count":4,"style":null}
```

SS TS C0 C1 C2 C3 SS TS C0 C1 C2 C3 SS TS C0 C1 C2 C3 SS TS C0 C1 C2 C3 SS 50 17 28 25 22 21 SS 50 18 23 25 19 19 SS 50 42 57 52 49 51 SS 50 56 65 60 60 60 TS 83 50 50 48 43 44 TS 82 50 50 50 43 46 TS 58 50 59 55 52 51 TS 44 50 55 52 51 52 C0 72 50 50 53 50 49 C0 77 50 50 50 46 44 C0 43 41 50 49 47 48 C0 35 45 50 47 48 48 C1 75 52 47 50 52 50 C1 75 50 50 50 44 45 C1 48 45 51 50 49 50 C1 40 48 53 50 50 50 C2 78 57 50 48 50 52 C2 81 57 54 56 50 48 C2 51 48 53 51 50 51 C2 40 49 52 50 50 50 C3 79 56 51 50 48 50 C3 81 54 56 55 52 50 C3 49 49 52 50 49 50 C3 40 48 52 50 50 50
```bgraph-paragraph
{"id":"ad017374-e550-5e01-9f4d-510759798cc3","node_type":"Paragraph","location":{"semantic":{"path":"1.9.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","Podcast transcripts"]},"physical":{"page":10,"bounding_box":{"x":111.3,"y":91.8,"width":381.7,"height":83.2}}},"text_order":97,"token_count":136,"style":null}
```

Comprehensiveness Diversity Empowerment Directness
```bgraph-paragraph
{"id":"414470a7-747e-573c-b6aa-5d26413b869f","node_type":"Paragraph","location":{"semantic":{"path":"1.9.2.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","Podcast transcripts"]},"physical":{"page":10,"bounding_box":{"x":122.5,"y":186.8,"width":354.40002,"height":8.599991}}},"text_order":98,"token_count":10,"style":null}
```

### News articles
```bgraph-section
{"id":"3da281a3-ed09-5e5e-bb69-2cef474b5b93","node_type":"Section","location":{"semantic":{"path":"1.9.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":277.9,"y":204.8,"width":56.2,"height":8.7}}},"text_order":99,"token_count":3,"style":null}
```

SS TS C0 C1 C2 C3 SS TS C0 C1 C2 C3 SS TS C0 C1 C2 C3 SS TS C0 C1 C2 C3 SS 50 20 28 25 21 21 SS 50 33 38 35 29 31 SS 50 47 57 49 50 50 SS 50 54 59 55 55 54 TS 80 50 44 41 38 36 TS 67 50 53 45 44 40 TS 53 50 58 50 50 48 TS 46 50 55 53 52 52 C0 72 56 50 52 54 52 C0 62 47 50 40 41 41 C0 43 42 50 42 45 44 C0 41 45 50 48 48 47 C1 75 59 48 50 58 55 C1 65 55 60 50 50 50 C1 51 50 58 50 52 51 C1 45 47 52 50 49 49 C2 79 62 46 42 50 59 C2 71 56 59 50 50 51 C2 50 50 55 48 50 50 C2 45 48 52 51 50 49 C3 79 64 48 45 41 50 C3 69 60 59 50 49 50 C3 50 52 56 49 50 50 C3 46 48 53 51 51 50
```bgraph-paragraph
{"id":"1c1e2af2-7534-5d7f-bc8a-4337bf598f1a","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":111.3,"y":222.7,"width":381.7,"height":83.2}}},"text_order":100,"token_count":136,"style":null}
```

Comprehensiveness Diversity Empowerment Directness
```bgraph-paragraph
{"id":"e22fe438-fe1d-586f-9797-c1d7f5f19be1","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":122.5,"y":317.7,"width":354.40002,"height":8.600006}}},"text_order":101,"token_count":10,"style":null}
```

Figure 2: Head-to-head win rate percentages of (row condition) over (column condition) across two datasets, four metrics, and 125 questions per comparison (each repeated five times and averaged). The overall winner per dataset and metric is shown in bold. Self-win rates were not computed but are shown as the expected 50% for reference. All Graph RAG conditions outperformed naı̈ve RAG on comprehensiveness and diversity. Conditions C1-C3 also showed slight improvements in answer comprehensiveness and diversity over TS (global text summarization without a graph index).
```bgraph-paragraph
{"id":"e8d1357e-5f5c-52ad-9bb9-1909c7248103","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":108.0,"y":337.0,"width":396.0,"height":63.30002}}},"text_order":102,"token_count":140,"style":null}
```

Table 2: Number of context units (community summaries for C0-C3 and text chunks for TS ), corre- sponding token counts, and percentage of the maximum token count. Map-reduce summarization of source texts is the most resource-intensive approach requiring the highest number of context tokens. Root-level community summaries ( C0 ) require dramatically fewer tokens per query (9x-43x).
```bgraph-paragraph
{"id":"c60513f4-e336-5070-b2bb-718414b8be88","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":108.0,"y":415.0,"width":396.0,"height":41.5}}},"text_order":103,"token_count":92,"style":null}
```

Podcast Transcripts News Articles C0 C1 C2 C3 TS C0 C1 C2 C3 TS Units 34 367 969 1310 1669 55 555 1797 2142 3197 Tokens 26657 225756 565720 746100 1014611 39770 352641 980898 1140266 1707694 % Max 2.6 22.2 55.8 73.5 100 2.3 20.7 57.4 66.8 100
```bgraph-paragraph
{"id":"aca6bd86-5ddb-511b-ae83-6e9223779778","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":114.0,"y":467.7,"width":391.8,"height":59.200012}}},"text_order":104,"token_count":56,"style":null}
```

ples, quotes, and citations was judged to be key to helping users reach an informed understanding. Tuning element extraction prompts may help to retain more of these details in the GraphRAG index. Community summaries vs. source texts . When comparing community summaries to source texts using GraphRAG, community summaries generally provided a small but consistent improvement in answer comprehensiveness and diversity, except for root-level summaries. Intermediate-level summaries in the Podcast dataset and low-level community summaries in the News dataset achieved comprehensiveness win rates of 57% (p < .001) and 64% (p < .001), respectively. Diversity win rates were 57% (p=.036) for Podcast intermediate-level summaries and 60% (p < .001) for News low-level community summaries. Table 2 also illustrates the scalability advantages of GraphRAG compared to source text summarization: for low-level community summaries ( C3 ), GraphRAG required 26- 33% fewer context tokens, while for root-level community summaries ( C0 ), it required over 97% fewer tokens. For a modest drop in performance compared with other global methods, root-level GraphRAG offers a highly efficient method for the iterative question answering that characterizes sensemaking activity, while retaining advantages in comprehensiveness (72% win rate) and diversity (62% win rate) over vector RAG.
```bgraph-paragraph
{"id":"92dc6fa0-3c52-545d-98b4-ea343c4e2a15","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":108.0,"y":555.1,"width":396.0,"height":166.90002}}},"text_order":105,"token_count":333,"internal_refs":[{"text":"","source_page":10,"source_bbox":{"x":205.2,"y":646.8,"width":31.7,"height":10.9},"target":{"kind":"named","name":"table.2","page":9,"point":{"x":142.0,"y":411.0}}}],"style":null}
```

10
```bgraph-paragraph
{"id":"66b0d138-a571-50d7-b75c-bd513ee4a406","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":10,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":106,"token_count":1,"style":null}
```

Table 3: Average number of extracted claims, reported by condition and dataset type. Bolded values represent the highest score in each column.
```bgraph-paragraph
{"id":"5915aa65-b713-5c2f-809e-4f7dce217054","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.8","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":11,"bounding_box":{"x":108.0,"y":72.1,"width":396.0,"height":19.599998}}},"text_order":107,"token_count":34,"style":null}
```

Average Number of Claims Condition News Articles Podcast Transcripts C0 34.18 32.21 C1 32.50 32.20 C2 31.62 32.46 C3 33.14 32.28 TS 32.89 31.39 SS 25.23 26.50
```bgraph-paragraph
{"id":"f7ee42db-3710-59e4-8ed3-285d668685c5","node_type":"Paragraph","location":{"semantic":{"path":"1.9.3.9","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","News articles"]},"physical":{"page":11,"bounding_box":{"x":200.0,"y":104.2,"width":209.5,"height":95.0}}},"text_order":108,"token_count":33,"style":null}
```

### 5.2 Experiment 2
```bgraph-section
{"id":"54f1f5cf-687f-51fc-a329-05aeec612a7a","node_type":"Section","location":{"semantic":{"path":"1.9.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":108.0,"y":229.8,"width":80.2,"height":8.699997}}},"text_order":109,"token_count":4,"style":null}
```

Table 3 shows the results for the average number of extracted claims (i.e., the claim-based measure of comprehensiveness) per condition. For both the News and Podcast datasets, all global search conditions ( C0-C3 ) and source text summarization ( TS ) had greater comprehensiveness than vector RAG ( SS ). The differences were statistically significant (p < .05) in all cases. These findings align with the LLM-based win rates from Experiment 1. Table 4 contains the results for the average number of clusters, the claim-based measure of diver- sity. For the Podcast dataset, all global search conditions had significantly greater diversity than SS across all distance thresholds (p < .05), consistent with the win rates observed in Experiment 1. For the News dataset, however, only C0 significantly outperformed SS across all distance thresholds (p < .05). While C1-C3 also achieved higher average cluster counts than SS , the differences were statistically significant only at certain distance thresholds. In Experiment 1, all global search con- ditions significantly outperformed SS in the News dataset – not just C0 . However, the differences in mean diversity scores between SS and the global search conditions were smaller for the News dataset than for the Podcast dataset, aligning directionally with the claim-based results. For both comprehensiveness and diversity, across both datasets, there were no statistically significant differences observed among the global search conditions or between global search and TS . Finally, for each pairwise comparison in Experiment 1, we tested whether the answer preferred by the LLM aligned with the winner based on the claim-based metrics. Since each pairwise comparison in Experiment 1 was performed five times, while the claim-based metrics provided only one outcome per comparison, we aggregated the Experiment 1 results into a single label using majority voting. For example, if C0 won over SS in three out of five judgments for comprehensiveness on a given question, C0 was labeled the winner and SS the loser. However, if C0 won twice, SS won once, and they tied twice, then there was no majority outcome, so the final label was a tie. We found that exact ties were rare for the claim-based metrics. One possible solution is to define a tie based on a threshold (e.g., the absolute difference between the claim-based results for condition A and condition B must be less than or equal to x ). However, we observed that the results were sensitive to the choice of threshold. As a result, we focused on cases where the aggregated LLM label was not a tie, representing 33% and 39% of pairwise comparisons for comprehensiveness and diversity, respectively. In these cases, the aggregated LLM label matched the claim-based label in 78% of pairwise comparisons for comprehensiveness and 69-70% for diversity (across all distance thresholds), indicating moderately strong alignment.
```bgraph-paragraph
{"id":"e50f5f1a-a02d-5ca7-ab89-9db1d50ad748","node_type":"Paragraph","location":{"semantic":{"path":"1.9.4.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":108.0,"y":251.5,"width":396.0,"height":357.90002}}},"text_order":110,"token_count":705,"internal_refs":[{"text":"","source_page":11,"source_bbox":{"x":107.0,"y":250.5,"width":31.5,"height":10.9},"target":{"kind":"named","name":"table.3","page":10,"point":{"x":142.0,"y":68.0}}},{"text":"","source_page":11,"source_bbox":{"x":107.0,"y":310.5,"width":32.0,"height":10.9},"target":{"kind":"named","name":"table.4","page":11,"point":{"x":143.0,"y":68.0}}}],"style":null}
```

## 6 Discussion
```bgraph-section
{"id":"7fc81358-d773-56c0-a715-592e823d692d","node_type":"Section","location":{"semantic":{"path":"1.10","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion"]},"physical":{"page":11,"bounding_box":{"x":108.0,"y":631.7,"width":71.7,"height":10.6}}},"text_order":111,"token_count":3,"style":null}
```

### 6.1 Limitations of evaluation approach
```bgraph-section
{"id":"b1a37d56-fcda-520c-bc40-336cb49371b3","node_type":"Section","location":{"semantic":{"path":"1.10.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":11,"bounding_box":{"x":108.0,"y":658.8,"width":171.70001,"height":8.700012}}},"text_order":112,"token_count":9,"style":null}
```

Our evaluation to date has focused on sensemaking questions specific to two corpora each containing approximately 1 million tokens. More work is needed to understand how performance generalizes to datasets from various domains with different use cases. Comparison of fabrication rates, e.g., using approaches like SelfCheckGPT ( Manakul et al. , 2023 ), would also strengthen the current analysis.
```bgraph-paragraph
{"id":"6c06cda3-f34b-5111-b964-400f09f263d2","node_type":"Paragraph","location":{"semantic":{"path":"1.10.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":11,"bounding_box":{"x":108.0,"y":680.6,"width":396.0,"height":41.400024}}},"text_order":113,"token_count":96,"internal_refs":[{"text":"","source_page":11,"source_bbox":{"x":237.8,"y":712.2,"width":59.3,"height":10.9},"target":{"kind":"named","name":"cite.manakul2023selfcheckgpt","page":13,"point":{"x":83.0,"y":618.0}}},{"text":"","source_page":11,"source_bbox":{"x":300.0,"y":712.2,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.manakul2023selfcheckgpt","page":13,"point":{"x":83.0,"y":618.0}}}],"style":null}
```

11
```bgraph-paragraph
{"id":"becef33e-7933-5e1a-ae6d-02485a6594da","node_type":"Paragraph","location":{"semantic":{"path":"1.10.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":11,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":114,"token_count":1,"style":null}
```

Table 4: Average number of clusters across different distance thresholds, reported by condition and dataset type. Bolded values represent the highest score in each row.
```bgraph-paragraph
{"id":"25322047-3b67-5506-bb7b-96398e7744d5","node_type":"Paragraph","location":{"semantic":{"path":"1.10.1.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":72.1,"width":396.0,"height":19.599998}}},"text_order":115,"token_count":41,"style":null}
```

Average Number of Clusters Dataset Distance Threshold C0 C1 C2 C3 TS SS 0.5 23.42 21.85 21.90 22.13 21.80 17.92 0.6 21.65 20.38 20.30 20.52 20.13 16.78 News Articles 0.7 20.19 19.06 19.03 19.13 18.62 15.80 0.8 18.86 17.78 17.82 17.79 17.30 14.80 0.5 23.16 22.62 22.52 21.93 21.14 18.55 0.6 21.65 21.33 21.21 20.62 19.70 17.39 Podcast Transcripts 0.7 20.41 20.04 19.79 19.22 18.08 16.28 0.8 19.26 18.77 18.46 17.89 16.66 15.07
```bgraph-paragraph
{"id":"b00ae54f-c139-5b45-a3ab-4133b762acd0","node_type":"Paragraph","location":{"semantic":{"path":"1.10.1.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":12,"bounding_box":{"x":115.4,"y":104.2,"width":378.7,"height":121.8}}},"text_order":116,"token_count":80,"style":null}
```

### 6.2 Future work
```bgraph-section
{"id":"858c0c7d-ec89-578b-825f-5bd2b0ece683","node_type":"Section","location":{"semantic":{"path":"1.10.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.2 Future work"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":251.5,"width":76.09999,"height":8.700012}}},"text_order":117,"token_count":3,"style":null}
```

The graph index, rich text annotations, and hierarchical community structure supporting the current GraphRAG approach offer many possibilities for refinement and adaptation. This includes RAG approaches that operate in a more local manner, via embedding-based matching of user queries and graph annotations. In particular, we see potential in hybrid RAG schemes that combine embedding- based matching with just-in-time community report generation before employing our map-reduce summarization mechanisms. This “roll-up” approach could also be extended across multiple levels of the community hierarchy, as well as implemented as a more exploratory “drill down” mechanism that follows the information scent contained in higher-level community summaries. Broader impacts . As a mechanism for question answering over large document collections, there are risks to downstream sensemaking and decision-making tasks if the generated answers do not accurately represent the source data. System use should be accompanied by clear disclosures of AI use and the potential for errors in outputs. Compared to vector RAG, however, GraphRAG shows promise as a way to mitigate these downstream risks for questions of a global nature, which might otherwise be answered by samples of retrieved facts falsely presented as global summaries.
```bgraph-paragraph
{"id":"a669b743-6be0-565f-a7ea-7cd64a0f947b","node_type":"Paragraph","location":{"semantic":{"path":"1.10.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.2 Future work"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":271.6,"width":396.0,"height":156.0}}},"text_order":118,"token_count":323,"style":null}
```

## 7 Conclusion
```bgraph-section
{"id":"77f0af18-c728-56f3-a43d-2185aa115273","node_type":"Section","location":{"semantic":{"path":"1.11","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","7 Conclusion"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":445.4,"width":75.1,"height":10.6}}},"text_order":119,"token_count":3,"style":null}
```

We have presented GraphRAG, a RAG approach that combines knowledge graph generation and query-focused summarization (QFS) to support human sensemaking over entire text corpora. Initial evaluations show substantial improvements over a vector RAG baseline for both the comprehensive- ness and diversity of answers, as well as favorable comparisons to a global but graph-free approach using map-reduce source text summarization. For situations requiring many global queries over the same dataset, summaries of root-level communities in the entity-based graph index provide a data index that is both superior to vector RAG and achieves competitive performance to other global methods at a fraction of the token cost.
```bgraph-paragraph
{"id":"af519d62-66a1-5873-b61a-7e5380a0106c","node_type":"Paragraph","location":{"semantic":{"path":"1.11.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","7 Conclusion"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":469.8,"width":396.0,"height":85.10004}}},"text_order":120,"token_count":174,"style":null}
```

## Acknowledgements
```bgraph-section
{"id":"bf721af4-f526-50a3-8f82-215761fb258e","node_type":"Section","location":{"semantic":{"path":"1.12","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Acknowledgements"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":572.7,"width":98.8,"height":10.6}}},"text_order":121,"token_count":4,"style":null}
```

We would also like to thank the following people who contributed to the work: Alonso Guevara Fernández, Amber Hoak, Andrés Morales Esquivel, Ben Cutler, Billie Rinaldi, Chris Sanchez, Chris Trevino, Christine Caggiano, David Tittsworth, Dayenne de Souza, Douglas Orbaker, Ed Clark, Gabriel Nieves-Ponce, Gaudy Blanco Meneses, Kate Lytvynets, Katy Smith, Mónica Carva- jal, Nathan Evans, Richard Ortega, Rodrigo Racanicci, Sarah Smith, and Shane Solomon.
```bgraph-paragraph
{"id":"700b2b5e-272f-5205-a18d-3db89119b988","node_type":"Paragraph","location":{"semantic":{"path":"1.12.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Acknowledgements"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":597.1,"width":396.0,"height":52.30005}}},"text_order":122,"token_count":112,"style":null}
```

## References
```bgraph-section
{"id":"81d76886-d091-58ad-ab58-97bc994c5eea","node_type":"Section","location":{"semantic":{"path":"1.13","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":672.5,"width":55.5,"height":10.6}}},"text_order":123,"token_count":2,"style":null}
```

Achiam, J., Adler, S., Agarwal, S., Ahmad, L., Akkaya, I., Aleman, F. L., Almeida, D., Al- tenschmidt, J., Altman, S., Anadkat, S., et al. (2023). Gpt-4 technical report. arXiv preprint arXiv:2303.08774 .
```bgraph-paragraph
{"id":"3d3b8b11-1281-5da8-9dc0-4c14e7e5914e","node_type":"Paragraph","location":{"semantic":{"path":"1.13.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":12,"bounding_box":{"x":108.0,"y":691.5,"width":396.0,"height":30.5}}},"text_order":124,"token_count":49,"style":null}
```

12
```bgraph-paragraph
{"id":"1b1142fe-4157-58aa-9dde-8e43ab8bbbbb","node_type":"Paragraph","location":{"semantic":{"path":"1.13.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":12,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":125,"token_count":1,"style":null}
```

Anil, R., Borgeaud, S., Wu, Y., Alayrac, J.-B., Yu, J., Soricut, R., Schalkwyk, J., Dai, A. M., Hauth, A., et al. (2023). Gemini: a family of highly capable multimodal models. arXiv preprint arXiv:2312.11805 .
```bgraph-paragraph
{"id":"8e4d326a-e4be-5076-89c9-b03c354b6807","node_type":"Paragraph","location":{"semantic":{"path":"1.13.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":30.499992}}},"text_order":126,"token_count":50,"style":null}
```

Baek, J., Aji, A. F., and Saffari, A. (2023). Knowledge-augmented language model prompting for zero-shot knowledge graph question answering. arXiv preprint arXiv:2306.04136 .
```bgraph-paragraph
{"id":"8e1143fa-3b19-5536-802b-c1746c7fd844","node_type":"Paragraph","location":{"semantic":{"path":"1.13.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":117.4,"width":396.0,"height":19.599998}}},"text_order":127,"token_count":42,"style":null}
```

Ban, T., Chen, L., Wang, X., and Chen, H. (2023). From query tools to causal architects: Harnessing large language models for advanced causal discovery from data.
```bgraph-paragraph
{"id":"95c44bae-d548-5068-93fa-780c8fb1245d","node_type":"Paragraph","location":{"semantic":{"path":"1.13.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":148.6,"width":396.0,"height":19.59999}}},"text_order":128,"token_count":39,"style":null}
```

Barlaug, N. and Gulla, J. A. (2021). Neural networks for entity matching: A survey. ACM Transac- tions on Knowledge Discovery from Data (TKDD) , 15(3):1–37.
```bgraph-paragraph
{"id":"1e52e108-5af3-55c2-8397-71fdbce5ea70","node_type":"Paragraph","location":{"semantic":{"path":"1.13.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":179.8,"width":396.0,"height":19.59999}}},"text_order":129,"token_count":37,"style":null}
```

Baumel, T., Eyal, M., and Elhadad, M. (2018). Query focused abstractive summarization: Incorpo- rating query relevance, multi-document coverage, and summary length constraints into seq2seq models. arXiv preprint arXiv:1801.07704 .
```bgraph-paragraph
{"id":"4182a272-e3f5-541e-9db0-7e1635e3fe1a","node_type":"Paragraph","location":{"semantic":{"path":"1.13.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":211.0,"width":396.0,"height":30.5}}},"text_order":130,"token_count":55,"style":null}
```

Blondel, V. D., Guillaume, J.-L., Lambiotte, R., and Lefebvre, E. (2008). Fast unfolding of communities in large networks. Journal of statistical mechanics: theory and experiment , 2008(10):P10008.
```bgraph-paragraph
{"id":"73a48361-175f-58cd-81a2-dfcc0040f153","node_type":"Paragraph","location":{"semantic":{"path":"1.13.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":253.1,"width":396.0,"height":30.5}}},"text_order":131,"token_count":47,"style":null}
```

Brown, T., Mann, B., Ryder, N., Subbiah, M., Kaplan, J. D., Dhariwal, P., Neelakantan, A., Shyam, P., Sastry, G., Askell, A., et al. (2020). Language models are few-shot learners. Advances in neural information processing systems , 33:1877–1901.
```bgraph-paragraph
{"id":"b966e743-7c30-59ab-8558-2dcd57769b77","node_type":"Paragraph","location":{"semantic":{"path":"1.13.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":295.2,"width":396.0,"height":30.5}}},"text_order":132,"token_count":59,"style":null}
```

Cheng, X., Luo, D., Chen, X., Liu, L., Zhao, D., and Yan, R. (2024). Lift yourself up: Retrieval- augmented text generation with self-memory. Advances in Neural Information Processing Sys- tems , 36.
```bgraph-paragraph
{"id":"527dce4b-665d-57db-b71b-e0a3ced38aed","node_type":"Paragraph","location":{"semantic":{"path":"1.13.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":337.3,"width":396.0,"height":30.50003}}},"text_order":133,"token_count":47,"style":null}
```

Christen, P. and Christen, P. (2012). The data matching process . Springer.
```bgraph-paragraph
{"id":"fa764480-fe72-5daf-89ee-51ceedfe78c0","node_type":"Paragraph","location":{"semantic":{"path":"1.13.11","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":379.4,"width":295.19998,"height":8.700012}}},"text_order":134,"token_count":17,"style":null}
```

Chung, J., Pedigo, B. D., Bridgeford, E. W., Varjavand, B. K., Helm, H. S., and Vogelstein, J. T. (2019). Graspy: Graph statistics in python. Journal of Machine Learning Research , 20(158):1–7.
```bgraph-paragraph
{"id":"ad07df72-ba82-565a-9ad2-4b1c9b4e8b1d","node_type":"Paragraph","location":{"semantic":{"path":"1.13.12","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":399.7,"width":396.0,"height":19.600006}}},"text_order":135,"token_count":47,"style":null}
```

Dang, H. T. (2006). Duc 2005: Evaluation of question-focused summarization systems. In Proceed- ings of the Workshop on Task-Focused Summarization and Question Answering , pages 48–55.
```bgraph-paragraph
{"id":"d16260a6-c9bb-55d1-81d7-59cacce645f4","node_type":"Paragraph","location":{"semantic":{"path":"1.13.13","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":430.9,"width":396.0,"height":19.600006}}},"text_order":136,"token_count":45,"style":null}
```

Elmagarmid, A. K., Ipeirotis, P. G., and Verykios, V. S. (2006). Duplicate record detection: A survey. IEEE Transactions on knowledge and data engineering , 19(1):1–16.
```bgraph-paragraph
{"id":"52b23a7c-b767-5282-b090-00db08b8f5fb","node_type":"Paragraph","location":{"semantic":{"path":"1.13.14","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":462.1,"width":396.0,"height":19.600006}}},"text_order":137,"token_count":39,"style":null}
```

Es, S., James, J., Espinosa-Anke, L., and Schockaert, S. (2023). Ragas: Automated evaluation of retrieval augmented generation. arXiv preprint arXiv:2309.15217 .
```bgraph-paragraph
{"id":"51f8b9d5-9029-554e-921b-7436156d2183","node_type":"Paragraph","location":{"semantic":{"path":"1.13.15","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":493.3,"width":396.0,"height":19.600037}}},"text_order":138,"token_count":38,"style":null}
```

Etzioni, O., Cafarella, M., Downey, D., Kok, S., Popescu, A.-M., Shaked, T., Soderland, S., Weld, D. S., and Yates, A. (2004). Web-scale information extraction in knowitall: (preliminary re- sults). In Proceedings of the 13th International Conference on World Wide Web , WWW ’04, page 100–110, New York, NY, USA. Association for Computing Machinery.
```bgraph-paragraph
{"id":"bc257d88-7e3b-5f03-80b8-68b9b346e082","node_type":"Paragraph","location":{"semantic":{"path":"1.13.16","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":524.5,"width":396.0,"height":41.5}}},"text_order":139,"token_count":85,"style":null}
```

Feng, Z., Feng, X., Zhao, D., Yang, M., and Qin, B. (2023). Retrieval-generation synergy augmented large language models. arXiv preprint arXiv:2310.05149 .
```bgraph-paragraph
{"id":"3c9e5115-9a35-59ca-8f78-15ef81aacbc1","node_type":"Paragraph","location":{"semantic":{"path":"1.13.17","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":577.6,"width":396.0,"height":19.600037}}},"text_order":140,"token_count":37,"style":null}
```

Fortunato, S. (2010). Community detection in graphs. Physics reports , 486(3-5):75–174.
```bgraph-paragraph
{"id":"37ec1684-1c6e-5bfd-ba51-46539bc92b80","node_type":"Paragraph","location":{"semantic":{"path":"1.13.18","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":608.8,"width":355.1,"height":8.700012}}},"text_order":141,"token_count":21,"style":null}
```

Gao, Y., Xiong, Y., Gao, X., Jia, K., Pan, J., Bi, Y., Dai, Y., Sun, J., and Wang, H. (2023). Retrieval- augmented generation for large language models: A survey. arXiv preprint arXiv:2312.10997 .
```bgraph-paragraph
{"id":"b6d66416-8a6f-515f-8dc0-78282bbcd650","node_type":"Paragraph","location":{"semantic":{"path":"1.13.19","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":629.1,"width":396.0,"height":19.600037}}},"text_order":142,"token_count":48,"style":null}
```

He, X., Tian, Y., Sun, Y., Chawla, N. V., Laurent, T., LeCun, Y., Bresson, X., and Hooi, B. (2024). G-retriever: Retrieval-augmented generation for textual graph understanding and question an- swering. arXiv preprint arXiv:2402.07630 .
```bgraph-paragraph
{"id":"cecbe7c3-a28b-5bd6-ab58-0cd7808e599e","node_type":"Paragraph","location":{"semantic":{"path":"1.13.20","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":660.3,"width":396.0,"height":30.5}}},"text_order":143,"token_count":57,"style":null}
```

Huang, J., Chen, X., Mishra, S., Zheng, H. S., Yu, A. W., Song, X., and Zhou, D. (2023). Large language models cannot self-correct reasoning yet. arXiv preprint arXiv:2310.01798 .
```bgraph-paragraph
{"id":"e7170a00-ce7c-5bd5-ad32-3921a46f267d","node_type":"Paragraph","location":{"semantic":{"path":"1.13.21","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":108.0,"y":702.4,"width":396.0,"height":19.599976}}},"text_order":144,"token_count":43,"style":null}
```

13
```bgraph-paragraph
{"id":"704e5cf1-937d-5b5b-831f-799582c8d12f","node_type":"Paragraph","location":{"semantic":{"path":"1.13.22","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":145,"token_count":1,"style":null}
```

Jacomy, M., Venturini, T., Heymann, S., and Bastian, M. (2014). Forceatlas2, a continuous graph layout algorithm for handy network visualization designed for the gephi software. PLoS ONE 9(6): e98679. https://doi.org/10.1371/journal.pone.0098679 .
```bgraph-paragraph
{"id":"e8e3aac0-738e-5600-82d1-e1c595b56765","node_type":"Paragraph","location":{"semantic":{"path":"1.13.23","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":30.499992}}},"text_order":146,"token_count":60,"style":null}
```

Jin, D., Yu, Z., Jiao, P., Pan, S., He, D., Wu, J., Philip, S. Y., and Zhang, W. (2021). A survey of community detection approaches: From statistical modeling to deep learning. IEEE Transactions on Knowledge and Data Engineering , 35(2):1149–1170.
```bgraph-paragraph
{"id":"dc094c5e-638f-51e1-b2eb-72b967781edc","node_type":"Paragraph","location":{"semantic":{"path":"1.13.24","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":116.8,"width":396.0,"height":30.5}}},"text_order":147,"token_count":60,"style":null}
```

Kang, M., Kwak, J. M., Baek, J., and Hwang, S. J. (2023). Knowledge graph-augmented language models for knowledge-grounded dialogue generation. arXiv preprint arXiv:2305.18846 .
```bgraph-paragraph
{"id":"22e45e0f-4b48-5264-9024-0a7e1d532362","node_type":"Paragraph","location":{"semantic":{"path":"1.13.25","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":158.3,"width":396.0,"height":19.59999}}},"text_order":148,"token_count":43,"style":null}
```

Khattab, O., Santhanam, K., Li, X. L., Hall, D., Liang, P., Potts, C., and Zaharia, M. (2022). Demonstrate-search-predict: Composing retrieval and language models for knowledge-intensive nlp. arXiv preprint arXiv:2212.14024 .
```bgraph-paragraph
{"id":"123338f4-a457-51da-bfc8-8c6bdec544fe","node_type":"Paragraph","location":{"semantic":{"path":"1.13.26","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":188.9,"width":396.0,"height":30.5}}},"text_order":149,"token_count":54,"style":null}
```

Kim, D., Xie, L., and Ong, C. S. (2016). Probabilistic knowledge graph construction: Compositional and incremental approaches. In Proceedings of the 25th ACM International on Conference on Information and Knowledge Management , CIKM ’16, page 2257–2262, New York, NY, USA. Association for Computing Machinery.
```bgraph-paragraph
{"id":"b4471f43-e01a-5a80-a784-04f3be4695da","node_type":"Paragraph","location":{"semantic":{"path":"1.13.27","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":230.4,"width":396.0,"height":41.400024}}},"text_order":150,"token_count":75,"style":null}
```

Kim, G., Kim, S., Jeon, B., Park, J., and Kang, J. (2023). Tree of clarifications: Answering ambigu- ous questions with retrieval-augmented large language models. arXiv preprint arXiv:2310.14696 .
```bgraph-paragraph
{"id":"57c30949-347c-5707-bc20-d0d06b87bde4","node_type":"Paragraph","location":{"semantic":{"path":"1.13.28","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":282.8,"width":396.0,"height":19.600037}}},"text_order":151,"token_count":48,"style":null}
```

Klein, G., Moon, B., and Hoffman, R. R. (2006). Making sense of sensemaking 1: Alternative perspectives. IEEE intelligent systems , 21(4):70–73.
```bgraph-paragraph
{"id":"e522f7c8-abeb-5f36-a6f8-a514bfc82c80","node_type":"Paragraph","location":{"semantic":{"path":"1.13.29","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":313.4,"width":396.0,"height":19.600006}}},"text_order":152,"token_count":35,"style":null}
```

Kosinski, M. (2024). Evaluating large language models in theory of mind tasks. Proceedings of the National Academy of Sciences , 121(45):e2405460121.
```bgraph-paragraph
{"id":"976ec1f8-92a9-577b-b199-91ea32a546aa","node_type":"Paragraph","location":{"semantic":{"path":"1.13.30","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":344.0,"width":396.09998,"height":19.600006}}},"text_order":153,"token_count":35,"style":null}
```

Kuratov, Y., Bulatov, A., Anokhin, P., Sorokin, D., Sorokin, A., and Burtsev, M. (2024). In search of needles in a 11m haystack: Recurrent memory finds what llms miss.
```bgraph-paragraph
{"id":"51ac7bea-d5f5-508a-b393-3f785cc761f5","node_type":"Paragraph","location":{"semantic":{"path":"1.13.31","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":374.6,"width":396.0,"height":19.600006}}},"text_order":154,"token_count":41,"style":null}
```

LangChain (2024). Langchain graphs. https://langchain-graphrag . readthedocs . io/en/latest/ .
```bgraph-paragraph
{"id":"78f7acde-c4be-5991-8e3c-76e023f1b05c","node_type":"Paragraph","location":{"semantic":{"path":"1.13.32","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":405.0,"width":364.3,"height":8.900024}}},"text_order":155,"token_count":22,"external_refs":[{"text":"","source_page":14,"source_bbox":{"x":263.6,"y":404.1,"width":207.2,"height":10.9},"target":{"kind":"uri","url":"https://langchain-graphrag.readthedocs.io/en/latest/"}},{"text":"","source_page":14,"source_bbox":{"x":263.6,"y":404.1,"width":207.2,"height":10.9},"target":{"kind":"uri","url":"https://langchain-graphrag.readthedocs.io/en/latest/"}},{"text":"","source_page":14,"source_bbox":{"x":263.6,"y":404.1,"width":207.2,"height":10.9},"target":{"kind":"uri","url":"https://langchain-graphrag.readthedocs.io/en/latest/"}},{"text":"","source_page":14,"source_bbox":{"x":263.6,"y":404.1,"width":207.2,"height":10.9},"target":{"kind":"uri","url":"https://langchain-graphrag.readthedocs.io/en/latest/"}},{"text":"","source_page":14,"source_bbox":{"x":263.6,"y":404.1,"width":207.2,"height":10.9},"target":{"kind":"uri","url":"https://langchain-graphrag.readthedocs.io/en/latest/"}}],"style":null}
```

Laskar, M. T. R., Hoque, E., and Huang, J. (2020). Query focused abstractive summarization via incorporating query relevance and transfer learning with transformer models. In Advances in Artificial Intelligence: 33rd Canadian Conference on Artificial Intelligence, Canadian AI 2020, Ottawa, ON, Canada, May 13–15, 2020, Proceedings 33 , pages 342–348. Springer.
```bgraph-paragraph
{"id":"10af7cab-4c9c-5780-a238-8cccef1ff6c1","node_type":"Paragraph","location":{"semantic":{"path":"1.13.33","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":424.9,"width":396.0,"height":41.400024}}},"text_order":156,"token_count":87,"style":null}
```

Lewis, P., Perez, E., Piktus, A., Petroni, F., Karpukhin, V., Goyal, N., Küttler, H., Lewis, M., Yih, W.-t., Rocktäschel, T., et al. (2020). Retrieval-augmented generation for knowledge-intensive nlp tasks. Advances in Neural Information Processing Systems , 33:9459–9474.
```bgraph-paragraph
{"id":"078461ea-d631-545a-99d2-a1fb122810c6","node_type":"Paragraph","location":{"semantic":{"path":"1.13.34","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":477.3,"width":396.0,"height":30.50003}}},"text_order":157,"token_count":66,"style":null}
```

Liu, N. F., Lin, K., Hewitt, J., Paranjape, A., Bevilacqua, M., Petroni, F., and Liang, P. (2023). Lost in the middle: How language models use long contexts. arXiv:2307.03172.
```bgraph-paragraph
{"id":"df51cfdf-eac8-58e0-9e9c-fce2b1b97193","node_type":"Paragraph","location":{"semantic":{"path":"1.13.35","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":518.8,"width":396.0,"height":19.600037}}},"text_order":158,"token_count":42,"style":null}
```

LlamaIndex (2024). GraphRAG Implementation with LlamaIndex - V2. https://github . com/run- llama/llama index/blob/main/docs/docs/examples/cookbooks/GraphRAG v2 . ipynb .
```bgraph-paragraph
{"id":"cfe3035c-8db9-5630-a1c8-379ca3db36c2","node_type":"Paragraph","location":{"semantic":{"path":"1.13.36","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":549.2,"width":395.5,"height":19.799988}}},"text_order":159,"token_count":42,"external_refs":[{"text":"","source_page":14,"source_bbox":{"x":410.1,"y":548.3,"width":94.9,"height":10.9},"target":{"kind":"uri","url":"https://github.com/run-llama/llama_index/blob/main/docs/docs/examples/cookbooks/GraphRAG_v2.ipynb"}},{"text":"","source_page":14,"source_bbox":{"x":410.1,"y":548.3,"width":94.9,"height":10.9},"target":{"kind":"uri","url":"https://github.com/run-llama/llama_index/blob/main/docs/docs/examples/cookbooks/GraphRAG_v2.ipynb"}},{"text":"","source_page":14,"source_bbox":{"x":410.1,"y":548.3,"width":94.9,"height":10.9},"target":{"kind":"uri","url":"https://github.com/run-llama/llama_index/blob/main/docs/docs/examples/cookbooks/GraphRAG_v2.ipynb"}},{"text":"","source_page":14,"source_bbox":{"x":117.0,"y":559.2,"width":333.2,"height":10.9},"target":{"kind":"uri","url":"https://github.com/run-llama/llama_index/blob/main/docs/docs/examples/cookbooks/GraphRAG_v2.ipynb"}},{"text":"","source_page":14,"source_bbox":{"x":117.0,"y":559.2,"width":333.2,"height":10.9},"target":{"kind":"uri","url":"https://github.com/run-llama/llama_index/blob/main/docs/docs/examples/cookbooks/GraphRAG_v2.ipynb"}},{"text":"","source_page":14,"source_bbox":{"x":117.0,"y":559.2,"width":333.2,"height":10.9},"target":{"kind":"uri","url":"https://github.com/run-llama/llama_index/blob/main/docs/docs/examples/cookbooks/GraphRAG_v2.ipynb"}}],"style":null}
```

Madaan, A., Tandon, N., Gupta, P., Hallinan, S., Gao, L., Wiegreffe, S., Alon, U., Dziri, N., Prabhu- moye, S., Yang, Y., et al. (2024). Self-refine: Iterative refinement with self-feedback. Advances in Neural Information Processing Systems , 36.
```bgraph-paragraph
{"id":"1482658e-f872-51aa-9ea2-56ede4d27976","node_type":"Paragraph","location":{"semantic":{"path":"1.13.37","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":580.0,"width":396.0,"height":30.5}}},"text_order":160,"token_count":60,"style":null}
```

Manakul, P., Liusie, A., and Gales, M. J. (2023). Selfcheckgpt: Zero-resource black-box hallucina- tion detection for generative large language models. arXiv preprint arXiv:2303.08896 .
```bgraph-paragraph
{"id":"6cdd4261-9f37-5e8c-85b5-19e0ef8256f5","node_type":"Paragraph","location":{"semantic":{"path":"1.13.38","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":621.5,"width":396.0,"height":19.600037}}},"text_order":161,"token_count":45,"style":null}
```

Mao, Y., He, P., Liu, X., Shen, Y., Gao, J., Han, J., and Chen, W. (2020). Generation-augmented retrieval for open-domain question answering. arXiv preprint arXiv:2009.08553 .
```bgraph-paragraph
{"id":"84d0d5c3-d8b4-532b-9714-e468ee169e10","node_type":"Paragraph","location":{"semantic":{"path":"1.13.39","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":652.1,"width":396.0,"height":19.600037}}},"text_order":162,"token_count":42,"style":null}
```

Martin, S., Brown, W. M., Klavans, R., and Boyack, K. (2011). Openord: An open-source toolbox for large graph layout. SPIE Conference on Visualization and Data Analysis (VDA) .
```bgraph-paragraph
{"id":"1465c67f-4a10-541e-8a5e-670a4f73a999","node_type":"Paragraph","location":{"semantic":{"path":"1.13.40","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":682.7,"width":396.0,"height":19.599976}}},"text_order":163,"token_count":43,"style":null}
```

Melnyk, I., Dognin, P., and Das, P. (2022). Knowledge graph generation from text.
```bgraph-paragraph
{"id":"1d6978af-9652-52c9-b630-a2ff5a34369a","node_type":"Paragraph","location":{"semantic":{"path":"1.13.41","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":108.0,"y":713.3,"width":329.8,"height":8.7}}},"text_order":164,"token_count":20,"style":null}
```

14
```bgraph-paragraph
{"id":"af8150a8-9bf8-5b38-b888-1dc3181ea18c","node_type":"Paragraph","location":{"semantic":{"path":"1.13.42","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":165,"token_count":1,"style":null}
```

Metropolitansky, D. and Larson, J. (2025). Towards effective extraction and evaluation of factual claims.
```bgraph-paragraph
{"id":"adeb6b71-8089-5ca4-84ae-72233aca8c61","node_type":"Paragraph","location":{"semantic":{"path":"1.13.43","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":19.59999}}},"text_order":166,"token_count":25,"style":null}
```

Microsoft (2023). The impact of large language models on scientific discovery: a preliminary study using gpt-4.
```bgraph-paragraph
{"id":"a843533c-e4d1-5719-88bb-7fd475fe9fa0","node_type":"Paragraph","location":{"semantic":{"path":"1.13.44","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":106.4,"width":396.0,"height":19.599998}}},"text_order":167,"token_count":27,"style":null}
```

Mooney, R. J. and Bunescu, R. (2005). Mining knowledge from text using information extraction. SIGKDD Explor. Newsl. , 7(1):3–10.
```bgraph-paragraph
{"id":"ec6178c3-6634-587f-a75c-2b936df29861","node_type":"Paragraph","location":{"semantic":{"path":"1.13.45","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":137.5,"width":396.0,"height":19.59999}}},"text_order":168,"token_count":31,"style":null}
```

NebulaGraph (2024). Nebulagraph launches industry-first graph rag: Retrieval-augmented genera- tion with llm based on knowledge graphs. https://www . nebula-graph . io/posts/graph-RAG .
```bgraph-paragraph
{"id":"7c7ca3cf-9280-5ff5-bbaf-51b2f734ed15","node_type":"Paragraph","location":{"semantic":{"path":"1.13.46","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":168.6,"width":396.0,"height":19.59999}}},"text_order":169,"token_count":45,"external_refs":[{"text":"","source_page":15,"source_bbox":{"x":287.2,"y":178.4,"width":190.6,"height":10.9},"target":{"kind":"uri","url":"https://www.nebula-graph.io/posts/graph-RAG"}},{"text":"","source_page":15,"source_bbox":{"x":287.2,"y":178.4,"width":190.6,"height":10.9},"target":{"kind":"uri","url":"https://www.nebula-graph.io/posts/graph-RAG"}},{"text":"","source_page":15,"source_bbox":{"x":287.2,"y":178.4,"width":190.6,"height":10.9},"target":{"kind":"uri","url":"https://www.nebula-graph.io/posts/graph-RAG"}},{"text":"","source_page":15,"source_bbox":{"x":287.2,"y":178.4,"width":190.6,"height":10.9},"target":{"kind":"uri","url":"https://www.nebula-graph.io/posts/graph-RAG"}},{"text":"","source_page":15,"source_bbox":{"x":287.2,"y":178.4,"width":190.6,"height":10.9},"target":{"kind":"uri","url":"https://www.nebula-graph.io/posts/graph-RAG"}}],"style":null}
```

Neo4J (2024). Get started with graphrag: Neo4j’s ecosystem tools. https://neo4j . com/developer- blog/graphrag-ecosystem-tools/ .
```bgraph-paragraph
{"id":"26b7e858-bd12-5490-abc2-b74eb1b6a50b","node_type":"Paragraph","location":{"semantic":{"path":"1.13.47","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":199.6,"width":395.5,"height":19.699997}}},"text_order":170,"token_count":31,"external_refs":[{"text":"","source_page":15,"source_bbox":{"x":388.0,"y":198.6,"width":117.0,"height":10.9},"target":{"kind":"uri","url":"https://neo4j.com/developer-blog/graphrag-ecosystem-tools/"}},{"text":"","source_page":15,"source_bbox":{"x":388.0,"y":198.6,"width":117.0,"height":10.9},"target":{"kind":"uri","url":"https://neo4j.com/developer-blog/graphrag-ecosystem-tools/"}},{"text":"","source_page":15,"source_bbox":{"x":388.0,"y":198.6,"width":117.0,"height":10.9},"target":{"kind":"uri","url":"https://neo4j.com/developer-blog/graphrag-ecosystem-tools/"}},{"text":"","source_page":15,"source_bbox":{"x":117.0,"y":209.5,"width":129.2,"height":10.9},"target":{"kind":"uri","url":"https://neo4j.com/developer-blog/graphrag-ecosystem-tools/"}}],"style":null}
```

Newman, M. E. (2006). Modularity and community structure in networks. Proceedings of the national academy of sciences , 103(23):8577–8582.
```bgraph-paragraph
{"id":"17a16494-0c49-5cff-be39-b071750be90b","node_type":"Paragraph","location":{"semantic":{"path":"1.13.48","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":230.8,"width":396.0,"height":19.59999}}},"text_order":171,"token_count":33,"style":null}
```

Ni, J., Shi, M., Stammbach, D., Sachan, M., Ash, E., and Leippold, M. (2024). AFaCTA: Assisting the annotation of factual claim detection with reliable LLM annotators. In Ku, L.-W., Martins, A., and Srikumar, V., editors, Proceedings of the 62nd Annual Meeting of the Association for Compu- tational Linguistics (Volume 1: Long Papers) , pages 1890–1912, Bangkok, Thailand. Association for Computational Linguistics.
```bgraph-paragraph
{"id":"ec404a2b-548c-5448-8a3e-67b8e9c65fdd","node_type":"Paragraph","location":{"semantic":{"path":"1.13.49","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":261.9,"width":396.0,"height":52.400024}}},"text_order":172,"token_count":100,"style":null}
```

OpenAI (2023). Chatgpt: Gpt-4 language model.
```bgraph-paragraph
{"id":"eabf4cf8-33c2-590f-9681-c53e43f19576","node_type":"Paragraph","location":{"semantic":{"path":"1.13.50","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":325.8,"width":196.2,"height":8.7}}},"text_order":173,"token_count":11,"style":null}
```

Padmakumar, V. and He, H. (2024). Does writing with language models reduce content diversity? ICLR .
```bgraph-paragraph
{"id":"1359f0fa-8c38-5f75-8cd2-823d316b7b39","node_type":"Paragraph","location":{"semantic":{"path":"1.13.51","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":346.0,"width":396.0,"height":19.600006}}},"text_order":174,"token_count":25,"style":null}
```

Pedregosa, F., Varoquaux, G., Gramfort, A., Michel, V., Thirion, B., Grisel, O., Blondel, M., Pret- tenhofer, P., Weiss, R., Dubourg, V., Vanderplas, J., Passos, A., Cournapeau, D., Brucher, M., Perrot, M., and Duchesnay, E. (2011). Scikit-learn: Machine learning in python. Journal of Machine Learning Research , 12:2825–2830.
```bgraph-paragraph
{"id":"5e7886db-20c4-5d5b-9a76-7e423f60445c","node_type":"Paragraph","location":{"semantic":{"path":"1.13.52","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":377.1,"width":396.0,"height":41.399994}}},"text_order":175,"token_count":78,"style":null}
```

Ram, O., Levine, Y., Dalmedigos, I., Muhlgay, D., Shashua, A., Leyton-Brown, K., and Shoham, Y. (2023). In-context retrieval-augmented language models. Transactions of the Association for Computational Linguistics , 11:1316–1331.
```bgraph-paragraph
{"id":"9961fe48-d2d9-509d-9747-802201c0d809","node_type":"Paragraph","location":{"semantic":{"path":"1.13.53","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":430.0,"width":396.0,"height":30.5}}},"text_order":176,"token_count":55,"style":null}
```

Ranade, P. and Joshi, A. (2023). Fabula: Intelligence report generation using retrieval-augmented narrative construction. arXiv preprint arXiv:2310.13848 .
```bgraph-paragraph
{"id":"17926a92-8e1d-5333-a47e-7632f88e570e","node_type":"Paragraph","location":{"semantic":{"path":"1.13.54","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":472.1,"width":396.0,"height":19.600006}}},"text_order":177,"token_count":37,"style":null}
```

Salminen, J., Liu, C., Pian, W., Chi, J., Häyhänen, E., and Jansen, B. J. (2024). Deus ex machina and personas from large language models: Investigating the composition of ai-generated persona descriptions. In Proceedings of the CHI Conference on Human Factors in Computing Systems , pages 1–20.
```bgraph-paragraph
{"id":"f3db0e6a-6b1d-5162-a6ca-639d321eda59","node_type":"Paragraph","location":{"semantic":{"path":"1.13.55","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":503.2,"width":396.0,"height":41.400024}}},"text_order":178,"token_count":72,"style":null}
```

Sarthi, P., Abdullah, S., Tuli, A., Khanna, S., Goldie, A., and Manning, C. D. (2024). Raptor: Recursive abstractive processing for tree-organized retrieval. arXiv preprint arXiv:2401.18059 .
```bgraph-paragraph
{"id":"4d65157d-fa49-556a-9e71-4ec7b0b5707d","node_type":"Paragraph","location":{"semantic":{"path":"1.13.56","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":556.1,"width":396.0,"height":19.600037}}},"text_order":179,"token_count":46,"style":null}
```

Scott, K. (2024). Behind the Tech. https://www . microsoft . com/en-us/behind-the-tech .
```bgraph-paragraph
{"id":"e8d8346a-5d0d-5fe4-ba99-11ac2a41a184","node_type":"Paragraph","location":{"semantic":{"path":"1.13.57","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":587.1,"width":345.3,"height":8.800049}}},"text_order":180,"token_count":21,"external_refs":[{"text":"","source_page":15,"source_bbox":{"x":248.5,"y":586.1,"width":203.3,"height":10.9},"target":{"kind":"uri","url":"https://www.microsoft.com/en-us/behind-the-tech"}},{"text":"","source_page":15,"source_bbox":{"x":248.5,"y":586.1,"width":203.3,"height":10.9},"target":{"kind":"uri","url":"https://www.microsoft.com/en-us/behind-the-tech"}},{"text":"","source_page":15,"source_bbox":{"x":248.5,"y":586.1,"width":203.3,"height":10.9},"target":{"kind":"uri","url":"https://www.microsoft.com/en-us/behind-the-tech"}},{"text":"","source_page":15,"source_bbox":{"x":248.5,"y":586.1,"width":203.3,"height":10.9},"target":{"kind":"uri","url":"https://www.microsoft.com/en-us/behind-the-tech"}},{"text":"","source_page":15,"source_bbox":{"x":248.5,"y":586.1,"width":203.3,"height":10.9},"target":{"kind":"uri","url":"https://www.microsoft.com/en-us/behind-the-tech"}}],"style":null}
```

Shao, Z., Gong, Y., Shen, Y., Huang, M., Duan, N., and Chen, W. (2023). Enhancing retrieval- augmented large language models with iterative retrieval-generation synergy. arXiv preprint arXiv:2305.15294 .
```bgraph-paragraph
{"id":"34b2919e-7bfd-5b04-af52-4a3907240ea5","node_type":"Paragraph","location":{"semantic":{"path":"1.13.58","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":607.4,"width":396.0,"height":30.5}}},"text_order":181,"token_count":50,"style":null}
```

Shin, J., Hedderich, M. A., Rey, B. J., Lucero, A., and Oulasvirta, A. (2024). Understanding human- ai workflows for generating personas. In Proceedings of the 2024 ACM Designing Interactive Systems Conference , pages 757–781.
```bgraph-paragraph
{"id":"96d1c2a3-49ca-55d8-962c-22a698697e19","node_type":"Paragraph","location":{"semantic":{"path":"1.13.59","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":649.4,"width":396.0,"height":30.599976}}},"text_order":182,"token_count":54,"style":null}
```

Shinn, N., Cassano, F., Gopinath, A., Narasimhan, K., and Yao, S. (2024). Reflexion: Language agents with verbal reinforcement learning. Advances in Neural Information Processing Systems , 36.
```bgraph-paragraph
{"id":"725811ee-d777-520a-bb63-4b27a11a23d3","node_type":"Paragraph","location":{"semantic":{"path":"1.13.60","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":108.0,"y":691.5,"width":396.0,"height":30.5}}},"text_order":183,"token_count":47,"style":null}
```

15
```bgraph-paragraph
{"id":"94d73003-8535-5313-8137-f2c16ed6335a","node_type":"Paragraph","location":{"semantic":{"path":"1.13.61","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":184,"token_count":1,"style":null}
```

Su, D., Xu, Y., Yu, T., Siddique, F. B., Barezi, E. J., and Fung, P. (2020). Caire-covid: A ques- tion answering and query-focused multi-document summarization system for covid-19 scholarly information management. arXiv preprint arXiv:2005.03975 .
```bgraph-paragraph
{"id":"07e47a32-9d9c-53a0-9279-10c3e778d518","node_type":"Paragraph","location":{"semantic":{"path":"1.13.62","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":30.499992}}},"text_order":185,"token_count":59,"style":null}
```

Tan, Z., Zhao, X., and Wang, W. (2017). Representation learning of large-scale knowledge graphs via entity feature combinations. In Proceedings of the 2017 ACM on Conference on Information and Knowledge Management , CIKM ’17, page 1777–1786, New York, NY, USA. Association for Computing Machinery.
```bgraph-paragraph
{"id":"edb7a0c7-19df-571c-93dd-5b3816c66631","node_type":"Paragraph","location":{"semantic":{"path":"1.13.63","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":116.0,"width":396.0,"height":41.399994}}},"text_order":186,"token_count":72,"style":null}
```

Tang, Y. and Yang, Y. (2024). MultiHop-RAG: Benchmarking retrieval-augmented generation for multi-hop queries. arXiv preprint arXiv:2401.15391 .
```bgraph-paragraph
{"id":"295e912d-d6eb-5a79-9675-dc2b6b3202a4","node_type":"Paragraph","location":{"semantic":{"path":"1.13.64","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":167.6,"width":396.0,"height":19.59999}}},"text_order":187,"token_count":34,"style":null}
```

Touvron, H., Martin, L., Stone, K., Albert, P., Almahairi, A., Babaei, Y., Bashlykov, N., Batra, S., Bhargava, P., Bhosale, S., et al. (2023). Llama 2: Open foundation and fine-tuned chat models. arXiv preprint arXiv:2307.09288 .
```bgraph-paragraph
{"id":"501c4971-f1e2-596e-98c8-e30986a6d471","node_type":"Paragraph","location":{"semantic":{"path":"1.13.65","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":197.5,"width":396.0,"height":30.5}}},"text_order":188,"token_count":56,"style":null}
```

Traag, V. A., Waltman, L., and Van Eck, N. J. (2019). From Louvain to Leiden: guaranteeing well-connected communities. Scientific Reports , 9(1).
```bgraph-paragraph
{"id":"a065f427-72f4-51a6-a292-639e67d1fd8c","node_type":"Paragraph","location":{"semantic":{"path":"1.13.66","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":238.2,"width":396.0,"height":19.600021}}},"text_order":189,"token_count":33,"style":null}
```

Trajanoska, M., Stojanov, R., and Trajanov, D. (2023). Enhancing knowledge graph construction using large language models. ArXiv , abs/2305.04676.
```bgraph-paragraph
{"id":"1a7d260a-0837-54f0-aec2-2b7f9b990768","node_type":"Paragraph","location":{"semantic":{"path":"1.13.67","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":268.0,"width":396.0,"height":19.700012}}},"text_order":190,"token_count":35,"style":null}
```

Trivedi, H., Balasubramanian, N., Khot, T., and Sabharwal, A. (2022). Interleaving retrieval with chain-of-thought reasoning for knowledge-intensive multi-step questions. arXiv preprint arXiv:2212.10509 .
```bgraph-paragraph
{"id":"02e59f07-f86e-5716-b75d-54729e788309","node_type":"Paragraph","location":{"semantic":{"path":"1.13.68","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":297.9,"width":396.0,"height":30.50003}}},"text_order":191,"token_count":50,"style":null}
```

Wang, J., Liang, Y., Meng, F., Sun, Z., Shi, H., Li, Z., Xu, J., Qu, J., and Zhou, J. (2023a). Is chatgpt a good nlg evaluator? a preliminary study. arXiv preprint arXiv:2303.04048 .
```bgraph-paragraph
{"id":"edb8f560-c1cf-5687-a428-da726c03be9d","node_type":"Paragraph","location":{"semantic":{"path":"1.13.69","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":338.6,"width":396.0,"height":19.600006}}},"text_order":192,"token_count":44,"style":null}
```

Wang, S., Khramtsova, E., Zhuang, S., and Zuccon, G. (2024). Feb4rag: Evaluating federated search in the context of retrieval augmented generation. arXiv preprint arXiv:2402.11891 .
```bgraph-paragraph
{"id":"9355589a-a7ea-5e18-a671-2becb27381ba","node_type":"Paragraph","location":{"semantic":{"path":"1.13.70","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":368.4,"width":396.0,"height":19.700012}}},"text_order":193,"token_count":44,"style":null}
```

Wang, X., Wei, J., Schuurmans, D., Le, Q., Chi, E., Narang, S., Chowdhery, A., and Zhou, D. (2022). Self-consistency improves chain of thought reasoning in language models. arXiv preprint arXiv:2203.11171 .
```bgraph-paragraph
{"id":"3bca9ff4-b299-5d58-b056-6c6afcb2ef49","node_type":"Paragraph","location":{"semantic":{"path":"1.13.71","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":398.3,"width":396.0,"height":30.50003}}},"text_order":194,"token_count":50,"style":null}
```

Wang, Y., Lipka, N., Rossi, R. A., Siu, A., Zhang, R., and Derr, T. (2023b). Knowledge graph prompting for multi-document question answering.
```bgraph-paragraph
{"id":"b32a2d2d-4b70-548d-a3e2-f84f7162b784","node_type":"Paragraph","location":{"semantic":{"path":"1.13.72","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":439.0,"width":396.0,"height":19.600006}}},"text_order":195,"token_count":35,"style":null}
```

Xu, Y. and Lapata, M. (2021). Text summarization with latent queries. arXiv preprint arXiv:2106.00104 .
```bgraph-paragraph
{"id":"ae1a56f8-8da2-55ef-adec-fae4a672b012","node_type":"Paragraph","location":{"semantic":{"path":"1.13.73","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":468.8,"width":396.0,"height":19.700012}}},"text_order":196,"token_count":24,"style":null}
```

Yang, Z., Qi, P., Zhang, S., Bengio, Y., Cohen, W. W., Salakhutdinov, R., and Manning, C. D. (2018). HotpotQA: A dataset for diverse, explainable multi-hop question answering. In Conference on Empirical Methods in Natural Language Processing (EMNLP) .
```bgraph-paragraph
{"id":"42178868-9410-52f1-8567-b4e2cb10155c","node_type":"Paragraph","location":{"semantic":{"path":"1.13.74","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":498.7,"width":396.0,"height":30.5}}},"text_order":197,"token_count":62,"style":null}
```

Yao, J.-g., Wan, X., and Xiao, J. (2017). Recent advances in document summarization. Knowledge and Information Systems , 53:297–336.
```bgraph-paragraph
{"id":"2ae145f0-a87d-588f-9f25-ec0a2322685d","node_type":"Paragraph","location":{"semantic":{"path":"1.13.75","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":539.4,"width":396.0,"height":19.599976}}},"text_order":198,"token_count":31,"style":null}
```

Yao, L., Peng, J., Mao, C., and Luo, Y. (2023). Exploring large language models for knowledge graph completion.
```bgraph-paragraph
{"id":"4925631c-3376-556b-9f68-c38ce6d6bdb0","node_type":"Paragraph","location":{"semantic":{"path":"1.13.76","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":569.2,"width":396.0,"height":19.700012}}},"text_order":199,"token_count":27,"style":null}
```

Yates, A., Banko, M., Broadhead, M., Cafarella, M., Etzioni, O., and Soderland, S. (2007). Tex- tRunner: Open information extraction on the web. In Carpenter, B., Stent, A., and Williams, J. D., editors, Proceedings of Human Language Technologies: The Annual Conference of the North American Chapter of the Association for Computational Linguistics (NAACL-HLT) , pages 25–26, Rochester, New York, USA. Association for Computational Linguistics.
```bgraph-paragraph
{"id":"30c9dd16-f312-55ed-ad55-7079b7206a8a","node_type":"Paragraph","location":{"semantic":{"path":"1.13.77","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":599.1,"width":396.0,"height":52.30005}}},"text_order":200,"token_count":106,"style":null}
```

Yuan, X., Li, J., Wang, D., Chen, Y., Mao, X., Huang, L., Xue, H., Wang, W., Ren, K., and Wang, J. (2024). S-eval: Automatic and adaptive test generation for benchmarking safety evaluation of large language models. arXiv preprint arXiv:2405.14191 .
```bgraph-paragraph
{"id":"3b6b671e-7156-5571-b039-25c17ee0ec59","node_type":"Paragraph","location":{"semantic":{"path":"1.13.78","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":661.6,"width":396.0,"height":30.600037}}},"text_order":201,"token_count":59,"style":null}
```

Zhang, J. (2023). Graph-toolformer: To empower llms with graph reasoning ability via prompt augmented by chatgpt. arXiv preprint arXiv:2304.11116 .
```bgraph-paragraph
{"id":"04d55b4d-39c3-546a-9fbd-d394f1613651","node_type":"Paragraph","location":{"semantic":{"path":"1.13.79","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":108.0,"y":702.4,"width":396.0,"height":19.599976}}},"text_order":202,"token_count":35,"style":null}
```

16
```bgraph-paragraph
{"id":"a72bc612-1b08-5687-98d2-67780f7975da","node_type":"Paragraph","location":{"semantic":{"path":"1.13.80","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":203,"token_count":1,"style":null}
```

Zhang, Y., Zhang, Y., Gan, Y., Yao, L., and Wang, C. (2024a). Causal graph discovery with retrieval- augmented generation based large language models. arXiv preprint arXiv:2402.15301 .
```bgraph-paragraph
{"id":"3c8c0e74-2c68-501b-a49a-29bf89fda8b2","node_type":"Paragraph","location":{"semantic":{"path":"1.13.81","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":17,"bounding_box":{"x":108.0,"y":75.3,"width":396.0,"height":19.59999}}},"text_order":204,"token_count":45,"style":null}
```

Zhang, Z., Chen, J., and Yang, D. (2024b). Darg: Dynamic evaluation of large language models via adaptive reasoning graph. arXiv preprint arXiv:2406.17271 .
```bgraph-paragraph
{"id":"ff860e4d-bc9b-5345-b38e-05441f801027","node_type":"Paragraph","location":{"semantic":{"path":"1.13.82","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":17,"bounding_box":{"x":108.0,"y":105.0,"width":396.0,"height":19.599998}}},"text_order":205,"token_count":38,"style":null}
```

Zheng, L., Chiang, W.-L., Sheng, Y., Zhuang, S., Wu, Z., Zhuang, Y., Lin, Z., Li, Z., Li, D., Xing, E., et al. (2024). Judging llm-as-a-judge with mt-bench and chatbot arena. Advances in Neural Information Processing Systems , 36.
```bgraph-paragraph
{"id":"58a8c8db-9faf-50ad-a129-32befc07691f","node_type":"Paragraph","location":{"semantic":{"path":"1.13.83","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":17,"bounding_box":{"x":108.0,"y":134.8,"width":396.0,"height":30.5}}},"text_order":206,"token_count":54,"style":null}
```

Zhu, Y., Wang, X., Chen, J., Qiao, S., Ou, Y., Yao, Y., Deng, S., Chen, H., and Zhang, N. (2024). Llms for knowledge graph construction and reasoning: Recent capabilities and future opportuni- ties.
```bgraph-paragraph
{"id":"405f42b4-9f69-5fb6-a9dd-6ed1bfeed103","node_type":"Paragraph","location":{"semantic":{"path":"1.13.84","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":17,"bounding_box":{"x":108.0,"y":175.5,"width":396.0,"height":30.5}}},"text_order":207,"token_count":48,"style":null}
```

17
```bgraph-paragraph
{"id":"65d7881a-cfeb-5df9-9324-34f7bd310b39","node_type":"Paragraph","location":{"semantic":{"path":"1.13.85","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":17,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":208,"token_count":1,"style":null}
```

## A Entity and Relationship Extraction Approach
```bgraph-section
{"id":"c8545a5e-a557-5bbd-93d0-ff34cd79a513","node_type":"Section","location":{"semantic":{"path":"1.14","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":73.6,"width":253.2,"height":10.6}}},"text_order":209,"token_count":11,"style":null}
```

The following prompts, designed for GPT-4, are used in the default GraphRAG initialization pipeline:
```bgraph-paragraph
{"id":"374c26b2-736f-5858-8b20-6efcdf527091","node_type":"Paragraph","location":{"semantic":{"path":"1.14.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":98.4,"width":396.0,"height":19.599998}}},"text_order":210,"token_count":24,"style":null}
```

• Default Graph Extraction Prompt
```bgraph-paragraph
{"id":"64160d6e-3755-5680-bb61-c57c05c89015","node_type":"Paragraph","location":{"semantic":{"path":"1.14.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach"]},"physical":{"page":18,"bounding_box":{"x":135.4,"y":129.6,"width":141.5,"height":8.699997}}},"text_order":211,"token_count":8,"external_refs":[{"text":"","source_page":18,"source_bbox":{"x":142.9,"y":128.6,"width":135.0,"height":10.9},"target":{"kind":"uri","url":"https://github.com/microsoft/graphrag/blob/6d21ef268377e319a165ca2250bd6841737df1ad/graphrag/prompts/index/entity_extraction.py#L6"}}],"style":null}
```

• Claim Extraction Prompt
```bgraph-paragraph
{"id":"cd4cdff7-31d3-5035-a1de-69a1fc6b86ae","node_type":"Paragraph","location":{"semantic":{"path":"1.14.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach"]},"physical":{"page":18,"bounding_box":{"x":135.4,"y":144.5,"width":108.7,"height":8.699997}}},"text_order":212,"token_count":6,"external_refs":[{"text":"","source_page":18,"source_bbox":{"x":142.9,"y":143.5,"width":102.2,"height":10.9},"target":{"kind":"uri","url":"https://github.com/microsoft/graphrag/blob/6d21ef268377e319a165ca2250bd6841737df1ad/graphrag/prompts/index/claim_extraction.py#L1"}}],"style":null}
```

### A.1 Entity Extraction
```bgraph-section
{"id":"365c6f14-efcd-54a6-a98f-d719a7e75ae3","node_type":"Section","location":{"semantic":{"path":"1.14.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.1 Entity Extraction"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":168.9,"width":99.1,"height":8.7}}},"text_order":213,"token_count":5,"style":null}
```

We do this using a multipart LLM prompt that first identifies all entities in the text, including their name, type, and description, before identifying all relationships between clearly related entities, including the source and target entities and a description of their relationship. Both kinds of element instance are output in a single list of delimited tuples.
```bgraph-paragraph
{"id":"424c424e-4c47-5595-9f55-310ad3cf9247","node_type":"Paragraph","location":{"semantic":{"path":"1.14.4.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.1 Entity Extraction"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":189.0,"width":396.0,"height":41.399994}}},"text_order":214,"token_count":87,"style":null}
```

### A.2 Self-Reflection
```bgraph-section
{"id":"c6c45038-eb3c-536d-93f0-5fefedb7e6b0","node_type":"Section","location":{"semantic":{"path":"1.14.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":246.0,"width":86.6,"height":8.7}}},"text_order":215,"token_count":4,"style":null}
```

The choice of prompt engineering techniques has a strong impact on the quality of knowledge graph extraction ( Zhu et al. , 2024 ), and different techniques have different costs in terms of tokens con- sumed and generated by the model. Self-reflection is a prompt engineering technique where the LLM generates an answer, and is then prompted to evaluate its output for correctness, clarity, or completeness, then finally generate an improved response based on that evaluation ( Huang et al. , 2023 ; Madaan et al. , 2024 ; Shinn et al. , 2024 ; Wang et al. , 2022 ). We leverage self-reflection in knowledge graph extraction, and explore ways how removing self-reflection affects performance and cost.
```bgraph-paragraph
{"id":"ef1cca62-4f31-588d-9252-2cc821bc6a66","node_type":"Paragraph","location":{"semantic":{"path":"1.14.5.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":266.1,"width":396.0,"height":85.100006}}},"text_order":216,"token_count":169,"internal_refs":[{"text":"","source_page":18,"source_bbox":{"x":155.8,"y":276.0,"width":40.7,"height":10.9},"target":{"kind":"named","name":"cite.zhu2024llms","page":16,"point":{"x":83.0,"y":172.0}}},{"text":"","source_page":18,"source_bbox":{"x":199.9,"y":276.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.zhu2024llms","page":16,"point":{"x":83.0,"y":172.0}}},{"text":"","source_page":18,"source_bbox":{"x":450.4,"y":308.7,"width":52.1,"height":10.9},"target":{"kind":"named","name":"cite.huang2023large","page":12,"point":{"x":83.0,"y":699.0}}},{"text":"","source_page":18,"source_bbox":{"x":107.0,"y":319.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.huang2023large","page":12,"point":{"x":83.0,"y":699.0}}},{"text":"","source_page":18,"source_bbox":{"x":133.1,"y":319.6,"width":57.7,"height":10.9},"target":{"kind":"named","name":"cite.madaan2024self","page":13,"point":{"x":83.0,"y":576.0}}},{"text":"","source_page":18,"source_bbox":{"x":194.6,"y":319.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.madaan2024self","page":13,"point":{"x":83.0,"y":576.0}}},{"text":"","source_page":18,"source_bbox":{"x":220.7,"y":319.6,"width":48.9,"height":10.9},"target":{"kind":"named","name":"cite.shinn2024reflexion","page":14,"point":{"x":83.0,"y":688.0}}},{"text":"","source_page":18,"source_bbox":{"x":273.4,"y":319.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.shinn2024reflexion","page":14,"point":{"x":83.0,"y":688.0}}},{"text":"","source_page":18,"source_bbox":{"x":299.5,"y":319.6,"width":48.6,"height":10.9},"target":{"kind":"named","name":"cite.wang2022self","page":15,"point":{"x":83.0,"y":395.0}}},{"text":"","source_page":18,"source_bbox":{"x":352.0,"y":319.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.wang2022self","page":15,"point":{"x":83.0,"y":395.0}}}],"style":null}
```

Using larger chunk size is less costly in terms of calls to the LLM. However, the LLM tends to extract few entities from chunks of larger size. For example, in a sample dataset (HotPotQA, Yang et al. , 2018 ), GPT-4 extracted almost twice as many entity references when the chunk size was 600 tokens than when it was 2400. To address this issue, we deploy a self-reflection prompt engineering approach. After entities are extracted from a chunk, we provide the extracted entities back to the LLM, prompting it to “glean” any entities that it may have missed. This is a multi-stage process in which we first ask the LLM to assess whether all entities were extracted, using a logit bias of 100 to force a yes/no decision. If the LLM responds that entities were missed, then a continuation indicating that “MANY entities were missed in the last extraction” encourages the LLM to detect these missing entities. This approach allows us to use larger chunk sizes without a drop in quality ( Figure 3 ) or the forced introduction of noise. We interate self-reflection steps up to a specified maximum number of times.
```bgraph-paragraph
{"id":"74efef7b-97a2-5303-8fc4-4b64aabb414d","node_type":"Paragraph","location":{"semantic":{"path":"1.14.5.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":358.9,"width":396.0,"height":128.70001}}},"text_order":217,"token_count":271,"internal_refs":[{"text":"","source_page":18,"source_bbox":{"x":482.4,"y":368.7,"width":22.6,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}},{"text":"","source_page":18,"source_bbox":{"x":107.0,"y":379.6,"width":21.5,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}},{"text":"","source_page":18,"source_bbox":{"x":131.7,"y":379.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}},{"text":"","source_page":18,"source_bbox":{"x":110.3,"y":466.9,"width":36.6,"height":10.9},"target":{"kind":"named","name":"figure.3","page":17,"point":{"x":168.0,"y":641.0}}}],"style":null}
```

d 30000 e t c 600 chunk size e t e 1200 chunk size d s 20000 2400 chunk size e c n e r e f e 10000 r y t i t n E 0 0 1 2 3
```bgraph-paragraph
{"id":"957eceb1-3323-5eac-be27-44a7d97c99e8","node_type":"Paragraph","location":{"semantic":{"path":"1.14.5.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":118.1,"y":500.9,"width":362.6,"height":114.99997}}},"text_order":218,"token_count":38,"style":null}
```

Number of self-reflection iterations performed
```bgraph-paragraph
{"id":"9aa0d6e5-ee77-5a4d-a587-c62f75bc01d2","node_type":"Paragraph","location":{"semantic":{"path":"1.14.5.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":239.1,"y":621.1,"width":184.8,"height":8.7}}},"text_order":219,"token_count":11,"style":null}
```

Figure 3: How the entity references detected in the HotPotQA dataset ( Yang et al. , 2018 ) varies with chunk size and self-reflection iterations for our generic entity extraction prompt with gpt-4-turbo .
```bgraph-paragraph
{"id":"91578238-0c54-57e3-94a9-d25ffe2be3e7","node_type":"Paragraph","location":{"semantic":{"path":"1.14.5.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":114.3,"y":644.4,"width":383.40002,"height":30.5}}},"text_order":220,"token_count":49,"internal_refs":[{"text":"","source_page":18,"source_bbox":{"x":411.4,"y":643.3,"width":44.4,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}},{"text":"","source_page":18,"source_bbox":{"x":458.8,"y":643.3,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.yang2018hotpotqa","page":15,"point":{"x":83.0,"y":495.0}}}],"style":null}
```

## B Example Community Detection
```bgraph-section
{"id":"6f7632aa-ab4d-5965-9187-35378e1ad334","node_type":"Section","location":{"semantic":{"path":"1.15","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":18,"bounding_box":{"x":108.0,"y":701.6,"width":180.6,"height":10.6}}},"text_order":221,"token_count":7,"style":null}
```

18
```bgraph-paragraph
{"id":"022c299d-e729-53f5-b8b0-622b60f0d24d","node_type":"Paragraph","location":{"semantic":{"path":"1.15.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":18,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":222,"token_count":1,"style":null}
```

(a) Root communities at level 0 (b) Sub-communities at level 1
```bgraph-paragraph
{"id":"8f63edf2-c27e-51bf-9b4a-4c5253247c50","node_type":"Paragraph","location":{"semantic":{"path":"1.15.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":19,"bounding_box":{"x":150.2,"y":283.1,"width":334.2,"height":8.700012}}},"text_order":223,"token_count":14,"style":null}
```

Figure 4: Graph communities detected using the Leiden algorithm ( Traag et al. , 2019 ) over the MultiHop-RAG ( Tang and Yang , 2024 ) dataset as indexed. Circles represent entity nodes with size proportional to their degree. Node layout was performed via OpenORD ( Martin et al. , 2011 ) and Force Atlas 2 ( Jacomy et al. , 2014 ). Node colors represent entity communities, shown at two levels of hierarchical clustering: (a) Level 0, corresponding to the hierarchical partition with maximum modularity, and (b) Level 1, which reveals internal structure within these root-level communities.
```bgraph-paragraph
{"id":"6fa22b1e-3276-50f1-b3cb-1dca7a3ce8b4","node_type":"Paragraph","location":{"semantic":{"path":"1.15.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":19,"bounding_box":{"x":108.0,"y":301.1,"width":396.0,"height":63.30002}}},"text_order":224,"token_count":141,"internal_refs":[{"text":"","source_page":19,"source_bbox":{"x":389.3,"y":300.1,"width":49.2,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}},{"text":"","source_page":19,"source_bbox":{"x":442.7,"y":300.1,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.Traag2019Leiden","page":15,"point":{"x":83.0,"y":235.0}}},{"text":"","source_page":19,"source_bbox":{"x":176.2,"y":311.0,"width":62.0,"height":10.9},"target":{"kind":"named","name":"cite.tang2024multihop","page":15,"point":{"x":83.0,"y":164.0}}},{"text":"","source_page":19,"source_bbox":{"x":241.3,"y":311.0,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.tang2024multihop","page":15,"point":{"x":83.0,"y":164.0}}},{"text":"","source_page":19,"source_bbox":{"x":406.2,"y":321.9,"width":52.3,"height":10.9},"target":{"kind":"named","name":"cite.martinbrown","page":13,"point":{"x":83.0,"y":679.0}}},{"text":"","source_page":19,"source_bbox":{"x":462.2,"y":321.9,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.martinbrown","page":13,"point":{"x":83.0,"y":679.0}}},{"text":"","source_page":19,"source_bbox":{"x":166.5,"y":332.8,"width":54.4,"height":10.9},"target":{"kind":"named","name":"cite.fa2","page":13,"point":{"x":83.0,"y":72.0}}},{"text":"","source_page":19,"source_bbox":{"x":223.9,"y":332.8,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.fa2","page":13,"point":{"x":83.0,"y":72.0}}}],"style":null}
```

## C Context Window Selection
```bgraph-section
{"id":"6b429c8f-60f3-5065-997f-dc3fe98974f9","node_type":"Section","location":{"semantic":{"path":"1.16","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","C Context Window Selection"]},"physical":{"page":19,"bounding_box":{"x":108.0,"y":386.1,"width":155.7,"height":10.6}}},"text_order":225,"token_count":6,"style":null}
```

The effect of context window size on any particular task is unclear, especially for models like gpt-4-turbo with a large context size of 128k tokens. Given the potential for information to be “lost in the middle” of longer contexts ( Kuratov et al. , 2024 ; Liu et al. , 2023 ), we wanted to ex- plore the effects of varying the context window size for our combinations of datasets, questions, and metrics. In particular, our goal was to determine the optimum context size for our baseline condition ( SS ) and then use this uniformly for all query-time LLM use. To that end, we tested four context window sizes: 8k, 16k, 32k and 64k. Surprisingly, the smallest context window size tested (8k) was universally better for all comparisons on comprehensiveness (average win rate of 58.1%), while performing comparably with larger context sizes on diversity (average win rate = 52.4%), and em- powerment (average win rate = 51.3%). Given our preference for more comprehensive and diverse answers, we therefore used a fixed context window size of 8k tokens for the final evaluation.
```bgraph-paragraph
{"id":"f4812641-ec7d-59de-8bf3-013b135b1292","node_type":"Paragraph","location":{"semantic":{"path":"1.16.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","C Context Window Selection"]},"physical":{"page":19,"bounding_box":{"x":108.0,"y":410.8,"width":396.0,"height":117.80005}}},"text_order":226,"token_count":263,"internal_refs":[{"text":"","source_page":19,"source_bbox":{"x":279.1,"y":431.6,"width":56.6,"height":10.9},"target":{"kind":"named","name":"cite.kuratov2024search","page":13,"point":{"x":83.0,"y":371.0}}},{"text":"","source_page":19,"source_bbox":{"x":339.0,"y":431.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.kuratov2024search","page":13,"point":{"x":83.0,"y":371.0}}},{"text":"","source_page":19,"source_bbox":{"x":364.4,"y":431.6,"width":38.1,"height":10.9},"target":{"kind":"named","name":"cite.liu-etal:2023:tacl","page":13,"point":{"x":83.0,"y":515.0}}},{"text":"","source_page":19,"source_bbox":{"x":405.7,"y":431.6,"width":21.9,"height":10.9},"target":{"kind":"named","name":"cite.liu-etal:2023:tacl","page":13,"point":{"x":83.0,"y":515.0}}}],"style":null}
```

19
```bgraph-paragraph
{"id":"84d01209-6e3d-5318-be29-c0a625953d27","node_type":"Paragraph","location":{"semantic":{"path":"1.16.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","C Context Window Selection"]},"physical":{"page":19,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":227,"token_count":1,"style":null}
```

## D Example Answer Comparison
```bgraph-section
{"id":"e3a5bd04-f314-5ba6-84ca-f26d300e4112","node_type":"Section","location":{"semantic":{"path":"1.17","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":108.0,"y":73.6,"width":174.0,"height":10.6}}},"text_order":228,"token_count":6,"style":null}
```

Table 5: Example question, answers, and LLM-generated assessments for the News article dataset.
```bgraph-paragraph
{"id":"e5e0f6eb-0a89-5172-9ce4-1f1f325a29eb","node_type":"Paragraph","location":{"semantic":{"path":"1.17.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":109.4,"y":103.2,"width":393.3,"height":8.7}}},"text_order":229,"token_count":23,"style":null}
```

Question Which public figures are repeatedly mentioned across various entertainment articles? Graph The entertainment industry is vast and diverse, encompassing film, television, music, sports, and RAG digital media. Certain public figures stand out due to their significant contributions and influence across these sectors. The following summary highlights key individuals who are repeatedly mentioned in various entertainment articles, reflecting their impact and presence within the industry. Actors and Directors [...] Public Figures in Controversy [...] Musicians and Executives [...] Athletes and Coaches [...] Influencers and Entrepreneurs [...] The repeated mention of these figures in entertainment articles signifies their ongoing relevance and the public’s interest in their work. Their influence spans across various aspects of entertainment, from shaping cultural narratives in film and television to driving trends in music and digital media. These individuals not only contribute to their respective fields but also influence the broader cultural landscape, often becoming central figures in social discussions and public discourse.
```bgraph-paragraph
{"id":"8e28dd00-0fe7-521b-8a6f-5fe92b532c6d","node_type":"Paragraph","location":{"semantic":{"path":"1.17.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":114.0,"y":123.0,"width":407.90002,"height":120.3}}},"text_order":230,"token_count":277,"style":null}
```

Naı̈ve Public figures who are repeatedly mentioned across various entertainment articles include Taylor RAG Swift, Travis Kelce, Britney Spears, and Justin Timberlake. These individuals have been highlighted for various reasons, ranging from their professional achievements to their personal lives. Taylor Swift [...] Travis Kelce [...] Britney Spears [...] Justin Timberlake [...] These figures are frequently covered due to their high-profile status and the public’s interest in their careers and personal lives. Their activities, whether in music, sports, or personal relationships, have significant cultural and economic impacts, as evidenced by the media coverage and public reactions.
```bgraph-paragraph
{"id":"d97d65fa-1d36-59cc-ad06-d0b991034587","node_type":"Paragraph","location":{"semantic":{"path":"1.17.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":114.0,"y":254.0,"width":403.40002,"height":67.600006}}},"text_order":231,"token_count":165,"style":null}
```

LLM Comprehensiveness: Winner=1 (Graph RAG) . Answer 1 is better because it provides a more Decision comprehensive and detailed list of public figures from a wider range of entertainment sectors, including film, television, music, sports, gaming, and digital media. It also includes examples of their contributions and the impact they have on their respective fields, as well as mentions of controversies and their implications. Answer 2, while detailed in its coverage of a few individuals, is limited to a smaller number of public figures and focuses primarily on their personal lives and relationships rather than a broad spectrum of their professional influence across the entertainment industry.
```bgraph-paragraph
{"id":"6663ffd0-fd36-5be9-af24-7ce4c33d24d3","node_type":"Paragraph","location":{"semantic":{"path":"1.17.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":114.0,"y":332.3,"width":407.90002,"height":67.60004}}},"text_order":232,"token_count":169,"style":null}
```

Diversity: Winner=1 (Graph RAG) . Answer 1 is better because it provides a more varied and rich response by covering a wide range of public figures from different sectors of the entertainment industry, including film, television, music, sports, gaming, and digital media. It offers insights into the contributions and influence of these figures, as well as controversies and their impact on public discourse. The answer also cites specific data sources for each mentioned figure, indicating a diverse range of evidence to support the claims. In contrast, Answer 2 focuses on a smaller group of public figures, primarily from the music industry and sports, and relies heavily on a single source for data, which makes it less diverse in perspectives and insights.
```bgraph-paragraph
{"id":"1f05e527-3db4-5340-85d9-6f0c3c1f3108","node_type":"Paragraph","location":{"semantic":{"path":"1.17.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":157.6,"y":407.7,"width":360.70004,"height":77.600006}}},"text_order":233,"token_count":183,"style":null}
```

Empowerment: Winner=1 (Graph RAG) . Answer 1 is better because it provides a comprehensive and structured overview of public figures across various sectors of the entertainment industry, including film, television, music, sports, and digital media. It lists multiple individuals, providing specific examples of their contributions and the context in which they are mentioned in entertainment articles, along with references to data reports for each claim. This approach helps the reader understand the breadth of the topic and make informed judgments without being misled. In contrast, Answer 2 focuses on a smaller group of public figures and primarily discusses their personal lives and relationships, which may not provide as broad an understanding of the topic. While Answer 2 also cites sources, it does not match the depth and variety of Answer 1.
```bgraph-paragraph
{"id":"9e6016be-6fd8-5e6d-aa2b-b740bb371bb1","node_type":"Paragraph","location":{"semantic":{"path":"1.17.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":157.6,"y":493.1,"width":364.30002,"height":87.50003}}},"text_order":234,"token_count":207,"style":null}
```

Directness: Winner=2 (Naı̈ve RAG) . Answer 2 is better because it directly lists specific public figures who are repeatedly mentioned across various entertainment articles, such as Taylor Swift, Travis Kelce, Britney Spears, and Justin Timberlake, and provides concise explanations for their frequent mentions. Answer 1, while comprehensive, includes a lot of detailed information about various figures in different sectors of entertainment, which, while informative, does not directly answer the question with the same level of conciseness and specificity as Answer 2.
```bgraph-paragraph
{"id":"67602b0b-4c44-5350-b06c-6fa3e6b5caee","node_type":"Paragraph","location":{"semantic":{"path":"1.17.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":157.6,"y":588.4,"width":347.7,"height":57.700012}}},"text_order":235,"token_count":138,"style":null}
```

20
```bgraph-paragraph
{"id":"7218cb6a-10b0-5be1-aa28-6c0526541b51","node_type":"Paragraph","location":{"semantic":{"path":"1.17.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":236,"token_count":1,"style":null}
```

## E System Prompts
```bgraph-section
{"id":"46178fd9-f5b5-5521-9bf9-e0088e460226","node_type":"Section","location":{"semantic":{"path":"1.18","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":73.6,"width":103.1,"height":10.6}}},"text_order":237,"token_count":4,"style":null}
```

### E.1 Element Instance Generation
```bgraph-section
{"id":"0ab7d208-ffb4-53fc-b3b8-e319b3d491a1","node_type":"Section","location":{"semantic":{"path":"1.18.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":98.2,"width":148.6,"height":8.7}}},"text_order":238,"token_count":7,"style":null}
```

---Goal--- Given a text document that is potentially relevant to this activity and a list of entity types, identify all entities of those types from the text and all relationships among the identified entities.
```bgraph-paragraph
{"id":"3546ca0a-4a82-52b4-b609-90343688592a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":118.1,"width":385.3,"height":21.299995}}},"text_order":239,"token_count":51,"style":null}
```

---Steps--- 1. Identify all entities. For each identified entity, extract the following information: - entity name: Name of the entity, capitalized - entity type: One of the following types: [ { entity types } ] - entity description: Comprehensive description of the entity’s attributes and activities
```bgraph-paragraph
{"id":"c47484ce-eb86-5fc9-b44b-cf41d39c3384","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":150.0,"width":333.4,"height":37.199997}}},"text_order":240,"token_count":74,"style":null}
```

Format each entity as ("entity" { tuple delimiter } <entity name> { tuple delimiter } <entity type> { tuple delimiter } <entity description>
```bgraph-paragraph
{"id":"20011ce7-74db-5538-8293-c1720a66a525","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":196.9,"width":357.1,"height":14.300003}}},"text_order":241,"token_count":33,"style":null}
```

2. From the entities identified in step 1, identify all pairs of (source entity, target entity) that are *clearly related* to each other For each pair of related entities, extract the following information: - source entity: name of the source entity, as identified in step 1 - target entity: name of the target entity, as identified in step 1 - relationship description: explanation as to why you think the source entity and the target entity are related to each other - relationship strength: a numeric score indicating strength of the relationship between the source entity and target entity
```bgraph-paragraph
{"id":"d73e0028-d58a-5d5e-be19-c2d32e79ce1b","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":221.7,"width":395.4,"height":69.2}}},"text_order":242,"token_count":142,"style":null}
```

Format each relationship as ("relationship" { tuple delimiter } <source entity> { tuple delimiter } <target entity> { tuple delimiter } <relationship description> { tuple delimiter } <relationship strength>)
```bgraph-paragraph
{"id":"4bc0cca8-e61f-5278-bb48-281d8625cdd4","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":300.5,"width":365.1,"height":14.299988}}},"text_order":243,"token_count":47,"style":null}
```

3. Return output in English as a single list of all the entities and relationships identified in steps 1 and 2. Use ** { record delimiter } ** as the list delimiter.
```bgraph-paragraph
{"id":"ea271c4a-bbc9-5e58-8b3a-5f0f687306e9","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":325.3,"width":389.0,"height":13.399994}}},"text_order":244,"token_count":41,"style":null}
```

4. When finished, output { completion delimiter }
```bgraph-paragraph
{"id":"779f241b-0863-5672-a78b-f575c169d66a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":348.3,"width":177.6,"height":6.3000183}}},"text_order":245,"token_count":13,"style":null}
```

---Examples---
```bgraph-paragraph
{"id":"e38c612e-cd0f-5d6c-9eab-e6708358965e","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.8","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":365.2,"width":51.9,"height":5.4}}},"text_order":246,"token_count":3,"style":null}
```

Entity types: ORGANIZATION,PERSON
```bgraph-paragraph
{"id":"0dff69e2-905a-5f29-b6a8-d08c2f64eb60","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.9","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":381.1,"width":124.9,"height":5.4}}},"text_order":247,"token_count":8,"style":null}
```

Input:
```bgraph-paragraph
{"id":"21df594b-d0ae-55a1-8d89-2a3eb4dea5bf","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.10","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":397.0,"width":22.2,"height":5.4}}},"text_order":248,"token_count":1,"style":null}
```

The Fed is scheduled to meet on Tuesday and Wednesday, with the central bank planning to release its latest policy decision on Wednesday at 2:00 p.m. ET, followed by a press conference where Fed Chair Jerome Powell will take questions. Investors expect the Federal Open Market Committee to hold its benchmark interest rate steady in a range of 5.25%-5.5%.
```bgraph-paragraph
{"id":"c29130bb-a96e-52a0-8a74-c69bd4070588","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.11","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":413.0,"width":370.5,"height":29.299988}}},"text_order":249,"token_count":87,"style":null}
```

Output:
```bgraph-paragraph
{"id":"47cbd5fa-d297-51d5-bcdf-40618a75ee99","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.12","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":452.8,"width":25.9,"height":5.4}}},"text_order":250,"token_count":1,"style":null}
```

("entity" { tuple delimiter } FED { tuple delimiter } ORGANIZATION { tuple delimiter } The Fed is the Federal Reserve, which is setting interest rates on Tuesday and Wednesday) { record delimiter } ("entity" { tuple delimiter } JEROME POWELL { tuple delimiter } PERSON { tuple delimiter } Jerome Powell is the chair of the Federal Reserve) { record delimiter } ("entity" { tuple delimiter } FEDERAL OPEN MARKET COMMITTEE { tuple delimiter } ORGANIZATION { tuple delimiter } The Federal Reserve committee makes key decisions about interest rates and the growth of the United States money supply) { record delimiter } ("relationship" { tuple delimiter } JEROME POWELL { tuple delimiter } FED { tuple delimiter } Jerome Powell is the Chair of the Federal Reserve and will answer questions at a press conference { tuple delimiter } 9) { completion delimiter }
```bgraph-paragraph
{"id":"197d6c28-38c7-50a3-9cab-1a81d13f1fed","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.13","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":467.8,"width":391.8,"height":102.00006}}},"text_order":251,"token_count":207,"style":null}
```

...More examples...
```bgraph-paragraph
{"id":"073af92d-3ee5-5f4d-b407-2274b1e3c2c4","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.14","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":580.0,"width":52.9,"height":5.6}}},"text_order":252,"token_count":4,"style":null}
```

---Real Data---
```bgraph-paragraph
{"id":"12774944-ae18-5cba-a1b6-0cc64bd91ec7","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.15","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":596.3,"width":55.6,"height":5.4}}},"text_order":253,"token_count":3,"style":null}
```

Entity types: { entity types } Input:
```bgraph-paragraph
{"id":"8d1442e1-9dbc-5c78-b4dd-d294f2f958c7","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.16","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":611.3,"width":106.100006,"height":14.300049}}},"text_order":254,"token_count":9,"style":null}
```

{ input text }
```bgraph-paragraph
{"id":"392e94de-bdd3-513c-8624-5c021a6267f2","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.17","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":635.2,"width":44.200012,"height":6.299988}}},"text_order":255,"token_count":4,"style":null}
```

Output:
```bgraph-paragraph
{"id":"7bb77307-40af-50c3-a73f-afbd5c62b8cc","node_type":"Paragraph","location":{"semantic":{"path":"1.18.1.18","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":652.1,"width":25.9,"height":5.4}}},"text_order":256,"token_count":1,"style":null}
```

### E.2 Community Summary Generation
```bgraph-section
{"id":"095c1f37-0c04-5daf-a71b-898af7da0ee6","node_type":"Section","location":{"semantic":{"path":"1.18.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":671.3,"width":170.2,"height":8.7}}},"text_order":257,"token_count":8,"style":null}
```

---Role--- You are an AI assistant that helps a human analyst to perform general information discovery. Information discovery is the process of identifying and assessing relevant information associated with certain entities (e.g., organizations and individuals) within a network.
```bgraph-paragraph
{"id":"c32e00c7-d196-5e9f-82a3-7976a5907d74","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":21,"bounding_box":{"x":108.0,"y":690.1,"width":389.0,"height":29.300049}}},"text_order":258,"token_count":68,"style":null}
```

21
```bgraph-paragraph
{"id":"c515d387-146b-56e1-b0ea-78d96c8bc673","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":21,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":259,"token_count":1,"style":null}
```

---Goal--- Write a comprehensive report of a community, given a list of entities that belong to the community as well as their relationships and optional associated claims. The report will be used to inform decision-makers about information associated with the community and their potential impact. The content of this report includes an overview of the community’s key entities, their legal compliance, technical capabilities, reputation, and noteworthy claims.
```bgraph-paragraph
{"id":"f92dfcdd-b29c-532c-b27f-25e93b11844a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":85.9,"width":392.7,"height":45.299995}}},"text_order":260,"token_count":112,"style":null}
```

---Report Structure---
```bgraph-paragraph
{"id":"232f42b9-335a-57c9-97e7-f268c422470b","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":141.7,"width":81.5,"height":5.4}}},"text_order":261,"token_count":5,"style":null}
```

The report should include the following sections:
```bgraph-paragraph
{"id":"8bfcb6e3-87dd-552c-8c4a-c75585bf81aa","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":157.6,"width":181.5,"height":5.4}}},"text_order":262,"token_count":12,"style":null}
```

- TITLE: community’s name that represents its key entities - title should be short but specific. When possible, include representative named entities in the title. - SUMMARY: An executive summary of the community’s overall structure, how its entities are related to each other, and significant information associated with its entities. - IMPACT SEVERITY RATING: a float score between 0-10 that represents the severity of IMPACT posed by entities within the community. IMPACT is the scored importance of a community. - RATING EXPLANATION: Give a single sentence explanation of the IMPACT severity rating. - DETAILED FINDINGS: A list of 5-10 key insights about the community. Each insight should have a short summary followed by multiple paragraphs of explanatory text grounded according to the grounding rules below. Be comprehensive.
```bgraph-paragraph
{"id":"54506630-e133-5814-af04-408b6891b08d","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":173.6,"width":392.7,"height":77.09999}}},"text_order":263,"token_count":204,"style":null}
```

Return output as a well-formed JSON-formatted string with the following format: {{ "title": <report title>, "summary": <executive summary>, "rating": <impact severity rating>, "rating explanation": <rating explanation>, "findings": [ {{ "summary":<insight 1 summary>, "explanation": <insight 1 explanation> }} , {{ "summary":<insight 2 summary>, "explanation": <insight 2 explanation> }} ] }}
```bgraph-paragraph
{"id":"1a9e4c90-21ed-5d0e-93f9-c22302809679","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":261.2,"width":292.7,"height":132.69998}}},"text_order":264,"token_count":93,"style":null}
```

---Grounding Rules---
```bgraph-paragraph
{"id":"f2dddf5e-fbc4-5c45-9625-e14ee60d10e5","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.8","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":404.7,"width":77.8,"height":5.4}}},"text_order":265,"token_count":5,"style":null}
```

Points supported by data should list their data references as follows:
```bgraph-paragraph
{"id":"8692fb6d-0aad-5f77-b1a7-adb984275d23","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.9","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":420.6,"width":259.3,"height":5.4}}},"text_order":266,"token_count":17,"style":null}
```

"This is an example sentence supported by multiple data references [Data: <dataset name> (record ids); <dataset name> (record ids)]."
```bgraph-paragraph
{"id":"cd688753-09f0-5a8d-b1a2-61d92956a997","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.10","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":436.6,"width":381.6,"height":13.399994}}},"text_order":267,"token_count":32,"style":null}
```

Do not list more than 5 record ids in a single reference. Instead, list the top 5 most relevant record ids and add "+more" to indicate that there are more.
```bgraph-paragraph
{"id":"f552df01-a2fe-5d3d-99b0-e8f4355c010a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.11","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":460.5,"width":381.6,"height":13.399994}}},"text_order":268,"token_count":38,"style":null}
```

For example: "Person X is the owner of Company Y and subject to many allegations of wrongdoing [Data: Reports (1), Entities (5, 7); Relationships (23); Claims (7, 2, 34, 64, 46, +more)]."
```bgraph-paragraph
{"id":"42dd5e94-3e02-5b8f-9061-678f35a42c72","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.12","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":484.4,"width":377.9,"height":21.299988}}},"text_order":269,"token_count":46,"style":null}
```

where 1, 5, 7, 23, 2, 34, 46, and 64 represent the id (not the index) of the relevant data record.
```bgraph-paragraph
{"id":"c8ac0f99-ba47-569a-beac-d75c96bc439f","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.13","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":516.3,"width":363.1,"height":5.4}}},"text_order":270,"token_count":24,"style":null}
```

Do not include information where the supporting evidence for it is not provided.
```bgraph-paragraph
{"id":"e492d04a-76a0-5028-8bee-20eed9a5aca9","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.14","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":532.2,"width":296.4,"height":5.4}}},"text_order":271,"token_count":20,"style":null}
```

---Example---
```bgraph-paragraph
{"id":"efcd4808-a7e2-5377-8874-5fb8c2598aea","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.15","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":548.2,"width":48.2,"height":5.4}}},"text_order":272,"token_count":3,"style":null}
```

Input:
```bgraph-paragraph
{"id":"891a96d0-b5a6-57b5-9c8f-6f27af31487a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.16","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":564.1,"width":22.2,"height":5.4}}},"text_order":273,"token_count":1,"style":null}
```

Entities
```bgraph-paragraph
{"id":"2cecedd0-a530-5d6d-9cca-c77b753129a9","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.17","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":580.0,"width":29.6,"height":5.4}}},"text_order":274,"token_count":2,"style":null}
```

id,entity,description 5,VERDANT OASIS PLAZA,Verdant Oasis Plaza is the location of the Unity March 6,HARMONY ASSEMBLY,Harmony Assembly is an organization that is holding a march at Verdant Oasis Plaza
```bgraph-paragraph
{"id":"4c697e2a-c311-5943-b63f-d4b37cd8d4cb","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.18","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":596.0,"width":374.2,"height":21.300049}}},"text_order":275,"token_count":49,"style":null}
```

Relationships
```bgraph-paragraph
{"id":"a9c7b80c-164d-5f47-8ea7-aa4a57b0a61a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.19","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":627.9,"width":48.2,"height":5.4}}},"text_order":276,"token_count":3,"style":null}
```

id,source,target,description 37,VERDANT OASIS PLAZA,UNITY MARCH,Verdant Oasis Plaza is the location of the Unity March 38,VERDANT OASIS PLAZA,HARMONY ASSEMBLY,Harmony Assembly is holding a march at Verdant Oasis Plaza 39,VERDANT OASIS PLAZA,UNITY MARCH,The Unity March is taking place at Verdant Oasis Plaza 40,VERDANT OASIS PLAZA,TRIBUNE SPOTLIGHT,Tribune Spotlight is reporting on the Unity march taking place at Verdant Oasis Plaza 41,VERDANT OASIS PLAZA,BAILEY ASADI,Bailey Asadi is speaking at Verdant Oasis Plaza about the march 43,HARMONY ASSEMBLY,UNITY MARCH,Harmony Assembly is organizing the Unity March
```bgraph-paragraph
{"id":"a6651612-455f-59fd-9fc7-d46ec9b7349e","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.20","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":643.8,"width":392.7,"height":61.200012}}},"text_order":277,"token_count":148,"style":null}
```

Output:
```bgraph-paragraph
{"id":"57152fc9-4859-5151-855c-a02f36c5e922","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.21","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":108.0,"y":715.5,"width":25.9,"height":5.4}}},"text_order":278,"token_count":1,"style":null}
```

22
```bgraph-paragraph
{"id":"95d3f48a-95ff-599c-9e24-6162c014f380","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.22","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":279,"token_count":1,"style":null}
```

{{ "title": "Verdant Oasis Plaza and Unity March", "summary": "The community revolves around the Verdant Oasis Plaza, which is the location of the Unity March. The plaza has relationships with the Harmony Assembly, Unity March, and Tribune Spotlight, all of which are associated with the march event.", "rating": 5.0, "rating explanation": "The impact severity rating is moderate due to the potential for unrest or conflict during the Unity March.", "findings": [ {{ "summary": "Verdant Oasis Plaza as the central location", "explanation": "Verdant Oasis Plaza is the central entity in this community, serving as the location for the Unity March. This plaza is the common link between all other entities, suggesting its significance in the community. The plaza’s association with the march could potentially lead to issues such as public disorder or conflict, depending on the nature of the march and the reactions it provokes. [Data: Entities (5), Relationships (37, 38, 39, 40, 41,+more)]" }} , {{ "summary": "Harmony Assembly’s role in the community", "explanation": "Harmony Assembly is another key entity in this community, being the organizer of the march at Verdant Oasis Plaza. The nature of Harmony Assembly and its march could be a potential source of threat, depending on their objectives and the reactions they provoke. The relationship between Harmony Assembly and the plaza is crucial in understanding the dynamics of this community. [Data: Entities(6), Relationships (38, 43)]" }} , {{ "summary": "Unity March as a significant event", "explanation": "The Unity March is a significant event taking place at Verdant Oasis Plaza. This event is a key factor in the community’s dynamics and could be a potential source of threat, depending on the nature of the march and the reactions it provokes. The relationship between the march and the plaza is crucial in understanding the dynamics of this community. [Data: Relationships (39)]" }} , {{ "summary": "Role of Tribune Spotlight", "explanation": "Tribune Spotlight is reporting on the Unity March taking place in Verdant Oasis Plaza. This suggests that the event has attracted media attention, which could amplify its impact on the community. The role of Tribune Spotlight could be significant in shaping public perception of the event and the entities involved. [Data: Relationships (40)]" }} ] }}
```bgraph-paragraph
{"id":"9d8e6061-b5cb-503b-8bae-3aeaed3df378","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.23","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":77.0,"width":392.7,"height":316.9}}},"text_order":280,"token_count":580,"style":null}
```

---Real Data---
```bgraph-paragraph
{"id":"b28ac7b1-e693-58bd-92d9-d85194a82e96","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.24","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":404.7,"width":55.6,"height":5.4}}},"text_order":281,"token_count":3,"style":null}
```

Use the following text for your answer. Do not make anything up in your answer.
```bgraph-paragraph
{"id":"f2cd41ad-897c-5f47-9a52-b9005af8cbcc","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.25","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":420.6,"width":296.4,"height":5.4}}},"text_order":282,"token_count":19,"style":null}
```

Input:
```bgraph-paragraph
{"id":"8b776b2f-0e24-5341-9303-00b71a2149b0","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.26","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":436.6,"width":22.2,"height":5.4}}},"text_order":283,"token_count":1,"style":null}
```

{ input text }
```bgraph-paragraph
{"id":"b983d1f9-248a-59a7-9c75-48b9dec0afd2","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.27","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":451.6,"width":44.200012,"height":6.299988}}},"text_order":284,"token_count":4,"style":null}
```

...Report Structure and Grounding Rules Repeated...
```bgraph-paragraph
{"id":"6d75a56c-aafe-5976-8668-3f2fb41e9198","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.28","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":468.1,"width":146.1,"height":5.6}}},"text_order":285,"token_count":12,"style":null}
```

Output:
```bgraph-paragraph
{"id":"431930b6-a747-5a25-b4a6-90b44e8a38ff","node_type":"Paragraph","location":{"semantic":{"path":"1.18.2.29","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":484.4,"width":25.9,"height":5.4}}},"text_order":286,"token_count":1,"style":null}
```

### E.3 Community Answer Generation
```bgraph-section
{"id":"044930d3-49c1-5ce9-b4e6-f07097b1949e","node_type":"Section","location":{"semantic":{"path":"1.18.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":503.6,"width":160.8,"height":8.7}}},"text_order":287,"token_count":7,"style":null}
```

---Role---
```bgraph-paragraph
{"id":"0f69a2ce-67d3-5fcd-87dd-9b28d28e334c","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":522.4,"width":37.1,"height":5.4}}},"text_order":288,"token_count":2,"style":null}
```

You are a helpful assistant responding to questions about a dataset by synthesizing perspectives from multiple analysts.
```bgraph-paragraph
{"id":"441c0803-434e-51c2-9fcc-d2418fbbb42a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":538.3,"width":374.2,"height":13.400024}}},"text_order":289,"token_count":29,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"7fe33a3a-8b0e-5381-879c-2808a45ec7b6","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":562.3,"width":37.1,"height":5.4}}},"text_order":290,"token_count":2,"style":null}
```

Generate a response of the target length and format that responds to the user’s question, summarize all the reports from multiple analysts who focused on different parts of the dataset, and incorporate any relevant general knowledge.
```bgraph-paragraph
{"id":"04582ffe-9e76-5f09-879f-59ac08d864e2","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":578.2,"width":389.0,"height":21.299988}}},"text_order":291,"token_count":57,"style":null}
```

Note that the analysts’ reports provided below are ranked in the **descending order of helpfulness**.
```bgraph-paragraph
{"id":"52db3e8a-833c-5786-b906-d82744e1e793","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":610.1,"width":374.2,"height":5.4}}},"text_order":292,"token_count":25,"style":null}
```

If you don’t know the answer, just say so. Do not make anything up.
```bgraph-paragraph
{"id":"fa6a2426-6416-5b5d-b38d-c5bce52badba","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":626.0,"width":251.9,"height":5.4}}},"text_order":293,"token_count":17,"style":null}
```

The final response should remove all irrelevant information from the analysts’ reports and merge the cleaned information into a comprehensive answer that provides explanations of all the key points and implications appropriate for the response length and format.
```bgraph-paragraph
{"id":"ad59e1aa-fc3c-5c9a-9b33-58a02d14ac5d","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":642.0,"width":370.5,"height":21.300049}}},"text_order":294,"token_count":65,"style":null}
```

Add sections and commentary to the response as appropriate for the length and format. Style the response in markdown.
```bgraph-paragraph
{"id":"1b63e4c0-987b-5844-bd75-3e49f8bc93cc","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.8","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":673.8,"width":389.0,"height":13.400024}}},"text_order":295,"token_count":29,"style":null}
```

The response shall preserve the original meaning and use of modal verbs such as "shall", "may" or "will".
```bgraph-paragraph
{"id":"91aba094-db55-5fb8-af8b-40f7e2016d87","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.9","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":697.7,"width":389.0,"height":5.4}}},"text_order":296,"token_count":26,"style":null}
```

The response should also preserve all the data references previously included in the analysts’ reports,
```bgraph-paragraph
{"id":"655bd91f-fc1a-556e-9e03-64fc6d90e41d","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.10","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":108.0,"y":713.7,"width":381.6,"height":5.4}}},"text_order":297,"token_count":26,"style":null}
```

23
```bgraph-paragraph
{"id":"78260adf-a2ef-5541-b4cd-f440e32c7591","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.11","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":298,"token_count":1,"style":null}
```

but do not mention the roles of multiple analysts in the analysis process.
```bgraph-paragraph
{"id":"feb4d21d-af65-5570-927a-329df14c9240","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.12","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":77.9,"width":274.2,"height":5.4}}},"text_order":299,"token_count":18,"style":null}
```

Do not list more than 5 record ids in a single reference. Instead, list the top 5 most relevant record ids and add "+more" to indicate that there are more.
```bgraph-paragraph
{"id":"2ba94ce6-6f12-5e01-bf70-781b25d664b9","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.13","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":93.9,"width":381.6,"height":13.300003}}},"text_order":300,"token_count":38,"style":null}
```

For example:
```bgraph-paragraph
{"id":"690eeed8-6282-53be-b036-9be16a970184","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.14","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":117.8,"width":44.5,"height":5.4}}},"text_order":301,"token_count":3,"style":null}
```

"Person X is the owner of Company Y and subject to many allegations of wrongdoing [Data: Reports (2, 7, 34, 46, 64, +more)]. He is also CEO of company X [Data: Reports (1, 3)]"
```bgraph-paragraph
{"id":"d48d6c3d-7d37-5a8b-8aa0-2ff2ce812488","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.15","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":133.7,"width":374.2,"height":13.399994}}},"text_order":302,"token_count":43,"style":null}
```

where 1, 2, 3, 7, 34, 46, and 64 represent the id (not the index) of the relevant data record.
```bgraph-paragraph
{"id":"c216d704-dbad-5e76-b78b-93e269c94ad4","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.16","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":157.6,"width":348.3,"height":5.4}}},"text_order":303,"token_count":23,"style":null}
```

Do not include information where the supporting evidence for it is not provided.
```bgraph-paragraph
{"id":"abbbf32a-e4e4-5813-a851-496ab6ede402","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.17","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":173.6,"width":296.4,"height":5.4}}},"text_order":304,"token_count":20,"style":null}
```

---Target response length and format---
```bgraph-paragraph
{"id":"05be9166-63e3-5bf4-9524-fac1bb911e84","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.18","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":189.5,"width":144.5,"height":5.4}}},"text_order":305,"token_count":9,"style":null}
```

{ response type }
```bgraph-paragraph
{"id":"a49c6fd7-e549-5ed7-82ac-8367014917ed","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.19","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":204.5,"width":55.300003,"height":6.399994}}},"text_order":306,"token_count":5,"style":null}
```

---Analyst Reports---
```bgraph-paragraph
{"id":"9aae87a6-515f-529e-ad52-b6e8c8e46c19","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.20","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":221.4,"width":77.8,"height":5.4}}},"text_order":307,"token_count":5,"style":null}
```

{ report data }
```bgraph-paragraph
{"id":"f24e8563-920d-564f-9bfe-b95cd5895dd0","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.21","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":236.4,"width":47.90001,"height":6.300003}}},"text_order":308,"token_count":4,"style":null}
```

...Goal and Target response length and format repeated...
```bgraph-paragraph
{"id":"229a64b0-3ea1-5913-bc6d-4b75a099f0f6","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.22","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":260.9,"width":159.9,"height":5.6}}},"text_order":309,"token_count":14,"style":null}
```

Add sections and commentary to the response as appropriate for the length and format. Style the response in markdown.
```bgraph-paragraph
{"id":"a2516cea-0664-547e-92cc-e4392599c857","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.23","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":277.2,"width":389.0,"height":13.399994}}},"text_order":310,"token_count":29,"style":null}
```

Output:
```bgraph-paragraph
{"id":"9a783177-15c9-5aad-b545-286ed2a32766","node_type":"Paragraph","location":{"semantic":{"path":"1.18.3.24","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":301.1,"width":25.9,"height":5.4}}},"text_order":311,"token_count":1,"style":null}
```

### E.4 Global Answer Generation
```bgraph-section
{"id":"846e4c17-dab5-513e-b8f1-181b1a3db6bc","node_type":"Section","location":{"semantic":{"path":"1.18.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":320.3,"width":138.6,"height":8.7}}},"text_order":312,"token_count":7,"style":null}
```

---Role---
```bgraph-paragraph
{"id":"a912eebb-77ca-582a-bafe-5a69d3aac71a","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":339.1,"width":37.1,"height":5.4}}},"text_order":313,"token_count":2,"style":null}
```

You are a helpful assistant responding to questions about data in the tables provided.
```bgraph-paragraph
{"id":"b33b7c3e-b3e3-5292-9037-ef0dd32a01d3","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":355.0,"width":318.6,"height":5.4}}},"text_order":314,"token_count":21,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"6232b014-14dd-5e91-944a-e1307a3a0de2","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":371.0,"width":37.1,"height":5.4}}},"text_order":315,"token_count":2,"style":null}
```

Generate a response of the target length and format that responds to the user’s question, summarize all relevant information in the input data tables appropriate for the response length and format, and incorporate any relevant general knowledge.
```bgraph-paragraph
{"id":"e72533da-e019-5a8e-95af-7a00390a797b","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":386.9,"width":374.2,"height":21.399994}}},"text_order":316,"token_count":60,"style":null}
```

If you don’t know the answer, just say so. Do not make anything up.
```bgraph-paragraph
{"id":"eec21f57-adcf-55b2-82b2-4f13ff18bcb2","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":418.8,"width":251.9,"height":5.4}}},"text_order":317,"token_count":17,"style":null}
```

The response shall preserve the original meaning and use of modal verbs such as "shall", "may" or "will".
```bgraph-paragraph
{"id":"b01bce14-8284-5c69-82df-b6e90737d0ad","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":434.7,"width":389.0,"height":5.4}}},"text_order":318,"token_count":26,"style":null}
```

Points supported by data should list the relevant reports as references as follows:
```bgraph-paragraph
{"id":"f27234df-980d-5a92-9e67-5b276292de1c","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":450.7,"width":307.5,"height":5.4}}},"text_order":319,"token_count":20,"style":null}
```

"This is an example sentence supported by data references [Data: Reports (report ids)]"
```bgraph-paragraph
{"id":"5fdf6bb5-2ec1-5dc7-a242-1025cf9d2f06","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.8","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":466.6,"width":326.0,"height":5.4}}},"text_order":320,"token_count":21,"style":null}
```

Note: the prompts for SS (semantic search) and TS (text summarization) conditions use ”Sources” in place of ”Reports” above.
```bgraph-paragraph
{"id":"15a89098-3720-52c0-85ed-94c9250d72d4","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.9","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":482.2,"width":356.6,"height":5.6}}},"text_order":321,"token_count":33,"style":null}
```

Do not list more than 5 record ids in a single reference. Instead, list the top 5 most relevant record ids and add "+more" to indicate that there are more.
```bgraph-paragraph
{"id":"f3e359e0-c409-58b2-bff6-679cb3864e07","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.10","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":498.5,"width":381.6,"height":13.399994}}},"text_order":322,"token_count":38,"style":null}
```

For example:
```bgraph-paragraph
{"id":"f9609193-3359-5f9b-b9cc-e57361d728bc","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.11","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":522.4,"width":44.5,"height":5.4}}},"text_order":323,"token_count":3,"style":null}
```

"Person X is the owner of Company Y and subject to many allegations of wrongdoing [Data: Reports (2, 7, 64, 46, 34, +more)]. He is also CEO of company X [Data: Reports (1, 3)]"
```bgraph-paragraph
{"id":"6f96550f-2e98-5414-ab6a-5911bdec6de3","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.12","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":538.3,"width":374.2,"height":13.400024}}},"text_order":324,"token_count":43,"style":null}
```

where 1, 2, 3, 7, 34, 46, and 64 represent the id (not the index) of the relevant data report in the provided tables.
```bgraph-paragraph
{"id":"c1930932-b2f1-5419-b2da-d02446a9cd3d","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.13","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":562.3,"width":370.5,"height":13.300049}}},"text_order":325,"token_count":29,"style":null}
```

Do not include information where the supporting evidence for it is not provided.
```bgraph-paragraph
{"id":"38d578e8-3fbd-5986-9228-5e453c03d31c","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.14","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":586.2,"width":296.4,"height":5.4}}},"text_order":326,"token_count":20,"style":null}
```

At the beginning of your response, generate an integer score between 0-100 that indicates how **helpful** is this response in answering the user’s question. Return the score in this format: <ANSWER HELPFULNESS> score value </ANSWER HELPFULNESS>.
```bgraph-paragraph
{"id":"1a40cd11-2f79-5d31-8692-5ac88d66a767","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.15","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":602.1,"width":391.7,"height":21.300049}}},"text_order":327,"token_count":60,"style":null}
```

---Target response length and format---
```bgraph-paragraph
{"id":"9f36682a-ee3f-522d-b3ab-349879a30350","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.16","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":634.0,"width":144.5,"height":5.4}}},"text_order":328,"token_count":9,"style":null}
```

{ response type }
```bgraph-paragraph
{"id":"519cf0af-c12d-5234-a297-343ba7ecaa7e","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.17","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":649.0,"width":55.300003,"height":6.300049}}},"text_order":329,"token_count":5,"style":null}
```

---Data tables---
```bgraph-paragraph
{"id":"1409b6df-0d22-53c9-b3db-8ab0f06d8eb5","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.18","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":665.9,"width":63.0,"height":5.4}}},"text_order":330,"token_count":4,"style":null}
```

{ context data }
```bgraph-paragraph
{"id":"3698d368-12ee-57af-ae4e-4ea0b9af3e2d","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.19","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":680.9,"width":51.600006,"height":6.299988}}},"text_order":331,"token_count":5,"style":null}
```

...Goal and Target response length and format repeated...
```bgraph-paragraph
{"id":"9c69229e-3eea-5238-a63b-79d56f633cd8","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.20","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":697.4,"width":159.9,"height":5.6}}},"text_order":332,"token_count":14,"style":null}
```

Output:
```bgraph-paragraph
{"id":"40d7f0a1-0289-5439-9280-9cb748d0c148","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.21","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":108.0,"y":713.2,"width":21.3,"height":5.8}}},"text_order":333,"token_count":1,"style":null}
```

24
```bgraph-paragraph
{"id":"6b82a95b-ab07-5126-8f5d-6f344605f75e","node_type":"Paragraph","location":{"semantic":{"path":"1.18.4.22","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":334,"token_count":1,"style":null}
```

## F Evaluation Prompts
```bgraph-section
{"id":"4e9b3538-e66d-5d80-af21-ae4b84332723","node_type":"Section","location":{"semantic":{"path":"1.19","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":73.6,"width":121.5,"height":10.6}}},"text_order":335,"token_count":5,"style":null}
```

### F.1 Relative Assessment Prompt
```bgraph-section
{"id":"5e063f82-708a-50eb-976c-01becd7684da","node_type":"Section","location":{"semantic":{"path":"1.19.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":96.2,"width":143.2,"height":8.699997}}},"text_order":336,"token_count":7,"style":null}
```

---Role---
```bgraph-paragraph
{"id":"8516dfd3-f685-5cd0-a473-69728978df9e","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":115.0,"width":37.1,"height":5.4}}},"text_order":337,"token_count":2,"style":null}
```

You are a helpful assistant responsible for grading two answers to a question that are provided by two different people.
```bgraph-paragraph
{"id":"ae86f59c-735b-5302-9c5a-448c6663c262","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":130.9,"width":377.9,"height":13.399994}}},"text_order":338,"token_count":29,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"15d191f2-4b47-5892-94bc-6f9881e58182","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":154.8,"width":37.1,"height":5.4}}},"text_order":339,"token_count":2,"style":null}
```

Given a question and two answers (Answer 1 and Answer 2), assess which answer is better according to the following measure:
```bgraph-paragraph
{"id":"a0abff14-e507-592d-ae3d-adce3d13dd14","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":170.8,"width":370.5,"height":13.299988}}},"text_order":340,"token_count":30,"style":null}
```

{ criteria }
```bgraph-paragraph
{"id":"2262d3c5-3d30-5371-b48a-a46574ed576b","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":193.8,"width":37.800003,"height":6.299988}}},"text_order":341,"token_count":4,"style":null}
```

Your assessment should include two parts: - Winner: either 1 (if Answer 1 is better) and 2 (if Answer 2 is better) or 0 if they are fundamentally similar and the differences are immaterial. - Reasoning: a short explanation of why you chose the winner with respect to the measure described above.
```bgraph-paragraph
{"id":"af6138aa-6c47-50af-b229-fd1bd467753c","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":210.6,"width":392.7,"height":29.299988}}},"text_order":342,"token_count":71,"style":null}
```

Format your response as a JSON object with the following structure: {{ "winner": <1, 2, or 0>, "reasoning": "Answer 1 is better because <your reasoning>." }}
```bgraph-paragraph
{"id":"009bcbde-d6c2-561f-9f16-5d94d779eb28","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":250.5,"width":248.20001,"height":37.0}}},"text_order":343,"token_count":37,"style":null}
```

---Question---
```bgraph-paragraph
{"id":"9dcc79a1-2076-5f61-b140-74806993518e","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.8","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":298.3,"width":51.9,"height":5.4}}},"text_order":344,"token_count":3,"style":null}
```

{ question }
```bgraph-paragraph
{"id":"9af7d3a5-d417-52a0-88de-1774efbde154","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.9","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":313.3,"width":37.800003,"height":6.3000183}}},"text_order":345,"token_count":4,"style":null}
```

---Answer 1---
```bgraph-paragraph
{"id":"ec3c4abe-9721-5615-a91e-a2c03de40b5d","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.10","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":330.2,"width":51.9,"height":5.4}}},"text_order":346,"token_count":3,"style":null}
```

{ answer1 }
```bgraph-paragraph
{"id":"9dc1b823-18ba-5342-8db8-1afcf0092bbd","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.11","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":345.2,"width":34.100006,"height":6.299988}}},"text_order":347,"token_count":3,"style":null}
```

---Answer 2---
```bgraph-paragraph
{"id":"07c814e9-fa6a-51d5-93d1-0be6195a173c","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.12","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":362.1,"width":51.9,"height":5.4}}},"text_order":348,"token_count":3,"style":null}
```

{ answer2 }
```bgraph-paragraph
{"id":"4595e385-b49b-5869-87ad-63a327a2c0cd","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.13","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":377.1,"width":34.100006,"height":6.299988}}},"text_order":349,"token_count":3,"style":null}
```

Assess which answer is better according to the following measure:
```bgraph-paragraph
{"id":"3d09cf74-78f2-51bb-9b2e-e75dd5998c07","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.14","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":393.9,"width":240.8,"height":5.4}}},"text_order":350,"token_count":16,"style":null}
```

{ criteria }
```bgraph-paragraph
{"id":"9fab84ed-6026-54ec-aa9c-701fbdbe85ff","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.15","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":409.0,"width":37.800003,"height":6.299988}}},"text_order":351,"token_count":4,"style":null}
```

Output:
```bgraph-paragraph
{"id":"475f8bbc-b40f-59fa-8105-6551d2bf3327","node_type":"Paragraph","location":{"semantic":{"path":"1.19.1.16","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":425.8,"width":25.9,"height":5.4}}},"text_order":352,"token_count":1,"style":null}
```

### F.2 Relative Assessment Metrics
```bgraph-section
{"id":"dd98ec51-a83a-5940-aafd-a473d8557aa6","node_type":"Section","location":{"semantic":{"path":"1.19.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":445.1,"width":143.4,"height":8.7}}},"text_order":353,"token_count":7,"style":null}
```

CRITERIA = { "comprehensiveness": "How much detail does the answer provide to cover all the aspects and details of the question? A comprehensive answer should be thorough and complete, without being redundant or irrelevant. For example, if the question is ’What are the benefits and drawbacks of nuclear energy?’, a comprehensive answer would provide both the positive and negative aspects of nuclear energy, such as its efficiency, environmental impact, safety, cost, etc. A comprehensive answer should not leave out any important points or provide irrelevant information. For example, an incomplete answer would only provide the benefits of nuclear energy without describing the drawbacks, or a redundant answer would repeat the same information multiple times.", "diversity": "How varied and rich is the answer in providing different perspectives and insights on the question? A diverse answer should be multi-faceted and multi-dimensional, offering different viewpoints and angles on the question. For example, if the question is ’What are the causes and effects of climate change?’, a diverse answer would provide different causes and effects of climate change, such as greenhouse gas emissions, deforestation, natural disasters, biodiversity loss, etc. A diverse answer should also provide different sources and evidence to support the answer. For example, a single-source answer would only cite one source or evidence, or a biased answer would only provide one perspective or opinion.", "directness": "How specifically and clearly does the answer address the question? A direct answer should provide a clear and concise answer to the question. For example, if the question is ’What is the capital of France?’, a direct answer would be ’Paris’. A direct answer should not provide any irrelevant or unnecessary information that does not answer the question. For example, an indirect answer would be ’The capital of France is located on the river Seine’.", "empowerment": "How well does the answer help the reader understand and make informed judgements about the topic without being misled or making fallacious assumptions. Evaluate each answer on the quality of answer as it relates to clearly explaining and providing reasoning and sources behind the claims in the answer." }
```bgraph-paragraph
{"id":"a780238a-6d26-5b43-8687-e96f5b74519c","node_type":"Paragraph","location":{"semantic":{"path":"1.19.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":108.0,"y":462.9,"width":392.7,"height":213.29996}}},"text_order":354,"token_count":560,"style":null}
```

25
```bgraph-paragraph
{"id":"da5d9742-8c91-59dd-bf3e-cd41604e509f","node_type":"Paragraph","location":{"semantic":{"path":"1.19.2.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":355,"token_count":1,"style":null}
```

## G Statistical Analysis
```bgraph-section
{"id":"5d17c1a3-4073-5608-a4c4-e1cd2045e4be","node_type":"Section","location":{"semantic":{"path":"1.20","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":73.6,"width":117.9,"height":10.6}}},"text_order":356,"token_count":5,"style":null}
```

Table 6: Pairwise comparisons of six conditions on four metrics across 125 questions and two
```bgraph-paragraph
{"id":"e01d2fb3-98bf-54ae-8e3d-d6669b7e039c","node_type":"Paragraph","location":{"semantic":{"path":"1.20.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":101.1,"width":396.0,"height":8.7}}},"text_order":357,"token_count":23,"style":null}
```

datasets. For each question and metric, the winning condition received a score of 100, the losing
```bgraph-paragraph
{"id":"aaa85523-4624-5436-826e-542152eb05b5","node_type":"Paragraph","location":{"semantic":{"path":"1.20.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":112.0,"width":396.0,"height":8.7}}},"text_order":358,"token_count":24,"style":null}
```

condition received a score of 0, and in the event of a tie, each condition was scored 50. These scores
```bgraph-paragraph
{"id":"35cdfd48-056f-502b-90e7-e14d4216dffc","node_type":"Paragraph","location":{"semantic":{"path":"1.20.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":123.0,"width":396.0,"height":8.7}}},"text_order":359,"token_count":25,"style":null}
```

were then averaged over five evaluation runs for each condition. Results of Shapiro-Wilk tests in-
```bgraph-paragraph
{"id":"15f66420-7167-5ef9-a098-89610a432beb","node_type":"Paragraph","location":{"semantic":{"path":"1.20.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":133.9,"width":396.0,"height":8.7}}},"text_order":360,"token_count":24,"style":null}
```

dicated that the data did not follow a normal distribution. Thus, non-parametric tests (Wilcoxon
```bgraph-paragraph
{"id":"351babeb-a5cf-5f73-bd7d-628d1ea88f0d","node_type":"Paragraph","location":{"semantic":{"path":"1.20.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":144.8,"width":396.0,"height":8.7}}},"text_order":361,"token_count":24,"style":null}
```

signed-rank tests) were employed to assess the performance differences between pairs of condi-
```bgraph-paragraph
{"id":"c990ec1f-dc6a-55a2-89b6-fe932064e538","node_type":"Paragraph","location":{"semantic":{"path":"1.20.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":155.7,"width":396.0,"height":8.7}}},"text_order":362,"token_count":23,"style":null}
```

tions, with Holm-Bonferroni correction applied to account for multiple pairwise comparisons. The
```bgraph-paragraph
{"id":"a7c89b9e-828b-5d37-a6c8-eac4cab7b01e","node_type":"Paragraph","location":{"semantic":{"path":"1.20.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":166.6,"width":396.0,"height":8.7}}},"text_order":363,"token_count":24,"style":null}
```

corrected p-values that indicated statistically significant differences are highlighted in bold.
```bgraph-paragraph
{"id":"9640766b-86b6-5104-b50a-731a975f3da9","node_type":"Paragraph","location":{"semantic":{"path":"1.20.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":108.0,"y":177.5,"width":363.4,"height":8.7}}},"text_order":364,"token_count":24,"style":null}
```

Podcast Transcripts News Articles Condition 1 Condition 2 Mean 1 Mean 2 Z-value p-value Mean 1 Mean 2 Z-value p-value C0 TS 50.24 49.76 -0.06 1 55.52 44.48 -2.03 0.17 C1 TS 51.92 48.08 -1.56 0.633 58.8 41.2 -3.62 0.002 C2 TS 57.28 42.72 -4.1 < 0.001 62.08 37.92 -5.07 < 0.001 C3 TS 56.48 43.52 -3.42 0.006 63.6 36.4 -5.63 < 0.001 C0 SS 71.92 28.08 -6.2 < 0.001 71.76 28.24 -6.3 < 0.001 C1 SS 75.44 24.56 -7.45 < 0.001 74.72 25.28 -7.78 < 0.001 C2 SS 77.76 22.24 -8.17 < 0.001 79.2 20.8 -8.34 < 0.001 Comprehensiveness C3 SS 78.96 21.04 -8.12 < 0.001 79.44 20.56 -8.44 < 0.001 TS SS 83.12 16.88 -8.85 < 0.001 79.6 20.4 -8.27 < 0.001 C0 C1 53.2 46.8 -1.96 0.389 51.92 48.08 -0.45 0.777 C0 C2 50.24 49.76 -0.23 1 53.68 46.32 -1.54 0.371 C1 C2 51.52 48.48 -1.62 0.633 57.76 42.24 -4.01 < 0.001 C0 C3 49.12 50.88 -0.56 1 52.16 47.84 -0.86 0.777 C1 C3 50.32 49.68 -0.66 1 55.12 44.88 -2.94 0.016 C2 C3 52.24 47.76 -1.97 0.389 58.64 41.36 -3.68 0.002 C0 TS 50.24 49.76 -0.11 1 46.88 53.12 -1.38 0.676 C1 TS 50.48 49.52 -0.12 1 54.64 45.36 -1.88 0.298 C2 TS 57.12 42.88 -2.84 0.036 55.76 44.24 -2.16 0.184 C3 TS 54.32 45.68 -2.39 0.1 60.16 39.84 -4.07 < 0.001 C0 SS 76.56 23.44 -7.12 < 0.001 62.08 37.92 -3.57 0.003 C1 SS 75.44 24.56 -7.33 < 0.001 64.96 35.04 -4.92 < 0.001 C2 SS 80.56 19.44 -8.21 < 0.001 70.56 29.44 -6.29 < 0.001 Diversity C3 SS 80.8 19.2 -8.3 < 0.001 69.12 30.88 -5.53 < 0.001 TS SS 82.08 17.92 -8.43 < 0.001 67.2 32.8 -4.85 < 0.001 C0 C1 49.76 50.24 -0.13 1 39.68 60.32 -3.61 0.003 C0 C2 46.32 53.68 -1.5 0.669 40.96 59.04 -3.14 0.012 C1 C2 44.08 55.92 -3.27 0.011 50.24 49.76 -0.22 1 C0 C3 44 56 -2.6 0.065 41.04 58.96 -3.47 0.004 C1 C3 45.44 54.56 -2.98 0.026 49.52 50.48 -0.01 1 C2 C3 48.48 51.52 -0.96 1 50.96 49.04 -0.39 1 C0 TS 40.96 59.04 -4.3 < 0.001 42.24 57.76 -3.32 0.012 C1 TS 45.2 54.8 -3.76 0.002 50 50 -0.12 1 C2 TS 47.68 52.32 -2.2 0.281 49.52 50.48 -0.22 1 C3 TS 48.72 51.28 -1.27 1 51.68 48.32 -1.2 1 C0 SS 42.96 57.04 -3.71 0.003 42.72 57.28 -3.12 0.022 C1 SS 47.68 52.32 -1.5 0.936 51.36 48.64 -0.84 1 C2 SS 50.72 49.28 -0.55 1 49.84 50.16 -0.2 1 Empowerment C3 SS 48.96 51.04 -0.57 1 49.52 50.48 -0.08 1 TS SS 57.52 42.48 -4.1 < 0.001 52.88 47.12 -1.1 1 C0 C1 48.72 51.28 -1.23 1 42.4 57.6 -3.9 0.001 C0 C2 46.64 53.36 -2.54 0.12 44.8 55.2 -2.16 0.336 C1 C2 49.28 50.72 -1.73 0.682 52 48 -1.45 1 C0 C3 47.6 52.4 -1.78 0.682 44.32 55.68 -3.45 0.008 C1 C3 50 50 0 1 51.44 48.56 -1.02 1 C2 C3 50.72 49.28 -0.86 1 50.4 49.6 -0.22 1 C0 TS 44.96 55.04 -4.09 < 0.001 45.2 54.8 -3.68 0.003 C1 TS 47.92 52.08 -2.41 0.126 46.64 53.36 -2.91 0.04 C2 TS 48.8 51.2 -2.23 0.179 48.32 51.68 -2.12 0.179 C3 TS 48.08 51.92 -2.23 0.179 48.32 51.68 -2.56 0.074 C0 SS 35.12 64.88 -6.17 < 0.001 41.44 58.56 -4.82 < 0.001 C1 SS 40.32 59.68 -4.83 < 0.001 45.2 54.8 -3.19 0.017 C2 SS 40.4 59.6 -4.67 < 0.001 44.88 55.12 -3.65 0.003 Directness C3 SS 40.48 59.52 -4.69 < 0.001 45.6 54.4 -2.86 0.043 TS SS 43.6 56.4 -3.96 < 0.001 46 54 -2.68 0.066 C0 C1 46.96 53.04 -2.87 0.037 47.6 52.4 -2.17 0.179 C0 C2 48.4 51.6 -2.06 0.197 48.48 51.52 -1.61 0.321 C1 C2 49.84 50.16 -1 0.952 49.28 50.72 -1.6 0.321 C0 C3 48.4 51.6 -1.8 0.29 47.2 52.8 -2.62 0.071 C1 C3 49.76 50.24 0 1 48.8 51.2 -1.29 0.321 C2 C3 50 50 0 1 48.8 51.2 -1.84 0.262
```bgraph-paragraph
{"id":"4d6a27fc-c489-519d-927d-e640d10bc310","node_type":"Paragraph","location":{"semantic":{"path":"1.20.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":114.0,"y":195.8,"width":386.7,"height":432.3}}},"text_order":365,"token_count":662,"style":null}
```

26
```bgraph-paragraph
{"id":"e79e2e9f-4dbf-5a0b-a36e-34c010057682","node_type":"Paragraph","location":{"semantic":{"path":"1.20.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":301.0,"y":743.2,"width":10.0,"height":8.7}}},"text_order":366,"token_count":1,"style":null}
```
