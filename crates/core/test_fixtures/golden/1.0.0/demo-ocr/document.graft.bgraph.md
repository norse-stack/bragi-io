```bgraph
{"schema":"1.2.0","kind":"document","bragi_version":"0.8.0","source":{"format":"ocr","sha256":"17ff80a804d1024e2b3926919d7d9907405ebd117e2bc4dca5175659c321fd77"},"flow_type":"Fixed","config_hash":"none","bgraph_sha256":"635703c94539d45c380bdc95bce6e341a8b6a9a7ff7f49f7e6591853d65fd1a6"}
```

```bgraph-metadata
{"title":"From Local to Global: A GraphRAG Approach to Query-Focused Summarization","author":null,"description":null,"language":null,"created":"2025-02-20T01:38:02Z","pdf":{"version":"1.5","producer":"pdfTeX-1.40.25","creator_tool":"LaTeX with hyperref","publisher":null,"page_count":26,"encrypted":false,"has_marked_content":false,"modified":"2025-02-20T01:38:02Z","extras":{"Content-Type":"application/pdf","X-TIKA:versionCount":"0","access_permission:assemble_document":"true","access_permission:can_modify":"true","access_permission:can_print":"true","access_permission:can_print_faithful":"true","access_permission:extract_content":"true","access_permission:extract_for_accessibility":"true","access_permission:fill_in_form":"true","access_permission:modify_annotations":"true","dc:format":"application/pdf; version=1.5","pdf:docinfo:created":"2025-02-20T01:38:02Z","pdf:docinfo:creator_tool":"LaTeX with hyperref","pdf:docinfo:custom:PTEX.Fullbanner":"This is pdfTeX, Version 3.141592653-2.6-1.40.25 (TeX Live 2023) kpathsea version 6.3.5","pdf:docinfo:modified":"2025-02-20T01:38:02Z","pdf:docinfo:producer":"pdfTeX-1.40.25","pdf:docinfo:trapped":"False","pdf:eofOffsets":"6893854","pdf:hasCollection":"false","pdf:hasXFA":"false","pdf:hasXMP":"false","pdf:incrementalUpdateCount":"0"}},"ocr":{"model":"mistral-ocr-4-0","pages_processed":26,"doc_size_bytes":6893854,"dpi":93,"supplier_sha256":"17ff80a804d1024e2b3926919d7d9907405ebd117e2bc4dca5175659c321fd77","extras":{}}}
```

```bgraph-outline
{"sections":[{"title":"From Local to Global: A GraphRAG Approach to Query-Focused Summarization","order":0,"level":1},{"title":"Abstract","order":1,"level":1},{"title":"1 Introduction","order":2,"level":1},{"title":"2 Background","order":3,"level":1},{"title":"2.1 RAG Approaches and Systems","order":4,"level":2},{"title":"2.2 Using Knowledge Graphs with LLMs and RAG","order":5,"level":2},{"title":"2.3 Adaptive benchmarking for RAG Evaluation","order":6,"level":2},{"title":"2.4 RAG evaluation criteria","order":7,"level":2},{"title":"3 Methods","order":8,"level":1},{"title":"3.1 GraphRAG Workflow","order":9,"level":2},{"title":"3.1.1 Source Documents → Text Chunks","order":10,"level":3},{"title":"3.1.2 Text Chunks → Entities & Relationships","order":11,"level":3},{"title":"3.1.3 Entities & Relationships → Knowledge Graph","order":12,"level":3},{"title":"3.1.4 Knowledge Graph → Graph Communities","order":13,"level":3},{"title":"3.1.5 Graph Communities → Community Summaries","order":14,"level":3},{"title":"3.1.6 Community Summaries → Community Answers → Global Answer","order":15,"level":3},{"title":"3.2 Global Sensemaking Question Generation","order":16,"level":2},{"title":"Algorithm 1: Prompting Procedure for Question Generation","order":17,"level":1},{"title":"3.3 Criteria for Evaluating Global Sensemaking","order":18,"level":2},{"title":"4 Analysis","order":19,"level":1},{"title":"4.1 Experiment 1","order":20,"level":2},{"title":"4.1.1 Datasets","order":21,"level":3},{"title":"4.1.2 Conditions","order":22,"level":3},{"title":"4.1.3 Configuration","order":23,"level":3},{"title":"4.2 Experiment 2","order":24,"level":2},{"title":"5 Results","order":25,"level":1},{"title":"5.1 Experiment 1","order":26,"level":2},{"title":"5.2 Experiment 2","order":27,"level":2},{"title":"6 Discussion","order":28,"level":1},{"title":"6.1 Limitations of evaluation approach","order":29,"level":2},{"title":"6.2 Future work","order":30,"level":2},{"title":"7 Conclusion","order":31,"level":1},{"title":"Acknowledgements","order":32,"level":1},{"title":"References","order":33,"level":1},{"title":"A Entity and Relationship Extraction Approach","order":34,"level":1},{"title":"A.1 Entity Extraction","order":35,"level":2},{"title":"A.2 Self-Reflection","order":36,"level":2},{"title":"B Example Community Detection","order":37,"level":1},{"title":"C Context Window Selection","order":38,"level":1},{"title":"D Example Answer Comparison","order":39,"level":1},{"title":"E System Prompts","order":40,"level":1},{"title":"E.1 Element Instance Generation","order":41,"level":2},{"title":"E.2 Community Summary Generation","order":42,"level":2},{"title":"E.3 Community Answer Generation","order":43,"level":2},{"title":"E.4 Global Answer Generation","order":44,"level":2},{"title":"F Evaluation Prompts","order":45,"level":1},{"title":"F.1 Relative Assessment Prompt","order":46,"level":2},{"title":"F.2 Relative Assessment Metrics","order":47,"level":2},{"title":"G Statistical Analysis","order":48,"level":1}]}
```

arXiv:2404.16130v2 [cs.CL] 19 Feb 2025
```bgraph-paragraph
{"id":"e0ce3fad-4677-5873-bfab-04cb1989b4c9","node_type":"Paragraph","location":{"semantic":{"path":"1","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":12.387096,"y":209.80644,"width":21.677418,"height":349.1613}}},"text_order":0,"token_count":9,"style":null}
```

# From Local to Global: A GraphRAG Approach to Query-Focused Summarization
```bgraph-section
{"id":"97488a75-0154-50d2-a24a-66b52e793154","node_type":"Section","location":{"semantic":{"path":"2","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":117.677414,"y":96.0,"width":373.1613,"height":38.709675}}},"text_order":1,"token_count":18,"style":null}
```

Darren Edge¹† Ha Trinh¹† Newman Cheng² Joshua Bradley² Alex Chao³
Apurva Mody³ Steven Truitt² Dasha Metropolitansky¹ Robert Osazuwa Ness¹
```bgraph-paragraph
{"id":"acb999a9-d309-55aa-8108-bcc8aa54b291","node_type":"Paragraph","location":{"semantic":{"path":"2.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":115.354836,"y":175.74193,"width":377.03226,"height":41.80645}}},"text_order":2,"token_count":37,"style":null}
```

Jonathan Larson¹
```bgraph-paragraph
{"id":"fd90a8f3-102b-5494-85dd-6dd74446089e","node_type":"Paragraph","location":{"semantic":{"path":"2.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":264.0,"y":233.03226,"width":78.96774,"height":11.612903}}},"text_order":3,"token_count":4,"style":null}
```

¹Microsoft Research
```bgraph-paragraph
{"id":"3666bfc2-04b0-5a15-bfdd-530c24cf871e","node_type":"Paragraph","location":{"semantic":{"path":"2.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":261.6774,"y":255.48386,"width":85.93548,"height":11.612903}}},"text_order":4,"token_count":5,"style":null}
```

²Microsoft Strategic Missions and Technologies
```bgraph-paragraph
{"id":"2e2c9767-4c83-5757-8019-ddf627382985","node_type":"Paragraph","location":{"semantic":{"path":"2.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":206.70967,"y":267.09677,"width":195.09677,"height":12.387096}}},"text_order":5,"token_count":11,"style":null}
```

³Microsoft Office of the CTO
```bgraph-paragraph
{"id":"8804fe46-3a83-53fb-93da-e269212fd60d","node_type":"Paragraph","location":{"semantic":{"path":"2.5","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":243.87096,"y":278.70966,"width":120.77419,"height":12.387096}}},"text_order":6,"token_count":7,"style":null}
```

{daedge,trinhha,newmancheng,joshbradley,achao,moapurva,
steventruitt,dasham,robertness,jolarso}@microsoft.com
```bgraph-paragraph
{"id":"1f04b703-2a59-548a-a13a-ec56842119a4","node_type":"Paragraph","location":{"semantic":{"path":"2.6","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":159.48387,"y":301.1613,"width":289.54837,"height":22.451612}}},"text_order":7,"token_count":27,"style":null}
```

†These authors contributed equally to this work
```bgraph-paragraph
{"id":"72b2447e-7dd8-5dc9-afae-48e74925db7e","node_type":"Paragraph","location":{"semantic":{"path":"2.7","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]},"physical":{"page":1,"bounding_box":{"x":217.54839,"y":333.6774,"width":174.19354,"height":11.612903}}},"text_order":8,"token_count":12,"style":null}
```

# Abstract
```bgraph-section
{"id":"489feb8e-0692-570c-9a30-4a94247c2ce3","node_type":"Section","location":{"semantic":{"path":"3","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Abstract"]},"physical":{"page":1,"bounding_box":{"x":280.25806,"y":373.9355,"width":48.0,"height":12.387096}}},"text_order":9,"token_count":2,"style":null}
```

The use of retrieval-augmented generation (RAG) to retrieve relevant information from an external knowledge source enables large language models (LLMs) to answer questions over private and/or previously unseen document collections. However, RAG fails on global questions directed at an entire text corpus, such as “What are the main themes in the dataset?”, since this is inherently a query-focused summarization (QFS) task, rather than an explicit retrieval task. Prior QFS methods, meanwhile, do not scale to the quantities of text indexed by typical RAG systems. To combine the strengths of these contrasting methods, we propose GraphRAG, a graph-based approach to question answering over private text corpora that scales with both the generality of user questions and the quantity of source text. Our approach uses an LLM to build a graph index in two stages: first, to derive an entity knowledge graph from the source documents, then to pre-generate community summaries for all groups of closely related entities. Given a question, each community summary is used to generate a partial response, before all partial responses are again summarized in a final response to the user. For a class of global sensemaking questions over datasets in the 1 million token range, we show that GraphRAG leads to substantial improvements over a conventional RAG baseline for both the comprehensiveness and diversity of generated answers.
```bgraph-paragraph
{"id":"ff047e7f-19a9-51f1-a230-725965b06a8a","node_type":"Paragraph","location":{"semantic":{"path":"3.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Abstract"]},"physical":{"page":1,"bounding_box":{"x":140.12903,"y":399.48386,"width":329.03226,"height":197.41934}}},"text_order":10,"token_count":357,"style":null}
```

# 1 Introduction
```bgraph-section
{"id":"70d57604-6f5f-51c4-85d0-7f4b6eaf0ffe","node_type":"Section","location":{"semantic":{"path":"4","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":1,"bounding_box":{"x":104.51613,"y":619.3548,"width":86.70967,"height":13.16129}}},"text_order":11,"token_count":3,"style":null}
```

Retrieval augmented generation (RAG) (Lewis et al., 2020) is an established approach to using LLMs to answer queries based on data that is too large to contain in a language model’s context window, meaning the maximum number of tokens (units of text) that can be processed by the LLM at once (Kuratov et al., 2024; Liu et al., 2023). In the canonical RAG setup, the system has access to a large external corpus of text records and retrieves a subset of records that are individually relevant to the query and collectively small enough to fit into the context window of the LLM. The LLM then
```bgraph-paragraph
{"id":"d98043cb-28fe-50ec-b662-b01e5819309e","node_type":"Paragraph","location":{"semantic":{"path":"4.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":1,"bounding_box":{"x":104.51613,"y":644.129,"width":400.25806,"height":67.354836}}},"text_order":12,"token_count":148,"style":null}
```

Preprint. Under review.
```bgraph-paragraph
{"id":"3f4b9053-3457-52bf-a905-9caeee7218a1","node_type":"Paragraph","location":{"semantic":{"path":"4.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":1,"bounding_box":{"x":104.51613,"y":729.2903,"width":86.70967,"height":10.838709}}},"text_order":13,"token_count":5,"style":null}
```

generates a response based on both the query and the retrieved records (Baumel et al., 2018; Dang, 2006; Laskar et al., 2020; Yao et al., 2017). This conventional approach, which we collectively call *vector RAG*, works well for queries that can be answered with information localized within a small set of records. However, vector RAG approaches do not support *sensemaking* queries, meaning queries that require global understanding of the entire dataset, such as “*What are the key trends in how scientific discoveries are influenced by interdisciplinary research over the past decade?*”
```bgraph-paragraph
{"id":"22b4166b-f8c0-5c4d-83b1-85ef2c4b1887","node_type":"Paragraph","location":{"semantic":{"path":"4.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":72.0,"width":400.25806,"height":66.58064}}},"text_order":14,"token_count":148,"style":null}
```

*Sensemaking* tasks require reasoning over “*connections (which can be among people, places, and events) in order to anticipate their trajectories and act effectively*” (Klein et al., 2006). LLMs such as GPT (Achiam et al., 2023; Brown et al., 2020), Llama (Touvron et al., 2023), and Gemini (Anil et al., 2023) excel at sensemaking in complex domains like scientific discovery (Microsoft, 2023) and intelligence analysis (Ranade and Joshi, 2023). Given a sensemaking query and a text with an implicit and interconnected set of concepts, an LLM can generate a summary that answers the query. The challenge, however, arises when the volume of data requires a RAG approach, since vector RAG approaches are unable to support sensemaking over an entire corpus.
```bgraph-paragraph
{"id":"547fd459-0a0b-5c87-89eb-cf87674e1108","node_type":"Paragraph","location":{"semantic":{"path":"4.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":142.45161,"width":400.25806,"height":89.03226}}},"text_order":15,"token_count":190,"style":null}
```

In this paper, we present **GraphRAG** – a graph-based RAG approach that enables sensemaking over the entirety of a large text corpus. GraphRAG first uses an LLM to construct a knowledge graph, where nodes correspond to key entities in the corpus and edges represent relationships between those entities. Next, it partitions the graph into a hierarchy of communities of closely related entities, before using an LLM to generate community-level summaries. These summaries are generated in a bottom-up manner following the hierarchical structure of extracted communities, with summaries at higher levels of the hierarchy recursively incorporating lower-level summaries. Together, these community summaries provide global descriptions and insights over the corpus. Finally, GraphRAG answers queries through map-reduce processing of community summaries; in the map step, the summaries are used to provide partial answers to the query independently and in parallel, then in the reduce step, the partial answers are combined and used to generate a final global answer.
```bgraph-paragraph
{"id":"caba63ab-ead3-596b-a291-30cb77734a4d","node_type":"Paragraph","location":{"semantic":{"path":"4.5","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":234.58064,"width":400.25806,"height":122.32258}}},"text_order":16,"token_count":266,"style":null}
```

The GraphRAG method and its ability to perform global sensemaking over an entire corpus form the main contribution of this work. To demonstrate this ability, we developed a novel application of the LLM-as-a-judge technique (Zheng et al., 2024) suitable for questions targeting broad issues and themes where there is no ground-truth answer. This approach first uses one LLM to generate a diverse set of global sensemaking questions based on corpus-specific use cases, before using a second LLM to judge the answers of two different RAG systems using predefined criteria (defined in Section 3.3). We use this approach to compare GraphRAG to vector RAG on two representative real-world text datasets. Results show GraphRAG strongly outperforms vector RAG when using GPT-4 as the LLM.
```bgraph-paragraph
{"id":"00ffea2c-d2d9-561a-a848-9ce13259b5f4","node_type":"Paragraph","location":{"semantic":{"path":"4.6","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":360.0,"width":400.25806,"height":99.870964}}},"text_order":17,"token_count":195,"style":null}
```

GraphRAG is available as open-source software at https://github.com/microsoft/graphrag. In addition, versions of the GraphRAG approach are also available as extensions to multiple open-source libraries, including LangChain (LangChain, 2024), LlamaIndex (LlamaIndex, 2024), NebulaGraph (NebulaGraph, 2024), and Neo4J (Neo4J, 2024).
```bgraph-paragraph
{"id":"533cc62f-5e0b-5ff3-82bb-95c2350a7952","node_type":"Paragraph","location":{"semantic":{"path":"4.7","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":463.7419,"width":400.25806,"height":45.677418}}},"text_order":18,"token_count":82,"style":null}
```

# 2 Background
```bgraph-section
{"id":"81d55c7b-072a-570c-be1f-dac52c532db0","node_type":"Section","location":{"semantic":{"path":"5","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":525.6774,"width":84.38709,"height":13.935484}}},"text_order":19,"token_count":3,"style":null}
```

## 2.1 RAG Approaches and Systems
```bgraph-section
{"id":"cf38b6a4-aae8-5af6-bdc1-f77b160313a0","node_type":"Section","location":{"semantic":{"path":"5.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":551.22577,"width":156.38708,"height":13.16129}}},"text_order":20,"token_count":7,"style":null}
```

RAG generally refers to any system where a user query is used to retrieve relevant information from external data sources, whereupon this information is incorporated into the generation of a response to the query by an LLM (or other generative AI model, such as a multi-media model). The query and retrieved records populate a prompt template, which is then passed to the LLM (Ram et al., 2023). RAG is ideal when the total number of records in a data source is too large to include in a single prompt to the LLM, i.e. the amount of text in the data source exceeds the LLM’s context window.
```bgraph-paragraph
{"id":"f0beca57-b8ca-56fb-adb8-421c664753df","node_type":"Paragraph","location":{"semantic":{"path":"5.1.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":572.9032,"width":400.25806,"height":66.58064}}},"text_order":21,"token_count":148,"style":null}
```

In canonical RAG approaches, the retrieval process returns a set number of records that are semantically similar to the query and the generated answer uses only the information in those retrieved records. A common approach to conventional RAG is to use text embeddings, retrieving records closest to the query in vector space where closeness corresponds to semantic similarity (Gao et al., 2023). While some RAG approaches may use alternative retrieval mechanisms, we collectively refer to the family of conventional approaches as *vector RAG*. GraphRAG contrasts with vector RAG in its ability to answer queries that require global sensemaking over the entire data corpus.
```bgraph-paragraph
{"id":"24a5fcca-d04d-5e41-a2be-409538ab71f1","node_type":"Paragraph","location":{"semantic":{"path":"5.1.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":641.80646,"width":400.25806,"height":79.741936}}},"text_order":22,"token_count":168,"style":null}
```

2
```bgraph-footer
{"id":"ffd1d701-0e8c-5c31-a582-53ac924dffe2","node_type":"Footer","location":{"semantic":{"path":"5.1.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":2,"bounding_box":{"x":298.8387,"y":737.80646,"width":10.838709,"height":12.387096}}},"text_order":23,"token_count":1,"style":null}
```

GraphRAG builds upon prior work on advanced RAG strategies. GraphRAG leverages summaries over large sections of the data source as a form of 'self-memory' (described in Cheng et al. 2024), which are later used to answer queries as in Mao et al. 2020). These summaries are generated in parallel and iteratively aggregated into global summaries, similar to prior techniques (Feng et al., 2023; Gao et al., 2023; Khattab et al., 2022; Shao et al., 2023; Su et al., 2020; Trivedi et al., 2022; Wang et al., 2024). In particular, GraphRAG is similar to other approaches that use hierarchical indexing to create summaries (similar to Kim et al. 2023; Sarthi et al. 2024). GraphRAG contrasts with these approaches by generating a graph index from the source data, then applying graph-based community detection to create a thematic partitioning of the data.
```bgraph-paragraph
{"id":"e320676f-4659-57e8-bc10-b9fad0cb67ce","node_type":"Paragraph","location":{"semantic":{"path":"5.1.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.1 RAG Approaches and Systems"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":72.0,"width":400.25806,"height":99.870964}}},"text_order":24,"token_count":212,"style":null}
```

## 2.2 Using Knowledge Graphs with LLMs and RAG
```bgraph-section
{"id":"dffba795-f47d-5bc7-b988-6829064e1ba8","node_type":"Section","location":{"semantic":{"path":"5.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.2 Using Knowledge Graphs with LLMs and RAG"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":185.80644,"width":227.6129,"height":12.387096}}},"text_order":25,"token_count":11,"style":null}
```

Approaches to knowledge graph extraction from natural language text corpora include rule-matching, statistical pattern recognition, clustering, and embeddings (Etzioni et al., 2004; Kim et al., 2016; Mooney and Bunescu, 2005; Yates et al., 2007). GraphRAG falls into a more recent body of research that use of LLMs for knowledge graph extraction (Ban et al., 2023; Melnyk et al., 2022; OpenAI, 2023; Tan et al., 2017; Trajanoska et al., 2023; Yao et al., 2023; Yates et al., 2007; Zhang et al., 2024a). It also adds to a growing body of RAG approaches that use a knowledge graph as an index (Gao et al., 2023). Some techniques use subgraphs, elements of the graph, or properties of the graph structure directly in the prompt (Baek et al., 2023; He et al., 2024; Zhang, 2023) or as factual grounding for generated outputs (Kang et al., 2023; Ranade and Joshi, 2023). Other techniques (Wang et al., 2023b) use the knowledge graph to enhance retrieval, where at query time an LLM-based agent dynamically traverses a graph with nodes representing document elements (e.g., passages, tables) and edges encoding lexical and semantical similarity or structural relationships. GraphRAG contrasts with these approaches by focusing on a previously unexplored quality of graphs in this context: their inherent *modularity* (Newman, 2006) and the ability to partition graphs into nested modular communities of closely related nodes (e.g., Louvain, Blondel et al. 2008; Leiden, Traag et al. 2019). Specifically, GraphRAG recursively creates increasingly global summaries by using the LLM to create summaries spanning this community hierarchy.
```bgraph-paragraph
{"id":"938d7975-ddcb-5767-b475-8f82d6882dd3","node_type":"Paragraph","location":{"semantic":{"path":"5.2.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.2 Using Knowledge Graphs with LLMs and RAG"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":206.70967,"width":400.25806,"height":186.58064}}},"text_order":26,"token_count":407,"style":null}
```

## 2.3 Adaptive benchmarking for RAG Evaluation
```bgraph-section
{"id":"5fb4493a-cf8d-59ab-8475-8b136b3d3ace","node_type":"Section","location":{"semantic":{"path":"5.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.3 Adaptive benchmarking for RAG Evaluation"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":406.4516,"width":217.54839,"height":12.387096}}},"text_order":27,"token_count":11,"style":null}
```

Many benchmark datasets for open-domain question answering exist, including HotPotQA (Yang et al., 2018), MultiHop-RAG (Tang and Yang, 2024), and MT-Bench (Zheng et al., 2024). However, these benchmarks are oriented towards vector RAG performance, i.e., they evaluate performance on explicit fact retrieval. In this work, we propose an approach for generating a set of questions for evaluating global sensemaking over the entirety of the corpus. Our approach is related to LLM methods that use a corpus to generate questions whose answers would be summaries of the corpus, such as in Xu and Lapata (2021). However, in order to produce a fair evaluation, our method avoids generating the questions directly from the corpus itself (as an alternative implementation, one can use a subset of the corpus held out from subsequent graph extraction and answer evaluation steps).
```bgraph-paragraph
{"id":"2a93f6d8-6196-52c4-a1d4-380a4517014c","node_type":"Paragraph","location":{"semantic":{"path":"5.3.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.3 Adaptive benchmarking for RAG Evaluation"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":427.35483,"width":400.25806,"height":99.870964}}},"text_order":28,"token_count":217,"style":null}
```

*Adaptive benchmarking* refers to the process of dynamically generating evaluation benchmarks tailored to specific domains or use cases. Recent work has used LLMs for adaptive benchmarking to ensure relevance, diversity, and alignment with the target application or task (Yuan et al., 2024; Zhang et al., 2024b). In this work, we propose an adaptive benchmarking approach to generating global sensemaking queries for the LLM. Our approach builds on prior work in LLM-based persona generation, where the LLM is used to generate diverse and authentic sets of personas (Kosinski, 2024; Salminen et al., 2024; Shin et al., 2024). Our adaptive benchmarking procedure uses persona generation to create queries that are representative of real-world RAG system usage. Specifically, our approach uses the LLM to infer the potential users would use the RAG system and their use cases, which guide the generation of corpus-specific sensemaking queries.
```bgraph-paragraph
{"id":"7f30e8e6-42e8-5bf5-9b63-ebb0e5c09bcb","node_type":"Paragraph","location":{"semantic":{"path":"5.3.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.3 Adaptive benchmarking for RAG Evaluation"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":531.09674,"width":400.25806,"height":109.93548}}},"text_order":29,"token_count":235,"style":null}
```

## 2.4 RAG evaluation criteria
```bgraph-section
{"id":"efcf448f-807b-5ecf-a8e4-8448319ca9c4","node_type":"Section","location":{"semantic":{"path":"5.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":654.9677,"width":128.51613,"height":12.387096}}},"text_order":30,"token_count":6,"style":null}
```

Our evaluation relies on the LLM to evaluate how well the RAG system answers the generated questions. Prior work has shown LLMs to be good evaluators of natural language generation, including work where LLMs evaluations were competitive with human evaluations (Wang et al., 2023a; Zheng et al., 2024). Some prior work proposes criteria for having LLMs quantify the quality of
```bgraph-paragraph
{"id":"1d3c8a18-725d-5d01-8e1b-56b30c71b847","node_type":"Paragraph","location":{"semantic":{"path":"5.4.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":675.09674,"width":400.25806,"height":45.677418}}},"text_order":31,"token_count":93,"style":null}
```

3
```bgraph-footer
{"id":"7aa95163-1b55-5d13-95a1-8badbe41a8cd","node_type":"Footer","location":{"semantic":{"path":"5.4.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":3,"bounding_box":{"x":298.8387,"y":737.80646,"width":10.838709,"height":12.387096}}},"text_order":32,"token_count":1,"style":null}
```

![img-0.jpeg](img-0.jpeg)
```bgraph-image
{"id":"2d6af4a4-34a8-50ee-ae22-493db1bf143f","node_type":"Image","location":{"semantic":{"path":"5.4.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":4,"bounding_box":{"x":130.8387,"y":68.90322,"width":332.12903,"height":206.70967}}},"text_order":33,"token_count":0,"style":null,"image":{"id":"sha256:3ce9c2820296e9ae759a12631f78ba066ecfe76d41ff118873704859fcb1562f","annotation":null,"base64":"data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAgGBgcGBQgHBwcJCQgKDBQNDAsLDBkSEw8UHRofHh0aHBwgJC4nICIsIxwcKDcpLDAxNDQ0Hyc5PTgyPC4zNDL/2wBDAQkJCQwLDBgNDRgyIRwhMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjL/wAARCAELAa0DASIAAhEBAxEB/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwD2fXNZTR44EW3e5u7mTy7a1jIBkYDJJJ4VQASWPTtyQDl/2t4rJ3LoOigNzhtWlz+OLb/PrRrPPj3Qhjj+zr5uR332vP16/wCTWrWsIJq7EZf9r+Lv+gHon/g2l/8Akej+2PF3/QD0T/wbS/8AyPWpRz2OD61fs4hcy/7X8Xf9APRP/BtL/wDI9H9r+Lv+gHon/g2l/wDkemnX7X7XLbxw3DpBcpayyogKJKwUhTzngMuTjHIGcital7OIXMv+1/F3/QD0T/wbS/8AyPR/a/i7/oB6J/4Npf8A5HrU9faij2cQuZf9seLv+gHon/g2l/8Akej+1/F4/wCYFon/AINpf/ketT6HntWI/iezhuNRia3u1XTmC3MqxhljDKHBwpJI2nJI9eelHs4hcn/tjxd/0A9E/wDBtL/8j0f2v4u/6Aeif+DaX/5HrRgmiuYIp4JFkhlXdG6tuDKRkEH0NPo9nELmX/a/i7/oB6J/4Npf/kej+1/F3/QD0T/wbS//ACPWp3x3FGQRnPFHs4hcy/7X8Xf9APRP/BtL/wDI9H9r+Lv+gHon/g2l/wDketT/ABxTDNEJ1g8xfNZS6pn5iPXHpyPzo9nELmd/a/i7/oB6J/4Npf8A5Ho/tfxd/wBAPRP/AAbS/wDyPWpRT9nELmX/AGv4u/6Aeif+DaX/AOR6T/hJ9T06SN9d0m3t7J2Cm6srxp1iJOAZA0aFV/2hnHfA5rVHXisPxkB/wg3iDg4/s24P5Rtik6aC52QpaZH/AKtPoKfWAwooooAKKKKACiiigAooooAKKKKACqepajbaTp09/eOUggXc2Bkn0AHck8AdyQKuVzHjrnQLdex1OwBHr/pUXFAmQLr/AImuF8638PWEUTcot3qTRy47blSF1U47bj9aX+2/Fv8A0A9E/wDBtL/8j1oHk85z6nvRSuQ5sz/7b8W/9APRP/BtL/8AI9H9t+Lf+gHon/g2l/8AketCj8ce9K4udmf/AG34tHXQ9E/8G0v/AMj0f234t/6AWif+DaX/AOR6l1DU7XS7ZZ7pyiyOscaICzO7cBFUdSaoXXiSCwgupryyvIfs0AuHDKpymSDtKnBxjkdenrTuHMy1/bfi3/oB6J/4Npf/AJHoOt+LQcHQtE/8G0v/AMj1oZ7jk4z7kfSqmmajDqumxX9sHEMwyoYYI5x296LhzMi/tvxb/wBALRP/AAbS/wDyPR/bfi3/AKAeif8Ag2l/+R6kvNQhsbuxt5UfdeTGGNlAI3hGfnPPRWqe5uIrS1muZm2xRIXdsdABk0XDmZU/tvxb/wBAPRP/AAbS/wDyPR/bfi3/AKAeif8Ag2l/+R6sWN7b6jYW99auJLeeMSIw7ggEfoasUXDmZn/234t/6Aeif+DaX/5Ho/tvxb/0A9E/8G0v/wAj1oYoyMZzRcOZmf8A234t/wCgHon/AINpf/kej+2/Fv8A0A9E/wDBtL/8j1odDg0UXDnZn/234t/6Aeif+DaX/wCR6P7b8W/9APRP/BtL/wDI9aFFK4c7M1te8SwI0lx4dsZYgMstnqTSSkd8K8KAn2LCuj0y/t9U0+C/tJPMt7hA6Ngg4PYg8g+oPIORWdVbwJk+Gmz21C/AxwP+PubimioyuRa1/wAj7of/AGDb/wD9GWtalUfEdjerqGna1YWzXctkksM1shAeSGTYWKEkDcGjQ4J5G4d6pjxGgHOka4D3H9mTf4H/AANb05JKxTNqjHtWN/wkkf8A0CNb/wDBXN/8TR/wkkf/AECNb/8ABXN/8TV86EY15pN+fEkOq6ZY3FleteIl3KsqGC7tlOCzqTy237vAYH25qldeGNYvNKNqYgl8qX0c9yZAVuVkVxGM53dWjb5gNvln2rpv+Ekj/wCgRrf/AIK5v/iaP+Ekj/6BGt/+Cub/AOJpXXcDl5tF1a+mllk0adUlutPk8t5oslYz+9Jw/pxjOSPXpV4aPqCNqNpDpzRToZW03UUmVY4UaHZGm0HcNucbSpXjd1ra/wCEkj/6BGt/+Cub/wCJo/4SSP8A6BGt/wDgrm/+Jouu4FfwtYy2sM80um3FhJOI/NhmliZS6jBKrH8oHT5j8xx0HBqraRavY6/4muo9HllF7NE1qzTRhH2wqnzHfkLkHsTjsa0v+Ekj/wCgRrf/AIK5v/iaP+Ekj/6BGt/+Cub/AOJovEDlbfwfqunaLNpKE3En9n28NjeI4UWsybtzAEgqNxDcZyOD0FXptG1C+sdZsrnS2N9It4tvqDTrslWXd5Y4bdwCi4ZQBsyD0rYj8U28u7y9M1l9jFG26bKdrDqD8vBp/wDwkkf/AECdb/8ABXN/8TRddwOZm0bVrxrh20aeNZbrTZBG80WdsTjzTw+Pugj1IPemHw5rEbBrCwa0uPtl+I5xJGoihkjk8roxITeUIQdMdK6n/hJYs4/snW8/9gub/wCJo/4SWP8A6BOt/wDgrm/+Jouu4HNx+H7mcafL/YdzDm9hku4Z5YCoVY5FZgqNtKncoPdh1AwDUdj4d1GKO2NzpM/y6ZeWX7qWISwgyZjVW3ED5OAQSB0OK6j/AISWP/oE63/4K5v/AImj/hJYs4/snW//AAVzf/E0XXcCTw7bX1rpzQ36xCQStteOMR+YvZigJCk9wDjuBzga1Yv/AAksf/QJ1v8A8Fc3/wATR/wkkf8A0Cdb/wDBXN/8TT5kBtVieMv+RH8Qf9g25/8ARTU7/hJIz/zCNb/8Fc3/AMTVPU5rzxPp8+iWWl38SXiGG4ubu3MCQxMNrsN2CzFSdoUcHqRSc1YdjuY/uL9BT6aOABinVzjCiiigAooooAKKKKACiiigAooooAD0rmfHJ/4kNt/2FLD/ANKoq6asbxNpc+r6HLbWrIl0skVxAz/d8yKRZEDexKgH2NAmR0VjJr0sahbvQdZgnUYdEsmmAPTh4wQ31B780v8AwkKf9AfXP/BXN/8AE1Jm4s2KOax/+EhT/oD65/4K5v8A4mj/AISFP+gPrn/grm/+JosLlZH4k027u5dKvrJBLLp14Lgw7gDKu1kYDPG4B+M4HGKx/ENhrerJrQgs7oW0+l/Z4bWSdBvnLHJxu29NvJ49K3P+EhT/AKA+uf8Agrm/+Jo/4SFP+gPrn/grm/8AiaB2Zk3Oj6ncXt45hbzpNRt7q1vA4Hkwjy/MTrkcLKMAYPme5xlWvhfUbPTI3g0sC8isbmG5TzVT7YzsNg3KwPA3EEkYzjPJx1f/AAkKf9AfXP8AwVzf/E0f8JCn/QH1z/wVzf8AxNAWZg2emawuoWZfTp1gi1t7vLtCBHE1syZ2q3HzvyB7muj1eC6vDaWkEbrC8oeaYbT5ap8yggnJy20EDIxnNRf8JCn/AECNb/8ABZN/8TR/wkUf/QI1v/wWTf8AxNAWZzf9h6vBo17opsZp7aDVLe4sZhJGN0PnJLIoG/K7cSADjIIFKmh6tCmy2sDEsF7cNBBIY2t2haRGCkBsp0JVl5XByOcHo/8AhIY/+gRrf/gsm/8AiaP+EhT/AKBGt/8Agrm/+JoCzMC58N6tba1JcaakIhguzPbFyMkTrslB9kIMnvkAdKnOh3MfiHY+n3EtlC0L2U0UsQSJERVaNs/PjIY4AIbdg1sf8JDH/wBAjW//AAWTf/E0f8JDH/0CNb/8Fk3/AMTQFmQ+DtJbSfDdrFcWX2a9Mai4BKszMOBuZSQ2AeDz+Fb1Y/8AwkUf/QI1v/wWTf8AxNH/AAkMf/QI1v8A8Fk3/wATQLlZsUVj/wDCQx/9AjW//BZN/wDE0f8ACQp/0CNc/wDBZN/8TRYOVmweKr+A/wDkWX/7CWof+lk1Zza9NIuyz0LWZ7g8IjWbwKT7u+FUe/Wt3w3pU+kaDBaXDI1yXknnKD5fMlkaRwuf4QzkD2ApouKNmiiimWFFFFABRRRQAUUUUAFFFFAHPadc/Y7DXroJ5hhup5NmcbsKDjPbpUdj4p+2GziTT7mSWVYvPMCFo4Hkj8wBm7DBXJ7bl684s6LGk0WsRSAMj30ysD3BApbXwxpdlOk0Ec6usSQj/S5SGVc7Sw3YYjPBOSMDHSgDPTxa1zaQTwac4ZtQSwkjlkAaKQkhwcZGVxj0OcgkddfV9XTSlhX7PNczzFhFDAuWbapY/wAsfiKhXwtpC7gls6gvDIds8n34gAjD5uCAACRywHOasahpNpqn2f7VHJut5PNieKZ4nRsEEhkIPIYgjoQaAKEXiQSam1h9huI5xZreKs2EJjI7jqCG+UjqCfSqun+L2u7aBjpVxvOmw6jLtkj2IkiuVALMCTmMjpWqfD+mteQ3TW7GeBy0b+a4K5QIR15UgfdORnnGarJ4Q0ZLaS2FvL5UlolmwNzKT5KElFB3ZGNxwRyM9aAG2/imC8zHa2s89whm3wxlcgRSbCQSQDk8jnkZ6HiqK+Lo0SO6vY5raBZ79G8sq6FLZpAxbI3f8syQF/OtFPCejRzpOsE6zI8km9bubJ8wguCd3zKSoJU/LkZxUn/CMaQJA7WhbbJNKEaZygaUHzPlLbcNk8YxyTigCnN4thgu47NrC6+2SmHZEGQ5WbeUOS2P+Wbg+hHcYNa+kalDrGlWuoQIyRzoHVX6r7cVUXwzpYaAm2cvA8bRuZnLL5YYIMlskAM3B4+Y9STV7TtOttKsorO0V0gi4RXkZyO/ViT39aALlFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAVwyw6/wD8Lca6Lw/2V/ZwhNv5jZCZLebjpu3jbj+6c57V3NYg/wCR1k/7Bq/+jDQBbuNa0y1leO4voI2QMW3OBjau5vyXk+g5ofWNNiSVnvYFWEgSZf7pK7hn/gPP05rkr7wvef2nb2f2hptOubm9lLpbndb+fHJu3PuII3OcfKOMelWR4Ku/tK3MmpWks4lRyklgWiKrCImBTzOpABBzwR0IOKAN8+I9FEjR/wBqWhdSoIWUHBbbtB+u9P8AvpfUVeW6heKSRHBSMkMQOmOtczN4dSy0vW4poWvYNTZFFvaQiNo8QpCMEtgAbAwPy7fwrUtNGMNlaCW5le9hjOZRK6pJK2S7tGCFbLEnBH0xQBDpfiazvtLtNRnkhtoryNZrcPLljGyqQWyBtOWx3HTnnFX5ta023kdJ763jZAWYM4GACFJ/AsoPoSPWuej8EY0hNObUd8b6MmkXB8n/AFkaggOoLHa2Gfru6j05l/4RS5OlajpjalGbW5eaS3Jtv3kLSSGQ7m3fONx/2SRwSTzQBvjVbAuYxdxFhIYiobkMCAQfoWUfVh60w6zp2+aP7bAXiA3rvGRlto492BX3Ix1rBHhCdp/Pk1KMT/a3vFnggMboW2ZC/OeCEIIOQ24Z+7UcPgqaC2t44721WWzuEltZhZAPtDE7JTv/AHgwcZBXnnGaAN7QtWXWtNF6sfljz5odu7cP3crxkg4HXYT+NalZWg6ZJpGmm1kuEnZrmecusZQfvJXkIwSehcjr2rVoAKKhury2sbWS6uriKC3jXc8srhVUepJ4FYP/AAnnhj/oMQH3UMQfxA5o1A6Siub/AOE98Mf9BaL/AL4f/Cj/AIT3wx/0Fov++H/woswOkorm/wDhPfDH/QWi/wC+H/wo/wCE98Mf9BaL/vh/8KLMDpKK5v8A4T3wx/0Fov8Avh/8KP8AhPfDH/QWi/74f/CizA6Siub/AOE98Mf9BaL/AL4f/CkPjzwuykHVYSCMEFH5/wDHaLMCTTblLSw125Z1VYrqeQsQWAAUHOByenauft/GV3e3MFvBfaejSagbbzWXd+7Nl9oDYD4HzZHUjjHJBNZ3gq68L+E5dXf+3I5ftd2xt8+Y3k2w/wBXEMjgLlq6z/hO/C4Gf7Wj47bH/wDiaLMDlj4qbX7bR5LiW2tZRdaZMsAxukEqo7MpJzgMWTv9xu5GOt8Qa2dLvLO1a6trJLqObbd3I/drKm3ZGeRy24nryEIHJBqP/hO/DA/5i8RIGM7H/wDiaX/hO/DH/QWiOOmUf/CizA5yLW7rTdZ1OO0ntnF3q00Ih2fMJFsFk3D5v70eCMfxHvU0HjW7vJraO3vbDbPLZpu2bsebE7MB845DL0PToeea3P8AhPPDGONViz/uN/hQvjvwwMY1aIcf3H/woswOX0vxYJfEVneXM9vCt9ptqk9zHgxo3mzgcZ+UORgE5AJAPUVJP4wvLPT52s5dLi8pNYuHURcM9vcfL0cY3hiWPOSSeK6T/hO/DH/QVh/74b/CgeOvDHbVovxRv8KLMCvZ+Iru98WNp8Vxp/kRyDfF5imRomgR1YDdu3b2IPy42gHvk9dXDxeIfB8V+92NZLM0hlEbmRlVzxkZXj6dB2Fb+neKtC1e6FrY6pbTXBBZYd2HIHUhTyQKLMDZoozRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRSHpigBaK52Xxz4YhlaI61auykqfKYyAEcEZUEZFN/4T3wx/0Fov8Avh/8KAudJRXN/wDCe+GP+gtF/wB8P/hR/wAJ74X/AOgtF/3w/wDhQFzpKK5v/hPfDH/QWi/74f8Awo/4T3wx/wBBaL/vh/8ACgLnSUVzf/Ce+GP+gtF/3w/+FH/Ce+GP+gtF/wB8P/hQFzpKxR/yOz/9g5f/AEYaqf8ACe+F8f8AIWix/uP/AIVk/wDCZeHh4pe7/tKIwfYRFvKN97zCcdPSgLmxN4rtIZ7iHyZDLbXUFvKmVDq0sixI2087SXBB6EBu4xVW68b29tDLKdPu5FiivZ32lOEtJRHIeW5OSCPXj3qlc614Euy5nvldnP3i8u5fnEmVYcr84BGCMEcYqu158PXjkha8ykkVxA4M05yk7Bphyf4mAJPXPSgLo3G8VKLpbRdNumumvfsQjDxj5vs5uAc7sAFMjjuD7ZTT/GFvqMCyx2c6LJbW11GJWRd0c5ZVJ+YgYKnI6+mTxWUNY8Ci+W8Gofv1n+0BzNP/AKwRGHd167CV/wA5qBLv4eRxQRx3pQQRQwxET3AKJES0YBzngk89cHBNAXRoy+NoJ9Ivbm3s9QWODS1v5LhPKDIjpIwIDNywMRGCMZx1Fad14gFrqclkbGeV47Z7pjGU+WMDjOWGNxDKATyVJ4AJrnIb34ewWl1aR3mILq2FnMpnnO6Eb8JnOQP3jdOfmp82p+ApxeCTUNwvF23A86cCRdnl4IzjG0njseevNAXRrp4qFzqENrBaswLXKSyBxlDCF+72bdvBB49Mc02z8YWb6bb3AjuZIRDamaVgm6MzgbNyrgHqC23gZ4z2yotT8BRTeal+wl3u/mGectl1CtyTnkKM+pGevNOg1TwBbRQxxXcXlxRQxBWMrArEcx7geGKnoTkj8KAuje0bxNFrN2sC2lzbs8byoZSpDBJDG33Sec/hzmt+uK0nV/A9nfQPZanDFMqNBG01zJgh5N7DMhwSW5z17V2uc9KAOS8RxpeeLtBsZ1D2yw3V75bDIMsZhVCR3wJWI9Dg9q1v5fyrL1o/8V7oX/YNvv8A0Za1qVvT2Exc0ZpKK0ELmkzRQOKAKcmr6bDK0Uuo2iSISrI0yggjqCM9auKwdQysGVgCCOcg9K4/WEsrbx14Yt9sEfmLesUOPmLKp6H1Of5VR0/W7m5sLoRaza2Jhgki+yKiySWcqsFjBQoCOm3aclsqRipuB324YzkY9abLKkETyyuscaDLO5wF+p7Vw0Ou3N2PD01zqMlrP/asttqEQKKsTiGXETHuu4JtPfcDxkY0PFTeVrvhq4u+NIiuZDcOw+SOQxnyS/oM5wTwDtzTuB0UWp2MxURXttIWbaoSVTk+g568H8qs5rjPEX9my6notxZXUEdzPqkUUlxCVPKwTADJypIDY6H+HI6Zx5vGWoRaDJcnVIlf+yb6SCTZGBLcQS7VwMYJIySvv0pXA9Lzg4zzTIZ4riISwSpLGSQHRgwODg8j34rh5vEt3HrcwGqobaLUrO2FviPayTRruycZ6kkYI6c5HFUvDevtHpt/aJfW9pMsl69iJGUQzk3EmX3YJG0kAjHAIY5DcFwPSaM1k+Hr+TUdJWeZZklEjo6y7cqQ5GAU+VgMYDDqPcmtWmgFzRmkopgGT9awPGkSnwlqV2OLixt3u7aXndHLGpZWHvkD8OO9b9YnjL/kRvEH/YNuP/RbUpLQZ16kkA88gU+mR/6tf90U+uUYUUUUAFFFFABRRRQAUUUUAFFFFABXNeOJHXwy0KsyrdXVtaSFSQfLlnjRwCOmVZhntmulPQ1zPjk/8SG2/wCwpYf+lUVAmTQQxW8CQwxrHHGoVEQYAA4wB2HoO1PooqTG5DdXdvY2z3N1MkMCDLSO2AO1QWuq2d3ctbRSMJ0QOY3iZG2k4Bw2Djg81k+MLW6nh0m6gha4jsdRiuriFF3FkAYEhR94qSrADk7eKdrN3Nc2Cz6Zb3RdJoVmcQOsnkGRfMCAgMx2A9BnHfOKBnQ+vtRnjNcQyao+vQwPJqn9lNeTCMq0wZYvs6nDMBuA83dtJ57DIxWWJ9fXQg//ABOjeTaLE8n7uYsLkOAcDHytgnIA5HUd6APS+f6UcntnNcMw1KDWJvJfV2hi1iFIt/nlWt3hXzCTjld5fk52kDpgVntca9/ZE0YXWftUei3qE+VNzdB1MW04+ZsZwR1Hc0DPRJ7iODaZGIDOsa4UnknA+n1qUHOcdutcNLLqkF1PDANXeFdTsnjJSZ8xts84AkY2cMSPuj0HSm28V/puhLeW63xuLDVrmSS1Z5FFzC88gwu7h/kdXXBxkD1OQR3Y5x3B/WoYru3nnnt45kea3IEiA8oSMjPpxzUERbTdGVrhprmSGHdKUDSPK2CSQOSxJzx+VcraW+paR4l07U5beSSPVY2gvvs8LsY3JMkbuMcBSzJk9ivpQCO3ozxntXnto+sx+STLq8b3EGoRyPJBPKsRE4EDbT0+TOMYJGTnOKuTRa9PY2Uka3sM8j3FjcLHcyELvzsuASVbCsBjOCFbvgUDO26DJoOR14riGm1tNM0+e8jvYvMlEN4IEllZAkbIHCxkOA0gLZXnBTPelit783d6kl7rQSDSYTBOYnG6X98rMUXCu+3yiU6k4yMjgEdtRWT4ekvJNOf7fA0dwsrKxLu6vyPmUv8APg+h6e4ANa1ArjJ4IrmCSG4jSWKQFWjdQykdwR3qLwMzP4UgjZ3YW1xc2sbM2W8uKeSNAT3IVAKsVX8B/wDIsv8A9hHUP/SyamjSDK/iORbLxZoWoXDeXaGC5sjKeFWWRoGQE9gfKYD3IHetb/DNaFzawXlu9vdQRzwSLteOVAysO4IPBrB/4QLwrk40GyAJzhY8D8q1jPlViy9RVP8A4QLwr/0A7P8A74o/4QLwr/0A7P8A74qvaisXKKp/8IF4V/6Adn/3xR/wgXhX/oB2f/fFHtQsXP8A0LOenf8AOkxgHA688VU/4QLwr/0A7P8A74o/4QLwr/0A7P8A74pe0CxcPXp3/L8etGM/5/zmqf8AwgXhX/oB2f8A3xR/wgXhX/oBWn/fFHtAsWwMKAB8o6Advz/nRwff0/z+tYGj+C/Ddw2o+do1o/l3skaZTooxgfrVqLwj4Nmvbi0TR7Q3EAQyoYz8oYEjrweh6elHtEFjW/E+3r9frSfgMHtVT/hA/Cn/AEA7P/vij/hA/Cv/AEA7P/vij2iCxbxj6dee/wDnrS1mw+CvCcxk2aFago5Rswlee/XqPepW8CeE1GTolkO3K96ftQsXaKzp/BPhG3t3lfQrUogJIWIuemeg5P0FRW3hLwbdzXMMej2bSWsgimBiI2sVDAc9flZTn3o9qFjWPQ1geNZU/wCET1KzBDXOoW8lnbRD70ssilVUD8ck9hzWkfAfhXBI0Oz/AO+TT00Tw34Xin1ePTLSzFvGzPcpBl0QD5jkDOMdcdqTqhY3kzgc9OtPpg5x19s0+shhRRRQAUUUUAFFFFABRRRQAUUUUAFc144jc+GWmRHkFrdW13IqLk+XFOkjkDuQqk49q6Wg9DQDOegnhubeOe3lSWKVdyPGcq4zwQakqObwR4YnmeV9Bsd7nLFYgoJ98YzTP+EB8Kf9AKz/AO+KVjNwJ+lGO1Qf8ID4U/6AVn/3xR/wgPhT/oBWf/fFFg5CfGKKg/4QHwp/0ArP/vij/hAfCn/QCs/++KLByE9GKg/4QHwp/wBAKz/74o/4QHwp/wBAKz/74osHIT1WurC2u7iCaeLe0Dlo8scBvp3xgEE9O1OPgLwoBn+wrP8A75rkBbeEz8Sz4Y/4R20+yi1z9p8lsfafveVu6f6sbvwNFg5Dtvw5oqD/AIQLwp/0A7P/AL4o/wCEC8Kf9AOz/wC+KLByE9L+FV/+EC8Kf9AOz/74o/4QLwoOmhWef92iwchPjFFZ1j4S8GajAJrTRbV4eNsnksquCAQykj5gQRyOOtWv+EC8J/8AQDs/++KLByE+PSioP+EC8KHpodn/AN8Uh8B+FMHGh2fTP3aLByD7i4htLaS4uJkhhjUs8jsAqgdSSaTwRFJH4WiklRovtFzdXSK4w2yW4kkTIPQ7XHHvUGmeF/CVxsvrDRbJhHK6K7Qco6OVbhhwQyn8q6kDHAAFCKjGwtFFFMoKKKKACiiigAooooAKKKOlAHN2qXkmk+Io9OkWO+a4uBbu3RZCg2k+2cVhWmk6yNYW8GmXVvZSPa/aoJLlGeVUimVuQ5Bw7RsQT8wBPJ4PTaG6Rpq7u6oi30pZmOAAAMk1Zk13TIUZ5b6GMJE8zb3xtRMb2OeyhlJPbcPWgDipdD8SBZN6X8siwweQY7/bjbdO5UkuMsIWUFiOdpGT3lu7DxJFbTQWmn3jbJrl7eQXaZVfNjaMYMncbwCeVGVwu6uufxDpEVy1tJqVqsyuY2j80bgwXcVx/ewQcdaafEeiq0edVs/3iRun75eVk+4evRscHv2oA5e40nXbi5cPb6iYDe3bgC9UExGDEY/1nTf0HY8/KKuX2ma7feF7OG6EdxNG1hM9tsxLvjlieXMm/aThXxjHXr69Cmt6Y6ROl/blZiVjPmD5iHCH8nIX6kDrUC6/pst5Db21/ZTO07RuouRuBCvkADO5gY2BXjAVj/DigDlNUsvE9wNV+y2F/H51pdxQmO+QFpPMUwMG8zIyu702/dxgZN/+zdX/AOEgvLhbOYWE2prNLGsqKZovsixg4DdFkX7rYzjIzgV0K69pTeYRfQHy8Kw3c5KlgMdeVBP0GagbxDYPcPFBdQO8c0MTGSTapMoBUK2CGJVgQB19RQBykeieJ447mRzeyzx2tr5H+mjD7ZJDKgBbG8xMqbmGCwznIzTdZ0PWp9EvNPgs9RubW5025igiuLqLzIJ3OVD4YLtxwuC2FBHGcV20muaVEJjLqFqixAs7NIAAA20nJPZjg+/HWrNne2t/CZbW4jmQMULRsCARwR9RQBKuSASpGex7U+iigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAQ9DWGq/8AFav6/wBnDt/00NbtYo/5HWT/ALBy/wDow0AcnqcOtyakthKlxDdXU9+kNwLgCOaMwyGEAK2Rs+QcqMEEg5PMjWHiKeaQzWOprZzyxiSGK6hE237OEJUlyuA4yRkcnIrr573SRcI9xLbedFIER3xlWZggAPYliF+pA702TxJo0ODNqlrH/rDhpQP9W2x+/wDC3B9O9AHLyaReQ2Wuy397dWj7o/sF5d32U4hi6gNjBlQkjA3ZPGGNb9lb6m2nxTSGNBOjTXFq8bPJucE7Fk3qFxkD7vbtnNTy63ockYMuoWTJ53lgNIvMirvwPU7cNj0walg13SrlGaDULaUKI2zHIG/1hITHruIIGOpBAoA4600HxBb+GrKxSKaGePQoreALcKv2W8UYLsQSCCdnI3cIwxzg2rjTtcuNM1SN7W6XUkMxguEulWOdDNvjCrv4IQBPmAx8wzg5O/deJ9HhtZXj1TT3kW3+0Kr3QRSpUspLc7VIUnODwCecVYk1nToJvJnvoIpArOQ8m0AKu5jk8YAOSfzoA5gaf4ge5aZLe4T/AImMkwgnmRozCxiIyQ+VbIYgjIGGBGGqGDTfEKRLPJDqL3AuIxexG6i8u6jEhJaIZyOCMhip2gJyOnUya/pqSxQi6j8ydZWQEkAeXjfubHyYyM5qSDWLJ4rQy3dsslyiOoSUMp3/AHcHjIY8D17UAUvCVjcadoZt7m1kt3+2XUqxuysQj3EjrypI+6w71v1QstZ03UZWjsr+2uHUbisUgYgZxnjtkEfUVfoAMj1ozXMeJLy9Oo6doljctaNerLNLcIoLpDHsDBM8Bi0iDPOBu4zg1U/4Ri3wN2p64zd2Or3AOfwcD8uKqMGwOyorjv8AhGbb/oI65/4OLn/45R/wjVrjP9pa5j1/ti5/+OVXs2B2NFcd/wAIzbf9BHXP/Bxc/wDxyj/hGbb/AKCOuf8Ag4uf/jlHs2I7GiuO/wCEZtv+gjrn/g4uf/jlH/CNWv8A0Etc6Z/5DFz/APHKPZsZ2NIeQQDg+tcf/wAIzbDrqOuf+Di5/wDjlB8M2vIOo657/wDE4uf/AI5R7NgUvBP9v30HiSDWFtYsXslvFiAlZWAw0pUn5kbK4X2IzVm68CyS2TWlvqjRIbW7sh5kTS+XDOI8hSXz8pjBGSQAcdhiT/hGbbvqOuZ/7DFz/wDHKP8AhGbb/oI65/4OLn/45R7NgWF8K3S6qL46hCSNTGoFRakdLQW2zO/PT5s/hVK38C3MFlFAdVifZb6fCD9kI/49J2lBx5nG7dj2x36VL/wjNsf+Ylrn/g4uf/jlH/CM23/QR1z/AMHFz/8AHKPZsQ248F6hNC8C6zEsTXEtzsa0JAZrhZl/5ac4wVPqDwFq3D4WuYRZY1CMm11S41EE2x+YSiYbPv8AGPOPPfHQVW/4Rm2/6COuf+Di5/8AjlH/AAjNt/0Edc/8HFz/APHKPZsCWHwrqMNgkA1iJpWuftN1N9kINy7Bg+795wOU2gHACAcjiq9n4JurSyitBqcTIkljIxNoeTbbOB+84DeWv0yevQP/AOEZtuf+JjrnH/UYuf8A45QfDNsDg6jrg7f8hi5/+OUezYxtz4Ea40tbEakFEMNxBbu0JYqsrqx3/N8+AuOoz1Nb2k6Y+nTag73Cym7u2uAFj27AVUbepz93OffpWGfDFt0Oo65/4OLn/wCOVS1O3uvC+nXGtadqWoyLZIZ7i1u7p7hJo1G51G8kq20Eggjng56UnTaFc77NFNBzg06oGFFFFABRRRQAUUUUAFFFFABRRRQAUZo7VjeJtUn0nQpbi1VWunkit4A/3RJLIsak+wLgn2BoA2Mg9xS1xieG/MAe71nWri4P35E1CWAHvkJGyqBnsBn1p3/CM23/AEEtc/8ABxc//HK0VNsVzsaK47/hGbb/AKCOuf8Ag4uf/jlH/CM23/QR1z/wcXP/AMco9mxnY0Vx3/CM23/QR1z/AMHFz/8AHKP+EZth/wAxHXP/AAcXP/xyl7NgdjRXHf8ACM2w/wCYjrn/AIOLn/45R/wjNt/0Edd/8HFz/wDHKfs2B2NYg/5HWT/sGr/6MNZP/CMWxHGo64f+4vc//HKjHhOxMxuBe6z5hTb5n9rXO7bkHGd/TOaPZsC1eeEZLiW7lhvjC1xPHceWsbGPzI5klRypbr8gUldu7OTzjFSfwLdXFpdQPqseZ7XUrcsLQ8fbJVkLY387SuAO4PUGpf8AhGbb/oI65/4OLn/45R/wjNt/0Edd/wDBxc//AByj2bAsjwtdDWk1D+0I/l1AXvli2I5Fobfbnf053Zx7dKpWXgu/0+0tIoNYjU29jZ2ZP2M4dYGZskCTI3biMA8evNSf8Izbf9BHXP8AwcXP/wAco/4Rm2xn+0dc/wDBxc//AByj2bAitfA9zDoup6c2qIRe6UumK/2Ugxqvm4b7/JxKeAR0q9deG9RuLq/uxq0SXFzbG2hcWhJtk2EfLl+pfDE9SAF7A1W/4Rm2z/yEdc4/6jFz/wDHKD4ZthnOo64Mdf8AicXP/wAco9mwJbTwld2l8LtdShMvmzyY+yttxMqBur5zuTOc9CQfWorPwR9l05LF9Q3wtDZRzYhwWNtjDLljt3BQCOe5ByaP+EZtv+gjrn/g4uf/AI5R/wAIzbEf8hLXP/Bxc/8Axyj2bAuaJ4Yn0m+t7h79Jlhgmh2CApkyTeZn75xjp3+tdLkHvXGN4bKLus9a1u2uB92Q6hLOoPbKSMyt+X41s+GdSn1rQYLu6RFuVaSCcIPl8yKRo3Kg5wCyEgemOTUyi4gZ2tf8j9of/YNv/wD0Za1pnoeKzNa/5H3Qj2OnXw/HzLX/AANala09hM4/U7m6HjuW1RNRurcaUJ/s9pc+X8/msu776dRgY/SpZ/E02juLR7N5ktLWzlmnknxJtldo8428sCmT0zk9+K2LjQbC51VtTdbgXjQiEyx3UqfJnOMKR35qu/hLRpEkR7aYiWJIZD9ql3MqOXXJ3g53EndnJyc9cVVgM631+5uDatc2yKG1mewj8icqAI/NX5xj5h8h47nB6VDaeLH+y6fqGqQGJZLO7uZPImYoqRFSdykDc2MYOfU963P+Ec0zzlm8mXcl2b1QtxIAJjnLABsfxHgDHJ4pkfhbR4WhKWrYhEyRo08hRVlOZF27iNpwOOg7YNFmBVl8Uy2tpNc3umyW9vFIi/aGZvKAZSdzEplQpG0naQMjnGcbEuowW2knUbp1jgWMSOUO8D6Y+9k4AwM81UtPDmm2VtHBAt0qRkbN93K7KACAoJY/KFYjb93npmp59G0+50T+xpLYHT/JWAQgkARrwoByCMADnORgHrRYDmfCus6ncX01pexlrtZmhuoJJfmg2lm80jBwGDxgKDjjqeSt3xrdz2kGkPBJcr5mpxRSpbSFWkRgxKjGOTgdxWgnhjS0vob5Y7j7VGu3zTdy7mXOQrnf86jspyBz6nNrUdJs9V+y/bI3b7LMs8O2Vo9si9D8pGe/XNFgOb0PxFcDRorqQtdLqN9LHpsbkmRYgrOFl2KTuBSQHgkZGeQSJF1u7GtC5lgmhX+yTM9lPKFVZBLjkk7Rxn5uOO1ak/hTRrh7hmtXRp7hbpjFNIh84Db5i7WGxsZBIwTznOadceF9Iu8me3lcmD7MT9okBKbw/wDe+9uAbd1yAc9qLAZ8Hie5vtVsLO3tUVZri5hm8yQ5UxAH5crzndnJ9MY71Fp/ivzdMU2tlJNLHZveyJNcknYHZAAxB3Mdreg4HNasPhnS4ZUmjhlEqztcBvtMpPmMNrHJPIIAyDnPUjPNEXhfSIYYooraRVjjeJT58m4o5yys27JB64J47YzRZgZ0XjHz7xFjsd1m1xbwee03z4mRWjIXb23AEE8dielVLzxNPqCWL2K+TPHqcMM9v55RwWDZjlG0FeQDnBBByCelbz+GtJknac2zh3miuDtmdRvjGIzgEAYAAwBjjkUn/CM6UTAxgkZ4GieNzPJkGPds78gb2wOnP0oswMgeKi4s7prNlf7NfFo1ufkD27hHXheeVO1sdM8c09fGTwxu93phgH2WC8QxSGVfJkbaWfCZGzvgEdCO9XLrwpYCykFlEyXKxXKwNJcSlVackuTljnLc8g4qTS/DtvZ6faxzI32uOGKOSRLiQnKAgANkHb8zcdPmPHNFmBp2Nyt5YQXKmLbNGr/upA6cjPDDhh2B+lZnjL/kRvEH/YNuf/RTVq2lrDZWkVtbJ5cMa7UQZ4H4k+lZPjMj/hBvEHP/ADDbj/0U1D2A7CP/AFa/7op9Mj/1a/7op9cwwooooAKKKKACiiigAooooAKKKKAA9K5rxx/yArX/ALCth/6VRV0tcz44/wCQDbnsNUsCf/AuKmtwL9Y3i3V59B8Kapqlqgee2gZ4wwyA3YkdwM5PsK2c/lTZYknikhmRXikUq6MAysD2INdJJiS2Y0izi1ObVL50sY3uLstKWW4URnOV6LzhhtA6DIqpeeL3sIWM2m/vDbRXMarNw6PIExuxwylh7EHg1rWmh2FlCkEUcpgRPLSKSd5EVcEYAZjjgkd+DgYFQv4Y0l7M2r2zmHYkePOk3BEYMiht2QAfQ9hSsMpReKLg6gtrNp6Rj+0G053W43ASeV5qkDb8ylSOeCCcciqS+N7l7EXK6OgDafNfov2vqkTAOpIThuVI657lelbp8OaWbgzGCTzGu/thP2iTHnBNobG7GNuBjp7VCvhLRkt0gFrL5a2klmF+1Sn9y5BZM7uSSByc49aLMCsfFg+0yItn8qX0FpvEuOJY1dXxt7bsEe3BPSqDeMJLjTLlptNaBX026u4/KuvnIhYK4yF+U/MpUgn8K2m8LaQ10lyYJPNRonBFxLgtGMIxG7BIHGSCeOtNHhLRRAIBbTCMW8tqAbqX/VStukXO/PzEA5yenWjUDPk8UXsC6zcHT45rfToYJAqynzGVkDOeVOSFyfU7eK6CxvhqAmmjCeUJCsUqvnzMAZbp65H4E96pvoy2Ymn0mGJbyWOOBzcyyNGyJwCy5O5gO55PQnFXdNsIdK0u10+2GyK2iWJMegAGfr/jRYDk7fUL2z8TfZdVluo5JrmeSymWTNreRbWKQkDIjdVwcYGdmcnOKsf8Jv8A8Sr7eunEqdKg1MIJhkrIcbPu4BGD9a2ovD+nQ3EEywuTbu8sCPPI6RM+dxVScD7xHHQEgcVVXwboSW0luLWYxPbralTdzcQq25UB3/KoPTGMZ4wKLAQL4puE1B7SfTo0EWoJp8rpclgGeNZI2XKfMCGGQcYPrVVPGd3LBFImkx5mtbq4iBuzyLd1RwTtyPvAgjOenHWtl/DmmSXDzvDKZZLlLtm8+QZlRdqtjdgYAAwOuORTE8K6PGkaC3mCxRTRL/pMpwkpzIPvc5OOvTtiizArjxOrXmnwJZny7x0SJjKCVLQtICVA+UfKV5IPcDFUIPFjTw6PqN1Yta2l2skkfl3O4hFheQ7lCgHG0459+ua1U8JaNHNFKltKrxNG6YuZcB408tWwW6hPlz1I4zUkXhnSYYNPgS3kMWn7hbI08jBQylSDliG4JHzZ60WYFI+K5YrKW8utMnht08tlnO4oEfOWbKZAUAZIBAyDnGcb1pOLqzguRsxLGr/u3DqARnAYcMo7HHpWfZ+G9NsLeOC2S4WOMgx5upW2AAgKu5jhQCfl6e1aFpawWVpDa20SxQQoERBnCqOg9wPenYCaqPgX/kW5P+wlqH/pZNV6qPgXnw059dRvyPobyas6uw0Xta0RdYjt2E8ttdWz+Zb3MWN0TYIPB4ZSCQQeD9RkZf8AYfir/oYtL49dIfP/AKP/AM+1dXRWSbQHKf2J4r/6GLSv/BQ//wAkUf2J4r/6GLSv/BO//wAkV1dFPmYHKf2J4r/6GLSv/BO//wAkUv8AYniv/oYtK/8ABQ//AMkV1VFHMwOU/sTxX/0MWlf+Cd//AJIo/sTxX/0MWlf+Cd//AJIrq6KOZgcp/Yniv/oYtK/8E7//ACRR/Yniv/oYtK/8FD//ACRXV0UczA4iws/Fd8br/ie6XH5Fw8P/ACCnO7GOf9f71c/sTxX/ANDFpX/gof8A+SKsWd9HpeleIdQmVmitbm4ncKMkhVDHH5VlWuv3kXiia2mCub9rWO3TzS0cRaGaRj6ciPGB944Pc4OZgXf7F8VH/mY9K/8ABQ//AMkUf2L4q/6GPSv/AAUP/wDJFUf+E4u3RpE0qErCkRl3XBBy1y8Dbfk5GY9wPcEdKsTeMpra1keW2hM0EkySxpKSWEcqJleOmHySehwvOc0c0u4E39ieK/8AoYtK/wDBO/8A8kUf2J4r/wChi0r/AMFD/wDyRVeTxNdW9xNDbWKO5vrqH97duQPLi8wHlTjPTA4XtmrF/wCKWOiWt5YQuk80lkf39vJ5QSeWJCA+ArECTseo6Uc0u4B/Yviv/oYtK/8ABQ//AMkUf2J4r/6GLSv/AAUP/wDJFVtX8WtBb6pELVZVgs7qcNDcsp/cyBGG8DIPzdujArngkSW+s3Fr4k1HTl3Stc6oLe386QlIgLOOVgO/XOFHqTxjFHMwJf7F8VkZHiPScev9jv8A/JFIPC+pX8saa5rMd1Zo4c2ttaGBZSpyocl3JUEZwMZ75HFU4PHdxMks/wDZsMcEUVq7lrg53TSyQgcqBtDpndnlTnjoIvEfiPUv+Ee1mzFsLHUoNMubostwMqqEhGUrnkj5sHpwDnOaOZisd2DyPWnU0dadUjCiiigAooooAKKKKACijIPeigAooooAKp6lp1tq2mz6fdxl7edCjrkg49QR0OeQauUUAcmNA8TQgRQeJbZ4l4VrvTDJIR/tMsqAnHfaKX+xPFf/AEMWlf8Agof/AOSK6uiq5mByn9ieK/8AoYtK/wDBQ/8A8kUf2J4r/wChi0r/AME7/wDyRXV0UczA5T+xPFf/AEMWlf8Agnf/AOSKP7E8V/8AQxaV/wCCd/8A5Irq6KOaXcDlP7E8V/8AQxaV/wCCd/8A5Ipf7E8V/wDQxaV/4KH/APkiuqoo5mByn9ieK/8AoYtK/wDBO/8A8kVn417/AISD+w/+Em0oXwtftfl/2O+PK3bM/wDHx/eruj0NccvhzTx8UH1jbML77GJPN8w9OY9uOm3GDj1GetHMwJ/7E8V/9DFpX/gof/5Io/sXxX/0MWlf+Ch//kis3WPFklvdQ6miuLCzlv45YUkO6XyInJBXp95Djnpg9+NK48TXsGrx6ULK3e5eaKMSiYiPbJHK4P3c5BhIxxkEHjOAczAP7E8V/wDQxaV/4KH/APkij+xfFX/Qx6V/4KH/APkiq+n+Mp7qGGS4tLe3M9vBPHiZmx5kvlsv3ckg8gAfMeOOtQP4ymEMd6NPBkeyWXY1023P2gREbduOM5DdT0o5pdwL/wDYviv/AKGLSv8AwUP/APJFH9ieK84/4SLSv/BQ/wD8kVd07X/tcZjuLeSK6F1La4hikmjDI2MllX5Rgg/Njv6Vj6d4vuZNH0K4uooJJ76CykmWJyCpuG2g4xhRnkZPOGxjFHMwLf8AYvir/oY9J/8ABQ//AMkUf2J4r/6GPSvw0h//AJIrI/4Sd7R7HU2jm8j+zLxmtzcs6l0uYUUlm9Nx+YjIBPpWs3iW+GuDRorC3acXLQNK0xWPAhWUEAKTnDEEduOoOaOZgB8P+JJwYrjxLapC3Dm00wxygf7LNKwH1Kmt/TrO10jTrextlEVvCgjjXOeB6k9T6k85zXN23i+e7TTpG05Y7bVGiWyledTkssjMHUEkYVPxLbeCMnS8Hz3Fx4agkupnkm82dWdmLk7ZXAGSMnAAHPpSbbA36KMj1pKQC0UlFAC0UlFAC0UlFAC0UlIxIUkDJx0oAx9CCsNWVgCDfygg9DwKmh0HR7dDHBpVjGjbdwW3QA7fu9u2SR6fjXNeE/E9vrdv4h/s5LuOSC4kdmktjmNyPubScs6leVHHI55qnf6l4og02Vxb3y3H9n3yIbeBpQ1yBEYHUbNwB/eYDDrlTng0Adm+iaVPu87S7KTdGsbB4EOUU7lU5HQHkDoDzTJvD2iT7zPpNhIWLsxe2Q7mfG4nI5JwM/SuZe41yXXivnaolm+reR8tpgLbGyDlgfL6eeNu71OKoQ6l4omtYJpG1eORbTSyyrY4zI87Jckgx9RHhiP4c59MFgO5Oh6RuLf2XZbmZ3P7hOWYbWJ46kcE9xxT5bXTRZwafNDai2fEUNu6rsfaCwVVPBwFJwOy57Vxc2r6/BbrGsOrPLFdTAObJ2V4lu1Ck4TJJhzjoCMn5iKs2lzri3FjPLPqbrLrN1DNG9nwlsouPLbiMEAnyfmzg8e9FgOjPh3RXMpk0awbzd/mFrZDuDEFs8c7sAn1wO9Pl07SYVkeWysY/PlRmZ4kHmSYCoT6t0Ud+1ctbahqx09p5V1nN1cgRQG0fdBCUcKXJTOSUDMBwpZFOBkmpY3uvmH7XPHqa3dxc6YzJ9kcrsKxCcYK4QA+Zn7pGM9xRYDtjo+msJgdOtG86IQy5hU70HRG45UZOAaZJoGizwRW8uj2DwRKyRxtbIVRW+8AMYAPeuRvb/xWulPNCt8t2YLj7QiW2QkolQRiMbTkFS3TORyTnmum8OzXjrqMV41y4hvWSCS4iKM8eFIPQZGSwBHpQBsIu0BQAAOBgdv6VJRRQAUZGcZoPSsTxCmo+VZyWNs91HHdBrq2jdVaaHawKgsQOGKMQSMhSOc4oA280ZHrXF/2VrfnssQulC3Fm9pLJdbzFAu3zo3JYlmYCTJ53b1+bjK466F4jTT8rb6kbr7ASN1+Dm4E2Y+smM7O/THHPSgD0zI9aTIPeuAl0XW5r+WSS31ExSXN5kC/4MTJ+648zAG7oOMH0p1pYeJVvbJrm2u2DXFrPPJ9pTCAW+yVT8+SQ4ycAg9QaAO3mnWCMOyuV3Ko2IW5JAHA+vXtUu9SeGGfrXnEOkeKIdNEYt9RMz2sBkDX4J81bjL8mQ4zGTnBwRx7VZfTr7TnzbsLbUZNWnkiiaZcXMMzYZwoPOwOr8jI2EdDkgHoFFRQRLbwxwxghEUKoJzwOKloAKKKCcDJoAKKKSgBaKSigBaKSigBaKSigBaxR/yO0n/YOX/0Ya2axf8AmdZB3/s5f/RhoAsjRdL+2G8Om2ZumJJm8hNxONuc4z0GPpTl0fTlEUY0602QuJIh5K4RgNoYDHBA4BHQcVzOoXuvw3N75MV1JCl1DJHLDC2fKFxGsqFCmSfLLkFSdy88ECsy81DxM1lcvbtqySRWmqywgWJyzpOptRyhJJQnC9SBzyDksB2v/CPaJ5Yj/siw8tUVAv2ZMBQ24DGOgbkD15pP+Ee0QLt/sfT9pUqQbZMYLBiOn94A/XnrXMm41uTWI7MTaqtodU2tMLTH+jmyZupTAAmwMjucewr6ZqniBbS2e/j1aR57Gwll/wBCKmOZndZl4jyABtyMEgcjGc0WA7ZILHTLe4lWO2tIAWnmcKsag4yzt2zjqx9KrDw/opMZOjWGYwFT/R0ygDbwBxxhueO/NcWl/wCKLrRNWneTV47uDRY5IIhYbd91tmVhgx/MciM7RxyD0Nal/qWp/wBo381sNYS2trRzHGlmzGedo9w2BkIAQDjOdztt7YJYDo10XSLdQV02xjCRvGCIEUCNuXXpwpPJHQ45p0Gn6W4tri3tLNgn7yCZIlO3cMbkIHdeMjtiuSt7vWJtUt5Jv7Tjt4ZLyNSLeXGCqNEWBX5v4wGIPIx1OKLG+8Ty6fbNJDerefZ7Bo1a22rIWwLjzMj5T97IOMYBA55LAdWug6OEnhGlWIinkEkqfZ02yODkMwxyc85PfmrdpZ21hbrb2lvHbwKSRHCgVQScngVyvhe71qTU4U1B7+SKW2mdzc2pjCMs5VOdi4JQ5x7A4rssg96AMbXNZ/shLeKG2a8vrpyltbKwTeQMszMQdqgclsdwACSAc5dS8WkZOn6Kp64+2SnHt/qhSayP+K80IemnXx5/66WvP15/U1qVrCCauxGb/aXiz/nw0X/wMl/+N0n9peLP+fDRf/AyX/43WnR+lX7OIXM3+0vFn/QP0X/wMl/+N0n9peLM/wDHhov/AIGS/wDxus3W/Ev9jeJLGxmMCwXdvJImQTI8quiLGvPJYyD/AL5zV/8AtY6ba2g1qREvLrOyK1ikfJClioxksQAT+BwKXJELkn9peLMZ/s/Rf/AyX/43Sf2l4sH/AC4aL/4GS/8Axuk/4SPSPJjuBeoYZI45RKoJVUk4QswHyg84Jx0pYvEGmzXEcEdwTLJLLEoMLjLx/fHK9vTvRyRC4v8AaXizJH9n6Lx/0+S//G6T+0/Fh/5cNF/8DJf/AI3VS18QxzanfQylRZRQWs0DhG3t5xfC7epPyjgDPNM1XxZZW3h271KxnS4ljtZbiOPY7bvLyGDAcqAwIJOMHNHJELi6dHr+lG7NjpOiRG7uGup8XcvzStgM3+r74FXf7S8WZx9g0XP/AF9y/wDxukOv6fHLBbTz/wCkSskexI3YCR1LbMgEA4GcE9OamtdXsL27a0gnDyiPzQpRgGTJG5SeGXPcZ6ijkiFyL+0/FnH+gaLz0/0yX/43R/afizGfsGi/+Bkv/wAbqudXubzX7zSrHyoxZRxvdXEwLAM+SqBQRk4GSxPHHBOcJZ+I7fbbwX9xD9umMm1LIPKrKsgTIIB4+aMnPI3c4FHJELlr+0vFn/QP0X/wMl/+N0n9p+LP+fDRf/AyX/43Uf8Awk+ji3M/20eWDDhhG5/1pxGwwOVZuAehNZ9x4ysor+zEbFrJ4rkznyJDLE8WzK+WBkcOTyOgB70ckQuan9peLP8Anw0X/wADJf8A43R/aXiz/oH6L/4GS/8AxutCKWOeJJYmDxyAMjKcgqRkEZ7Gn0/ZxC5mf2l4s/6B+i/+Bkv/AMbqNvEeraU8cmuaZZrYFgj3Vncs5gJ4DOjIvyerAnHcAc1r1ieM/wDkRvEOf+gbcE8/9M2odNWC52INLTI/9Wv0FPrnGFB6UUHpQBjarr0GmTWkJ2yTT3MUBTzMMnmNgHofQnHfB9KfN4j0mBlDXkbMZo4NsZ3Hc/3eB0BwcHpwazb3wpcXOqz3MWopHbz3lveyRNAWYSRBVwrBhgFUUdDg5OTnAgs/Bs1pplpbLd2gntJIGiuEsAjOkROFkw/znBPIKjJJx2oA6H+2NPMk0QvITJDjeoYEjLFRx1+8CPqMdaoyeJdMzMkN5bu0UMc5ZpNseyRmVTvxjqjD6jFZMPgq4t7SGOC/tUuLOVZLS4+w/NtViQkxD5kGCRkFeeTk1PqfhO61D7eRqUMb3tlDayn7KSoMcjvuUBxgHzWGCSeBz1yAbcmr6dFI8c19bxtGrM4aQALtAZuT6Agn0Bp1neadqh+0Wk9vcGE7d8bBihIBx7ZBH4VgXfgw3VtNb/bcRG6ubuEGHLJJNHIjZO4blBlcgcH7ozxzpaLok2k3V1K90s4uFhGFiKbTGgQkncc5xnt+NAG3RRRQAVS1PUbfSNOnv7osIYVy21csx6AAdySQAO5IFWpZY4IXlldY40Us7scBQOSSewrmfGM8Nz4atJreWOaKTUtPKujbldftUXQjqKFuBGmreKZ1WWPR9MtojzHFPfOZAO27bGVB9gSO2TUn9peLP+fDRf8AwMl/+N1pd/x60V0eziK5m/2l4s/6B+i/+Bkv/wAbpP7T8WH/AJcNF9f+PyX/AON1oTSpbwSTSsFjjUuxPYAZNc5ba7qN5otjrQFjb2t20UkcFzIVbyXIO4uTt3FCW24xnjJNJwiFzU/tLxYT/wAg/Rf/AAMl/wDjdH9peLP+fDRf/AyX/wCN1GniTSpWlRLlnMUxhYmF8GQMV28rgtnOQOQMGopfF2hQqGe9PlmFZ9whcqsTEgOxC4UZBBJ4BGDRyxC5Z/tLxZ/0D9F/8DJf/jdJ/afiz/nw0X/wMl/+N1T07xLDc6pd6fdssc63r20BWJtsmIhJy33QcFjjOcDpVjVtYltdV0/SrSNHvb9pWDSZ2RxoAXdgOWxuAC8ZJ64FHLELko1LxZn/AI8NF/8AAyX/AON1TD+K/wC1/wC0fsejbzbiDy/tcuOGLZ/1fvVmO+ubKd4tU8jDuqWjW6MWmO0lhs5IICscDIHXJ6CObxXokMKzPe5iNubnckTsBEG2lzhflAPXP3e9HJELk39peLc/8g/Ref8Ap7l/+N0f2l4sxn7BouP+vyX/AON0w+JNJQOGuWBSdLdlMLg+Y4ygxt5yOjdD2pkfinRpBuW7YfJJJlreReI8+YD8v3lwcp94dxRyRC5N/aXizn/QNF4/6fJf/jdH9p+LMZ+waLj/AK/Jf/jdMfxJpMSMzXW0RiFjmJiAJjtj7dCeOO/Wm/8ACQWU8tmbe5HlzyTRlXgkEhMasWUDGQw28g846UckQuS/2l4s/wCfDRf/AAMl/wDjdL/aXiwHH2DRc/8AX5L/APG6qWPiayv47W6WZYre4sPt+yaNldUyOcn5cAN25544q0PEGmnzUFw2+Jyjw+S4kUhQxyhXPQg59CPWjkiFw/tLxZnH2DRf/AuX/wCN0f2l4s/58NF/8C5f/jdaQII4OR/k5pafs4hcypNX8VQIZZNG0y5jUfNFBfOJCP8AZ3R7SfYkfUVv6Zf22r6bBf2uXgmXcu5cMvYgg9CDkEdiDVWqPgXjw2+BwNRvwPYC7mrOpFR2GQ61/wAj9oX/AGDb7/0Za1qVS8R6ffG+0/WdOg+0z2Qlje1DhTLFJt3BScAMCikZODgjvVP+3p8Y/wCEe1we32YdfwbH5VVOSSsJmzSdu341j/29P/0L+t/+Ao/+Ko/t6f8A6F/W/wDwFH/xVXzoRX1nwtDruswXV3LE9mlrLbNbPCSW8xkO7eGG1hsGCBketRWnh3VETRvtutx3kmmSu4la1KtMrRNGA+H6gP8Ae/ix0HJN3+3p/wDoX9b/APAUf/FUf29P/wBC/rf/AICj/wCKpc0QMaz8Cm10efTH1JZobuxisrsmDG8ICu5fm+UlTjnPIBq3qHg23vtRuLsXcsZlmSZVUA+W2AkpB7mSNQpPbk9Savf29P8A9C/rf/gKP/iqP7en/wChf1v/AMBR/wDFUXiBQ1XwnLqOpXd8l/DG8n2Ywo9tvVDEXPzjcN4YSMMDbj8KjvvCV1dR3Xk6lbwte6e9jcgWf7vDFirRrvBQje3BLdetaf8Ab0//AEL+t/8AgKP/AIqj+3p/+hf1v/wFH/xVHNEDCjsbxPFTOLZHEckIw9pKqyhIwvmiRX8sEBmwCu4YA54xseHvD8ugxiBrq3nghQxQMloI5imePMfcdxGAMjbnHQ9pf7en/wChf1v/AMBR/wDFUf29P/0L+t/+Ao/+KovEAfRZbfXrnVdOukgku40juYpI96OY8hXUggqQDg9QRiqeleFX0q6sJlvlkW1jvEZBBgu1xKsrEHfhQCoA4OR+dXP7en/6F/W//AUf/FUf29P/ANC/rf8A4Cj/AOKovEDmNS8Nanp9ibhr7+0bqaTTYpGSzcuxguvMMzYdifldiRx0GMdK2D4TlOqPqCahEs0wuWmDW5Kl5UjQEAMMBVhQY5zzyKv/ANvT/wDQv63/AOAo/wDiqP7en/6F/W//AAFH/wAVReIF3SbI6bo1lYNKJTbQRw+Zt27tqgZxk+nrVysb+3p/+hf1v/wFH/xVH9vT/wDQv63/AOAo/wDiqfNEDZzWJ4z/AORG8Qdv+Jbcf+imp39vTf8AQv63/wCAo/8AiqqaiNS8TWE+j2+j3trDeKYrm6vEWNY4mGH2jJZnKkgcAZ5OOlJzQ7HQp4Z0plDGCbJ55uZOv/fVO/4RjSv+eEv/AIFS/wDxVayjAGM496dXOMx/+EY0r/nhL/4FS/8AxVH/AAjGlf8APCX/AMCpf/iq2KKAMf8A4RjSv+eEv/gVL/8AFUf8IxpX/PCX/wACpf8A4qtiigDH/wCEY0r/AJ4S/wDgVL/8VR/wjGlf88Jf/AqX/wCKrYooAx/+EY0r/nhL/wCBUv8A8VR/wjGlf88Jf/AqX/4qtiigDH/4RjSv+eEv/gVL/wDFUf8ACMaV/wA8Jf8AwKl/+KrYooAxJfC2mvC6xxzI5UhXFxIdp9cFua5C68EaN4L8MwQaUk+6XVdPM0k0zO0h+1RHJH3Qfoor0qsjxHpUur6HLa28ipcq8U8DSD5RLFIsibvbcoz7ULcAorFXW76MBLnw3q8c6jawijSRf+Aur4I78gH6dKX+3p/+hf1v/wABR/8AFV0qaFY1pYo54nhlUNHIpVlPQg8EVzQ8KXH9hWmhnUkbTrWWB4t9vmYJE6uqFt2P4FGcdO3er/8Ab0//AEL+t/8AgKP/AIqj+3p/+hf1v/wFH/xVHNERWj8PXsGnzW8OqojSag95uFsdrIzljEwEgJHOOo6dDmsG18LXUd/d6I9wTZTaQLSS5W0ZQytLKSqtuwGCv1569OMV0/8Ab0//AEL+t/8AgKP/AIqj+3p/+hf1v/wFH/xVK8RlKPwpLHeJMNQh8tNW/tHy/sx6eT5WzO7053Y/Cr+raKdQvtP1G3uFt7+wL+U7pvQo4wysuRkHC8gggjPTim/29P8A9C/rf/gKP/iqP7en/wChf1v/AMBR/wDFUXiAXOkXN3e2F896i3VnI7Jth+Qo6lWUjdk9jnPYe4rJm8EF7W6gh1FEW50+6s2zb5w9w5kdx83QFjgfhmtb+3p/+hf1v/wFH/xVH9vT/wDQv63/AOAo/wDiqLxEUJvCtzNqD3f9oxDfcWlxt+zHrD1H+s/iPftjvUf/AAh0jBI5NRRohNfSMqW5UlbksdoO/wCXbu984HArT/t6f/oX9b/8BR/8VR/b0/8A0L+t/wDgKP8A4qjmiBkL4NvTFIsurQM7rYruWzIGLWUyDjzOrdDzVqDwtPHqMF02pRskWoXF6UEBBYSoyhc7uMbic4/Crv8Ab0//AEL+t/8AgKP/AIqj+3p/+hf1v/wFH/xVF4gY6eCJjp1rZTaqpig0htLVo7fY2Ds2yZLnBAjUYIOeelWLnwpNfaetvcXVkkqksstpZeQYnxhZEw5IYc5+Ygg4xjrof29P/wBC/rf/AICj/wCKo/t6f/oX9b/8BR/8VRzRA2F4XBYk4HJ7+/1paxv7en/6F/W//AUf/FUf29P/ANC9rf8A4Cj/AOKp86A2ao+BSP8AhGn5/wCYlqH/AKWTVTfXL+RStp4a1eW4I/drMixJn/aYt8o+nNbPh7SZtH0OCzmdJrjdJNO4yFMkjtI+0Hou5jgemKzqNMaNiiiishhRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFV729tdOs5Lu9uIre3iGXllYKq/iaw/wDhOvDx5F3MQehWzmI/PZyKLMDpKK5z/hOfD/8Az93H/gFP/wDEUf8ACc+H/wDn7uP/AACn/wDiKdmB0dFc5/wnPh//AJ+7j/wCn/8AiKP+E58P/wDP3cf+AU//AMRRZgdHRXOf8Jx4f/5+rj/wCn/+Io/4Tjw//wA/Vxx/05T/APxFFmB0dFc5/wAJz4f/AOfu4/8AAKf/AOIo/wCE58P/APP3cf8AgFP/APEUWYHR0Vzn/Cc+H/8An7uP/AKf/wCIo/4Tnw//AM/dx/4BT/8AxFFmB0dFc5/wnHh//n6uP/AKf/4ij/hOfD//AD93H/gFP/8AEUWYHR0Vzn/Cc+H/APn7uP8AwCn/APiKP+E58P8A/P3cf+AU/wD8RRZgdHRXOf8ACc+H/wDn7uP/AACn/wDiKP8AhOfD/wDz93H/AIBT/wDxFFmB0dFc5/wnPh//AJ+rj/wCn/8AiKsWHivRNSvEs7a/X7S4JSKVGiaQDk7Q4BbHtSswNuiiigAooyPWigAooooAKKKKACijIziigAooooAKKKaSACcigB1Fc4fHPhwOwTUfOCnBeCCSVCfZkUg/nSf8J14e/wCfu4/8Ap//AIigV0dJRXN/8J14e/5+7j/wCn/+Io/4Trw9/wA/dx/4BT//ABFAXOkorm/+E68Pf8/dx/4BT/8AxFH/AAnPh7/n7uP/AACn/wDiKAudJRXN/wDCc+Hv+fu4/wDAKf8A+Io/4Trw9/z93H/gFP8A/EUBc6Siub/4Trw9/wA/dx/4BT//ABFH/Cc+Hv8An7uP/AKf/wCIoC50lFc3/wAJz4e/5+7j/wAAp/8A4ij/AITnw9/z93H/AIBT/wDxFA7o6Siub/4Tnw9/z93H/gFP/wDEUf8ACdeHv+fu4/8AAKf/AOIoFc6Siub/AOE68Pf8/dx/4BT/APxFH/CdeHv+fu4/8Ap//iKAudJRXN/8J14e/wCfu4/8Ap//AIij/hOfD3/P3cf+AU//AMRQFzpKK5xPHPhxnVX1HyQxADTwyRLn0LMoH610eQe9AzkvEKLc+MdAtJ1DwJb3d4EYZHmxmFFbHsJXx7kHtWtz9PpWXrX/ACPuhf8AYNv/AP0Za1qV0U/hEwoooqxBTJZUhgeaRtsaKXZuThR1NPrA8Ztdf8Ipe29lFPLc3ai1QwxM5TedhchQcBVYtn2oYGxaXUF/aRXVq6ywzKGjdRwykZBGeoIpLi8t7SS3jmk2tcSeVENpO5sFscD2P5GuTaDVItVWO1+3QacmoW9rbxIrFVhjj3O5GOEYgx5PHT61QN9rV59m1N9NvpLy1sbq9W3lgdUW5I2RxYI5KoXBI+8WyM9BNwPQ6i+0RG6+zeavn7PM2Z525xnHpnjNcjdtqsOiPNb3Oo3F95kVwNtvImI0K+YNpHzMwDDbgdRtAwTTbr+15ZdRhsW1FFe5s7a2eXzMInyvLNk46qzKT6pincDtc84z/nrUc88dtbyXEzhIolLu7dFUDJJrhLrU9RtNPDTXd5Epvroyeesi+VCN6xM7KCyR7grZHHPULmu2to3ttNhju52upIogJZdmWlIAy20ep5AHr60XApHxLo4d1a+VDGSreYhQIQu8gkjg7fmwe3PStb888Vwmmxah/wAI9b3C215b3uqa19puMROkkKFycMCMqPKRY8nH3qrzN4kNtNdI2pA/Z725SAKwI3Ptt4xxkNtGT3UdhnNF2B6Hn3orlrU6vda4baee6htrTyFjk8ph9pAQM7FsbRliVIJONpwBu3V1NNAFFFFAB1/+vWD40iVvB+q3GMTWltJd27kcxyxqXRx6YKit6sTxlz4G8QY/6Btx/wCimpNaDOvQ5VTzyO/enHoabH/q1/3RTz0rlGc7B428O3E3kx6kob7UbLMkbxqLgf8ALLcygB/9nrWppmp2er2a3djOs1uzugcAgEqxVuvuDXmXhLwgmtXfiA61BqcFtH4nn1S2t5LdoUnH8D5ZAzDrwCOnOazrC+1eN/BlvrNzr0IuLnVFu4i1ws0qKrNHlR87YG0ggE/TFAHtZIwcHp6dqxbfxVotw0CQ3ykzXb2Ua7GGZkBLJgjgjBrzTQT46lk0e21261W0Z9OieKdLWSb96s5JWXaQA7RhATJxgnocml0a31qzvbLyrLWLaKXxTeyXCR20yK1uwyrOAoBUnbgnjr70Aey5FIemOD7V4bpj+OYNJ017Ua+dRm0zUUnF4k7KsqyMbfPmAqrbeh6ngHIr0Pwc16NDubu6m1C4EsaOLW8tpYnhYRgMmZCWbJGSemScdcUAbVp4i0u/nt4ra4MhugzwN5ThJVUclWIwR05B7itOGaK4hSaGVJInG5XRgVYeoI615FpehatprHTdGm1G68P3mnXMn2HUIHSTTZNmERJCAclmK7fTJ561mTSeIbDwNpVhpln4jguodCJLCOcIlwrpldoXdv8AlYDJC7c4DZoA9zyPWjI9a8cvddnm8dfYrvxHc2P+mWUvlT+fCpQwjfBhV2fMzLksVIJxg4FdB8OP+EgnXz9du9RF4Inju7W5tZEQSiVmDo7HaQVbAEY242+mKAPQ65nx0xPhhoMkJd3dray47xyXEaOPxVmH410x6VzPjk50G2/7Clh/6VRUCexPHGkUaRxoERVAVV4CjsOnpT/89aSipMRf89aP89aSigBf89aT/PWj69KwvGKXsnhLUY7BpVnZFUmL74TcN+3vnZux74oGaMOqWVxcpBHco8sm7ZjhZNv3tp6NjvjpV3tn+tcXqatd3Gm3OmWz/wBm6Pby3USpGymd/JZI44gOWGCxyOvA55xWjvvEcMEqqt8/lw2VqskkLkkn/XTY2klsEdAduOR1FAHe9f5daTtn1GevUVy/2LU59RsbP7bfLbeRcS3FwNyElnURxgtnoC2MkkACrWjx3o1S/hmlnaC0uHWJpGz5gkVJBk9SUyy8+3XGKAL95rWn6fOIbm5CSFkXG1jy5wo4HUngDvUtlqVnqKSNaXCSrG2xwucq3ocjIrF1uJ73xRoVmLaY20Usl7PKsLFNyptjUvjGdzbsZ/g+lZnm6yRE6pqCJ9svbqZFjcHyY9yxxAY/jyjr3J3dcUAjuMf5zR1//XXAQyeJ7C28uR724nW2soJJTGzhXYnzpF+UkkAqMgE55bpiuy0yGWCxVZp5J3LO+6RSCAWJAAOTgZwAegFAMu/560f560lFAhf89aP89aSigLjZYknieKVUkjcFXV1yGB7EHqKh8CMx8KQxEsVt7m6tY9x5EcVxJGg/BUAqwelV/An/ACLL/wDYS1D/ANLJqaNIMreJZU0/xRoeqXLCOzENzZPMThUklMLJuPQA+SVye7Ad61xyoYcgjINX7iKOe2eKaNJI3BV0dQysPQg8GsX/AIQXwkW/5FjRuOf+PGPv17VrGfKrFluiqv8AwgvhH/oWNH/8Ao/8KP8AhBfCP/QsaP8A+AUf+FV7UVi1RVX/AIQXwj/0LGj/APgFH/hR/wAIL4R/6FjR/wDwCj/wo9qFi1R+FVf+EF8I/wDQsaP/AOAUf+FH/CC+Ef8AoWNH/wDAKP8Awpe1Cxb/AApOnQVV/wCEF8I/9Cxo/wD4BR/4Uf8ACC+Ef+hY0f8A8Ao/8KPa+QWC5061u3Lzw72KGNjuI3oedrYIyOvBz1q0BjgAYHpVX/hBfCP/AELGj/8AgFH/AIUf8IL4R/6FjR//AACj/wAKPaBYtUVV/wCEF8I/9Cxo/wD4BR/4Uf8ACC+Ef+hY0f8A8Ao/8KftQsWsUVV/4QXwj/0LGj/+AUf+FH/CC+Ef+hY0f/wCj/wo9qFi1RVX/hBfCP8A0LGj/wDgFH/hR/wgvhH/AKFjR/8AwCj/AMKPahYtdOa5/wAaTRnwzqGmqVa91KCSztYB9+WSRSowPQZyT2GSa1h4F8JZ/wCRY0f/AMAo/wDCrOmeHND0efzdM0fT7KRsoXtrZI2IHQEgCk6oWNZc4GadRRWQwPSs670XT7/UbLULm233dizG2k3sDHuGDjBxyODntWjRQAlLRRQAUlLRQAmKMce9LRQBlnQ7BriSc23M0qzSJvbY7rtwxTO0kbV5x2HpWnS0UAFc343hll8NPLFE8ptbm2u3RASzJFMkj4A6napwO5rpKKAZzlrd297ax3NpPFPBIu5ZY2BVhnjB/SpqbJ4P8M3s5uLnw7pM08g3vI9nGWY9Mk4yaT/hBPCP/QsaP/4BR/4UrEcg+imf8IJ4R/6FjR//AACj/wAKP+EE8I/9Cxo//gFH/hRYXIPz71FPBDdQNb3ESSwv96N1yp+o7/Q8U7/hBPCP/QsaP/4BR/4Uf8IJ4R/6FjR//AKP/Ciw+Upafoek6UzNp+nW1qT/AM8owuB6ACtDI9qZ/wAIJ4R/6FjR/wDwCj/wo/4QTwj/ANCxo/8A4BR/4UWDlFdQ6MjdGGDimwwxW8KRQqFjUbVUHOB9f6/4ml/4QTwj/wBCxo//AIBR/wCFH/CCeEf+hY0f/wAAo/8ACiwuQfRTP+EE8I/9Cxo//gFH/hR/wgnhH/oWNH/8Ao/8KLByD6KZ/wAIJ4R/6FjR/wDwCj/wo/4QTwj/ANCxo/8A4BR/4UWDkH0Uz/hBPCP/AELGj/8AgFH/AIUf8IJ4R/6FjR//AACj/wAKLByD6M0z/hBPCP8A0LGj/wDgFH/hR/wgvhL/AKFjR/8AwCj/AMKLByDLu7t7C2kubueOCCMZeSRgoUfU0eCYJYfCsDSxyRG4nubtUZSGVJp3lUMOx2uMipIPB/hmymW4tfD2lQzREFHjs41ZT7ECttiVwQevJoSKUbH/2Q=="}}
```

Figure 1: Graph RAG pipeline using an LLM-derived graph index of source document text. This graph index spans nodes (e.g., entities), edges (e.g., relationships), and covariates (e.g., claims) that have been detected, extracted, and summarized by LLM prompts tailored to the domain of the dataset. Community detection (e.g., Leiden, Traag et al., 2019) is used to partition the graph index into groups of elements (nodes, edges, covariates) that the LLM can summarize in parallel at both indexing time and query time. The “global answer” to a given query is produced using a final round of query-focused summarization over all community summaries reporting relevance to that query.
```bgraph-paragraph
{"id":"f7b7970a-0caa-5462-98df-9989fa65696d","node_type":"Paragraph","location":{"semantic":{"path":"5.4.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":282.58063,"width":400.25806,"height":78.19354}}},"text_order":34,"token_count":171,"style":null}
```

generated texts such as “fluency” (Wang et al., 2023a) Some of these criteria are generic to vector RAG systems and not relevant to global sensemaking, such as “context relevance”, “faithfulness”, and “answer relevance” (RAGAS, Es et al. 2023). Lacking a gold standard for evaluation, one can quantify relative performance for a given criterion by prompting the LLM to compare generations from two different competing models (LLM-as-a-judge, (Zheng et al., 2024)). In this work, we design criteria for evaluating RAG-generated answers to global sensemaking questions and evaluate our results using the comparative approach. We also validate results using statistics derived from LLM-extracted statements of verifiable facts, or “claims.”
```bgraph-paragraph
{"id":"758117a3-70f9-544a-a3a5-6d1edb7db194","node_type":"Paragraph","location":{"semantic":{"path":"5.4.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","2 Background","2.4 RAG evaluation criteria"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":384.0,"width":400.25806,"height":87.48387}}},"text_order":35,"token_count":189,"style":null}
```

# 3 Methods
```bgraph-section
{"id":"47898d75-923b-5dd6-8731-dab3fd3ce31a","node_type":"Section","location":{"semantic":{"path":"6","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods"]},"physical":{"page":4,"bounding_box":{"x":105.29032,"y":490.83868,"width":66.58064,"height":12.387096}}},"text_order":36,"token_count":2,"style":null}
```

## 3.1 GraphRAG Workflow
```bgraph-section
{"id":"12130d0e-da37-54e4-b235-0ef45d604f9c","node_type":"Section","location":{"semantic":{"path":"6.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow"]},"physical":{"page":4,"bounding_box":{"x":105.29032,"y":517.9355,"width":120.0,"height":11.612903}}},"text_order":37,"token_count":5,"style":null}
```

Figure 1 illustrates the high-level data flow of the GraphRAG approach and pipeline. In this section, we describe the key design parameters, techniques, and implementation details for each step.
```bgraph-paragraph
{"id":"6a7c74e6-61cf-563a-a4a6-b0c1e7167f36","node_type":"Paragraph","location":{"semantic":{"path":"6.1.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":538.8387,"width":400.25806,"height":24.0}}},"text_order":38,"token_count":48,"style":null}
```

### 3.1.1 Source Documents → Text Chunks
```bgraph-section
{"id":"0ca6fb91-5a13-5287-b9d2-0b922e2524ca","node_type":"Section","location":{"semantic":{"path":"6.1.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.1 Source Documents → Text Chunks"]},"physical":{"page":4,"bounding_box":{"x":105.29032,"y":576.7742,"width":182.70967,"height":11.612903}}},"text_order":39,"token_count":9,"style":null}
```

To start, the documents in the corpus are split into text chunks. The LLM extracts information from each chunk for downstream processing. Selecting the size of the chunk is a fundamental design decision; longer text chunks require fewer LLM calls for such extraction (which reduces cost) but suffer from degraded recall of information that appears early in the chunk (Kuratov et al., 2024; Liu et al., 2023). See Section A.1 for prompts and examples of the recall-precision trade-offs.
```bgraph-paragraph
{"id":"3c3ff697-40ac-50f2-bb06-569c62e3a442","node_type":"Paragraph","location":{"semantic":{"path":"6.1.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.1 Source Documents → Text Chunks"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":597.6774,"width":400.25806,"height":56.51613}}},"text_order":40,"token_count":121,"style":null}
```

### 3.1.2 Text Chunks → Entities & Relationships
```bgraph-section
{"id":"82066496-c303-5448-affb-9e98ee95ab0a","node_type":"Section","location":{"semantic":{"path":"6.1.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":4,"bounding_box":{"x":105.29032,"y":668.9032,"width":205.93547,"height":12.387096}}},"text_order":41,"token_count":11,"style":null}
```

In this step, the LLM is prompted to extract instances of important entities and the relationships between the entities from a given chunk. Additionally, the LLM generates short descriptions for the entities and relationships. To illustrate, suppose a chunk contained the following text:
```bgraph-paragraph
{"id":"b9b1e03d-bf4e-5808-b6ca-bc7bfcee6362","node_type":"Paragraph","location":{"semantic":{"path":"6.1.3.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":689.0322,"width":400.25806,"height":34.064514}}},"text_order":42,"token_count":71,"style":null}
```

4
```bgraph-footer
{"id":"5a449bd2-d046-5519-8909-b5536b1d42c4","node_type":"Footer","location":{"semantic":{"path":"6.1.3.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":4,"bounding_box":{"x":301.9355,"y":741.67737,"width":6.967742,"height":9.290322}}},"text_order":43,"token_count":1,"style":null}
```

NeoChip's (NC) shares surged in their first week of trading on the NewTech Exchange. However, market analysts caution that the chipmaker's public debut may not reflect trends for other technology IPOs. NeoChip, previously a private entity, was acquired by Quantum Systems in 2016. The innovative semiconductor firm specializes in low-power processors for wearables and IoT devices.
```bgraph-paragraph
{"id":"901013f8-64e6-585f-9160-4a876970da09","node_type":"Paragraph","location":{"semantic":{"path":"6.1.3.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":140.12903,"y":70.451614,"width":329.03226,"height":55.741936}}},"text_order":44,"token_count":95,"style":null}
```

The LLM is prompted such that it extracts the following:
```bgraph-paragraph
{"id":"0d24c39a-53b0-5e12-8cc2-0bbaffc52c75","node_type":"Paragraph","location":{"semantic":{"path":"6.1.3.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":136.25806,"width":229.93547,"height":11.612903}}},"text_order":45,"token_count":14,"style":null}
```

- The entity NeoChip, with description "NeoChip is a publicly traded company specializing in low-power processors for wearables and IoT devices."
- The entity Quantum Systems, with description "Quantum Systems is a firm that previously owned NeoChip."
- A relationship between NeoChip and Quantum Systems, with description "Quantum Systems owned NeoChip from 2016 until NeoChip became publicly traded."
```bgraph-list
{"id":"c8f4c396-3d8e-50b3-bbe3-1259902c808e","node_type":"List","location":{"semantic":{"path":"6.1.3.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":131.6129,"y":157.93549,"width":373.1613,"height":77.41935}}},"text_order":46,"token_count":100,"style":null}
```

These prompts can be tailored to the domain of the document corpus by choosing domain appropriate few-shot exemplars for in-context learning (Brown et al., 2020). For example, while our default prompt extracts the broad class of "named entities" like people, places, and organizations and is generally applicable, domains with specialized knowledge (e.g., science, medicine, law) will benefit from few-shot exemplars specialized to those domains.
```bgraph-paragraph
{"id":"11026a01-aee5-5603-9fc0-39c3599ca569","node_type":"Paragraph","location":{"semantic":{"path":"6.1.3.6","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":244.64516,"width":400.25806,"height":54.96774}}},"text_order":47,"token_count":111,"style":null}
```

The LLM can also be prompted to extract claims about detected entities. Claims are important factual statements about entities, such as dates, events, and interactions with other entities. As with entities and relationships, in-context learning exemplars can provide domain-specific guidance. Claim descriptions extracted from the example tetx chunk are as follows:
```bgraph-paragraph
{"id":"73978c87-d439-53c0-9ab2-c804081e30b7","node_type":"Paragraph","location":{"semantic":{"path":"6.1.3.7","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":304.25806,"width":400.25806,"height":45.677418}}},"text_order":48,"token_count":91,"style":null}
```

- NeoChip's shares surged during their first week of trading on the NewTech Exchange.
- NeoChip debuted as a publicly listed company on the NewTech Exchange.
- Quantum Systems acquired NeoChip in 2016 and held ownership until NeoChip went public.
```bgraph-list
{"id":"1ea8d56a-62fe-5d70-9b20-399eeefa1bdf","node_type":"List","location":{"semantic":{"path":"6.1.3.8","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":131.6129,"y":358.4516,"width":373.1613,"height":55.741936}}},"text_order":49,"token_count":61,"style":null}
```

See Appendix A for prompts and details on our implementation of entity and claim extraction.
```bgraph-paragraph
{"id":"cb990536-5c6e-50db-95af-3598f7f4351f","node_type":"Paragraph","location":{"semantic":{"path":"6.1.3.9","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.2 Text Chunks → Entities & Relationships"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":423.48386,"width":379.35483,"height":12.387096}}},"text_order":50,"token_count":23,"style":null}
```

### 3.1.3 Entities & Relationships → Knowledge Graph
```bgraph-section
{"id":"383074d8-8252-538c-b3ac-a155be38250d","node_type":"Section","location":{"semantic":{"path":"6.1.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.3 Entities & Relationships → Knowledge Graph"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":448.25806,"width":229.93547,"height":12.387096}}},"text_order":51,"token_count":12,"style":null}
```

The use of an LLM to extract entities, relationships, and claims is a form of abstractive summarization – these are meaningful summaries of concepts that, in the case of relationships and claims, may not be explicitly stated in the text. The entity/relationship/claim extraction processes creates multiple instances of a single element because an element is typically detected and extracted multiple times across documents.
```bgraph-paragraph
{"id":"45454673-63f4-5db8-ae23-e49ad86d33b3","node_type":"Paragraph","location":{"semantic":{"path":"6.1.4.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.3 Entities & Relationships → Knowledge Graph"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":468.3871,"width":400.25806,"height":54.96774}}},"text_order":52,"token_count":106,"style":null}
```

In the final step of the knowledge graph extraction process, these instances of entities and relationships become individual nodes and edges in the graph. Entity descriptions are aggregated and summarized for each node and edge. Relationships are aggregated into graph edges, where the number of duplicates for a given relationship becomes edge weights. Claims are aggregated similarly.
```bgraph-paragraph
{"id":"8e2689bc-57e5-538c-bd64-38d72637a2fc","node_type":"Paragraph","location":{"semantic":{"path":"6.1.4.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.3 Entities & Relationships → Knowledge Graph"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":528.7742,"width":400.25806,"height":44.129032}}},"text_order":53,"token_count":96,"style":null}
```

In this manuscript, our analysis uses exact string matching for entity matching – the task of reconciling different extracted names for the same entity (Barlaug and Gulla, 2021; Christen and Christen, 2012; Elmagarmid et al., 2006). However, softer matching approaches can be used with minor adjustments to prompts or code. Furthermore, GraphRAG is generally resilient to duplicate entities since duplicates are typically clustered together for summarization in subsequent steps.
```bgraph-paragraph
{"id":"5113dafc-1cd2-5077-84df-ba2eb32bc9a5","node_type":"Paragraph","location":{"semantic":{"path":"6.1.4.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.3 Entities & Relationships → Knowledge Graph"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":576.7742,"width":400.25806,"height":55.741936}}},"text_order":54,"token_count":120,"style":null}
```

### 3.1.4 Knowledge Graph → Graph Communities
```bgraph-section
{"id":"d27de480-86be-5cfd-9179-a7831001cfc0","node_type":"Section","location":{"semantic":{"path":"6.1.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":645.6774,"width":215.2258,"height":12.387096}}},"text_order":55,"token_count":10,"style":null}
```

Given the graph index created in the previous step, a variety of community detection algorithms may be used to partition the graph into communities of strongly connected nodes (e.g., see the surveys by Fortunato (2010) and Jin et al. (2021)). In our pipeline, we use Leiden community detection (Traag et al., 2019) in a hierarchical manner, recursively detecting sub-communities within each detected community until reaching leaf communities that can no longer be partitioned.
```bgraph-paragraph
{"id":"4d3d0106-0083-5ce7-9acb-bcce58f088fe","node_type":"Paragraph","location":{"semantic":{"path":"6.1.5.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":665.80646,"width":400.25806,"height":54.96774}}},"text_order":56,"token_count":119,"style":null}
```

5
```bgraph-footer
{"id":"44b4ecf0-a03d-50e5-878c-13a53043236a","node_type":"Footer","location":{"semantic":{"path":"6.1.5.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":5,"bounding_box":{"x":299.61288,"y":738.5806,"width":10.064516,"height":11.612903}}},"text_order":57,"token_count":1,"style":null}
```

Each level of this hierarchy provides a community partition that covers the nodes of the graph in a mutually exclusive, collectively exhaustive way, enabling divide-and-conquer global summarization. An illustration of such hierarchical partitioning on an example dataset can be found in Appendix B.
```bgraph-paragraph
{"id":"8046eba0-2602-5632-9d45-8d18601dc4df","node_type":"Paragraph","location":{"semantic":{"path":"6.1.5.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.4 Knowledge Graph → Graph Communities"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":70.451614,"width":401.03226,"height":34.064514}}},"text_order":58,"token_count":74,"style":null}
```

### 3.1.5 Graph Communities → Community Summaries
```bgraph-section
{"id":"1fda9490-1caf-5598-b3ad-04b02e0a2318","node_type":"Section","location":{"semantic":{"path":"6.1.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities → Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":114.58064,"width":238.4516,"height":13.16129}}},"text_order":59,"token_count":11,"style":null}
```

The next step creates report-like summaries of each community in the community hierarchy, using a method designed to scale to very large datasets. These summaries are independently useful as a way to understand the global structure and semantics of the dataset, and may themselves be used to make sense of a corpus in the absence of a specific query. For example, a user may scan through community summaries at one level looking for general themes of interest, then read linked reports at a lower level that provide additional details for each subtopic. Here, however, we focus on their utility as part of a graph-based index used for answering global queries.
```bgraph-paragraph
{"id":"3b484eb0-8110-5942-a823-4b1ee14f3b1e","node_type":"Paragraph","location":{"semantic":{"path":"6.1.6.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities → Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":135.48387,"width":401.03226,"height":76.64516}}},"text_order":60,"token_count":165,"style":null}
```

GraphRAG generates community summaries by adding various element summaries (for nodes, edges, and related claims) to a community summary template. Community summaries from lower-level communities are used to generate summaries for higher-level communities as follows:
```bgraph-paragraph
{"id":"4d9f0421-3618-5213-9e07-55ee3618f161","node_type":"Paragraph","location":{"semantic":{"path":"6.1.6.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities → Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":216.77419,"width":401.03226,"height":34.838707}}},"text_order":61,"token_count":66,"style":null}
```

- *Leaf-level communities.* The element summaries of a leaf-level community are prioritized and then iteratively added to the LLM context window until the token limit is reached. The prioritization is as follows: for each community edge in decreasing order of combined source and target node degree (i.e., overall prominence), add descriptions of the source node, target node, the edge itself, and related claims.
- *Higher-level communities.* If all element summaries fit within the token limit of the context window, proceed as for leaf-level communities and summarize all element summaries within the community. Otherwise, rank sub-communities in decreasing order of element summary tokens and iteratively substitute sub-community summaries (shorter) for their associated element summaries (longer) until they fit within the context window.
```bgraph-list
{"id":"7a6dd8cc-c9e9-502c-a6d0-c0dd8848d62c","node_type":"List","location":{"semantic":{"path":"6.1.6.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.5 Graph Communities → Community Summaries"]},"physical":{"page":6,"bounding_box":{"x":131.6129,"y":260.12903,"width":373.9355,"height":115.354836}}},"text_order":62,"token_count":210,"style":null}
```

### 3.1.6 Community Summaries → Community Answers → Global Answer
```bgraph-section
{"id":"9199b275-83c2-56de-8f65-2bffb4b60efd","node_type":"Section","location":{"semantic":{"path":"6.1.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries → Community Answers → Global Answer"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":387.09677,"width":319.7419,"height":12.387096}}},"text_order":63,"token_count":16,"style":null}
```

Given a user query, the community summaries generated in the previous step can be used to generate a final answer in a multi-stage process. The hierarchical nature of the community structure also means that questions can be answered using the community summaries from different levels, raising the question of whether a particular level in the hierarchical community structure offers the best balance of summary detail and scope for general sensemaking questions (evaluated in section 4).
```bgraph-paragraph
{"id":"343dbf33-97c5-5109-b54b-347a10661644","node_type":"Paragraph","location":{"semantic":{"path":"6.1.7.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries → Community Answers → Global Answer"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":407.2258,"width":401.03226,"height":55.741936}}},"text_order":64,"token_count":122,"style":null}
```

For a given community level, the global answer to any user query is generated as follows:
```bgraph-paragraph
{"id":"d84d2594-c312-5c3d-a7e2-10b4b0ee8fea","node_type":"Paragraph","location":{"semantic":{"path":"6.1.7.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries → Community Answers → Global Answer"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":466.83868,"width":360.77417,"height":12.387096}}},"text_order":65,"token_count":22,"style":null}
```

- *Prepare community summaries.* Community summaries are randomly shuffled and divided into chunks of pre-specified token size. This ensures relevant information is distributed across chunks, rather than concentrated (and potentially lost) in a single context window.
- *Map community answers.* Intermediate answers are generated in parallel. The LLM is also asked to generate a score between 0-100 indicating how helpful the generated answer is in answering the target question. Answers with score 0 are filtered out.
- *Reduce to global answer.* Intermediate community answers are sorted in descending order of helpfulness score and iteratively added into a new context window until the token limit is reached. This final context is used to generate the global answer returned to the user.
```bgraph-list
{"id":"1b99eda9-b4bd-51db-a932-89b61cc64d82","node_type":"List","location":{"semantic":{"path":"6.1.7.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.1 GraphRAG Workflow","3.1.6 Community Summaries → Community Answers → Global Answer"]},"physical":{"page":6,"bounding_box":{"x":131.6129,"y":488.5161,"width":373.9355,"height":109.93548}}},"text_order":66,"token_count":197,"style":null}
```

## 3.2 Global Sensemaking Question Generation
```bgraph-section
{"id":"f42b1a99-8fcb-522f-9f75-e4918ffce767","node_type":"Section","location":{"semantic":{"path":"6.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.2 Global Sensemaking Question Generation"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":610.0645,"width":205.16129,"height":13.935484}}},"text_order":67,"token_count":10,"style":null}
```

To evaluate the effectiveness of RAG systems for global sensemaking tasks, we use an LLM to generate a set of corpus-specific questions designed to assess high-level understanding of a given corpus, without requiring retrieval of specific low-level facts. Instead, given a high-level description of a corpus and its purposes, the LLM is prompted to generate personas of hypothetical users of the RAG system. For each hypothetical user, the LLM is then prompted to specify tasks that this user would use the RAG system to complete. Finally, for each combination of user and task, the LLM is prompted to generate questions that require understanding of the entire corpus. Algorithm 1 describes the approach.
```bgraph-paragraph
{"id":"ea98b472-dec2-56ce-8266-2000c5bbba4c","node_type":"Paragraph","location":{"semantic":{"path":"6.2.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.2 Global Sensemaking Question Generation"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":631.74194,"width":401.03226,"height":89.03226}}},"text_order":68,"token_count":176,"style":null}
```

6
```bgraph-footer
{"id":"695f4abf-262e-5a93-8a7f-55595ba396de","node_type":"Footer","location":{"semantic":{"path":"6.2.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","3 Methods","3.2 Global Sensemaking Question Generation"]},"physical":{"page":6,"bounding_box":{"x":299.61288,"y":738.5806,"width":10.064516,"height":11.612903}}},"text_order":69,"token_count":1,"style":null}
```

# Algorithm 1: Prompting Procedure for Question Generation
```bgraph-section
{"id":"836f879f-bf20-5b40-aea2-ba083ff05e6b","node_type":"Section","location":{"semantic":{"path":"7","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":105.29032,"y":84.38709,"width":260.12903,"height":12.387096}}},"text_order":70,"token_count":14,"style":null}
```

1: Input: Description of a corpus, number of users K, number of tasks per user N, number of questions per (user, task) combination M.
2: Output: A set of K * N * M high-level questions requiring global understanding of the corpus.
3: procedure GENERATEQUESTIONS
4: Based on the corpus description, prompt the LLM to:
```bgraph-list
{"id":"c2b84562-5c81-5a92-83f8-698d9bab3802","node_type":"List","location":{"semantic":{"path":"7.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":109.93548,"y":102.19354,"width":395.61288,"height":56.51613}}},"text_order":71,"token_count":79,"style":null}
```

1. Describe personas of K potential users of the dataset.
2. For each user, identify N tasks relevant to the user.
3. Specific to each user & task pair, generate M high-level questions that:
```bgraph-list
{"id":"7175abdf-8e62-5586-ac6c-399b03902df4","node_type":"List","location":{"semantic":{"path":"7.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":129.29031,"y":161.03226,"width":298.8387,"height":39.48387}}},"text_order":72,"token_count":47,"style":null}
```

- Require understanding of the entire corpus.
- Do not require retrieval of specific low-level facts.
```bgraph-list
{"id":"42c080bc-11b0-5a1f-8429-c466a0a3846b","node_type":"List","location":{"semantic":{"path":"7.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":147.87096,"y":202.06451,"width":212.12903,"height":24.0}}},"text_order":73,"token_count":25,"style":null}
```

5: Collect the generated questions to produce K * N * M test questions for the dataset.
6: end procedure
```bgraph-list
{"id":"1191af15-ec8c-5311-93b1-d55449082221","node_type":"List","location":{"semantic":{"path":"7.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":109.93548,"y":228.38708,"width":367.7419,"height":23.225805}}},"text_order":74,"token_count":26,"style":null}
```

For our evaluation, we set K = M = N = 5 for a total of 125 test questions per dataset. Table 1 shows example questions for each of the two evaluation datasets.
```bgraph-paragraph
{"id":"2ce86283-fc38-5e42-b996-f3c7a94ae8e8","node_type":"Paragraph","location":{"semantic":{"path":"7.5","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":273.2903,"width":400.25806,"height":24.0}}},"text_order":75,"token_count":40,"style":null}
```

## 3.3 Criteria for Evaluating Global Sensemaking
```bgraph-section
{"id":"e58185b7-4a0b-52ec-9c60-91cac9d62bce","node_type":"Section","location":{"semantic":{"path":"7.6","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":311.2258,"width":215.2258,"height":13.16129}}},"text_order":76,"token_count":11,"style":null}
```

Given the lack of gold standard answers to our activity-based sensemaking questions, we adopt the head-to-head comparison approach using an LLM evaluator that judges relative performance according to specific criteria. We designed three target criteria capturing qualities that are desirable for global sensemaking activities.
```bgraph-paragraph
{"id":"39583757-6d3c-5a60-8ce0-b6509260413a","node_type":"Paragraph","location":{"semantic":{"path":"7.6.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":103.741936,"y":332.90323,"width":401.80643,"height":44.129032}}},"text_order":77,"token_count":81,"style":null}
```

Appendix F shows the prompts for our head-to-head measures computed using an LLM evaluator, summarized as:
```bgraph-paragraph
{"id":"4e092b48-3ea1-5939-9b96-85c43463fea8","node_type":"Paragraph","location":{"semantic":{"path":"7.6.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":381.6774,"width":400.25806,"height":21.677418}}},"text_order":78,"token_count":26,"style":null}
```

- Comprehensiveness. How much detail does the answer provide to cover all aspects and details of the question?
- Diversity. How varied and rich is the answer in providing different perspectives and insights on the question?
- Empowerment. How well does the answer help the reader understand and make informed judgments about the topic?
```bgraph-list
{"id":"2f40124a-ed4a-5437-8fd3-6974c7bec5a4","node_type":"List","location":{"semantic":{"path":"7.6.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":131.6129,"y":414.19354,"width":373.1613,"height":78.96774}}},"text_order":79,"token_count":83,"style":null}
```

Table 1: Examples of potential users, tasks, and questions generated by the LLM based on short descriptions of the target datasets. Questions target global understanding rather than specific details.
```bgraph-paragraph
{"id":"6abff33c-6e78-5f5f-8d38-bd02280606cd","node_type":"Paragraph","location":{"semantic":{"path":"7.6.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":510.19354,"width":401.03226,"height":24.0}}},"text_order":80,"token_count":49,"style":null}
```

|  Dataset | Example activity framing and generation of global sensemaking questions  |
| --- | --- |
|  Podcast transcripts | User: A tech journalist looking for insights and trends in the tech industryTask: Understanding how tech leaders view the role of policy and regulationQuestions:1. Which episodes deal primarily with tech policy and government regulation?2. How do guests perceive the impact of privacy laws on technology development?3. Do any guests discuss the balance between innovation and ethical considerations?4. What are the suggested changes to current policies mentioned by the guests?5. Are collaborations between tech companies and governments discussed and how?  |
|  News articles | User: Educator incorporating current affairs into curriculaTask: Teaching about health and wellnessQuestions:1. What current topics in health can be integrated into health education curricula?2. How do news articles address the concepts of preventive medicine and wellness?3. Are there examples of health articles that contradict each other, and if so, why?4. What insights can be gleaned about public health priorities based on news coverage?5. How can educators use the dataset to highlight the importance of health literacy?  |
```bgraph-table
{"id":"133b9d64-00ef-56ed-b67a-65de2acbe2f2","node_type":"Table","location":{"semantic":{"path":"7.6.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":105.29032,"y":538.8387,"width":423.48386,"height":181.93549}}},"text_order":81,"token_count":309,"style":null}
```

7
```bgraph-footer
{"id":"2bd23501-7c28-58d7-a194-d308c1fa9019","node_type":"Footer","location":{"semantic":{"path":"7.6.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":7,"bounding_box":{"x":301.9355,"y":741.67737,"width":7.7419353,"height":9.290322}}},"text_order":82,"token_count":1,"style":null}
```

Furthermore, we use a “control criterion” called *Directness* that answers “*How specifically and clearly does the answer address the question?*”. In plain terms, directness evaluates the concision of an answer in a generic sense that applies to any generated LLM summarization. We include it to behave as a reference against which we can judge the soundness of results for the other criteria. Since directness is effectively in opposition to comprehensiveness and diversity, we would not expect any method to win across all four criteria.
```bgraph-paragraph
{"id":"68569f07-0e10-5a4e-aaf0-fc4574f63fb4","node_type":"Paragraph","location":{"semantic":{"path":"7.6.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":70.451614,"width":401.03226,"height":65.80645}}},"text_order":83,"token_count":136,"style":null}
```

In our evaluations, the LLM is provided with the question, the generated answers from two competing systems, and prompted to compare the two answers according to the criterion before giving a final judgment of which answer is preferred. The LLM either indicates a winner; or, it returns a tie if they are fundamentally similar. To account for the inherent stochasticity of LLM generation, we run each comparison with multiple replicates and average the results across replicates and questions. An illustration of LLM assessment for answers to a sample question can be found in Appendix D.
```bgraph-paragraph
{"id":"ad576d6b-e0aa-5878-9885-c7315da4a613","node_type":"Paragraph","location":{"semantic":{"path":"7.6.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Algorithm 1: Prompting Procedure for Question Generation","3.3 Criteria for Evaluating Global Sensemaking"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":140.90323,"width":401.03226,"height":66.58064}}},"text_order":84,"token_count":147,"style":null}
```

# 4 Analysis
```bgraph-section
{"id":"38ef6d1e-273f-5c79-a122-2690c888cfc5","node_type":"Section","location":{"semantic":{"path":"8","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":225.29031,"width":65.03226,"height":13.935484}}},"text_order":85,"token_count":2,"style":null}
```

## 4.1 Experiment 1
```bgraph-section
{"id":"b6621968-0b63-56af-a9c1-51f4fb280b61","node_type":"Section","location":{"semantic":{"path":"8.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":251.6129,"width":82.83871,"height":12.387096}}},"text_order":86,"token_count":4,"style":null}
```

### 4.1.1 Datasets
```bgraph-section
{"id":"7cf3fe14-78f7-5cd6-bb89-da85abebe6f0","node_type":"Section","location":{"semantic":{"path":"8.1.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.1 Datasets"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":272.5161,"width":69.677414,"height":12.387096}}},"text_order":87,"token_count":3,"style":null}
```

We selected two datasets in the one million token range, each representative of corpora that users may encounter in their real-world activities:
```bgraph-paragraph
{"id":"8aeaa0be-2640-5fbb-b856-ba5e8319efc4","node_type":"Paragraph","location":{"semantic":{"path":"8.1.1.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.1 Datasets"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":293.41934,"width":401.03226,"height":23.225805}}},"text_order":88,"token_count":36,"style":null}
```

**Podcast transcripts.** Public transcripts of *Behind the Tech with Kevin Scott*, a podcast featuring conversations between Microsoft CTO Kevin Scott and various thought leaders in science and technology (Scott, 2024). This corpus was divided into 1669 × 600-token text chunks, with 100-token overlaps between chunks (~1 million tokens).
```bgraph-paragraph
{"id":"fb511d42-eddf-52db-8441-96ba46c08382","node_type":"Paragraph","location":{"semantic":{"path":"8.1.1.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.1 Datasets"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":320.5161,"width":401.03226,"height":44.129032}}},"text_order":89,"token_count":84,"style":null}
```

**News articles.** A benchmark dataset comprised of news articles published from September 2013 to December 2023 in a range of categories, including entertainment, business, sports, technology, health, and science (Tang and Yang, 2024). The corpus is divided into 3197 × 600-token text chunks, with 100-token overlaps between chunks (~1.7 million tokens).
```bgraph-paragraph
{"id":"03067a9b-6ccf-51b9-bd22-dbffc884bb3e","node_type":"Paragraph","location":{"semantic":{"path":"8.1.1.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.1 Datasets"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":369.2903,"width":401.03226,"height":45.677418}}},"text_order":90,"token_count":89,"style":null}
```

### 4.1.2 Conditions
```bgraph-section
{"id":"1188bc73-1d2d-5366-bfa7-4026ec7b2467","node_type":"Section","location":{"semantic":{"path":"8.1.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":428.12903,"width":80.51613,"height":11.612903}}},"text_order":91,"token_count":4,"style":null}
```

We compared six conditions including GraphRAG at four different graph community levels (**C0**, **C1**, **C2**, **C3**), a text summarization method that applies our map-reduce approach directly to source texts (**TS**), and a vector RAG “semantic search” approach (**SS**):
```bgraph-paragraph
{"id":"871389ca-9127-57b2-8c39-653163ab7c06","node_type":"Paragraph","location":{"semantic":{"path":"8.1.2.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":448.25806,"width":401.03226,"height":34.064514}}},"text_order":92,"token_count":69,"style":null}
```

- **C0.** Uses root-level community summaries (fewest in number) to answer user queries.
- **C1.** Uses high-level community summaries to answer queries. These are sub-communities of C0, if present, otherwise C0 communities projected downwards.
- **C2.** Uses intermediate-level community summaries to answer queries. These are sub-communities of C1, if present, otherwise C1 communities projected downwards.
- **C3.** Uses low-level community summaries (greatest in number) to answer queries. These are sub-communities of C2, if present, otherwise C2 communities projected downwards.
- **TS.** The same method as in Section 3.1.6, except source texts (rather than community summaries) are shuffled and chunked for the map-reduce summarization stages.
- **SS.** An implementation of vector RAG in which text chunks are retrieved and added to the available context window until the specified token limit is reached.
```bgraph-list
{"id":"2c0b4eb5-37cd-522e-9f81-9d626c5ed68f","node_type":"List","location":{"semantic":{"path":"8.1.2.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":131.6129,"y":493.1613,"width":373.9355,"height":154.8387}}},"text_order":93,"token_count":228,"style":null}
```

The size of the context window and the prompts used for answer generation are the same across all six conditions (except for minor modifications to reference styles to match the types of context information used). Conditions only differ in how the contents of the context window are created.
```bgraph-paragraph
{"id":"60bc46bf-09cc-54ed-a7c0-2bb7bc05329b","node_type":"Paragraph","location":{"semantic":{"path":"8.1.2.3","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":659.61285,"width":401.03226,"height":34.064514}}},"text_order":94,"token_count":72,"style":null}
```

The graph index supporting conditions **C0-C3** was created using our generic prompts for entity and relationship extraction, with entity types and few-shot examples tailored to the domain of the data.
```bgraph-paragraph
{"id":"36a2c2ab-e0a8-5c1b-946a-9ac5965f7447","node_type":"Paragraph","location":{"semantic":{"path":"8.1.2.4","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":697.54834,"width":401.03226,"height":23.225805}}},"text_order":95,"token_count":50,"style":null}
```

8
```bgraph-footer
{"id":"bdf29f04-a8f1-5bcb-bd0b-535e7056caa5","node_type":"Footer","location":{"semantic":{"path":"8.1.2.5","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.2 Conditions"]},"physical":{"page":8,"bounding_box":{"x":299.61288,"y":738.5806,"width":10.064516,"height":11.612903}}},"text_order":96,"token_count":1,"style":null}
```

### 4.1.3 Configuration
```bgraph-section
{"id":"ddfea33c-7ddf-5143-81c8-b75e866a859b","node_type":"Section","location":{"semantic":{"path":"8.1.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.3 Configuration"]},"physical":{"page":9,"bounding_box":{"x":105.29032,"y":72.77419,"width":93.677414,"height":12.387096}}},"text_order":97,"token_count":4,"style":null}
```

We used a fixed context window size of 8k tokens for generating community summaries, community answers, and global answers (explained in Appendix C). Graph indexing with a 600 token window (explained in Section A.2) took 281 minutes for the Podcast dataset, running on a virtual machine (16GB RAM, Intel(R) Xeon(R) Platinum 8171M CPU @ 2.60GHz) and using a public OpenAI endpoint for gpt-4-turbo (2M TPM, 10k RPM).
```bgraph-paragraph
{"id":"1a53b8a1-e319-5a3e-b669-39b9b8b0e300","node_type":"Paragraph","location":{"semantic":{"path":"8.1.3.1","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.3 Configuration"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":91.354836,"width":400.25806,"height":55.741936}}},"text_order":98,"token_count":103,"style":null}
```

We implemented Leiden community detection using the graspologic library (Chung et al., 2019). The prompts used to generate the graph index and global answers can be found in Appendix E, while the prompts used to evaluate LLM responses against our criteria can be found in Appendix F. A full statistical analysis of the results presented in the next section can be found in Appendix G.
```bgraph-paragraph
{"id":"7638ae20-d5e5-5ea4-aaab-0d6329db26db","node_type":"Paragraph","location":{"semantic":{"path":"8.1.3.2","depth":4,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.1 Experiment 1","4.1.3 Configuration"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":151.74193,"width":400.25806,"height":44.903225}}},"text_order":99,"token_count":96,"style":null}
```

## 4.2 Experiment 2
```bgraph-section
{"id":"530fee32-8085-5389-93ae-434b16b56f30","node_type":"Section","location":{"semantic":{"path":"8.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":105.29032,"y":209.03226,"width":84.38709,"height":12.387096}}},"text_order":100,"token_count":4,"style":null}
```

To validate the comprehensiveness and diversity results from Experiment 1, we implemented claim-based measures of these qualities. We use the definition of a factual claim from Ni et al. (2024), which is “a statement that explicitly presents some verifiable facts.” For example, the sentence “California and New York implemented incentives for renewable energy adoption, highlighting the broader importance of sustainability in policy decisions” contains two factual claims: (1) California implemented incentives for renewable energy adoption, and (2) New York implemented incentives for renewable energy adoption.
```bgraph-paragraph
{"id":"83cec38d-0717-5f6b-8804-2eae95b83faf","node_type":"Paragraph","location":{"semantic":{"path":"8.2.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":230.70967,"width":400.25806,"height":76.64516}}},"text_order":101,"token_count":155,"style":null}
```

To extract factual claims, we used Claimify (Metropolitansky and Larson, 2025), an LLM-based method that identifies sentences in an answer containing at least one factual claim, then decomposes these sentences into simple, self-contained factual claims. We applied Claimify to the answers generated under the conditions from Experiment 1. After removing duplicate claims from each answer, we extracted 47,075 unique claims, with an average of 31 claims per answer.
```bgraph-paragraph
{"id":"9082587c-763f-5ca2-aff4-9b177dff632e","node_type":"Paragraph","location":{"semantic":{"path":"8.2.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":311.2258,"width":400.25806,"height":57.29032}}},"text_order":102,"token_count":116,"style":null}
```

We defined two metrics, with higher values indicating better performance:
```bgraph-paragraph
{"id":"13a1af36-3442-5d19-adda-73ceefc521e5","node_type":"Paragraph","location":{"semantic":{"path":"8.2.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":105.29032,"y":371.61288,"width":298.8387,"height":13.16129}}},"text_order":103,"token_count":18,"style":null}
```

1. Comprehensiveness: Measured as the average number of claims extracted from the answers generated under each condition.
2. Diversity: Measured by clustering the claims for each answer and calculating the average number of clusters.
```bgraph-list
{"id":"06e7b13f-839d-56a1-b2f5-a889c40d1d28","node_type":"List","location":{"semantic":{"path":"8.2.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":127.74193,"y":393.2903,"width":377.03226,"height":48.0}}},"text_order":104,"token_count":58,"style":null}
```

For clustering, we followed the approach described by Padmakumar and He (2024), which involved using Scikit-learn’s implementation of agglomerative clustering (Pedregosa et al., 2011). Clusters were merged through “complete” linkage, meaning they were combined only if the maximum distance between their farthest points was less than or equal to a predefined distance threshold. The distance metric used was 1 – ROUGE-L. Since the distance threshold influences the number of clusters, we report results across a range of thresholds.
```bgraph-paragraph
{"id":"6d44b82d-c95b-5e36-ada3-1c8d6b0b6462","node_type":"Paragraph","location":{"semantic":{"path":"8.2.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","4 Analysis","4.2 Experiment 2"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":451.35483,"width":400.25806,"height":67.354836}}},"text_order":105,"token_count":135,"style":null}
```

# 5 Results
```bgraph-section
{"id":"b376ff9a-73f5-5244-a9f2-1a7369b78416","node_type":"Section","location":{"semantic":{"path":"9","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results"]},"physical":{"page":9,"bounding_box":{"x":105.29032,"y":534.19354,"width":58.838707,"height":12.387096}}},"text_order":106,"token_count":2,"style":null}
```

## 5.1 Experiment 1
```bgraph-section
{"id":"e48824fa-3094-53b1-b810-7abf83f39675","node_type":"Section","location":{"semantic":{"path":"9.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":105.29032,"y":558.9677,"width":83.6129,"height":11.612903}}},"text_order":107,"token_count":4,"style":null}
```

The indexing process resulted in a graph consisting of 8,564 nodes and 20,691 edges for the Podcast dataset, and a larger graph of 15,754 nodes and 19,520 edges for the News dataset. Table 2 shows the number of community summaries at different levels of each graph community hierarchy.
```bgraph-paragraph
{"id":"25b54c65-fd9a-5735-b46d-619c9fd21dcd","node_type":"Paragraph","location":{"semantic":{"path":"9.1.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":579.871,"width":400.25806,"height":34.064514}}},"text_order":108,"token_count":71,"style":null}
```

Global approaches vs. vector RAG. As shown in Figure 2 and Table 6, global approaches significantly outperformed conventional vector RAG (SS) in both comprehensiveness and diversity criteria across datasets. Specifically, global approaches achieved comprehensiveness win rates between 72-83% (p<.001) for Podcast transcripts and 72-80% (p<.001) for News articles, while diversity win rates ranged from 75-82% (p<.001) and 62-71% (p<.01) respectively. Our use of directness as a validity test confirmed that vector RAG produces the most direct responses across all comparisons.
```bgraph-paragraph
{"id":"5264c995-af4b-5688-8173-afb9a86c7494","node_type":"Paragraph","location":{"semantic":{"path":"9.1.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":617.80646,"width":400.25806,"height":67.354836}}},"text_order":109,"token_count":144,"style":null}
```

Empowerment. Empowerment comparisons showed mixed results for both global approaches versus vector RAG (SS) and GraphRAG approaches versus source text summarization (TS). Using an LLM to analyze LLM reasoning for this measure indicated that the ability to provide specific exam-
```bgraph-paragraph
{"id":"683d5933-581a-5355-858c-6aae6ef4cb91","node_type":"Paragraph","location":{"semantic":{"path":"9.1.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":689.0322,"width":400.25806,"height":34.064514}}},"text_order":110,"token_count":69,"style":null}
```

9
```bgraph-footer
{"id":"306fd6b7-5aaf-5f30-9acf-40d0352d0ca2","node_type":"Footer","location":{"semantic":{"path":"9.1.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":9,"bounding_box":{"x":301.9355,"y":741.67737,"width":6.967742,"height":8.516129}}},"text_order":111,"token_count":1,"style":null}
```

Podcast transcripts
```bgraph-paragraph
{"id":"af641297-17e7-5d0c-bfca-252d0874a151","node_type":"Paragraph","location":{"semantic":{"path":"9.1.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":262.4516,"y":69.677414,"width":85.93548,"height":11.612903}}},"text_order":112,"token_count":4,"style":null}
```

|   | SS | TS | C0 | C1 | C2 | C3 |  | SS | TS | C0 | C1 | C2 | C3 |  | SS | TS | C0 | C1 | C2 | C3 |  | SS | TS | C0 | C1 | C2 | C3  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  SS | 50 | 17 | 28 | 25 | 22 | 21 | SS | 50 | 18 | 23 | 25 | 19 | 19 | SS | 50 | 42 | 57 | 52 | 49 | 51 | SS | 50 | 56 | 65 | 60 | 60 | 60  |
|  TS | 83 | 50 | 50 | 48 | 43 | 44 | TS | 82 | 50 | 50 | 50 | 43 | 46 | TS | 58 | 50 | 59 | 55 | 52 | 51 | TS | 44 | 50 | 55 | 52 | 51 | 52  |
|  C0 | 72 | 50 | 50 | 53 | 50 | 49 | C0 | 77 | 50 | 50 | 50 | 46 | 44 | C0 | 43 | 41 | 50 | 49 | 47 | 48 | C0 | 35 | 45 | 50 | 47 | 48 | 48  |
|  C1 | 75 | 52 | 47 | 50 | 52 | 50 | C1 | 75 | 50 | 50 | 50 | 44 | 45 | C1 | 48 | 45 | 51 | 50 | 49 | 50 | C1 | 40 | 48 | 53 | 50 | 50 | 50  |
|  C2 | 78 | 57 | 50 | 48 | 50 | 52 | C2 | 81 | 57 | 54 | 56 | 50 | 48 | C2 | 51 | 48 | 53 | 51 | 50 | 51 | C2 | 40 | 49 | 52 | 50 | 50 | 50  |
|  C3 | 79 | 56 | 51 | 50 | 48 | 50 | C3 | 81 | 54 | 56 | 55 | 52 | 50 | C3 | 49 | 49 | 52 | 50 | 49 | 50 | C3 | 40 | 48 | 52 | 50 | 50 | 50  |
```bgraph-table
{"id":"b33718bb-bc2e-5e84-9630-c7c2cdb6c505","node_type":"Table","location":{"semantic":{"path":"9.1.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":107.6129,"y":88.258064,"width":389.41934,"height":108.38709}}},"text_order":113,"token_count":292,"style":null}
```

News articles
```bgraph-paragraph
{"id":"863d8732-b288-5e4b-a502-53ee63bc3237","node_type":"Paragraph","location":{"semantic":{"path":"9.1.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":275.61288,"y":202.06451,"width":58.838707,"height":10.064516}}},"text_order":114,"token_count":3,"style":null}
```

|   | SS | TS | C0 | C1 | C2 | C3 |  | SS | TS | C0 | C1 | C2 | C3 |  | SS | TS | C0 | C1 | C2 | C3 |  | SS | TS | C0 | C1 | C2 | C3  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  SS | 50 | 20 | 28 | 25 | 21 | 21 | SS | 50 | 33 | 38 | 35 | 29 | 31 | SS | 50 | 47 | 57 | 49 | 50 | 50 | SS | 50 | 54 | 59 | 55 | 55 | 54  |
|  TS | 80 | 50 | 44 | 41 | 38 | 36 | TS | 67 | 50 | 53 | 45 | 44 | 40 | TS | 53 | 50 | 58 | 50 | 50 | 48 | TS | 46 | 50 | 55 | 53 | 52 | 52  |
|  C0 | 72 | 56 | 50 | 52 | 54 | 52 | C0 | 62 | 47 | 50 | 40 | 41 | 41 | C0 | 43 | 42 | 50 | 42 | 45 | 44 | C0 | 41 | 45 | 50 | 48 | 48 | 47  |
|  C1 | 75 | 59 | 48 | 50 | 58 | 55 | C1 | 65 | 55 | 60 | 50 | 50 | 50 | C1 | 51 | 50 | 58 | 50 | 52 | 51 | C1 | 45 | 47 | 52 | 50 | 49 | 49  |
|  C2 | 79 | 62 | 46 | 42 | 50 | 59 | C2 | 71 | 56 | 59 | 50 | 50 | 51 | C2 | 50 | 50 | 55 | 48 | 50 | 50 | C2 | 45 | 48 | 52 | 51 | 50 | 49  |
|  C3 | 79 | 64 | 48 | 45 | 41 | 50 | C3 | 69 | 60 | 59 | 50 | 49 | 50 | C3 | 50 | 52 | 56 | 49 | 50 | 50 | C3 | 46 | 48 | 53 | 51 | 51 | 50  |
|  Comprehensiveness |   |   |   |   |   |   | Diversity |   |   |   |   |   |   | Empowerment |   |   |   |   |   |   | Directness  |   |   |   |   |   |   |
```bgraph-table
{"id":"963e9e73-57e9-5964-81bf-a0bd23a17b81","node_type":"Table","location":{"semantic":{"path":"9.1.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":107.6129,"y":219.09677,"width":389.41934,"height":108.38709}}},"text_order":115,"token_count":332,"style":null}
```

Figure 2: Head-to-head win rate percentages of (row condition) over (column condition) across two datasets, four metrics, and 125 questions per comparison (each repeated five times and averaged). The overall winner per dataset and metric is shown in bold. Self-win rates were not computed but are shown as the expected 50% for reference. All Graph RAG conditions outperformed naïve RAG on comprehensiveness and diversity. Conditions C1-C3 also showed slight improvements in answer comprehensiveness and diversity over TS (global text summarization without a graph index).
```bgraph-paragraph
{"id":"077fde7d-b0e5-582f-9a80-4dd68121a918","node_type":"Paragraph","location":{"semantic":{"path":"9.1.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":333.6774,"width":400.25806,"height":67.354836}}},"text_order":116,"token_count":143,"style":null}
```

Table 2: Number of context units (community summaries for C0-C3 and text chunks for TS), corresponding token counts, and percentage of the maximum token count. Map-reduce summarization of source texts is the most resource-intensive approach requiring the highest number of context tokens. Root-level community summaries (C0) require dramatically fewer tokens per query (9x-43x).
```bgraph-paragraph
{"id":"ca489927-8314-5a2b-9dea-88fe1745a20b","node_type":"Paragraph","location":{"semantic":{"path":"9.1.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":411.87094,"width":400.25806,"height":46.45161}}},"text_order":117,"token_count":94,"style":null}
```

|   | Podcast Transcripts |   |   |   |   | News Articles  |   |   |   |   |   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  C0 | C1 | C2 | C3 | TS | C0 | C1 | C2 | C3 | TS |   |
|  Units | 34 | 367 | 969 | 1310 | 1669 | 55 | 555 | 1797 | 2142 | 3197 |   |
|  Tokens | 26657 | 225756 | 565720 | 746100 | 1014611 | 39770 | 352641 | 980898 | 1140266 | 1707694 |   |
|  % Max | 2.6 | 22.2 | 55.8 | 73.5 | 100 | 2.3 | 20.7 | 57.4 | 66.8 | 100 |   |
```bgraph-table
{"id":"fb55e39b-37f9-5a7e-8168-fde3909b1bd4","node_type":"Table","location":{"semantic":{"path":"9.1.11","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":106.064514,"y":462.19354,"width":400.25806,"height":68.90322}}},"text_order":118,"token_count":120,"style":null}
```

ples, quotes, and citations was judged to be key to helping users reach an informed understanding. Tuning element extraction prompts may help to retain more of these details in the GraphRAG index.
```bgraph-paragraph
{"id":"93ce6223-eda6-5c92-be1a-e14211a7ae8a","node_type":"Paragraph","location":{"semantic":{"path":"9.1.12","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":552.7742,"width":400.25806,"height":23.225805}}},"text_order":119,"token_count":49,"style":null}
```

Community summaries vs. source texts. When comparing community summaries to source texts using GraphRAG, community summaries generally provided a small but consistent improvement in answer comprehensiveness and diversity, except for root-level summaries. Intermediate-level summaries in the Podcast dataset and low-level community summaries in the News dataset achieved comprehensiveness win rates of 57% (p<.001) and 64% (p<.001), respectively. Diversity win rates were 57% (p=.036) for Podcast intermediate-level summaries and 60% (p<.001) for News low-level community summaries. Table 2 also illustrates the scalability advantages of GraphRAG compared to source text summarization: for low-level community summaries (C3), GraphRAG required 26-33% fewer context tokens, while for root-level community summaries (C0), it required over 97% fewer tokens. For a modest drop in performance compared with other global methods, root-level GraphRAG offers a highly efficient method for the iterative question answering that characterizes sensemaking activity, while retaining advantages in comprehensiveness (72% win rate) and diversity (62% win rate) over vector RAG.
```bgraph-paragraph
{"id":"963f9f42-3fcf-5ea7-88ed-5e7d5bb5f3cc","node_type":"Paragraph","location":{"semantic":{"path":"9.1.13","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":579.871,"width":400.25806,"height":143.2258}}},"text_order":120,"token_count":290,"style":null}
```

10
```bgraph-footer
{"id":"53bbc5b4-38dc-566a-a42e-762e560a7ce1","node_type":"Footer","location":{"semantic":{"path":"9.1.14","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":10,"bounding_box":{"x":300.3871,"y":741.67737,"width":10.838709,"height":9.290322}}},"text_order":121,"token_count":1,"style":null}
```

Table 3: Average number of extracted claims, reported by condition and dataset type. Bolded values represent the highest score in each column.
```bgraph-paragraph
{"id":"485c7cbf-42c7-53c0-b4b4-5cb3309df6d7","node_type":"Paragraph","location":{"semantic":{"path":"9.1.15","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":69.677414,"width":400.25806,"height":23.225805}}},"text_order":122,"token_count":35,"style":null}
```

|  Condition | Average Number of Claims  |   |
| --- | --- | --- |
|   |  News Articles | Podcast Transcripts  |
|  C0 | 34.18 | 32.21  |
|  C1 | 32.50 | 32.20  |
|  C2 | 31.62 | 32.46  |
|  C3 | 33.14 | 32.28  |
|  TS | 32.89 | 31.39  |
|  SS | 25.23 | 26.50  |
```bgraph-table
{"id":"252ea156-437a-5899-98e4-a20dd137b7c7","node_type":"Table","location":{"semantic":{"path":"9.1.16","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.1 Experiment 1"]},"physical":{"page":11,"bounding_box":{"x":192.77419,"y":98.32258,"width":223.74193,"height":105.29032}}},"text_order":123,"token_count":65,"style":null}
```

## 5.2 Experiment 2
```bgraph-section
{"id":"f54690a8-dde9-5b8d-a809-3bde8c3bb90a","node_type":"Section","location":{"semantic":{"path":"9.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":227.6129,"width":85.161285,"height":11.612903}}},"text_order":124,"token_count":4,"style":null}
```

Table 3 shows the results for the average number of extracted claims (i.e., the claim-based measure of comprehensiveness) per condition. For both the News and Podcast datasets, all global search conditions (C0-C3) and source text summarization (TS) had greater comprehensiveness than vector RAG (SS). The differences were statistically significant (p<.05) in all cases. These findings align with the LLM-based win rates from Experiment 1.
```bgraph-paragraph
{"id":"cbad99ef-c7d9-5df6-a937-a543c1e34c3c","node_type":"Paragraph","location":{"semantic":{"path":"9.2.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":249.29031,"width":400.25806,"height":54.96774}}},"text_order":125,"token_count":109,"style":null}
```

Table 4 contains the results for the average number of clusters, the claim-based measure of diversity. For the Podcast dataset, all global search conditions had significantly greater diversity than SS across all distance thresholds (p<.05), consistent with the win rates observed in Experiment 1. For the News dataset, however, only C0 significantly outperformed SS across all distance thresholds (p<.05). While C1-C3 also achieved higher average cluster counts than SS, the differences were statistically significant only at certain distance thresholds. In Experiment 1, all global search conditions significantly outperformed SS in the News dataset – not just C0. However, the differences in mean diversity scores between SS and the global search conditions were smaller for the News dataset than for the Podcast dataset, aligning directionally with the claim-based results.
```bgraph-paragraph
{"id":"23aac3f1-6792-5f3b-ac78-aa6518e7eb8b","node_type":"Paragraph","location":{"semantic":{"path":"9.2.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":308.12903,"width":400.25806,"height":99.870964}}},"text_order":126,"token_count":219,"style":null}
```

For both comprehensiveness and diversity, across both datasets, there were no statistically significant differences observed among the global search conditions or between global search and TS.
```bgraph-paragraph
{"id":"d3d927ea-ec9f-5121-8e2c-24e0558fb625","node_type":"Paragraph","location":{"semantic":{"path":"9.2.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":411.87094,"width":400.25806,"height":24.0}}},"text_order":127,"token_count":48,"style":null}
```

Finally, for each pairwise comparison in Experiment 1, we tested whether the answer preferred by the LLM aligned with the winner based on the claim-based metrics. Since each pairwise comparison in Experiment 1 was performed five times, while the claim-based metrics provided only one outcome per comparison, we aggregated the Experiment 1 results into a single label using majority voting. For example, if C0 won over SS in three out of five judgments for comprehensiveness on a given question, C0 was labeled the winner and SS the loser. However, if C0 won twice, SS won once, and they tied twice, then there was no majority outcome, so the final label was a tie.
```bgraph-paragraph
{"id":"b0e04f1d-861a-5639-88f8-aa97faa08d61","node_type":"Paragraph","location":{"semantic":{"path":"9.2.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":439.7419,"width":400.25806,"height":77.41935}}},"text_order":128,"token_count":166,"style":null}
```

We found that exact ties were rare for the claim-based metrics. One possible solution is to define a tie based on a threshold (e.g., the absolute difference between the claim-based results for condition A and condition B must be less than or equal to x). However, we observed that the results were sensitive to the choice of threshold. As a result, we focused on cases where the aggregated LLM label was not a tie, representing 33% and 39% of pairwise comparisons for comprehensiveness and diversity, respectively. In these cases, the aggregated LLM label matched the claim-based label in 78% of pairwise comparisons for comprehensiveness and 69-70% for diversity (across all distance thresholds), indicating moderately strong alignment.
```bgraph-paragraph
{"id":"9d68fba6-ea0e-5c7d-b823-75e5e7430b8f","node_type":"Paragraph","location":{"semantic":{"path":"9.2.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","5 Results","5.2 Experiment 2"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":521.80646,"width":400.25806,"height":89.80645}}},"text_order":129,"token_count":184,"style":null}
```

# 6 Discussion
```bgraph-section
{"id":"0aa8bded-9e9f-583c-8888-624dbf1f4123","node_type":"Section","location":{"semantic":{"path":"10","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":629.4193,"width":75.870964,"height":12.387096}}},"text_order":130,"token_count":3,"style":null}
```

## 6.1 Limitations of evaluation approach
```bgraph-section
{"id":"ac196c5a-c785-5a39-85d5-daff9608470a","node_type":"Section","location":{"semantic":{"path":"10.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":656.5161,"width":176.51613,"height":12.387096}}},"text_order":131,"token_count":9,"style":null}
```

Our evaluation to date has focused on sensemaking questions specific to two corpora each containing approximately 1 million tokens. More work is needed to understand how performance generalizes to datasets from various domains with different use cases. Comparison of fabrication rates, e.g., using approaches like SelfCheckGPT (Manakul et al., 2023), would also strengthen the current analysis.
```bgraph-paragraph
{"id":"c1b6f7cf-3a0f-5322-b96a-0c1f39360b2b","node_type":"Paragraph","location":{"semantic":{"path":"10.1.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":11,"bounding_box":{"x":104.51613,"y":678.9677,"width":400.25806,"height":44.903225}}},"text_order":132,"token_count":98,"style":null}
```

11
```bgraph-footer
{"id":"8bfd0ebf-c9d3-5fcf-999c-de076b2d4d40","node_type":"Footer","location":{"semantic":{"path":"10.1.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":11,"bounding_box":{"x":300.3871,"y":741.67737,"width":10.064516,"height":9.290322}}},"text_order":133,"token_count":1,"style":null}
```

Table 4: Average number of clusters across different distance thresholds, reported by condition and dataset type. Bolded values represent the highest score in each row.
```bgraph-paragraph
{"id":"08f88091-d09c-597c-b9b1-c5f8a13f2886","node_type":"Paragraph","location":{"semantic":{"path":"10.1.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":67.354836,"width":400.25806,"height":23.225805}}},"text_order":134,"token_count":42,"style":null}
```

|  Dataset | Distance Threshold | Average Number of Clusters  |   |   |   |   |   |
| --- | --- | --- | --- | --- | --- | --- | --- |
|   |   |  C0 | C1 | C2 | C3 | TS | SS  |
|  News Articles | 0.5 | **23.42** | 21.85 | 21.90 | 22.13 | 21.80 | 17.92  |
|   |  0.6 | **21.65** | 20.38 | 20.30 | 20.52 | 20.13 | 16.78  |
|   |  0.7 | **20.19** | 19.06 | 19.03 | 19.13 | 18.62 | 15.80  |
|   |  0.8 | **18.86** | 17.78 | 17.82 | 17.79 | 17.30 | 14.80  |
|  Podcast Transcripts | 0.5 | **23.16** | 22.62 | 22.52 | 21.93 | 21.14 | 18.55  |
|   |  0.6 | **21.65** | 21.33 | 21.21 | 20.62 | 19.70 | 17.39  |
|   |  0.7 | **20.41** | 20.04 | 19.79 | 19.22 | 18.08 | 16.28  |
|   |  0.8 | **19.26** | 18.77 | 18.46 | 17.89 | 16.66 | 15.07  |
```bgraph-table
{"id":"3c5a3120-f37f-5c1f-9c3a-17f93a690e0d","node_type":"Table","location":{"semantic":{"path":"10.1.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.1 Limitations of evaluation approach"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":95.22581,"width":391.7419,"height":130.8387}}},"text_order":135,"token_count":183,"style":null}
```

## 6.2 Future work
```bgraph-section
{"id":"3f2ab27a-777e-52ae-a493-bac93f856131","node_type":"Section","location":{"semantic":{"path":"10.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.2 Future work"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":245.41934,"width":79.741936,"height":13.16129}}},"text_order":136,"token_count":3,"style":null}
```

The graph index, rich text annotations, and hierarchical community structure supporting the current GraphRAG approach offer many possibilities for refinement and adaptation. This includes RAG approaches that operate in a more local manner, via embedding-based matching of user queries and graph annotations. In particular, we see potential in hybrid RAG schemes that combine embedding-based matching with just-in-time community report generation before employing our map-reduce summarization mechanisms. This “roll-up” approach could also be extended across multiple levels of the community hierarchy, as well as implemented as a more exploratory “drill down” mechanism that follows the information scent contained in higher-level community summaries.
```bgraph-paragraph
{"id":"ba4c4a30-05d3-58e6-a71d-4c9861c928c0","node_type":"Paragraph","location":{"semantic":{"path":"10.2.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.2 Future work"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":266.32257,"width":400.25806,"height":89.03226}}},"text_order":137,"token_count":189,"style":null}
```

*Broader impacts.* As a mechanism for question answering over large document collections, there are risks to downstream sensemaking and decision-making tasks if the generated answers do not accurately represent the source data. System use should be accompanied by clear disclosures of AI use and the potential for errors in outputs. Compared to vector RAG, however, GraphRAG shows promise as a way to mitigate these downstream risks for questions of a global nature, which might otherwise be answered by samples of retrieved facts falsely presented as global summaries.
```bgraph-paragraph
{"id":"815cafb6-fc3c-5bab-b3c7-20d3b9fadb95","node_type":"Paragraph","location":{"semantic":{"path":"10.2.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","6 Discussion","6.2 Future work"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":358.4516,"width":400.25806,"height":68.12903}}},"text_order":138,"token_count":142,"style":null}
```

# 7 Conclusion
```bgraph-section
{"id":"32e6931a-d233-5177-8583-ffe25137feda","node_type":"Section","location":{"semantic":{"path":"11","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","7 Conclusion"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":439.7419,"width":78.96774,"height":13.935484}}},"text_order":139,"token_count":3,"style":null}
```

We have presented GraphRAG, a RAG approach that combines knowledge graph generation and query-focused summarization (QFS) to support human sensemaking over entire text corpora. Initial evaluations show substantial improvements over a vector RAG baseline for both the comprehensiveness and diversity of answers, as well as favorable comparisons to a global but graph-free approach using map-reduce source text summarization. For situations requiring many global queries over the same dataset, summaries of root-level communities in the entity-based graph index provide a data index that is both superior to vector RAG and achieves competitive performance to other global methods at a fraction of the token cost.
```bgraph-paragraph
{"id":"50d0845c-5135-584f-8957-06f53c77ac7d","node_type":"Paragraph","location":{"semantic":{"path":"11.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","7 Conclusion"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":464.5161,"width":400.25806,"height":89.03226}}},"text_order":140,"token_count":177,"style":null}
```

# Acknowledgements
```bgraph-section
{"id":"88d8d8d9-731e-5575-9354-67a0133edf1f","node_type":"Section","location":{"semantic":{"path":"12","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Acknowledgements"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":567.4838,"width":102.967735,"height":13.935484}}},"text_order":141,"token_count":4,"style":null}
```

We would also like to thank the following people who contributed to the work: Alonso Guevara Fernández, Amber Hoak, Andrés Morales Esquivel, Ben Cutler, Billie Rinaldi, Chris Sanchez, Chris Trevino, Christine Caggiano, David Tittsworth, Dayenne de Souza, Douglas Orbaker, Ed Clark, Gabriel Nieves-Ponce, Gaudy Blanco Meneses, Kate Lytvynets, Katy Smith, Mónica Carvajal, Nathan Evans, Richard Ortega, Rodrigo Racanicci, Sarah Smith, and Shane Solomon.
```bgraph-paragraph
{"id":"e6205a7d-cc53-5a30-be2e-578f678b2985","node_type":"Paragraph","location":{"semantic":{"path":"12.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","Acknowledgements"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":592.25806,"width":400.25806,"height":55.741936}}},"text_order":142,"token_count":113,"style":null}
```

# References
```bgraph-section
{"id":"51b0de81-acca-5396-a48d-853f69a96e6c","node_type":"Section","location":{"semantic":{"path":"13","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":667.3548,"width":58.838707,"height":13.16129}}},"text_order":143,"token_count":2,"style":null}
```

Achiam, J., Adler, S., Agarwal, S., Ahmad, L., Akkaya, I., Aleman, F. L., Almeida, D., Altenschmidt, J., Altman, S., Anadkat, S., et al. (2023). Gpt-4 technical report. *arXiv preprint arXiv:2303.08774*.
```bgraph-paragraph
{"id":"daec777c-e787-5e5d-a79d-5d53127c63ad","node_type":"Paragraph","location":{"semantic":{"path":"13.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":685.9355,"width":400.25806,"height":34.064514}}},"text_order":144,"token_count":50,"style":null}
```

12
```bgraph-footer
{"id":"533f3150-c919-54f1-a670-bf54bece0a22","node_type":"Footer","location":{"semantic":{"path":"13.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":12,"bounding_box":{"x":296.5161,"y":737.80646,"width":15.4838705,"height":12.387096}}},"text_order":145,"token_count":1,"style":null}
```

Anil, R., Borgeaud, S., Wu, Y., Alayrac, J.-B., Yu, J., Soricut, R., Schalkwyk, J., Dai, A. M., Hauth, A., et al. (2023). Gemini: a family of highly capable multimodal models. *arXiv preprint arXiv:2312.11805*.Baek, J., Aji, A. F., and Saffari, A. (2023). Knowledge-augmented language model prompting for zero-shot knowledge graph question answering. *arXiv preprint arXiv:2306.04136*.Ban, T., Chen, L., Wang, X., and Chen, H. (2023). From query tools to causal architects: Harnessing large language models for advanced causal discovery from data.Barlaug, N. and Gulla, J. A. (2021). Neural networks for entity matching: A survey. *ACM Transactions on Knowledge Discovery from Data (TKDD)*, 15(3):1–37.Baumel, T., Eyal, M., and Elhadad, M. (2018). Query focused abstractive summarization: Incorporating query relevance, multi-document coverage, and summary length constraints into seq2seq models. *arXiv preprint arXiv:1801.07704*.Blondel, V. D., Guillaume, J.-L., Lambiotte, R., and Lefebvre, E. (2008). Fast unfolding of communities in large networks. *Journal of statistical mechanics: theory and experiment*, 2008(10):P10008.Brown, T., Mann, B., Ryder, N., Subbiah, M., Kaplan, J. D., Dhariwal, P., Neelakantan, A., Shyam, P., Sastry, G., Askell, A., et al. (2020). Language models are few-shot learners. *Advances in neural information processing systems*, 33:1877–1901.Cheng, X., Luo, D., Chen, X., Liu, L., Zhao, D., and Yan, R. (2024). Lift yourself up: Retrieval-augmented text generation with self-memory. *Advances in Neural Information Processing Systems*, 36.Christen, P. and Christen, P. (2012). *The data matching process*. Springer.Chung, J., Pedigo, B. D., Bridgeford, E. W., Varjavand, B. K., Helm, H. S., and Vogelstein, J. T. (2019). Graspy: Graph statistics in python. *Journal of Machine Learning Research*, 20(158):1–7.Dang, H. T. (2006). Duc 2005: Evaluation of question-focused summarization systems. In *Proceedings of the Workshop on Task-Focused Summarization and Question Answering*, pages 48–55.Elmagarmid, A. K., Ipeirotis, P. G., and Verykios, V. S. (2006). Duplicate record detection: A survey. *IEEE Transactions on knowledge and data engineering*, 19(1):1–16.Es, S., James, J., Espinosa-Anke, L., and Schockaert, S. (2023). Ragas: Automated evaluation of retrieval augmented generation. *arXiv preprint arXiv:2309.15217*.Etzioni, O., Cafarella, M., Downey, D., Kok, S., Popescu, A.-M., Shaked, T., Soderland, S., Weld, D. S., and Yates, A. (2004). Web-scale information extraction in knowitall: (preliminary results). In *Proceedings of the 13th International Conference on World Wide Web*, WWW '04, page 100–110, New York, NY, USA. Association for Computing Machinery.Feng, Z., Feng, X., Zhao, D., Yang, M., and Qin, B. (2023). Retrieval-generation synergy augmented large language models. *arXiv preprint arXiv:2310.05149*.Fortunato, S. (2010). Community detection in graphs. *Physics reports*, 486(3-5):75–174.Gao, Y., Xiong, Y., Gao, X., Jia, K., Pan, J., Bi, Y., Dai, Y., Sun, J., and Wang, H. (2023). Retrieval-augmented generation for large language models: A survey. *arXiv preprint arXiv:2312.10997*.He, X., Tian, Y., Sun, Y., Chawla, N. V., Laurent, T., LeCun, Y., Bresson, X., and Hooi, B. (2024). G-retriever: Retrieval-augmented generation for textual graph understanding and question answering. *arXiv preprint arXiv:2402.07630*.Huang, J., Chen, X., Mishra, S., Zheng, H. S., Yu, A. W., Song, X., and Zhou, D. (2023). Large language models cannot self-correct reasoning yet. *arXiv preprint arXiv:2310.01798*.
```bgraph-list
{"id":"c0154d32-ad1a-582e-8a49-52118d463f80","node_type":"List","location":{"semantic":{"path":"13.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":104.51613,"y":72.0,"width":401.03226,"height":648.7742}}},"text_order":146,"token_count":893,"style":null}
```

13
```bgraph-footer
{"id":"ccd942aa-d165-5691-8c1b-8abe9d59791e","node_type":"Footer","location":{"semantic":{"path":"13.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":13,"bounding_box":{"x":295.7419,"y":737.80646,"width":14.709677,"height":12.387096}}},"text_order":147,"token_count":1,"style":null}
```

Jacomy, M., Venturini, T., Heymann, S., and Bastian, M. (2014). Forceatlas2, a continuous graph layout algorithm for handy network visualization designed for the gephi software. *PLoS ONE* 9(6): e98679. https://doi.org/10.1371/journal.pone.0098679.

Jin, D., Yu, Z., Jiao, P., Pan, S., He, D., Wu, J., Philip, S. Y., and Zhang, W. (2021). A survey of community detection approaches: From statistical modeling to deep learning. *IEEE Transactions on Knowledge and Data Engineering*, 35(2):1149–1170.

Kang, M., Kwak, J. M., Baek, J., and Hwang, S. J. (2023). Knowledge graph-augmented language models for knowledge-grounded dialogue generation. *arXiv preprint arXiv:2305.18846*.

Khattab, O., Santhanam, K., Li, X. L., Hall, D., Liang, P., Potts, C., and Zaharia, M. (2022). Demonstrate-search-predict: Composing retrieval and language models for knowledge-intensive nlp. *arXiv preprint arXiv:2212.14024*.

Kim, D., Xie, L., and Ong, C. S. (2016). Probabilistic knowledge graph construction: Compositional and incremental approaches. In *Proceedings of the 25th ACM International on Conference on Information and Knowledge Management*, CIKM '16, page 2257–2262, New York, NY, USA. Association for Computing Machinery.

Kim, G., Kim, S., Jeon, B., Park, J., and Kang, J. (2023). Tree of clarifications: Answering ambiguous questions with retrieval-augmented large language models. *arXiv preprint arXiv:2310.14696*.

Klein, G., Moon, B., and Hoffman, R. R. (2006). Making sense of sensemaking 1: Alternative perspectives. *IEEE intelligent systems*, 21(4):70–73.

Kosinski, M. (2024). Evaluating large language models in theory of mind tasks. *Proceedings of the National Academy of Sciences*, 121(45):e2405460121.

Kuratov, Y., Bulatov, A., Anokhin, P., Sorokin, D., Sorokin, A., and Burtsev, M. (2024). In search of needles in a 11m haystack: Recurrent memory finds what llms miss.

LangChain (2024). Langchain graphs. https://langchain-graphrag.readthedocs.io/en/latest/.

Laskar, M. T. R., Hoque, E., and Huang, J. (2020). Query focused abstractive summarization via incorporating query relevance and transfer learning with transformer models. In *Advances in Artificial Intelligence: 33rd Canadian Conference on Artificial Intelligence, Canadian AI 2020, Ottawa, ON, Canada, May 13–15, 2020, Proceedings 33*, pages 342–348. Springer.

Lewis, P., Perez, E., Piktus, A., Petroni, F., Karpukhin, V., Goyal, N., Küttler, H., Lewis, M., Yih, W.-t., Rocktäschel, T., et al. (2020). Retrieval-augmented generation for knowledge-intensive nlp tasks. *Advances in Neural Information Processing Systems*, 33:9459–9474.

Liu, N. F., Lin, K., Hewitt, J., Paranjape, A., Bevilacqua, M., Petroni, F., and Liang, P. (2023). Lost in the middle: How language models use long contexts. arXiv:2307.03172.

LlamaIndex (2024). GraphRAG Implementation with LlamaIndex - V2. https://github.com/runllama/llama_index/blob/main/docs/docs/examples/cookbooks/GraphRAG_v2.ipynb.

Madaan, A., Tandon, N., Gupta, P., Hallinan, S., Gao, L., Wiegreffe, S., Alon, U., Dziri, N., Prabhumoye, S., Yang, Y., et al. (2024). Self-refine: Iterative refinement with self-feedback. *Advances in Neural Information Processing Systems*, 36.

Manakul, P., Liusie, A., and Gales, M. J. (2023). Selfcheckgpt: Zero-resource black-box hallucination detection for generative large language models. *arXiv preprint arXiv:2303.08896*.

Mao, Y., He, P., Liu, X., Shen, Y., Gao, J., Han, J., and Chen, W. (2020). Generation-augmented retrieval for open-domain question answering. *arXiv preprint arXiv:2009.08553*.

Martin, S., Brown, W. M., Klavans, R., and Boyack, K. (2011). Openord: An open-source toolbox for large graph layout. *SPIE Conference on Visualization and Data Analysis (VDA)*.

Melnyk, I., Dognin, P., and Das, P. (2022). Knowledge graph generation from text.
```bgraph-paragraph
{"id":"49fe96a2-b0e1-51a7-961a-6a7db3e26987","node_type":"Paragraph","location":{"semantic":{"path":"13.5","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":104.51613,"y":72.0,"width":401.03226,"height":648.7742}}},"text_order":148,"token_count":960,"style":null}
```

14
```bgraph-footer
{"id":"89f9ba24-af64-57da-9393-e0ff7106be34","node_type":"Footer","location":{"semantic":{"path":"13.6","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":14,"bounding_box":{"x":295.7419,"y":737.80646,"width":14.709677,"height":12.387096}}},"text_order":149,"token_count":1,"style":null}
```

Metropolitansky, D. and Larson, J. (2025). Towards effective extraction and evaluation of factual claims.

Microsoft (2023). The impact of large language models on scientific discovery: a preliminary study using gpt-4.

Mooney, R. J. and Bunescu, R. (2005). Mining knowledge from text using information extraction. SIGKDD Explor. Newsl., 7(1):3–10.

NebulaGraph (2024). Nebulagraph launches industry-first graph rag: Retrieval-augmented generation with llm based on knowledge graphs. https://www.nebula-graph.io/posts/graph-RAG.

Neo4J (2024). Get started with graphrag: Neo4j’s ecosystem tools. https://neo4j.com/developer-blog/graphrag-ecosystem-tools/.

Newman, M. E. (2006). Modularity and community structure in networks. Proceedings of the national academy of sciences, 103(23):8577–8582.

Ni, J., Shi, M., Stammbach, D., Sachan, M., Ash, E., and Leippold, M. (2024). AFaCTA: Assisting the annotation of factual claim detection with reliable LLM annotators. In Ku, L.-W., Martins, A., and Srikumar, V., editors, Proceedings of the 62nd Annual Meeting of the Association for Computational Linguistics (Volume 1: Long Papers), pages 1890–1912, Bangkok, Thailand. Association for Computational Linguistics.

OpenAI (2023). Chatgpt: Gpt-4 language model.

Padmakumar, V. and He, H. (2024). Does writing with language models reduce content diversity? ICLR.

Pedregosa, F., Varoquaux, G., Gramfort, A., Michel, V., Thirion, B., Grisel, O., Blondel, M., Prettenhofer, P., Weiss, R., Dubourg, V., Vanderplas, J., Passos, A., Cournapeau, D., Brucher, M., Perrot, M., and Duchesnay, E. (2011). Scikit-learn: Machine learning in python. Journal of Machine Learning Research, 12:2825–2830.

Ram, O., Levine, Y., Dalmedigos, I., Muhlgay, D., Shashua, A., Leyton-Brown, K., and Shoham, Y. (2023). In-context retrieval-augmented language models. Transactions of the Association for Computational Linguistics, 11:1316–1331.

Ranade, P. and Joshi, A. (2023). Fabula: Intelligence report generation using retrieval-augmented narrative construction. arXiv preprint arXiv:2310.13848.

Salminen, J., Liu, C., Pian, W., Chi, J., Häyhänen, E., and Jansen, B. J. (2024). Deus ex machina and personas from large language models: Investigating the composition of ai-generated persona descriptions. In Proceedings of the CHI Conference on Human Factors in Computing Systems, pages 1–20.

Sarthi, P., Abdullah, S., Tuli, A., Khanna, S., Goldie, A., and Manning, C. D. (2024). Raptor: Recursive abstractive processing for tree-organized retrieval. arXiv preprint arXiv:2401.18059.

Scott, K. (2024). Behind the Tech. https://www.microsoft.com/en-us/behind-the-tech.

Shao, Z., Gong, Y., Shen, Y., Huang, M., Duan, N., and Chen, W. (2023). Enhancing retrieval-augmented large language models with iterative retrieval-generation synergy. arXiv preprint arXiv:2305.15294.

Shin, J., Hedderich, M. A., Rey, B. J., Lucero, A., and Oulasvirta, A. (2024). Understanding human-ai workflows for generating personas. In Proceedings of the 2024 ACM Designing Interactive Systems Conference, pages 757–781.

Shinn, N., Cassano, F., Gopinath, A., Narasimhan, K., and Yao, S. (2024). Reflexion: Language agents with verbal reinforcement learning. Advances in Neural Information Processing Systems, 36.
```bgraph-paragraph
{"id":"dd05d3a9-20ab-58dc-9dc9-8f6d4aa6a640","node_type":"Paragraph","location":{"semantic":{"path":"13.7","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":104.51613,"y":72.0,"width":401.03226,"height":648.0}}},"text_order":150,"token_count":820,"style":null}
```

15
```bgraph-footer
{"id":"ac9e28ef-2591-5e2d-9760-35ad164eaf74","node_type":"Footer","location":{"semantic":{"path":"13.8","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":15,"bounding_box":{"x":295.7419,"y":737.80646,"width":14.709677,"height":12.387096}}},"text_order":151,"token_count":1,"style":null}
```

Su, D., Xu, Y., Yu, T., Siddique, F. B., Barezi, E. J., and Fung, P. (2020). Caire-covid: A question answering and query-focused multi-document summarization system for covid-19 scholarly information management. *arXiv preprint arXiv:2005.03975*.Tan, Z., Zhao, X., and Wang, W. (2017). Representation learning of large-scale knowledge graphs via entity feature combinations. In *Proceedings of the 2017 ACM on Conference on Information and Knowledge Management*, CIKM '17, page 1777–1786, New York, NY, USA. Association for Computing Machinery.Tang, Y. and Yang, Y. (2024). MultiHop-RAG: Benchmarking retrieval-augmented generation for multi-hop queries. *arXiv preprint arXiv:2401.15391*.Touvron, H., Martin, L., Stone, K., Albert, P., Almahairi, A., Babaei, Y., Bashlykov, N., Batra, S., Bhargava, P., Bhosale, S., et al. (2023). Llama 2: Open foundation and fine-tuned chat models. *arXiv preprint arXiv:2307.09288*.Traag, V. A., Waltman, L., and Van Eck, N. J. (2019). From Louvain to Leiden: guaranteeing well-connected communities. *Scientific Reports*, 9(1).Trajanoska, M., Stojanov, R., and Trajanov, D. (2023). Enhancing knowledge graph construction using large language models. *ArXiv*, abs/2305.04676.Trivedi, H., Balasubramanian, N., Khot, T., and Sabharwal, A. (2022). Interleaving retrieval with chain-of-thought reasoning for knowledge-intensive multi-step questions. *arXiv preprint arXiv:2212.10509*.Wang, J., Liang, Y., Meng, F., Sun, Z., Shi, H., Li, Z., Xu, J., Qu, J., and Zhou, J. (2023a). Is chatgpt a good nlg evaluator? a preliminary study. *arXiv preprint arXiv:2303.04048*.Wang, S., Khramtsova, E., Zhuang, S., and Zuccon, G. (2024). Feb4rag: Evaluating federated search in the context of retrieval augmented generation. *arXiv preprint arXiv:2402.11891*.Wang, X., Wei, J., Schuurmans, D., Le, Q., Chi, E., Narang, S., Chowdhery, A., and Zhou, D. (2022). Self-consistency improves chain of thought reasoning in language models. *arXiv preprint arXiv:2203.11171*.Wang, Y., Lipka, N., Rossi, R. A., Siu, A., Zhang, R., and Derr, T. (2023b). Knowledge graph prompting for multi-document question answering.Xu, Y. and Lapata, M. (2021). Text summarization with latent queries. *arXiv preprint arXiv:2106.00104*.Yang, Z., Qi, P., Zhang, S., Bengio, Y., Cohen, W. W., Salakhutdinov, R., and Manning, C. D. (2018). HotpotQA: A dataset for diverse, explainable multi-hop question answering. In *Conference on Empirical Methods in Natural Language Processing (EMNLP)*.Yao, J.-g., Wan, X., and Xiao, J. (2017). Recent advances in document summarization. *Knowledge and Information Systems*, 53:297–336.Yao, L., Peng, J., Mao, C., and Luo, Y. (2023). Exploring large language models for knowledge graph completion.Yates, A., Banko, M., Broadhead, M., Cafarella, M., Etzioni, O., and Soderland, S. (2007). TextRunner: Open information extraction on the web. In Carpenter, B., Stent, A., and Williams, J. D., editors, *Proceedings of Human Language Technologies: The Annual Conference of the North American Chapter of the Association for Computational Linguistics (NAACL-HLT)*, pages 25–26, Rochester, New York, USA. Association for Computational Linguistics.Yuan, X., Li, J., Wang, D., Chen, Y., Mao, X., Huang, L., Xue, H., Wang, W., Ren, K., and Wang, J. (2024). S-eval: Automatic and adaptive test generation for benchmarking safety evaluation of large language models. *arXiv preprint arXiv:2405.14191*.Zhang, J. (2023). Graph-toolformer: To empower llms with graph reasoning ability via prompt augmented by chatgpt. *arXiv preprint arXiv:2304.11116*.
```bgraph-list
{"id":"6d17df4e-8c74-5a1d-95a8-171aaac86e7b","node_type":"List","location":{"semantic":{"path":"13.9","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":104.51613,"y":72.0,"width":401.03226,"height":648.7742}}},"text_order":152,"token_count":894,"style":null}
```

16
```bgraph-footer
{"id":"07f245cc-fa3e-5bba-b49f-52456190b9b0","node_type":"Footer","location":{"semantic":{"path":"13.10","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":16,"bounding_box":{"x":295.7419,"y":737.80646,"width":15.4838705,"height":12.387096}}},"text_order":153,"token_count":1,"style":null}
```

Zhang, Y., Zhang, Y., Gan, Y., Yao, L., and Wang, C. (2024a). Causal graph discovery with retrieval-augmented generation based large language models. *arXiv preprint arXiv:2402.15301*.

Zhang, Z., Chen, J., and Yang, D. (2024b). Darg: Dynamic evaluation of large language models via adaptive reasoning graph. *arXiv preprint arXiv:2406.17271*.

Zheng, L., Chiang, W.-L., Sheng, Y., Zhuang, S., Wu, Z., Zhuang, Y., Lin, Z., Li, Z., Li, D., Xing, E., et al. (2024). Judging llm-as-a-judge with mt-bench and chatbot arena. *Advances in Neural Information Processing Systems*, 36.

Zhu, Y., Wang, X., Chen, J., Qiao, S., Ou, Y., Yao, Y., Deng, S., Chen, H., and Zhang, N. (2024). Llms for knowledge graph construction and reasoning: Recent capabilities and future opportunities.
```bgraph-paragraph
{"id":"953d4b5b-a3c6-5577-9b03-bb62ba1f5235","node_type":"Paragraph","location":{"semantic":{"path":"13.11","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":17,"bounding_box":{"x":103.741936,"y":72.0,"width":401.80643,"height":133.93549}}},"text_order":154,"token_count":193,"style":null}
```

17
```bgraph-footer
{"id":"69ddc3b5-725c-5a24-8675-90db1307a308","node_type":"Footer","location":{"semantic":{"path":"13.12","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","References"]},"physical":{"page":17,"bounding_box":{"x":295.7419,"y":737.0322,"width":16.258064,"height":13.935484}}},"text_order":155,"token_count":1,"style":null}
```

# A Entity and Relationship Extraction Approach
```bgraph-section
{"id":"6b3708f5-2893-5e47-a5cc-6c51d112d83f","node_type":"Section","location":{"semantic":{"path":"14","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":71.22581,"width":257.80646,"height":14.709677}}},"text_order":156,"token_count":11,"style":null}
```

The following prompts, designed for GPT-4, are used in the default GraphRAG initialization pipeline:
```bgraph-paragraph
{"id":"d615be6d-b08a-5d0f-8842-77d034aef530","node_type":"Paragraph","location":{"semantic":{"path":"14.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":95.22581,"width":400.25806,"height":24.774193}}},"text_order":157,"token_count":25,"style":null}
```

- Default Graph Extraction Prompt
- Claim Extraction Prompt
```bgraph-list
{"id":"95ae77bb-7af4-552d-aec9-f006c02ad5ea","node_type":"List","location":{"semantic":{"path":"14.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach"]},"physical":{"page":18,"bounding_box":{"x":132.3871,"y":126.967735,"width":145.54839,"height":26.32258}}},"text_order":158,"token_count":14,"style":null}
```

## A.1 Entity Extraction
```bgraph-section
{"id":"febf4ebc-e43c-5ef1-9864-b8564e3cd2bf","node_type":"Section","location":{"semantic":{"path":"14.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.1 Entity Extraction"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":166.45161,"width":103.741936,"height":11.612903}}},"text_order":159,"token_count":5,"style":null}
```

We do this using a multipart LLM prompt that first identifies all entities in the text, including their name, type, and description, before identifying all relationships between clearly related entities, including the source and target entities and a description of their relationship. Both kinds of element instance are output in a single list of delimited tuples.
```bgraph-paragraph
{"id":"92740447-4709-58c3-a14b-6017691ac271","node_type":"Paragraph","location":{"semantic":{"path":"14.3.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.1 Entity Extraction"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":186.58064,"width":400.25806,"height":44.903225}}},"text_order":160,"token_count":91,"style":null}
```

## A.2 Self-Reflection
```bgraph-section
{"id":"8c32debd-b554-58a5-b506-ad7735b57f42","node_type":"Section","location":{"semantic":{"path":"14.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":243.09677,"width":91.354836,"height":11.612903}}},"text_order":161,"token_count":4,"style":null}
```

The choice of prompt engineering techniques has a strong impact on the quality of knowledge graph extraction (Zhu et al., 2024), and different techniques have different costs in terms of tokens consumed and generated by the model. Self-reflection is a prompt engineering technique where the LLM generates an answer, and is then prompted to evaluate its output for correctness, clarity, or completeness, then finally generate an improved response based on that evaluation (Huang et al., 2023; Madaan et al., 2024; Shinn et al., 2024; Wang et al., 2022). We leverage self-reflection in knowledge graph extraction, and explore ways how removing self-reflection affects performance and cost.
```bgraph-paragraph
{"id":"f29356f5-b0bb-57bf-8240-af19d62f9e60","node_type":"Paragraph","location":{"semantic":{"path":"14.4.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":262.4516,"width":400.25806,"height":89.03226}}},"text_order":162,"token_count":171,"style":null}
```

Using larger chunk size is less costly in terms of calls to the LLM. However, the LLM tends to extract few entities from chunks of larger size. For example, in a sample dataset (HotPotQA, Yang et al., 2018), GPT-4 extracted almost twice as many entity references when the chunk size was 600 tokens than when it was 2400. To address this issue, we deploy a self-reflection prompt engineering approach. After entities are extracted from a chunk, we provide the extracted entities back to the LLM, prompting it to “glean” any entities that it may have missed. This is a multi-stage process in which we first ask the LLM to assess whether all entities were extracted, using a logit bias of 100 to force a yes/no decision. If the LLM responds that entities were missed, then a continuation indicating that “MANY entities were missed in the last extraction” encourages the LLM to detect these missing entities. This approach allows us to use larger chunk sizes without a drop in quality (Figure 3) or the forced introduction of noise. We interate self-reflection steps up to a specified maximum number of times.
```bgraph-paragraph
{"id":"876bf6ce-40e0-5fe2-8c1e-dd5b0365d283","node_type":"Paragraph","location":{"semantic":{"path":"14.4.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":356.12903,"width":401.80643,"height":131.6129}}},"text_order":163,"token_count":278,"style":null}
```

![img-1.jpeg](img-1.jpeg)
```bgraph-image
{"id":"35a15a1b-7489-5462-9b4b-2c9bc8ca161d","node_type":"Image","location":{"semantic":{"path":"14.4.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":107.6129,"y":498.58063,"width":402.58063,"height":133.16129}}},"text_order":164,"token_count":0,"style":null,"image":{"id":"sha256:f76cecb4fc4fffb668a5c7bf70ae1fb89182103a67fe29aeebea93558b23fd83","annotation":null,"base64":"data:image/jpeg;base64,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"}}
```

Figure 3: How the entity references detected in the HotPotQA dataset (Yang et al., 2018) varies with chunk size and self-reflection iterations for our generic entity extraction prompt with gpt-4-turbo.
```bgraph-paragraph
{"id":"3a30133f-f7dd-5930-bf27-4580df3d50c8","node_type":"Paragraph","location":{"semantic":{"path":"14.4.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","A Entity and Relationship Extraction Approach","A.2 Self-Reflection"]},"physical":{"page":18,"bounding_box":{"x":110.70967,"y":641.0322,"width":388.64514,"height":34.838707}}},"text_order":165,"token_count":50,"style":null}
```

# B Example Community Detection
```bgraph-section
{"id":"17481a1e-cfde-51d6-bbf2-61ce08f8d2e7","node_type":"Section","location":{"semantic":{"path":"15","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":18,"bounding_box":{"x":104.51613,"y":699.09674,"width":185.03226,"height":13.935484}}},"text_order":166,"token_count":7,"style":null}
```

18
```bgraph-footer
{"id":"88ce5e73-70ba-5df6-bb95-83b8f5ac6008","node_type":"Footer","location":{"semantic":{"path":"15.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":18,"bounding_box":{"x":300.3871,"y":741.67737,"width":10.838709,"height":9.290322}}},"text_order":167,"token_count":1,"style":null}
```

![img-2.jpeg](img-2.jpeg)
```bgraph-image
{"id":"4d17e2e1-e3f7-56a7-9f65-40abf11fa367","node_type":"Image","location":{"semantic":{"path":"15.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":19,"bounding_box":{"x":116.12903,"y":70.451614,"width":193.54839,"height":205.16129}}},"text_order":168,"token_count":0,"style":null,"image":{"id":"sha256:8b6c1e9f0258aaab289ffa8dffd06e5c135592e5bd43c95b6118c509d284c99b","annotation":null,"base64":"data:image/jpeg;base64,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"}}
```

(a) Root communities at level 0
```bgraph-paragraph
{"id":"53f4eebb-57b6-5cf7-a072-312c20303fad","node_type":"Paragraph","location":{"semantic":{"path":"15.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":19,"bounding_box":{"x":148.64516,"y":279.48386,"width":127.74193,"height":11.612903}}},"text_order":169,"token_count":7,"style":null}
```

![img-3.jpeg](img-3.jpeg)
```bgraph-image
{"id":"41387fec-c902-5b3c-a744-b7b334c9cf45","node_type":"Image","location":{"semantic":{"path":"15.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":19,"bounding_box":{"x":325.1613,"y":71.22581,"width":194.32257,"height":204.38708}}},"text_order":170,"token_count":0,"style":null,"image":{"id":"sha256:67514a705d3fcd7e9f6bb4913556db000c90393126db1ee538b07559a95fae03","annotation":null,"base64":"data:image/jpeg;base64,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"}}
```

(b) Sub-communities at level 1
```bgraph-paragraph
{"id":"b879a9b3-d68a-5037-a39b-0c7d3a6d8c26","node_type":"Paragraph","location":{"semantic":{"path":"15.5","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":19,"bounding_box":{"x":360.0,"y":279.48386,"width":124.64516,"height":11.612903}}},"text_order":171,"token_count":7,"style":null}
```

Figure 4: Graph communities detected using the Leiden algorithm (Traag et al., 2019) over the MultiHop-RAG (Tang and Yang, 2024) dataset as indexed. Circles represent entity nodes with size proportional to their degree. Node layout was performed via OpenORD (Martin et al., 2011) and Force Atlas 2 (Jacomy et al., 2014). Node colors represent entity communities, shown at two levels of hierarchical clustering: (a) Level 0, corresponding to the hierarchical partition with maximum modularity, and (b) Level 1, which reveals internal structure within these root-level communities.
```bgraph-paragraph
{"id":"58a4d748-77cf-51c4-a8cf-4ea541a047ce","node_type":"Paragraph","location":{"semantic":{"path":"15.6","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","B Example Community Detection"]},"physical":{"page":19,"bounding_box":{"x":104.51613,"y":298.0645,"width":400.25806,"height":66.58064}}},"text_order":172,"token_count":144,"style":null}
```

# C Context Window Selection
```bgraph-section
{"id":"06c2dbf1-b7ee-55df-a281-6fc084d56ede","node_type":"Section","location":{"semantic":{"path":"16","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","C Context Window Selection"]},"physical":{"page":19,"bounding_box":{"x":105.29032,"y":383.2258,"width":158.70967,"height":13.16129}}},"text_order":173,"token_count":6,"style":null}
```

The effect of context window size on any particular task is unclear, especially for models like gpt-4-turbo with a large context size of 128k tokens. Given the potential for information to be “lost in the middle” of longer contexts (Kuratov et al., 2024; Liu et al., 2023), we wanted to explore the effects of varying the context window size for our combinations of datasets, questions, and metrics. In particular, our goal was to determine the optimum context size for our baseline condition (SS) and then use this uniformly for all query-time LLM use. To that end, we tested four context window sizes: 8k, 16k, 32k and 64k. Surprisingly, the smallest context window size tested (8k) was universally better for all comparisons on comprehensiveness (average win rate of 58.1%), while performing comparably with larger context sizes on diversity (average win rate = 52.4%), and empowerment (average win rate = 51.3%). Given our preference for more comprehensive and diverse answers, we therefore used a fixed context window size of 8k tokens for the final evaluation.
```bgraph-paragraph
{"id":"c0c2533a-f155-5100-bc44-dca9f4661fc5","node_type":"Paragraph","location":{"semantic":{"path":"16.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","C Context Window Selection"]},"physical":{"page":19,"bounding_box":{"x":104.51613,"y":407.2258,"width":400.25806,"height":121.548386}}},"text_order":174,"token_count":267,"style":null}
```

19
```bgraph-footer
{"id":"01263497-aead-55ce-b2ec-54d34322d3e2","node_type":"Footer","location":{"semantic":{"path":"16.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","C Context Window Selection"]},"physical":{"page":19,"bounding_box":{"x":300.3871,"y":740.9032,"width":10.838709,"height":10.064516}}},"text_order":175,"token_count":1,"style":null}
```

# D Example Answer Comparison
```bgraph-section
{"id":"3aa4e170-2bfe-58f6-94d4-b52d85d32a1b","node_type":"Section","location":{"semantic":{"path":"17","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":104.51613,"y":68.12903,"width":178.06451,"height":13.935484}}},"text_order":176,"token_count":6,"style":null}
```

Table 5: Example question, answers, and LLM-generated assessments for the News article dataset.
```bgraph-paragraph
{"id":"6ece3167-6f40-5bb9-9950-258e48480100","node_type":"Paragraph","location":{"semantic":{"path":"17.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":104.51613,"y":98.32258,"width":398.70966,"height":12.387096}}},"text_order":177,"token_count":23,"style":null}
```

|  Question | Which public figures are repeatedly mentioned across various entertainment articles?  |
| --- | --- |
|  **Graph RAG** | The entertainment industry is vast and diverse, encompassing film, television, music, sports, and digital media. Certain public figures stand out due to their significant contributions and influence across these sectors. The following summary highlights key individuals who are repeatedly mentioned in various entertainment articles, reflecting their impact and presence within the industry. **Actors and Directors [...] Public Figures in Controversy [...] Musicians and Executives [...] Athletes and Coaches [...] Influencers and Entrepreneurs [...]** The repeated mention of these figures in entertainment articles signifies their ongoing relevance and the public's interest in their work. Their influence spans across various aspects of entertainment, from shaping cultural narratives in film and television to driving trends in music and digital media. These individuals not only contribute to their respective fields but also influence the broader cultural landscape, often becoming central figures in social discussions and public discourse.  |
|  **Naïve RAG** | Public figures who are repeatedly mentioned across various entertainment articles include Taylor Swift, Travis Kelce, Britney Spears, and Justin Timberlake. These individuals have been highlighted for various reasons, ranging from their professional achievements to their personal lives. **Taylor Swift [...] Travis Kelce [...] Britney Spears [...] Justin Timberlake [...]** These figures are frequently covered due to their high-profile status and the public's interest in their careers and personal lives. Their activities, whether in music, sports, or personal relationships, have significant cultural and economic impacts, as evidenced by the media coverage and public reactions.  |
|  **LLM Decision** | **Comprehensiveness: Winner=1 (Graph RAG).** Answer 1 is better because it provides a more comprehensive and detailed list of public figures from a wider range of entertainment sectors, including film, television, music, sports, gaming, and digital media. It also includes examples of their contributions and the impact they have on their respective fields, as well as mentions of controversies and their implications. Answer 2, while detailed in its coverage of a few individuals, is limited to a smaller number of public figures and focuses primarily on their personal lives and relationships rather than a broad spectrum of their professional influence across the entertainment industry. **Diversity: Winner=1 (Graph RAG).** Answer 1 is better because it provides a more varied and rich response by covering a wide range of public figures from different sectors of the entertainment industry, including film, television, music, sports, gaming, and digital media. It offers insights into the contributions and influence of these figures, as well as controversies and their impact on public discourse. The answer also cites specific data sources for each mentioned figure, indicating a diverse range of evidence to support the claims. In contrast, Answer 2 focuses on a smaller group of public figures, primarily from the music industry and sports, and relies heavily on a single source for data, which makes it less diverse in perspectives and insights. **Empowerment: Winner=1 (Graph RAG).** Answer 1 is better because it provides a comprehensive and structured overview of public figures across various sectors of the entertainment industry, including film, television, music, sports, and digital media. It lists multiple individuals, providing specific examples of their contributions and the context in which they are mentioned in entertainment articles, along with references to data reports for each claim. This approach helps the reader understand the breadth of the topic and make informed judgments without being misled. In contrast, Answer 2 focuses on a smaller group of public figures and primarily discusses their personal lives and relationships, which may not provide as broad an understanding of the topic. While Answer 2 also cites sources, it does not match the depth and variety of Answer 1. **Directness: Winner=2 (Naïve RAG).** Answer 2 is better because it directly lists specific public figures who are repeatedly mentioned across various entertainment articles, such as Taylor Swift, Travis Kelce, Britney Spears, and Justin Timberlake, and provides concise explanations for their frequent mentions. Answer 1, while comprehensive, includes a lot of detailed information about various figures in different sectors of entertainment, which, while informative, does not directly answer the question with the same level of conciseness and specificity as Answer 2.  |
```bgraph-table
{"id":"fb682c23-0cfa-5a6e-93eb-12fc606fadec","node_type":"Table","location":{"semantic":{"path":"17.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":104.51613,"y":114.58064,"width":421.1613,"height":536.5161}}},"text_order":178,"token_count":1200,"style":null}
```

20
```bgraph-footer
{"id":"0317c739-4dd0-5fb6-9cfd-e99876fb1835","node_type":"Footer","location":{"semantic":{"path":"17.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","D Example Answer Comparison"]},"physical":{"page":20,"bounding_box":{"x":296.5161,"y":737.80646,"width":15.4838705,"height":12.387096}}},"text_order":179,"token_count":1,"style":null}
```

# E System Prompts
```bgraph-section
{"id":"fdee096d-2cb8-508d-8b34-d6754bc58340","node_type":"Section","location":{"semantic":{"path":"18","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":71.22581,"width":106.83871,"height":14.709677}}},"text_order":180,"token_count":4,"style":null}
```

## E.1 Element Instance Generation
```bgraph-section
{"id":"e3a77cea-a8f4-5620-9f74-7d5c118d0ec8","node_type":"Section","location":{"semantic":{"path":"18.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":96.0,"width":152.51613,"height":10.838709}}},"text_order":181,"token_count":7,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"ceabd15c-825f-508f-9c9b-3640dd4319f4","node_type":"Paragraph","location":{"semantic":{"path":"18.1.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":116.12903,"width":41.032257,"height":6.967742}}},"text_order":182,"token_count":2,"style":null}
```

Given a text document that is potentially relevant to this activity and a list of entity types, identify all entities of those types from the text and all relationships among the identified entities.
```bgraph-paragraph
{"id":"48b0b6c6-eb45-5973-bd21-803f781ce000","node_type":"Paragraph","location":{"semantic":{"path":"18.1.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":123.09677,"width":388.64514,"height":17.032257}}},"text_order":183,"token_count":49,"style":null}
```

---Steps---
```bgraph-paragraph
{"id":"ccf5a7c0-1d5f-50ee-9523-feb09431b095","node_type":"Paragraph","location":{"semantic":{"path":"18.1.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":147.09677,"width":44.903225,"height":8.516129}}},"text_order":184,"token_count":2,"style":null}
```

1. Identify all entities. For each identified entity, extract the following information:
```bgraph-list
{"id":"e7198810-d9e9-5143-8075-b0776ae7080d","node_type":"List","location":{"semantic":{"path":"18.1.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":155.6129,"width":336.0,"height":7.7419353}}},"text_order":185,"token_count":22,"style":null}
```

- entity_name: Name of the entity, capitalized
```bgraph-list
{"id":"34e5c970-8834-5698-a018-5ac8382f069b","node_type":"List","location":{"semantic":{"path":"18.1.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":163.35483,"width":176.51613,"height":7.7419353}}},"text_order":186,"token_count":11,"style":null}
```

- entity_type: One of the following types: {{entity_types}}
```bgraph-list
{"id":"12293df1-36bc-52c7-902d-2f9a2c202868","node_type":"List","location":{"semantic":{"path":"18.1.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":171.09677,"width":226.8387,"height":8.516129}}},"text_order":187,"token_count":14,"style":null}
```

- entity_description: Comprehensive description of the entity's attributes and activities
```bgraph-list
{"id":"b15d94f2-a68b-59c8-a2c4-149a15a49dfe","node_type":"List","location":{"semantic":{"path":"18.1.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":179.6129,"width":335.2258,"height":8.516129}}},"text_order":188,"token_count":22,"style":null}
```

Format each entity as ("entity"{tuple_delimiter}<entity_name>{tuple_delimiter}<entity_type>{tuple_delimiter}<entity_description>
```bgraph-paragraph
{"id":"9f34be95-eb58-53ae-bb28-1c6f611cbdae","node_type":"Paragraph","location":{"semantic":{"path":"18.1.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":195.09677,"width":363.87097,"height":17.032257}}},"text_order":189,"token_count":32,"style":null}
```

2. From the entities identified in step 1, identify all pairs of (source_entity, target_entity) that are *clearly related* to each other
```bgraph-list
{"id":"6c897398-eb7d-58bb-b798-9c1af4085443","node_type":"List","location":{"semantic":{"path":"18.1.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":219.87096,"width":376.25806,"height":15.4838705}}},"text_order":190,"token_count":34,"style":null}
```

For each pair of related entities, extract the following information:
```bgraph-paragraph
{"id":"6f2a7457-142a-5265-aed0-cb30797137ba","node_type":"Paragraph","location":{"semantic":{"path":"18.1.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":235.35483,"width":257.80646,"height":7.7419353}}},"text_order":191,"token_count":17,"style":null}
```

- source_entity: name of the source entity, as identified in step 1
```bgraph-list
{"id":"7a056346-473d-56ce-8423-60c973acd01e","node_type":"List","location":{"semantic":{"path":"18.1.11","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":243.09677,"width":254.70967,"height":8.516129}}},"text_order":192,"token_count":16,"style":null}
```

- target_entity: name of the target entity, as identified in step 1
```bgraph-list
{"id":"08de8530-2b99-58bb-95a2-7ea4d197d681","node_type":"List","location":{"semantic":{"path":"18.1.12","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":251.6129,"width":254.70967,"height":7.7419353}}},"text_order":193,"token_count":16,"style":null}
```

- relationship_description: explanation as to why you think the source entity and the target entity are related to each other
```bgraph-list
{"id":"fd08495e-8d98-520c-9054-f3f03bdbfaa4","node_type":"List","location":{"semantic":{"path":"18.1.13","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":259.35483,"width":387.09677,"height":14.709677}}},"text_order":194,"token_count":31,"style":null}
```

- relationship_strength: a numeric score indicating strength of the relationship between the source entity and target entity
```bgraph-list
{"id":"e4b86294-71cf-5993-938d-7d21b235b5af","node_type":"List","location":{"semantic":{"path":"18.1.14","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":274.0645,"width":397.9355,"height":17.806452}}},"text_order":195,"token_count":31,"style":null}
```

Format each relationship as ("relationship"{tuple_delimiter}<source_entity>{tuple_delimiter}<target_entity>{tuple_delimiter}<relationship_description>{tuple_delimiter}<relationship_strength>
```bgraph-paragraph
{"id":"f74c8a43-2008-5d4c-bb78-0edfb4cadd50","node_type":"Paragraph","location":{"semantic":{"path":"18.1.15","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":298.8387,"width":371.61288,"height":17.806452}}},"text_order":196,"token_count":47,"style":null}
```

3. Return output in English as a single list of all the entities and relationships identified in steps 1 and 2. Use **{record_delimiter}** as the list delimiter.
```bgraph-list
{"id":"454b1550-8568-56eb-ba9a-4775108d2d04","node_type":"List","location":{"semantic":{"path":"18.1.16","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":322.0645,"width":391.7419,"height":17.032257}}},"text_order":197,"token_count":40,"style":null}
```

4. When finished, output {completion_delimiter}
```bgraph-list
{"id":"44ec6df0-9174-520e-817a-6f8e29e493c6","node_type":"List","location":{"semantic":{"path":"18.1.17","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":345.2903,"width":180.38708,"height":10.064516}}},"text_order":198,"token_count":11,"style":null}
```

---Examples---
```bgraph-paragraph
{"id":"a7254f00-e45d-5580-8751-3609b7677bc6","node_type":"Paragraph","location":{"semantic":{"path":"18.1.18","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":362.32257,"width":54.96774,"height":8.516129}}},"text_order":199,"token_count":3,"style":null}
```

Entity_types: ORGANIZATION,PERSON
```bgraph-paragraph
{"id":"12f8b3e1-357c-57cc-b982-7334ab37600d","node_type":"Paragraph","location":{"semantic":{"path":"18.1.19","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":377.03226,"width":128.51613,"height":10.064516}}},"text_order":200,"token_count":8,"style":null}
```

Input:
```bgraph-paragraph
{"id":"f9a36487-1407-5d13-b7f0-6237387c3b72","node_type":"Paragraph","location":{"semantic":{"path":"18.1.20","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":394.0645,"width":24.774193,"height":8.516129}}},"text_order":201,"token_count":1,"style":null}
```

The Fed is scheduled to meet on Tuesday and Wednesday, with the central bank planning to release its latest policy decision on Wednesday at 2:00 p.m. ET, followed by a press conference where Fed Chair Jerome Powell will take questions. Investors expect the Federal Open Market Committee to hold its benchmark interest rate steady in a range of 5.25%-5.5%.
```bgraph-paragraph
{"id":"d99363ee-c880-548f-883c-b306c90abc17","node_type":"Paragraph","location":{"semantic":{"path":"18.1.21","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":409.54837,"width":374.70966,"height":33.29032}}},"text_order":202,"token_count":88,"style":null}
```

Output:
```bgraph-paragraph
{"id":"2df4d94a-3a45-562c-914f-3a2629c7ba1d","node_type":"Paragraph","location":{"semantic":{"path":"18.1.22","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":449.80643,"width":28.64516,"height":8.516129}}},"text_order":203,"token_count":1,"style":null}
```

("entity"{tuple_delimiter}FED{tuple_delimiter}ORGANIZATION{tuple_delimiter}The Fed is the Federal Reserve, which is setting interest rates on Tuesday and Wednesday)
{record_delimiter}
```bgraph-paragraph
{"id":"576cbb92-e430-5026-8fc3-57717ec26949","node_type":"Paragraph","location":{"semantic":{"path":"18.1.23","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":465.2903,"width":394.83868,"height":24.774193}}},"text_order":204,"token_count":45,"style":null}
```

("entity"{tuple_delimiter}JEROME POWELL{tuple_delimiter}PERSON{tuple_delimiter}Jerome Powell is the chair of the Federal Reserve)
{record_delimiter}
```bgraph-paragraph
{"id":"3c4a0688-2ae9-5542-8062-923f693931bb","node_type":"Paragraph","location":{"semantic":{"path":"18.1.24","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":490.0645,"width":390.96774,"height":24.0}}},"text_order":205,"token_count":37,"style":null}
```

("entity"{tuple_delimiter}FEDERAL OPEN MARKET COMMITTEE{tuple_delimiter}ORGANIZATION{tuple_delimiter}The Federal Reserve committee makes key decisions about interest rates and the growth of the United States money supply)
```bgraph-paragraph
{"id":"cee031da-30c9-539d-9610-4150f2ebaaf2","node_type":"Paragraph","location":{"semantic":{"path":"18.1.25","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":514.0645,"width":387.87094,"height":24.0}}},"text_order":206,"token_count":55,"style":null}
```

{record_delimiter}
```bgraph-paragraph
{"id":"6b37aec5-8e4c-51b9-97a2-a9e7a1aab9b4","node_type":"Paragraph","location":{"semantic":{"path":"18.1.26","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":538.0645,"width":68.90322,"height":8.516129}}},"text_order":207,"token_count":4,"style":null}
```

("relationship"{tuple_delimiter}JEROME POWELL{tuple_delimiter}FED{tuple_delimiter}Jerome Powell is the Chair of the Federal Reserve and will answer questions at a press conference{tuple_delimiter}9)
{completion_delimiter}
```bgraph-paragraph
{"id":"1f7b0234-32fa-5247-9734-86ff4a1aa637","node_type":"Paragraph","location":{"semantic":{"path":"18.1.27","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":546.5806,"width":380.12903,"height":24.0}}},"text_order":208,"token_count":55,"style":null}
```

...More examples...
```bgraph-paragraph
{"id":"b13d176d-9bde-5c37-8256-c56224b4cb8c","node_type":"Paragraph","location":{"semantic":{"path":"18.1.28","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":576.7742,"width":54.96774,"height":9.290322}}},"text_order":209,"token_count":4,"style":null}
```

---Real Data---
```bgraph-paragraph
{"id":"cb310e46-095f-5df9-adf8-9c609771f117","node_type":"Paragraph","location":{"semantic":{"path":"18.1.29","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":593.0322,"width":58.838707,"height":7.7419353}}},"text_order":210,"token_count":3,"style":null}
```

Entity_types: {entity_types}
```bgraph-paragraph
{"id":"1fb175d9-2c43-5e03-9785-10ae65bf0db9","node_type":"Paragraph","location":{"semantic":{"path":"18.1.30","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":608.5161,"width":108.38709,"height":10.064516}}},"text_order":211,"token_count":7,"style":null}
```

Input:
```bgraph-paragraph
{"id":"fe8a6325-002b-5d55-9a58-0a6cce2238a6","node_type":"Paragraph","location":{"semantic":{"path":"18.1.31","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":618.5806,"width":24.774193,"height":6.967742}}},"text_order":212,"token_count":1,"style":null}
```

{input_text}
```bgraph-paragraph
{"id":"db689a58-8c90-5d1d-a109-7ed03e0fd712","node_type":"Paragraph","location":{"semantic":{"path":"18.1.32","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":632.5161,"width":46.45161,"height":9.290322}}},"text_order":213,"token_count":3,"style":null}
```

Output:
```bgraph-paragraph
{"id":"ec1b641f-a506-5dac-a97d-32c3d83d32fc","node_type":"Paragraph","location":{"semantic":{"path":"18.1.33","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.1 Element Instance Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":648.7742,"width":28.64516,"height":8.516129}}},"text_order":214,"token_count":1,"style":null}
```

## E.2 Community Summary Generation
```bgraph-section
{"id":"71744c0b-dc24-5526-b845-927793e5372d","node_type":"Section","location":{"semantic":{"path":"18.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":668.9032,"width":173.41934,"height":12.387096}}},"text_order":215,"token_count":8,"style":null}
```

---Role---
```bgraph-paragraph
{"id":"af015533-e910-5cc1-8855-734af944a874","node_type":"Paragraph","location":{"semantic":{"path":"18.2.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":21,"bounding_box":{"x":105.29032,"y":688.25806,"width":41.032257,"height":6.193548}}},"text_order":216,"token_count":2,"style":null}
```

You are an AI assistant that helps a human analyst to perform general information discovery. Information discovery is the process of identifying and assessing relevant information associated with certain entities (e.g., organizations and individuals) within a network.
```bgraph-paragraph
{"id":"5f3de58a-fef7-5c41-b039-6679c4b2b5e2","node_type":"Paragraph","location":{"semantic":{"path":"18.2.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":21,"bounding_box":{"x":104.51613,"y":695.22577,"width":393.2903,"height":24.774193}}},"text_order":217,"token_count":67,"style":null}
```

21
```bgraph-footer
{"id":"70bf7121-b96e-5fd5-b453-73efba66b69b","node_type":"Footer","location":{"semantic":{"path":"18.2.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":21,"bounding_box":{"x":299.61288,"y":740.9032,"width":10.838709,"height":10.064516}}},"text_order":218,"token_count":1,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"6abc8b41-433e-5319-bdc4-18d3ade78330","node_type":"Paragraph","location":{"semantic":{"path":"18.2.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":83.6129,"width":40.258064,"height":6.967742}}},"text_order":219,"token_count":2,"style":null}
```

Write a comprehensive report of a community, given a list of entities that belong to the community as well as their relationships and optional associated claims. The report will be used to inform decision-makers about information associated with the community and their potential impact. The content of this report includes an overview of the community's key entities, their legal compliance, technical capabilities, reputation, and noteworthy claims.
```bgraph-paragraph
{"id":"9ca08d3d-77fe-542f-9b4c-ae26aaea996e","node_type":"Paragraph","location":{"semantic":{"path":"18.2.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":104.51613,"y":90.58064,"width":395.61288,"height":41.80645}}},"text_order":220,"token_count":112,"style":null}
```

---Report Structure---
```bgraph-paragraph
{"id":"1b6b5762-8c4f-5f4c-8f6d-ae4c9ffc74d4","node_type":"Paragraph","location":{"semantic":{"path":"18.2.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":138.58064,"width":84.38709,"height":8.516129}}},"text_order":221,"token_count":5,"style":null}
```

The report should include the following sections:
```bgraph-paragraph
{"id":"81466b04-30d6-5304-90c9-9140ca1e5264","node_type":"Paragraph","location":{"semantic":{"path":"18.2.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":154.8387,"width":183.48387,"height":8.516129}}},"text_order":222,"token_count":12,"style":null}
```

- TITLE: community's name that represents its key entities - title should be short but specific. When possible, include representative named entities in the title.
- SUMMARY: An executive summary of the community's overall structure, how its entities are related to each other, and significant information associated with its entities.
- IMPACT SEVERITY RATING: a float score between 0-10 that represents the severity of IMPACT posed by entities within the community. IMPACT is the scored importance of a community.
- RATING EXPLANATION: Give a single sentence explanation of the IMPACT severity rating.
- DETAILED FINDINGS: A list of 5-10 key insights about the community. Each insight should have a short summary followed by multiple paragraphs of explanatory text grounded according to the grounding rules below. Be comprehensive.
```bgraph-list
{"id":"7d057617-bb9f-5872-9c4a-aa8df889d7ac","node_type":"List","location":{"semantic":{"path":"18.2.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":104.51613,"y":170.32257,"width":397.1613,"height":81.29032}}},"text_order":223,"token_count":208,"style":null}
```

Return output as a well-formed JSON-formatted string with the following format:
```bgraph-paragraph
{"id":"dad91468-38cf-5983-835a-794ef4d563d0","node_type":"Paragraph","location":{"semantic":{"path":"18.2.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":258.58063,"width":294.96774,"height":8.516129}}},"text_order":224,"token_count":19,"style":null}
```

{{
    "title": <report_title>,
    "summary": <executive_summary>,
    "rating": <impact_severity_rating>,
    "rating_explanation": <rating_explanation>,
    "findings": [
    {{
    "summary":<insight_1_summary>,
    "explanation": <insight_1_explanation>
    }},
    {{
    "summary":<insight_2_summary>,
    "explanation": <insight_2_explanation>
    }
    ]
    ]
}</insight_2_explanation></insight_2_summary></insight_1_summary></insight_1_summary></insight_1_explanation></executive_summary></report_title>
```bgraph-code-block
{"id":"c08350f2-bf99-5b3f-b38e-92bdf0bf3126","node_type":"CodeBlock","location":{"semantic":{"path":"18.2.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":267.09677,"width":163.35483,"height":127.74193}}},"text_order":225,"token_count":128,"style":null}
```

---Grounding Rules---
```bgraph-paragraph
{"id":"c55b3d4d-a647-5592-abb8-0cf38b4e4ba0","node_type":"Paragraph","location":{"semantic":{"path":"18.2.11","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":401.80643,"width":80.51613,"height":8.516129}}},"text_order":226,"token_count":5,"style":null}
```

Points supported by data should list their data references as follows:
```bgraph-paragraph
{"id":"5a5c1c54-f8c8-5113-a8cd-8ffef6493e81","node_type":"Paragraph","location":{"semantic":{"path":"18.2.12","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":418.0645,"width":260.90323,"height":8.516129}}},"text_order":227,"token_count":17,"style":null}
```

"This is an example sentence supported by multiple data references [Data: <dataset name="" (record ids);="" <dataset name="" (record ids)]."</dataset></dataset></dataset>
```bgraph-paragraph
{"id":"8559d044-0a22-5876-bdc4-06c6cc95dd0a","node_type":"Paragraph","location":{"semantic":{"path":"18.2.13","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":104.51613,"y":433.54837,"width":384.77417,"height":17.032257}}},"text_order":228,"token_count":42,"style":null}
```

Do not list more than 5 record ids in a single reference. Instead, list the top 5 most relevant record ids and add "*more" to indicate that there are more.
```bgraph-paragraph
{"id":"b3563f82-e473-572a-9f8d-872b1734e57a","node_type":"Paragraph","location":{"semantic":{"path":"18.2.14","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":104.51613,"y":457.54837,"width":385.54837,"height":16.258064}}},"text_order":229,"token_count":38,"style":null}
```

For example:
```bgraph-paragraph
{"id":"3395ca38-e18c-59a3-a1c5-fbdbdaf0dd7c","node_type":"Paragraph","location":{"semantic":{"path":"18.2.15","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":482.32257,"width":46.45161,"height":7.7419353}}},"text_order":230,"token_count":3,"style":null}
```

"Person X is the owner of Company Y and subject to many allegations of wrongdoing [Data: Reports (1), Entities (5, 7); Relationships (23); Claims (7, 2, 34, 64, 46, *more)]."
```bgraph-paragraph
{"id":"6441af47-f6ed-53ca-8b2b-4849302a74e1","node_type":"Paragraph","location":{"semantic":{"path":"18.2.16","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":104.51613,"y":490.0645,"width":380.90323,"height":15.4838705}}},"text_order":231,"token_count":43,"style":null}
```

where 1, 5, 7, 23, 2, 34, 46, and 64 represent the id (not the index) of the relevant data record.
```bgraph-paragraph
{"id":"bdb80a50-ce9f-5552-9429-bba40b75edd5","node_type":"Paragraph","location":{"semantic":{"path":"18.2.17","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":104.51613,"y":513.2903,"width":365.41934,"height":9.290322}}},"text_order":232,"token_count":24,"style":null}
```

Do not include information where the supporting evidence for it is not provided.
```bgraph-paragraph
{"id":"c9b6d70a-f0e1-5ed1-9fc0-83281bb29160","node_type":"Paragraph","location":{"semantic":{"path":"18.2.18","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":104.51613,"y":529.5484,"width":298.8387,"height":8.516129}}},"text_order":233,"token_count":20,"style":null}
```

---Example---
```bgraph-paragraph
{"id":"6981e023-899e-5ee8-a74a-0f2fa92fca70","node_type":"Paragraph","location":{"semantic":{"path":"18.2.19","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":545.80646,"width":50.32258,"height":8.516129}}},"text_order":234,"token_count":3,"style":null}
```

Input:
```bgraph-paragraph
{"id":"db56d676-06ae-5b1b-9f18-b8755b537b12","node_type":"Paragraph","location":{"semantic":{"path":"18.2.20","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":562.0645,"width":24.774193,"height":7.7419353}}},"text_order":235,"token_count":1,"style":null}
```

Entities
```bgraph-paragraph
{"id":"e5181a45-e2fd-516d-9808-6684ae47af72","node_type":"Paragraph","location":{"semantic":{"path":"18.2.21","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":576.7742,"width":31.741934,"height":8.516129}}},"text_order":236,"token_count":2,"style":null}
```

id,entity,description
```bgraph-paragraph
{"id":"efb56c2f-0004-5f22-973d-d4d374bed5b8","node_type":"Paragraph","location":{"semantic":{"path":"18.2.22","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":593.80646,"width":80.51613,"height":6.967742}}},"text_order":237,"token_count":5,"style":null}
```

5,VERDANT OASIS PLAZA,Verdant Oasis Plaza is the location of the Unity March
6,HARMONY ASSEMBLY,Harmony Assembly is an organization that is holding a march at Verdant Oasis Plaza
```bgraph-paragraph
{"id":"f3339da3-5169-55af-8d9c-68c8168f447d","node_type":"Paragraph","location":{"semantic":{"path":"18.2.23","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":600.7742,"width":377.03226,"height":17.806452}}},"text_order":238,"token_count":44,"style":null}
```

Relationships
```bgraph-paragraph
{"id":"666e5773-4eed-50d4-8d26-8a56c2b46d29","node_type":"Paragraph","location":{"semantic":{"path":"18.2.24","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":625.54834,"width":50.32258,"height":7.7419353}}},"text_order":239,"token_count":3,"style":null}
```

id,source,target,description
```bgraph-paragraph
{"id":"3451a38c-824d-50c7-852f-0313c8f545e6","node_type":"Paragraph","location":{"semantic":{"path":"18.2.25","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":641.80646,"width":106.83871,"height":7.7419353}}},"text_order":240,"token_count":7,"style":null}
```

37,VERDANT OASIS PLAZA,UNITY MARCH,Verdant Oasis Plaza is the location of the Unity March
38,VERDANT OASIS PLAZA,HARMONY ASSEMBLY,Harmony Assembly is holding a march at Verdant Oasis Plaza
39,VERDANT OASIS PLAZA,UNITY MARCH,The Unity March is taking place at Verdant Oasis Plaza
40,VERDANT OASIS PLAZA,TRIBUNE SPOTLIGHT,Trishune Spotlight is reporting on the Unity march taking place at Verdant Oasis Plaza
41,VERDANT OASIS PLAZA,BAILEY ASADI,Bailey Asadi is speaking at Verdant Oasis Plaza about the march
43,HARMONY ASSEMBLY,UNITY MARCH,Harmony Assembly is organizing the Unity March
```bgraph-paragraph
{"id":"6353c934-c9fc-5fd2-b752-eaa3baab3cd9","node_type":"Paragraph","location":{"semantic":{"path":"18.2.26","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":649.54834,"width":395.61288,"height":55.741936}}},"text_order":241,"token_count":146,"style":null}
```

Output:
```bgraph-paragraph
{"id":"a107fbfd-7dad-5924-9858-2a4cbcb0acf4","node_type":"Paragraph","location":{"semantic":{"path":"18.2.27","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":105.29032,"y":713.0322,"width":27.870968,"height":8.516129}}},"text_order":242,"token_count":1,"style":null}
```

22
```bgraph-footer
{"id":"96c441e5-f704-541f-9a04-73e5fcd3efc2","node_type":"Footer","location":{"semantic":{"path":"18.2.28","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":22,"bounding_box":{"x":298.8387,"y":741.67737,"width":12.387096,"height":9.290322}}},"text_order":243,"token_count":1,"style":null}
```

{{  
'title': 'Verdant Oasis Plaza and Unity March',  
'summary': 'The community revolves around the Verdant Oasis Plaza, which is the location of the Unity  
March. The plaza has relationships with the Harmony Assembly, Unity March, and Tribune Spotlight, all of  
which are associated with the march event.',  
'rating': 5.0,  
'rating.explanation': 'The impact severity rating is moderate due to the potential for unrest or conflict  
during the Unity March.',  
'findings': [  
{{  
'summary': 'Verdant Oasis Plaza as the central location',  
'explanation': 'Verdant Oasis Plaza is the central entity in this community, serving as the location for  
the Unity March. This plaza is the common link between all other entities, suggesting its significance  
in the community. The plaza's association with the march could potentially lead to issues such as  
public disorder or conflict, depending on the nature of the march and the reactions it provokes. [Data:  
Entities (5), Relationships (37, 38, 39, 40, 41,+more)]'  
}},  
{{  
'summary': 'Harmony Assembly's role in the community',  
'explanation': 'Harmony Assembly is another key entity in this community, being the organizer of the  
march at Verdant Oasis Plaza. The nature of Harmony Assembly and its march could be a potential source of  
threat, depending on their objectives and the reactions they provoke. The relationship between Harmony  
Assembly and the plaza is crucial in understanding the dynamics of this community. [Data: Entities(6),  
Relationships (38, 43)]'  
}},  
{{  
'summary': 'Unity March as a significant event',  
'explanation': 'The Unity March is a significant event taking place at Verdant Oasis Plaza. This event  
is a key factor in the community's dynamics and could be a potential source of threat, depending on the  
nature of the march and the reactions it provokes. The relationship between the march and the plaza is  
crucial in understanding the dynamics of this community. [Data: Relationships (39)]'  
}},  
{{  
'summary': 'Role of Tribune Spotlight', 'explanation': 'Tribune Spotlight is reporting on the Unity  
March taking place in Verdant Oasis Plaza. This suggests that the event has attracted media attention,  
which could amplify its impact on the community. The role of Tribune Spotlight could be significant in  
shaping public perception of the event and the entities involved. [Data: Relationships (40)]'  
}}  
}}
```bgraph-paragraph
{"id":"cc785c0a-1036-51d5-b4f9-bc371e7e657f","node_type":"Paragraph","location":{"semantic":{"path":"18.2.29","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":73.548386,"width":394.83868,"height":318.96774}}},"text_order":244,"token_count":607,"style":null}
```

---Real Data---
```bgraph-paragraph
{"id":"7615205d-2a82-54cf-af16-befbff90756b","node_type":"Paragraph","location":{"semantic":{"path":"18.2.30","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":400.25806,"width":58.064514,"height":7.7419353}}},"text_order":245,"token_count":3,"style":null}
```

Use the following text for your answer. Do not make anything up in your answer.
```bgraph-paragraph
{"id":"d1039a02-e91e-53e9-8165-c17fffb5a12c","node_type":"Paragraph","location":{"semantic":{"path":"18.2.31","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":416.5161,"width":298.8387,"height":8.516129}}},"text_order":246,"token_count":19,"style":null}
```

Input:
```bgraph-paragraph
{"id":"74d7191b-54ce-5a2b-8c85-9602e9d63e5e","node_type":"Paragraph","location":{"semantic":{"path":"18.2.32","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":432.77417,"width":24.774193,"height":7.7419353}}},"text_order":247,"token_count":1,"style":null}
```

{input.text}
```bgraph-paragraph
{"id":"fd1f9e7b-f0c1-591b-8f12-363bb710eebd","node_type":"Paragraph","location":{"semantic":{"path":"18.2.33","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":448.25806,"width":46.45161,"height":8.516129}}},"text_order":248,"token_count":3,"style":null}
```

...Report Structure and Grounding Rules Repeated...
```bgraph-paragraph
{"id":"a2ac9ed2-9764-511f-a540-ff2988429889","node_type":"Paragraph","location":{"semantic":{"path":"18.2.34","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":463.7419,"width":148.64516,"height":8.516129}}},"text_order":249,"token_count":12,"style":null}
```

Output:
```bgraph-paragraph
{"id":"3255c0ba-81bc-5842-bd92-66a0e98ef0f4","node_type":"Paragraph","location":{"semantic":{"path":"18.2.35","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.2 Community Summary Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":480.0,"width":27.870968,"height":8.516129}}},"text_order":250,"token_count":1,"style":null}
```

## E.3 Community Answer Generation
```bgraph-section
{"id":"5d739d66-ca48-506d-9a1f-76d27360ce5c","node_type":"Section","location":{"semantic":{"path":"18.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":499.35483,"width":164.12903,"height":10.838709}}},"text_order":251,"token_count":7,"style":null}
```

---Role---
```bgraph-paragraph
{"id":"ae632471-d954-5afa-b419-a3523f8d9582","node_type":"Paragraph","location":{"semantic":{"path":"18.3.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":518.70966,"width":40.258064,"height":7.7419353}}},"text_order":252,"token_count":2,"style":null}
```

You are a helpful assistant responding to questions about a dataset by synthesizing perspectives from multiple analysts.
```bgraph-paragraph
{"id":"b3fe75d9-0bb4-511e-a2c7-fca4c8918ed0","node_type":"Paragraph","location":{"semantic":{"path":"18.3.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":534.9677,"width":377.03226,"height":16.258064}}},"text_order":253,"token_count":30,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"d07fd7a7-64ee-5140-8785-8f5cc12a577a","node_type":"Paragraph","location":{"semantic":{"path":"18.3.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":558.19354,"width":40.258064,"height":7.7419353}}},"text_order":254,"token_count":2,"style":null}
```

Generate a response of the target length and format that responds to the user's question, summarize all the reports from multiple analysts who focused on different parts of the dataset, and incorporate any relevant general knowledge.
```bgraph-paragraph
{"id":"73b429f9-87dc-51af-a4a2-ab1646b0b8a4","node_type":"Paragraph","location":{"semantic":{"path":"18.3.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":574.4516,"width":392.5161,"height":24.774193}}},"text_order":255,"token_count":58,"style":null}
```

Note that the analysts' reports provided below are ranked in the **descending order of helpfulness**.
```bgraph-paragraph
{"id":"4c8b519f-9272-5ffe-a5a3-49ae3facdfa4","node_type":"Paragraph","location":{"semantic":{"path":"18.3.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":606.19354,"width":376.25806,"height":8.516129}}},"text_order":256,"token_count":25,"style":null}
```

If you don't know the answer, just say so. Do not make anything up.
```bgraph-paragraph
{"id":"a21f1ebf-616e-5839-811e-d737fa4994d5","node_type":"Paragraph","location":{"semantic":{"path":"18.3.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":622.4516,"width":253.93547,"height":8.516129}}},"text_order":257,"token_count":16,"style":null}
```

The final response should remove all irrelevant information from the analysts' reports and merge the cleaned information into a comprehensive answer that provides explanations of all the key points and implications appropriate for the response length and format.
```bgraph-paragraph
{"id":"210609e7-9892-52ab-8034-4a795d3e633b","node_type":"Paragraph","location":{"semantic":{"path":"18.3.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":637.9355,"width":373.9355,"height":24.774193}}},"text_order":258,"token_count":65,"style":null}
```

Add sections and commentary to the response as appropriate for the length and format. Style the response in markdown.
```bgraph-paragraph
{"id":"624aad16-cf60-5ffd-bb84-3d5fb8915ada","node_type":"Paragraph","location":{"semantic":{"path":"18.3.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":669.6774,"width":392.5161,"height":16.258064}}},"text_order":259,"token_count":29,"style":null}
```

The response shall preserve the original meaning and use of modal verbs such as 'shall', 'may' or 'will'.
```bgraph-paragraph
{"id":"53a5f3a8-170a-59d2-aa97-fdd151f7c868","node_type":"Paragraph","location":{"semantic":{"path":"18.3.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":693.67737,"width":390.96774,"height":7.7419353}}},"text_order":260,"token_count":26,"style":null}
```

The response should also preserve all the data references previously included in the analysts' reports,
```bgraph-paragraph
{"id":"946992db-9933-52ff-b181-353215fcd55a","node_type":"Paragraph","location":{"semantic":{"path":"18.3.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":105.29032,"y":709.9355,"width":384.0,"height":8.516129}}},"text_order":261,"token_count":25,"style":null}
```

23
```bgraph-footer
{"id":"1510aae2-95f3-585b-a298-03d153b5434b","node_type":"Footer","location":{"semantic":{"path":"18.3.11","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":23,"bounding_box":{"x":297.2903,"y":740.129,"width":13.16129,"height":10.064516}}},"text_order":262,"token_count":1,"style":null}
```

but do not mention the roles of multiple analysts in the analysis process.
```bgraph-paragraph
{"id":"433ed439-1181-5381-84f9-02f89ce603f9","node_type":"Paragraph","location":{"semantic":{"path":"18.3.12","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":75.09677,"width":277.1613,"height":10.064516}}},"text_order":263,"token_count":18,"style":null}
```

Do not list more than 5 record ids in a single reference. Instead, list the top 5 most relevant record ids and add "+more" to indicate that there are more.
```bgraph-paragraph
{"id":"d3c26e23-b06a-59a7-8442-423ff70e9948","node_type":"Paragraph","location":{"semantic":{"path":"18.3.13","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":91.354836,"width":385.54837,"height":16.258064}}},"text_order":264,"token_count":38,"style":null}
```

For example:
```bgraph-paragraph
{"id":"837217bd-af0f-5516-b4aa-15cbdb1956e9","node_type":"Paragraph","location":{"semantic":{"path":"18.3.14","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":114.58064,"width":47.225803,"height":9.290322}}},"text_order":265,"token_count":3,"style":null}
```

"Person X is the owner of Company Y and subject to many allegations of wrongdoing [Data: Reports (2, 7, 34, 46, 64, +more)]. He is also CEO of company X [Data: Reports (1, 3)]"
```bgraph-paragraph
{"id":"564d5d81-3ff8-52e2-85f1-3c778ebc47ed","node_type":"Paragraph","location":{"semantic":{"path":"18.3.15","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":130.8387,"width":377.03226,"height":17.032257}}},"text_order":266,"token_count":44,"style":null}
```

where 1, 2, 3, 7, 34, 46, and 64 represent the id (not the index) of the relevant data record.
```bgraph-paragraph
{"id":"9f3730a1-716c-57f9-a4eb-f06d9c036c37","node_type":"Paragraph","location":{"semantic":{"path":"18.3.16","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":154.8387,"width":350.70966,"height":9.290322}}},"text_order":267,"token_count":23,"style":null}
```

Do not include information where the supporting evidence for it is not provided.
```bgraph-paragraph
{"id":"61a284b3-e5ec-56d9-911f-cb7dd595b1a1","node_type":"Paragraph","location":{"semantic":{"path":"18.3.17","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":170.32257,"width":299.61288,"height":9.290322}}},"text_order":268,"token_count":20,"style":null}
```

---Target response length and format---
```bgraph-paragraph
{"id":"de28f121-938d-5053-b240-c69c88d790ec","node_type":"Paragraph","location":{"semantic":{"path":"18.3.18","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":187.35483,"width":148.64516,"height":9.290322}}},"text_order":269,"token_count":9,"style":null}
```

{response_type}
```bgraph-paragraph
{"id":"d5428b3d-87c9-511c-b218-943cb1c09369","node_type":"Paragraph","location":{"semantic":{"path":"18.3.19","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":202.8387,"width":57.29032,"height":10.064516}}},"text_order":270,"token_count":3,"style":null}
```

---Analyst Reports---
```bgraph-paragraph
{"id":"3a3c25c1-d351-5965-a94c-dd6dc92b93e9","node_type":"Paragraph","location":{"semantic":{"path":"18.3.20","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":219.09677,"width":80.51613,"height":8.516129}}},"text_order":271,"token_count":5,"style":null}
```

{report_data}
```bgraph-paragraph
{"id":"68b11dba-a400-5b02-94ef-bd32ce86c5ce","node_type":"Paragraph","location":{"semantic":{"path":"18.3.21","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":234.58064,"width":49.548386,"height":9.290322}}},"text_order":272,"token_count":3,"style":null}
```

...Goal and Target response length and format repeated...
```bgraph-paragraph
{"id":"0dba12ab-2639-580f-9520-17c81f7c7cc0","node_type":"Paragraph","location":{"semantic":{"path":"18.3.22","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":258.58063,"width":163.35483,"height":9.290322}}},"text_order":273,"token_count":14,"style":null}
```

Add sections and commentary to the response as appropriate for the length and format. Style the response in markdown.
```bgraph-paragraph
{"id":"17220e61-cb33-56ec-9841-f16427581a22","node_type":"Paragraph","location":{"semantic":{"path":"18.3.23","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":274.0645,"width":393.2903,"height":17.806452}}},"text_order":274,"token_count":29,"style":null}
```

Output:
```bgraph-paragraph
{"id":"f2c2b191-326a-524c-84f3-d0fb9aaf8c37","node_type":"Paragraph","location":{"semantic":{"path":"18.3.24","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.3 Community Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":298.8387,"width":28.64516,"height":8.516129}}},"text_order":275,"token_count":1,"style":null}
```

## E.4 Global Answer Generation
```bgraph-section
{"id":"829562f3-dc58-51d7-9550-0715fad2f20d","node_type":"Section","location":{"semantic":{"path":"18.4","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":318.96774,"width":142.45161,"height":10.838709}}},"text_order":276,"token_count":7,"style":null}
```

---Role---
```bgraph-paragraph
{"id":"5e555ae1-ed5e-5823-ba36-8e01144d047b","node_type":"Paragraph","location":{"semantic":{"path":"18.4.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":336.77417,"width":39.48387,"height":7.7419353}}},"text_order":277,"token_count":2,"style":null}
```

You are a helpful assistant responding to questions about data in the tables provided.
```bgraph-paragraph
{"id":"cf5f0f76-2c80-594e-9a8e-7334773d2f2f","node_type":"Paragraph","location":{"semantic":{"path":"18.4.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":353.03226,"width":321.2903,"height":9.290322}}},"text_order":278,"token_count":21,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"07b7d075-9875-587b-a386-def0d19a3cff","node_type":"Paragraph","location":{"semantic":{"path":"18.4.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":368.5161,"width":39.48387,"height":7.7419353}}},"text_order":279,"token_count":2,"style":null}
```

Generate a response of the target length and format that responds to the user's question, summarize all relevant information in the input data tables appropriate for the response length and format, and incorporate any relevant general knowledge.
```bgraph-paragraph
{"id":"dce8facf-a141-5306-b2cc-a522ecc3da1e","node_type":"Paragraph","location":{"semantic":{"path":"18.4.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":384.77417,"width":378.58063,"height":24.0}}},"text_order":280,"token_count":61,"style":null}
```

If you don't know the answer, just say so. Do not make anything up.
```bgraph-paragraph
{"id":"449388e5-659e-5979-b962-ab14eac7f628","node_type":"Paragraph","location":{"semantic":{"path":"18.4.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":415.7419,"width":254.70967,"height":9.290322}}},"text_order":281,"token_count":16,"style":null}
```

The response shall preserve the original meaning and use of modal verbs such as "shall", "may" or "will".
```bgraph-paragraph
{"id":"8fa19f4e-d893-5c56-9678-c49340083278","node_type":"Paragraph","location":{"semantic":{"path":"18.4.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":431.2258,"width":391.7419,"height":9.290322}}},"text_order":282,"token_count":26,"style":null}
```

Points supported by data should list the relevant reports as references as follows:
```bgraph-paragraph
{"id":"e1c64a24-3534-5208-aad3-cb00bb67d492","node_type":"Paragraph","location":{"semantic":{"path":"18.4.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":447.48386,"width":310.4516,"height":9.290322}}},"text_order":283,"token_count":20,"style":null}
```

"This is an example sentence supported by data references [Data: Reports (report ids)]"
```bgraph-paragraph
{"id":"e3b91580-5217-566e-895e-4da073e41677","node_type":"Paragraph","location":{"semantic":{"path":"18.4.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":462.96774,"width":329.03226,"height":9.290322}}},"text_order":284,"token_count":21,"style":null}
```

Note: the prompts for SS (semantic search) and TS (text summarization) conditions use "Sources" in place of "Reports" above.
```bgraph-paragraph
{"id":"45c5732d-bd5b-5f4f-addb-3638e0761147","node_type":"Paragraph","location":{"semantic":{"path":"18.4.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":480.0,"width":360.77417,"height":9.290322}}},"text_order":285,"token_count":31,"style":null}
```

Do not list more than 5 record ids in a single reference. Instead, list the top 5 most relevant record ids and add "+more" to indicate that there are more.
```bgraph-paragraph
{"id":"cab01f9a-4c82-53c0-bd8a-90b769cfcb6b","node_type":"Paragraph","location":{"semantic":{"path":"18.4.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":495.48386,"width":385.54837,"height":17.032257}}},"text_order":286,"token_count":38,"style":null}
```

For example:
```bgraph-paragraph
{"id":"d1fcbb17-a238-58e2-aa38-6666aeedc1e4","node_type":"Paragraph","location":{"semantic":{"path":"18.4.11","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":519.4838,"width":46.45161,"height":8.516129}}},"text_order":287,"token_count":3,"style":null}
```

"Person X is the owner of Company Y and subject to many allegations of wrongdoing [Data: Reports (2, 7, 64, 46, 34, +more)]. He is also CEO of company X [Data: Reports (1, 3)]"
```bgraph-paragraph
{"id":"35705c21-a11a-5509-9278-05c02aa5bef2","node_type":"Paragraph","location":{"semantic":{"path":"18.4.12","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":534.9677,"width":377.03226,"height":17.806452}}},"text_order":288,"token_count":44,"style":null}
```

where 1, 2, 3, 7, 34, 46, and 64 represent the id (not the index) of the relevant data report in the provided tables.
```bgraph-paragraph
{"id":"179965d8-85fc-5aa4-a22e-00d502f7b834","node_type":"Paragraph","location":{"semantic":{"path":"18.4.13","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":558.9677,"width":374.70966,"height":17.032257}}},"text_order":289,"token_count":29,"style":null}
```

Do not include information where the supporting evidence for it is not provided.
```bgraph-paragraph
{"id":"6fbd1563-e8fd-55b5-b0b0-49958dd61b47","node_type":"Paragraph","location":{"semantic":{"path":"18.4.14","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":582.9677,"width":299.61288,"height":9.290322}}},"text_order":290,"token_count":20,"style":null}
```

At the beginning of your response, generate an integer score between 0-100 that indicates how **helpful** is this response in answering the user's question. Return the score in this format: <ANSWER_HELPFULNESS> score.value </ANSWER_HELPFULNESS>.
```bgraph-paragraph
{"id":"84d3da41-841c-59ce-8d91-93ecfb03fb37","node_type":"Paragraph","location":{"semantic":{"path":"18.4.15","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":104.51613,"y":599.22577,"width":394.83868,"height":24.774193}}},"text_order":291,"token_count":61,"style":null}
```

---Target response length and format---
```bgraph-paragraph
{"id":"ef88fc62-7285-5f32-be36-c8cc7986a79c","node_type":"Paragraph","location":{"semantic":{"path":"18.4.16","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":630.9677,"width":148.64516,"height":9.290322}}},"text_order":292,"token_count":9,"style":null}
```

{response_type}
```bgraph-paragraph
{"id":"f051fadd-7134-5301-bacc-f9d83b406602","node_type":"Paragraph","location":{"semantic":{"path":"18.4.17","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":647.22577,"width":57.29032,"height":10.064516}}},"text_order":293,"token_count":3,"style":null}
```

---Data tables---
```bgraph-paragraph
{"id":"573cbe12-954c-51f3-9f1c-245a895d59ef","node_type":"Paragraph","location":{"semantic":{"path":"18.4.18","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":662.70966,"width":65.80645,"height":8.516129}}},"text_order":294,"token_count":4,"style":null}
```

{context_data}
```bgraph-paragraph
{"id":"40680b2b-cf5f-5beb-8d69-bd72576c2c15","node_type":"Paragraph","location":{"semantic":{"path":"18.4.19","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":106.064514,"y":678.9677,"width":54.193546,"height":10.064516}}},"text_order":295,"token_count":3,"style":null}
```

...Goal and Target response length and format repeated...
```bgraph-paragraph
{"id":"221af3ce-25d9-569f-9034-0c8e8543853b","node_type":"Paragraph","location":{"semantic":{"path":"18.4.20","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":694.4516,"width":163.35483,"height":9.290322}}},"text_order":296,"token_count":14,"style":null}
```

Output:
```bgraph-paragraph
{"id":"dc98252b-86a9-5921-a0d6-4b0dd1308e86","node_type":"Paragraph","location":{"semantic":{"path":"18.4.21","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":105.29032,"y":710.70966,"width":24.774193,"height":9.290322}}},"text_order":297,"token_count":1,"style":null}
```

24
```bgraph-footer
{"id":"203c4f0d-4682-501d-a3f7-5ff02e013251","node_type":"Footer","location":{"semantic":{"path":"18.4.22","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","E System Prompts","E.4 Global Answer Generation"]},"physical":{"page":24,"bounding_box":{"x":299.61288,"y":740.9032,"width":11.612903,"height":10.064516}}},"text_order":298,"token_count":1,"style":null}
```

# F Evaluation Prompts
```bgraph-section
{"id":"e497e9f9-b53f-5904-b6b3-56dfcf6cf717","node_type":"Section","location":{"semantic":{"path":"19","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":71.22581,"width":125.41935,"height":14.709677}}},"text_order":299,"token_count":5,"style":null}
```

## F.1 Relative Assessment Prompt
```bgraph-section
{"id":"471cdc15-23f6-59b2-86d3-aad71d7db8b1","node_type":"Section","location":{"semantic":{"path":"19.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":93.677414,"width":146.32257,"height":12.387096}}},"text_order":300,"token_count":7,"style":null}
```

---Role---
```bgraph-paragraph
{"id":"fbc4e2bf-644b-5bad-a7ac-6daf935c9f00","node_type":"Paragraph","location":{"semantic":{"path":"19.1.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":112.258064,"width":41.032257,"height":8.516129}}},"text_order":301,"token_count":2,"style":null}
```

You are a helpful assistant responsible for grading two answers to a question that are provided by two different people.
```bgraph-paragraph
{"id":"f8a7f92c-ec4a-54f2-b6e7-f30d37a094af","node_type":"Paragraph","location":{"semantic":{"path":"19.1.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":127.74193,"width":381.6774,"height":17.806452}}},"text_order":302,"token_count":30,"style":null}
```

---Goal---
```bgraph-paragraph
{"id":"d1cf958e-aa9a-5c1b-9570-140a99afc8d0","node_type":"Paragraph","location":{"semantic":{"path":"19.1.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":152.51613,"width":41.032257,"height":7.7419353}}},"text_order":303,"token_count":2,"style":null}
```

Given a question and two answers (Answer 1 and Answer 2), assess which answer is better according to the following measure:
```bgraph-paragraph
{"id":"6ee84d64-56de-5f75-a728-361ac8eeba66","node_type":"Paragraph","location":{"semantic":{"path":"19.1.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":168.0,"width":373.9355,"height":17.032257}}},"text_order":304,"token_count":30,"style":null}
```

{criteria}
```bgraph-paragraph
{"id":"d2ba0134-e760-55d2-b317-aac61f622bdd","node_type":"Paragraph","location":{"semantic":{"path":"19.1.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":192.0,"width":41.032257,"height":9.290322}}},"text_order":305,"token_count":2,"style":null}
```

Your assessment should include two parts:
```bgraph-paragraph
{"id":"e5a4cb3f-4587-526d-b24a-fc4711592764","node_type":"Paragraph","location":{"semantic":{"path":"19.1.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":207.48387,"width":154.8387,"height":8.516129}}},"text_order":306,"token_count":10,"style":null}
```

- Winner: either 1 (if Answer 1 is better) and 2 (if Answer 2 is better) or 0 if they are fundamentally similar and the differences are immaterial.
- Reasoning: a short explanation of why you chose the winner with respect to the measure described above.
```bgraph-list
{"id":"fb9c5d53-7a5e-5ad6-ad0c-396d9a03792e","node_type":"List","location":{"semantic":{"path":"19.1.7","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":216.77419,"width":394.83868,"height":24.0}}},"text_order":307,"token_count":63,"style":null}
```

Format your response as a JSON object with the following structure:
```bgraph-paragraph
{"id":"26f1e936-8d99-5f5c-a701-a0a8f24c84d9","node_type":"Paragraph","location":{"semantic":{"path":"19.1.8","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":248.51613,"width":250.8387,"height":8.516129}}},"text_order":308,"token_count":16,"style":null}
```

{{
"winner": <1, 2, or 0>,
"reasoning": "Answer 1 is better because <your reasoning="">."
}}
```bgraph-code-block
{"id":"2c43bd70-eac9-5a01-9e0b-a83a7b2867e4","node_type":"CodeBlock","location":{"semantic":{"path":"19.1.9","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":257.03226,"width":226.06451,"height":32.51613}}},"text_order":309,"token_count":23,"style":null}
```

---Question---
```bgraph-paragraph
{"id":"1446d321-5a75-5a75-8a7e-641393501e79","node_type":"Paragraph","location":{"semantic":{"path":"19.1.10","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":296.5161,"width":54.96774,"height":7.7419353}}},"text_order":310,"token_count":3,"style":null}
```

{question}
```bgraph-paragraph
{"id":"0a9ad7cc-0e3d-54f6-8a78-a8364812eb9d","node_type":"Paragraph","location":{"semantic":{"path":"19.1.11","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":311.2258,"width":41.032257,"height":10.064516}}},"text_order":311,"token_count":2,"style":null}
```

---Answer 1---
```bgraph-paragraph
{"id":"db666681-871a-5017-b229-3ab339f7204f","node_type":"Paragraph","location":{"semantic":{"path":"19.1.12","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":328.25806,"width":54.96774,"height":7.7419353}}},"text_order":312,"token_count":3,"style":null}
```

{answer1}
```bgraph-paragraph
{"id":"0d1cff43-d889-5f29-9fc5-8cd2a2b475d5","node_type":"Paragraph","location":{"semantic":{"path":"19.1.13","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":342.96774,"width":36.387096,"height":10.064516}}},"text_order":313,"token_count":2,"style":null}
```

---Answer 2---
```bgraph-paragraph
{"id":"302dffd3-e391-56f7-bb39-02070bb61d02","node_type":"Paragraph","location":{"semantic":{"path":"19.1.14","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":360.0,"width":54.96774,"height":7.7419353}}},"text_order":314,"token_count":3,"style":null}
```

{answer2}
```bgraph-paragraph
{"id":"31834e30-4253-521c-8169-38a668af11f9","node_type":"Paragraph","location":{"semantic":{"path":"19.1.15","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":374.70966,"width":36.387096,"height":10.064516}}},"text_order":315,"token_count":2,"style":null}
```

Assess which answer is better according to the following measure:
```bgraph-paragraph
{"id":"c31e1b73-fa74-5490-9688-bcba60a7e334","node_type":"Paragraph","location":{"semantic":{"path":"19.1.16","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":391.7419,"width":243.87096,"height":8.516129}}},"text_order":316,"token_count":16,"style":null}
```

{criteria}
```bgraph-paragraph
{"id":"7c0e300a-a5b2-57fc-bfcf-d811e51c919b","node_type":"Paragraph","location":{"semantic":{"path":"19.1.17","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":407.2258,"width":41.032257,"height":10.064516}}},"text_order":317,"token_count":2,"style":null}
```

Output:
```bgraph-paragraph
{"id":"b7399a44-9e54-55df-8957-ff445e13e588","node_type":"Paragraph","location":{"semantic":{"path":"19.1.18","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.1 Relative Assessment Prompt"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":423.48386,"width":27.870968,"height":7.7419353}}},"text_order":318,"token_count":1,"style":null}
```

## F.2 Relative Assessment Metrics
```bgraph-section
{"id":"68b3a295-213f-5633-b505-f782bfc046f4","node_type":"Section","location":{"semantic":{"path":"19.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":442.83868,"width":146.32257,"height":10.838709}}},"text_order":319,"token_count":7,"style":null}
```

CRITERIA = {
```bgraph-paragraph
{"id":"8e9a6fdf-29cb-5710-818e-266f8cd25887","node_type":"Paragraph","location":{"semantic":{"path":"19.2.1","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":105.29032,"y":460.64514,"width":48.0,"height":8.516129}}},"text_order":320,"token_count":3,"style":null}
```

"comprehensiveness": "How much detail does the answer provide to cover all the aspects and details of the question? A comprehensive answer should be thorough and complete, without being redundant or irrelevant. For example, if the question is 'What are the benefits and drawbacks of nuclear energy?', a comprehensive answer would provide both the positive and negative aspects of nuclear energy, such as its efficiency, environmental impact, safety, cost, etc. A comprehensive answer should not leave out any important points or provide irrelevant information. For example, an incomplete answer would only provide the benefits of nuclear energy without describing the drawbacks, or a redundant answer would repeat the same information multiple times.",
```bgraph-paragraph
{"id":"a911faeb-00f2-52ac-95b9-e9081e0b4fb6","node_type":"Paragraph","location":{"semantic":{"path":"19.2.2","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":469.1613,"width":397.1613,"height":64.258064}}},"text_order":321,"token_count":188,"style":null}
```

"diversity": "How varied and rich is the answer in providing different perspectives and insights on the question? A diverse answer should be multi-faceted and multi-dimensional, offering different viewpoints and angles on the question. For example, if the question is 'What are the causes and effects of climate change?', a diverse answer would provide different causes and effects of climate change, such as greenhouse gas emissions, deforestation, natural disasters, biodiversity loss, etc. A diverse answer should also provide different sources and evidence to support the answer. For example, a single-source answer would only cite one source or evidence, or a biased answer would only provide one perspective or opinion.",
```bgraph-paragraph
{"id":"57a28449-0748-5d4d-9876-2d669e569deb","node_type":"Paragraph","location":{"semantic":{"path":"19.2.3","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":533.4193,"width":397.1613,"height":63.483868}}},"text_order":322,"token_count":181,"style":null}
```

"directness": "How specifically and clearly does the answer address the question? A direct answer should provide a clear and concise answer to the question. For example, if the question is 'What is the capital of France?', a direct answer would be 'Paris'. A direct answer should not provide any irrelevant or unnecessary information that does not answer the question. For example, an indirect answer would be 'The capital of France is located on the river Seine'."
```bgraph-paragraph
{"id":"dfa370e3-8e5c-5ffe-9141-34adc2c70467","node_type":"Paragraph","location":{"semantic":{"path":"19.2.4","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":596.9032,"width":397.1613,"height":40.258064}}},"text_order":323,"token_count":116,"style":null}
```

"empowerment": "How well does the answer help the reader understand and make informed judgements about the topic without being misled or making fallacious assumptions. Evaluate each answer on the quality of answer as it relates to clearly explaining and providing reasoning and sources behind the claims in the answer."
}
```bgraph-paragraph
{"id":"b4e1be29-474f-54fa-95c1-8c95ddead535","node_type":"Paragraph","location":{"semantic":{"path":"19.2.5","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":104.51613,"y":637.16125,"width":389.41934,"height":41.032257}}},"text_order":324,"token_count":80,"style":null}
```

25
```bgraph-footer
{"id":"68a233b0-84e7-53bd-89e3-ffe33bf598f6","node_type":"Footer","location":{"semantic":{"path":"19.2.6","depth":3,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","F Evaluation Prompts","F.2 Relative Assessment Metrics"]},"physical":{"page":25,"bounding_box":{"x":299.61288,"y":741.67737,"width":10.838709,"height":9.290322}}},"text_order":325,"token_count":1,"style":null}
```

# G Statistical Analysis
```bgraph-section
{"id":"cf2401e0-a2e8-5436-9e0c-da13f18ed7fc","node_type":"Section","location":{"semantic":{"path":"20","depth":1,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":105.29032,"y":71.22581,"width":121.548386,"height":14.709677}}},"text_order":326,"token_count":5,"style":null}
```

Table 6: Pairwise comparisons of six conditions on four metrics across 125 questions and two datasets. For each question and metric, the winning condition received a score of 100, the losing condition received a score of 0, and in the event of a tie, each condition was scored 50. These scores were then averaged over five evaluation runs for each condition. Results of Shapiro-Wilk tests indicated that the data did not follow a normal distribution. Thus, non-parametric tests (Wilcoxon signed-rank tests) were employed to assess the performance differences between pairs of conditions, with Holm-Bonferroni correction applied to account for multiple pairwise comparisons. The corrected p-values that indicated statistically significant differences are highlighted in bold.
```bgraph-paragraph
{"id":"77f4fe83-ed2b-55a6-9615-43d867a73f3f","node_type":"Paragraph","location":{"semantic":{"path":"20.1","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":104.51613,"y":98.32258,"width":400.25806,"height":89.80645}}},"text_order":327,"token_count":193,"style":null}
```

|   | Podcast Transcripts |   |   |   |   | News Articles  |   |   |   |   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  Condition 1 | Condition 2 | Mean 1 | Mean 2 | Z-value | p-value | Mean 1 | Mean 2 | Z-value | p-value  |
|  Comprehensiveness | C0 | TS | 50.24 | 49.76 | -0.06 | 1 | 55.52 | 44.48 | -2.03 | 0.17  |
|   |  C1 | TS | 51.92 | 48.08 | -1.56 | 0.633 | 58.8 | 41.2 | -3.62 | 0.002  |
|   |  C2 | TS | 57.28 | 42.72 | -4.1 | <0.001 | 62.08 | 37.92 | -5.07 | <0.001  |
|   |  C3 | TS | 56.48 | 43.52 | -3.42 | 0.006 | 63.6 | 36.4 | -5.63 | <0.001  |
|   |  C0 | SS | 71.92 | 28.08 | -6.2 | <0.001 | 71.76 | 28.24 | -6.3 | <0.001  |
|   |  C1 | SS | 75.44 | 24.56 | -7.45 | <0.001 | 74.72 | 25.28 | -7.78 | <0.001  |
|   |  C2 | SS | 77.76 | 22.24 | -8.17 | <0.001 | 79.2 | 20.8 | -8.34 | <0.001  |
|   |  C3 | SS | 78.96 | 21.04 | -8.12 | <0.001 | 79.44 | 20.56 | -8.44 | <0.001  |
|   |  TS | SS | 83.12 | 16.88 | -8.85 | <0.001 | 79.6 | 20.4 | -8.27 | <0.001  |
|   |  C0 | C1 | 53.2 | 46.8 | -1.96 | 0.389 | 51.92 | 48.08 | -0.45 | 0.777  |
|   |  C0 | C2 | 50.24 | 49.76 | -0.23 | 1 | 53.68 | 46.32 | -1.54 | 0.371  |
|   |  C1 | C2 | 51.52 | 48.48 | -1.62 | 0.633 | 57.76 | 42.24 | -4.01 | <0.001  |
|   |  C0 | C3 | 49.12 | 50.88 | -0.56 | 1 | 52.16 | 47.84 | -0.86 | 0.777  |
|   |  C1 | C3 | 50.32 | 49.68 | -0.66 | 1 | 55.12 | 44.88 | -2.94 | 0.016  |
|   |  C2 | C3 | 52.24 | 47.76 | -1.97 | 0.389 | 58.64 | 41.36 | -3.68 | 0.002  |
|  Diversity | C0 | TS | 50.24 | 49.76 | -0.11 | 1 | 46.88 | 53.12 | -1.38 | 0.676  |
|   |  C1 | TS | 50.48 | 49.52 | -0.12 | 1 | 54.64 | 45.36 | -1.88 | 0.298  |
|   |  C2 | TS | 57.12 | 42.88 | -2.84 | 0.036 | 55.76 | 44.24 | -2.16 | 0.184  |
|   |  C3 | TS | 54.32 | 45.68 | -2.39 | 0.1 | 60.16 | 39.84 | -4.07 | <0.001  |
|   |  C0 | SS | 76.56 | 23.44 | -7.12 | <0.001 | 62.08 | 37.92 | -3.57 | 0.003  |
|   |  C1 | SS | 75.44 | 24.56 | -7.33 | <0.001 | 64.96 | 35.04 | -4.92 | <0.001  |
|   |  C2 | SS | 80.56 | 19.44 | -8.21 | <0.001 | 70.56 | 29.44 | -6.29 | <0.001  |
|   |  C3 | SS | 80.8 | 19.2 | -8.3 | <0.001 | 69.12 | 30.88 | -5.53 | <0.001  |
|   |  TS | SS | 82.08 | 17.92 | -8.43 | <0.001 | 67.2 | 32.8 | -4.85 | <0.001  |
|   |  C0 | C1 | 49.76 | 50.24 | -0.13 | 1 | 39.68 | 60.32 | -3.61 | 0.003  |
|   |  C0 | C2 | 46.32 | 53.68 | -1.5 | 0.669 | 40.96 | 59.04 | -3.14 | 0.012  |
|   |  C1 | C2 | 44.08 | 55.92 | -3.27 | 0.011 | 50.24 | 49.76 | -0.22 | 1  |
|   |  C0 | C3 | 44 | 56 | -2.6 | 0.065 | 41.04 | 58.96 | -3.47 | 0.004  |
|   |  C1 | C3 | 45.44 | 54.56 | -2.98 | 0.026 | 49.52 | 50.48 | -0.01 | 1  |
|   |  C2 | C3 | 48.48 | 51.52 | -0.96 | 1 | 50.96 | 49.04 | -0.39 | 1  |
|  Empowerment | C0 | TS | 40.96 | 59.04 | -4.3 | <0.001 | 42.24 | 57.76 | -3.32 | 0.012  |
|   |  C1 | TS | 45.2 | 54.8 | -3.76 | 0.002 | 50 | 50 | -0.12 | 1  |
|   |  C2 | TS | 47.68 | 52.32 | -2.2 | 0.281 | 49.52 | 50.48 | -0.22 | 1  |
|   |  C3 | TS | 48.72 | 51.28 | -1.27 | 1 | 51.68 | 48.32 | -1.2 | 1  |
|   |  C0 | SS | 42.96 | 57.04 | -3.71 | 0.003 | 42.72 | 57.28 | -3.12 | 0.022  |
|   |  C1 | SS | 47.68 | 52.32 | -1.5 | 0.936 | 51.36 | 48.64 | -0.84 | 1  |
|   |  C2 | SS | 50.72 | 49.28 | -0.55 | 1 | 49.84 | 50.16 | -0.2 | 1  |
|   |  C3 | SS | 48.96 | 51.04 | -0.57 | 1 | 49.52 | 50.48 | -0.08 | 1  |
|   |  TS | SS | 57.52 | 42.48 | -4.1 | <0.001 | 52.88 | 47.12 | -1.1 | 1  |
|   |  C0 | C1 | 48.72 | 51.28 | -1.23 | 1 | 42.4 | 57.6 | -3.9 | 0.001  |
|   |  C0 | C2 | 46.64 | 53.36 | -2.54 | 0.12 | 44.8 | 55.2 | -2.16 | 0.336  |
|   |  C1 | C2 | 49.28 | 50.72 | -1.73 | 0.682 | 52 | 48 | -1.45 | 1  |
|   |  C0 | C3 | 47.6 | 52.4 | -1.78 | 0.682 | 44.32 | 55.68 | -3.45 | 0.008  |
|   |  C1 | C3 | 50 | 50 | 0 | 1 | 51.44 | 48.56 | -1.02 | 1  |
|   |  C2 | C3 | 50.72 | 49.28 | -0.86 | 1 | 50.4 | 49.6 | -0.22 | 1  |
|  Directness | C0 | TS | 44.96 | 55.04 | -4.09 | <0.001 | 45.2 | 54.8 | -3.68 | 0.003  |
|   |  C1 | TS | 47.92 | 52.08 | -2.41 | 0.126 | 46.64 | 53.36 | -2.91 | 0.04  |
|   |  C2 | TS | 48.8 | 51.2 | -2.23 | 0.179 | 48.32 | 51.68 | -2.12 | 0.179  |
|   |  C3 | TS | 48.08 | 51.92 | -2.23 | 0.179 | 48.32 | 51.68 | -2.56 | 0.074  |
|   |  C0 | SS | 35.12 | 64.88 | -6.17 | <0.001 | 41.44 | 58.56 | -4.82 | <0.001  |
|   |  C1 | SS | 40.32 | 59.68 | -4.83 | <0.001 | 45.2 | 54.8 | -3.19 | 0.017  |
|   |  C2 | SS | 40.4 | 59.6 | -4.67 | <0.001 | 44.88 | 55.12 | -3.65 | 0.003  |
|   |  C3 | SS | 40.48 | 59.52 | -4.69 | <0.001 | 45.6 | 54.4 | -2.86 | 0.043  |
|   |  TS | SS | 43.6 | 56.4 | -3.96 | <0.001 | 46 | 54 | -2.68 | 0.066  |
|   |  C0 | C1 | 46.96 | 53.04 | -2.87 | 0.037 | 47.6 | 52.4 | -2.17 | 0.179  |
|   |  C0 | C2 | 48.4 | 51.6 | -2.06 | 0.197 | 48.48 | 51.52 | -1.61 | 0.321  |
|   |  C1 | C2 | 49.84 | 50.16 | -1 | 0.952 | 49.28 | 50.72 | -1.6 | 0.321  |
|   |  C0 | C3 | 48.4 | 51.6 | -1.8 | 0.29 | 47.2 | 52.8 | -2.62 | 0.071  |
|   |  C1 | C3 | 49.76 | 50.24 | 0 | 1 | 48.8 | 51.2 | -1.29 | 0.321  |
|   |  C2 | C3 | 50 | 50 | 0 | 1 | 48.8 | 51.2 | -1.84 | 0.262  |
```bgraph-table
{"id":"f5da26b8-0c0f-53ba-a606-fde34a0351fe","node_type":"Table","location":{"semantic":{"path":"20.2","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":106.064514,"y":193.54839,"width":401.80643,"height":436.64514}}},"text_order":328,"token_count":1253,"style":null}
```

26
```bgraph-footer
{"id":"13426a9c-084f-57e3-9b8f-60a46a3b80c4","node_type":"Footer","location":{"semantic":{"path":"20.3","depth":2,"breadcrumbs":["From Local to Global: A GraphRAG Approach to Query-Focused Summarization","G Statistical Analysis"]},"physical":{"page":26,"bounding_box":{"x":300.3871,"y":741.67737,"width":10.838709,"height":9.290322}}},"text_order":329,"token_count":1,"style":null}
```
