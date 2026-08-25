```bgraph
{"schema":"1.1.0","kind":"document","bragi_version":"0.5.0","source":{"format":"ocr","sha256":"818fa0c521fec6d424fae41f412ecfdaa1c486d5a9db24ae87db0091b0b0b94e"},"flow_type":"Fixed","config_hash":"none","bgraph_sha256":"3576d88930372a5363acd378b786eae918891df246aac05810d48873bd54c195"}
```

```bgraph-metadata
{"title":null,"author":null,"description":null,"language":null,"created":null,"ocr":{"model":"mistral-ocr-4-0","pages_processed":15,"doc_size_bytes":2215244,"dpi":93,"extras":{}}}
```

```bgraph-outline
{"sections":[{"title":"Attention Is All You Need","order":0,"level":2},{"title":"Abstract","order":1,"level":3},{"title":"1 Introduction","order":2,"level":1},{"title":"2 Background","order":3,"level":1},{"title":"3 Model Architecture","order":4,"level":1},{"title":"3.1 Encoder and Decoder Stacks","order":5,"level":3},{"title":"3.2 Attention","order":6,"level":3},{"title":"3.2.1 Scaled Dot-Product Attention","order":7,"level":3},{"title":"3.2.2 Multi-Head Attention","order":8,"level":3},{"title":"3.2.3 Applications of Attention in our Model","order":9,"level":3},{"title":"3.3 Position-wise Feed-Forward Networks","order":10,"level":3},{"title":"3.4 Embeddings and Softmax","order":11,"level":3},{"title":"3.5 Positional Encoding","order":12,"level":3},{"title":"4 Why Self-Attention","order":13,"level":2},{"title":"5 Training","order":14,"level":2},{"title":"5.1 Training Data and Batching","order":15,"level":3},{"title":"5.2 Hardware and Schedule","order":16,"level":3},{"title":"5.3 Optimizer","order":17,"level":3},{"title":"5.4 Regularization","order":18,"level":3},{"title":"6 Results","order":19,"level":2},{"title":"6.1 Machine Translation","order":20,"level":3},{"title":"6.2 Model Variations","order":21,"level":3},{"title":"6.3 English Constituency Parsing","order":22,"level":3},{"title":"7 Conclusion","order":23,"level":2},{"title":"References","order":24,"level":2},{"title":"Attention Visualizations","order":25,"level":2}]}
```

arXiv:1706.03762v7 [cs.CL] 2 Aug 2023
```bgraph-header
{"id":"cdb4ab2d-ac4d-5a6d-a195-d3def98b259a","node_type":"Header","location":{"semantic":{"path":"1","depth":1,"breadcrumbs":[]},"physical":{"page":1,"bounding_box":{"x":12.387096,"y":209.80644,"width":21.677418,"height":344.5161}}},"text_order":0,"token_count":9,"style":null}
```

Provided proper attribution is provided, Google hereby grants permission to reproduce the tables and figures in this paper solely for use in journalistic or scholarly works.
```bgraph-paragraph
{"id":"e1c5df07-a7b3-5a48-a588-e8e3f04f7f12","node_type":"Paragraph","location":{"semantic":{"path":"2","depth":1,"breadcrumbs":[]},"physical":{"page":1,"bounding_box":{"x":120.77419,"y":68.90322,"width":366.96774,"height":41.032257}}},"text_order":1,"token_count":43,"style":null}
```

## Attention Is All You Need
```bgraph-section
{"id":"7962788f-d2e7-50bc-8359-c47c4b37c03d","node_type":"Section","location":{"semantic":{"path":"3","depth":2,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":208.25806,"y":144.77419,"width":191.2258,"height":17.032257}}},"text_order":2,"token_count":6,"style":null}
```

**Ashish Vaswani***
Google Brain
avaswani@google.com
```bgraph-paragraph
{"id":"ee38678d-a9a5-5625-90b8-5602c44e0981","node_type":"Paragraph","location":{"semantic":{"path":"3.1","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":113.80645,"y":229.93547,"width":102.967735,"height":34.838707}}},"text_order":3,"token_count":13,"style":null}
```

**Noam Shazeer***
Google Brain
noam@google.com
```bgraph-paragraph
{"id":"c359bd88-46f5-527b-9347-ca61a63695c8","node_type":"Paragraph","location":{"semantic":{"path":"3.2","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":227.6129,"y":229.93547,"width":82.064514,"height":34.838707}}},"text_order":4,"token_count":11,"style":null}
```

**Niki Parmar***
Google Research
nikip@google.com
```bgraph-paragraph
{"id":"37d225ff-4508-5407-acec-66580c8f6977","node_type":"Paragraph","location":{"semantic":{"path":"3.3","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":319.7419,"y":229.93547,"width":88.258064,"height":34.838707}}},"text_order":5,"token_count":12,"style":null}
```

**Jakob Uszkoreit***
Google Research
usz@google.com
```bgraph-paragraph
{"id":"bbe992f0-1cbc-536a-959e-f96e07700690","node_type":"Paragraph","location":{"semantic":{"path":"3.4","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":418.83868,"y":229.93547,"width":77.41935,"height":34.838707}}},"text_order":6,"token_count":12,"style":null}
```

**Llion Jones***
Google Research
llion@google.com
```bgraph-paragraph
{"id":"804e1606-47bd-5fca-9a0a-69a50aaf965f","node_type":"Paragraph","location":{"semantic":{"path":"3.5","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":123.870964,"y":279.48386,"width":87.48387,"height":34.838707}}},"text_order":7,"token_count":12,"style":null}
```

**Aidan N. Gomez*** †
University of Toronto
aidan@cs.toronto.edu
```bgraph-paragraph
{"id":"5d13743e-3470-565f-9072-cc03a90d2b64","node_type":"Paragraph","location":{"semantic":{"path":"3.6","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":232.25806,"y":279.48386,"width":107.6129,"height":34.838707}}},"text_order":8,"token_count":16,"style":null}
```

**Łukasz Kaiser***
Google Brain
lukaszkaiser@google.com
```bgraph-paragraph
{"id":"658e54ea-aac7-591b-820b-58bd0c9d13a2","node_type":"Paragraph","location":{"semantic":{"path":"3.7","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":361.54837,"y":279.48386,"width":123.09677,"height":34.838707}}},"text_order":9,"token_count":14,"style":null}
```

**Illia Polosukhin*** ‡
illia.polosukhin@gmail.com
```bgraph-paragraph
{"id":"d11be79e-cea3-55f2-a14c-1a6d23a51a09","node_type":"Paragraph","location":{"semantic":{"path":"3.8","depth":3,"breadcrumbs":["Attention Is All You Need"]},"physical":{"page":1,"bounding_box":{"x":235.35483,"y":329.80643,"width":138.58064,"height":24.0}}},"text_order":10,"token_count":13,"style":null}
```

### Abstract
```bgraph-section
{"id":"825d317f-dd4e-54b4-905b-65f977c92665","node_type":"Section","location":{"semantic":{"path":"3.9","depth":3,"breadcrumbs":["Attention Is All You Need","Abstract"]},"physical":{"page":1,"bounding_box":{"x":281.03226,"y":381.6774,"width":47.225803,"height":13.16129}}},"text_order":11,"token_count":2,"style":null}
```

The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely. Experiments on two machine translation tasks show these models to be superior in quality while being more parallelizable and requiring significantly less time to train. Our model achieves 28.4 BLEU on the WMT 2014 English-to-German translation task, improving over the existing best results, including ensembles, by over 2 BLEU. On the WMT 2014 English-to-French translation task, our model establishes a new single-model state-of-the-art BLEU score of 41.8 after training for 3.5 days on eight GPUs, a small fraction of the training costs of the best models from the literature. We show that the Transformer generalizes well to other tasks by applying it successfully to English constituency parsing both with large and limited training data.
```bgraph-paragraph
{"id":"9baa74a5-61c8-5f11-93e2-7688d7961c82","node_type":"Paragraph","location":{"semantic":{"path":"3.9.1","depth":4,"breadcrumbs":["Attention Is All You Need","Abstract"]},"physical":{"page":1,"bounding_box":{"x":140.12903,"y":409.54837,"width":329.80643,"height":164.12903}}},"text_order":12,"token_count":284,"style":null}
```

*Equal contribution. Listing order is random. Jakob proposed replacing RNNs with self-attention and started the effort to evaluate this idea. Ashish, with Illia, designed and implemented the first Transformer models and has been crucially involved in every aspect of this work. Noam proposed scaled dot-product attention, multi-head attention and the parameter-free position representation and became the other person involved in nearly every detail. Niki designed, implemented, tuned and evaluated countless model variants in our original codebase and tensor2tensor. Llion also experimented with novel model variants, was responsible for our initial codebase, and efficient inference and visualizations. Lukasz and Aidan spent countless long days designing various parts of and implementing tensor2tensor, replacing our earlier codebase, greatly improving results and massively accelerating our research.
```bgraph-paragraph
{"id":"fd48ca8f-6dc2-5cf7-97d7-6c8c3a570156","node_type":"Paragraph","location":{"semantic":{"path":"3.9.2","depth":4,"breadcrumbs":["Attention Is All You Need","Abstract"]},"physical":{"page":1,"bounding_box":{"x":104.51613,"y":595.3548,"width":400.25806,"height":90.58064}}},"text_order":13,"token_count":226,"style":null}
```

†Work performed while at Google Brain.
```bgraph-paragraph
{"id":"35124059-5863-576f-86be-1794bfaf9a89","node_type":"Paragraph","location":{"semantic":{"path":"3.9.3","depth":4,"breadcrumbs":["Attention Is All You Need","Abstract"]},"physical":{"page":1,"bounding_box":{"x":116.90322,"y":685.9355,"width":150.19354,"height":11.612903}}},"text_order":14,"token_count":10,"style":null}
```

‡Work performed while at Google Research.
```bgraph-paragraph
{"id":"d44c00bd-e67e-580b-bd0f-76793a1f391c","node_type":"Paragraph","location":{"semantic":{"path":"3.9.4","depth":4,"breadcrumbs":["Attention Is All You Need","Abstract"]},"physical":{"page":1,"bounding_box":{"x":116.90322,"y":696.7742,"width":164.12903,"height":10.838709}}},"text_order":15,"token_count":10,"style":null}
```

31st Conference on Neural Information Processing Systems (NIPS 2017), Long Beach, CA, USA.
```bgraph-footer
{"id":"6d5e19b1-7b01-5231-a551-2a71f4e14606","node_type":"Footer","location":{"semantic":{"path":"3.9.5","depth":4,"breadcrumbs":["Attention Is All You Need","Abstract"]},"physical":{"page":1,"bounding_box":{"x":104.51613,"y":729.2903,"width":356.12903,"height":11.612903}}},"text_order":16,"token_count":22,"style":null}
```

# 1 Introduction
```bgraph-section
{"id":"70d57604-6f5f-51c4-85d0-7f4b6eaf0ffe","node_type":"Section","location":{"semantic":{"path":"4","depth":1,"breadcrumbs":["1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":105.29032,"y":71.22581,"width":86.70967,"height":13.16129}}},"text_order":17,"token_count":3,"style":null}
```

Recurrent neural networks, long short-term memory [13] and gated recurrent [7] neural networks in particular, have been firmly established as state of the art approaches in sequence modeling and transduction problems such as language modeling and machine translation [35, 2, 5]. Numerous efforts have since continued to push the boundaries of recurrent language models and encoder-decoder architectures [38, 24, 15].
```bgraph-paragraph
{"id":"29075eeb-91f8-5104-a598-1785af4f0493","node_type":"Paragraph","location":{"semantic":{"path":"4.1","depth":2,"breadcrumbs":["1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":97.548386,"width":400.25806,"height":55.741936}}},"text_order":18,"token_count":104,"style":null}
```

Recurrent models typically factor computation along the symbol positions of the input and output sequences. Aligning the positions to steps in computation time, they generate a sequence of hidden states $h_t$, as a function of the previous hidden state $h_{t-1}$ and the input for position $t$. This inherently sequential nature precludes parallelization within training examples, which becomes critical at longer sequence lengths, as memory constraints limit batching across examples. Recent work has achieved significant improvements in computational efficiency through factorization tricks [21] and conditional computation [32], while also improving model performance in case of the latter. The fundamental constraint of sequential computation, however, remains.
```bgraph-paragraph
{"id":"0cae10b0-cb24-5f84-9272-3472d4ea5d0e","node_type":"Paragraph","location":{"semantic":{"path":"4.2","depth":2,"breadcrumbs":["1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":157.16129,"width":400.25806,"height":88.258064}}},"text_order":19,"token_count":191,"style":null}
```

Attention mechanisms have become an integral part of compelling sequence modeling and transduction models in various tasks, allowing modeling of dependencies without regard to their distance in the input or output sequences [2, 19]. In all but a few cases [27], however, such attention mechanisms are used in conjunction with a recurrent network.
```bgraph-paragraph
{"id":"cc288101-51b2-5cc5-b590-2e61c1b75137","node_type":"Paragraph","location":{"semantic":{"path":"4.3","depth":2,"breadcrumbs":["1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":250.06451,"width":400.25806,"height":44.129032}}},"text_order":20,"token_count":86,"style":null}
```

In this work we propose the Transformer, a model architecture eschewing recurrence and instead relying entirely on an attention mechanism to draw global dependencies between input and output. The Transformer allows for significantly more parallelization and can reach a new state of the art in translation quality after being trained for as little as twelve hours on eight P100 GPUs.
```bgraph-paragraph
{"id":"a69ca963-3f67-5063-826b-09abb9f666d5","node_type":"Paragraph","location":{"semantic":{"path":"4.4","depth":2,"breadcrumbs":["1 Introduction"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":299.61288,"width":400.25806,"height":44.129032}}},"text_order":21,"token_count":95,"style":null}
```

# 2 Background
```bgraph-section
{"id":"81d55c7b-072a-570c-be1f-dac52c532db0","node_type":"Section","location":{"semantic":{"path":"5","depth":1,"breadcrumbs":["2 Background"]},"physical":{"page":2,"bounding_box":{"x":105.29032,"y":361.54837,"width":84.38709,"height":13.16129}}},"text_order":22,"token_count":3,"style":null}
```

The goal of reducing sequential computation also forms the foundation of the Extended Neural GPU [16], ByteNet [18] and ConvS2S [9], all of which use convolutional neural networks as basic building block, computing hidden representations in parallel for all input and output positions. In these models, the number of operations required to relate signals from two arbitrary input or output positions grows in the distance between positions, linearly for ConvS2S and logarithmically for ByteNet. This makes it more difficult to learn dependencies between distant positions [12]. In the Transformer this is reduced to a constant number of operations, albeit at the cost of reduced effective resolution due to averaging attention-weighted positions, an effect we counteract with Multi-Head Attention as described in section 3.2.
```bgraph-paragraph
{"id":"b9688a04-04c0-5ec5-980f-b32e8677eb78","node_type":"Paragraph","location":{"semantic":{"path":"5.1","depth":2,"breadcrumbs":["2 Background"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":387.87094,"width":400.25806,"height":97.548386}}},"text_order":23,"token_count":206,"style":null}
```

Self-attention, sometimes called intra-attention is an attention mechanism relating different positions of a single sequence in order to compute a representation of the sequence. Self-attention has been used successfully in a variety of tasks including reading comprehension, abstractive summarization, textual entailment and learning task-independent sentence representations [4, 27, 28, 22].
```bgraph-paragraph
{"id":"d6905a37-e8fb-5319-9c6c-0aff0c7c3ddd","node_type":"Paragraph","location":{"semantic":{"path":"5.2","depth":2,"breadcrumbs":["2 Background"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":491.61288,"width":400.25806,"height":44.903225}}},"text_order":24,"token_count":98,"style":null}
```

End-to-end memory networks are based on a recurrent attention mechanism instead of sequence-aligned recurrence and have been shown to perform well on simple-language question answering and language modeling tasks [34].
```bgraph-paragraph
{"id":"996dc5d3-3acf-5bd9-b1f0-5b752b0049f0","node_type":"Paragraph","location":{"semantic":{"path":"5.3","depth":2,"breadcrumbs":["2 Background"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":540.3871,"width":400.25806,"height":33.29032}}},"text_order":25,"token_count":54,"style":null}
```

To the best of our knowledge, however, the Transformer is the first transduction model relying entirely on self-attention to compute representations of its input and output without using sequence-aligned RNNs or convolution. In the following sections, we will describe the Transformer, motivate self-attention and discuss its advantages over models such as [17, 18] and [9].
```bgraph-paragraph
{"id":"7da79e57-1b2d-5352-b0c2-28bc41bd217c","node_type":"Paragraph","location":{"semantic":{"path":"5.4","depth":2,"breadcrumbs":["2 Background"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":578.3226,"width":400.25806,"height":45.677418}}},"text_order":26,"token_count":93,"style":null}
```

# 3 Model Architecture
```bgraph-section
{"id":"eaf9a1b0-eb02-5c03-8cf8-ce755ba78f33","node_type":"Section","location":{"semantic":{"path":"6","depth":1,"breadcrumbs":["3 Model Architecture"]},"physical":{"page":2,"bounding_box":{"x":105.29032,"y":641.0322,"width":122.32258,"height":13.16129}}},"text_order":27,"token_count":5,"style":null}
```

Most competitive neural sequence transduction models have an encoder-decoder structure [5, 2, 35]. Here, the encoder maps an input sequence of symbol representations $(x_1, ..., x_n)$ to a sequence of continuous representations $\mathbf{z} = (z_1, ..., z_n)$. Given $\mathbf{z}$, the decoder then generates an output sequence $(y_1, ..., y_m)$ of symbols one element at a time. At each step the model is auto-regressive [10], consuming the previously generated symbols as additional input when generating the next.
```bgraph-paragraph
{"id":"d14de51a-ca51-55b5-8e25-7b045ef76927","node_type":"Paragraph","location":{"semantic":{"path":"6.1","depth":2,"breadcrumbs":["3 Model Architecture"]},"physical":{"page":2,"bounding_box":{"x":104.51613,"y":667.3548,"width":400.25806,"height":56.51613}}},"text_order":28,"token_count":128,"style":null}
```

2
```bgraph-footer
{"id":"c77cc7f0-c6cf-5126-a7a8-28c9a494edb4","node_type":"Footer","location":{"semantic":{"path":"6.2","depth":2,"breadcrumbs":["3 Model Architecture"]},"physical":{"page":2,"bounding_box":{"x":301.9355,"y":741.67737,"width":7.7419353,"height":9.290322}}},"text_order":29,"token_count":1,"style":null}
```

Figure 1: The Transformer - model architecture.
```bgraph-paragraph
{"id":"4d25248f-0c22-590c-89f5-9319b3f9a087","node_type":"Paragraph","location":{"semantic":{"path":"6.3","depth":2,"breadcrumbs":["3 Model Architecture"]},"physical":{"page":3,"bounding_box":{"x":205.93547,"y":400.25806,"width":195.87096,"height":13.16129}}},"text_order":30,"token_count":11,"style":null}
```

The Transformer follows this overall architecture using stacked self-attention and point-wise, fully connected layers for both the encoder and decoder, shown in the left and right halves of Figure 1, respectively.
```bgraph-paragraph
{"id":"a0b500e0-fc4c-58b7-a56e-e7a56af26847","node_type":"Paragraph","location":{"semantic":{"path":"6.4","depth":2,"breadcrumbs":["3 Model Architecture"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":432.77417,"width":401.03226,"height":34.064514}}},"text_order":31,"token_count":53,"style":null}
```

### 3.1 Encoder and Decoder Stacks
```bgraph-section
{"id":"033562ef-355c-5236-9a5c-3e639309a823","node_type":"Section","location":{"semantic":{"path":"6.5","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":477.6774,"width":148.64516,"height":13.16129}}},"text_order":32,"token_count":7,"style":null}
```

**Encoder:** The encoder is composed of a stack of $N = 6$ identical layers. Each layer has two sub-layers. The first is a multi-head self-attention mechanism, and the second is a simple, position-wise fully connected feed-forward network. We employ a residual connection [11] around each of the two sub-layers, followed by layer normalization [1]. That is, the output of each sub-layer is $\text{LayerNorm}(x + \text{Sublayer}(x))$, where $\text{Sublayer}(x)$ is the function implemented by the sub-layer itself. To facilitate these residual connections, all sub-layers in the model, as well as the embedding layers, produce outputs of dimension $d_{\text{model}} = 512$.
```bgraph-paragraph
{"id":"94b27b4e-7c73-5256-881c-5804d5201e07","node_type":"Paragraph","location":{"semantic":{"path":"6.5.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":498.58063,"width":401.03226,"height":78.19354}}},"text_order":33,"token_count":168,"style":null}
```

**Decoder:** The decoder is also composed of a stack of $N = 6$ identical layers. In addition to the two sub-layers in each encoder layer, the decoder inserts a third sub-layer, which performs multi-head attention over the output of the encoder stack. Similar to the encoder, we employ residual connections around each of the sub-layers, followed by layer normalization. We also modify the self-attention sub-layer in the decoder stack to prevent positions from attending to subsequent positions. This masking, combined with fact that the output embeddings are offset by one position, ensures that the predictions for position $i$ can depend only on the known outputs at positions less than $i$.
```bgraph-paragraph
{"id":"892bfe8d-9f44-5eeb-bc15-bb8b86f40221","node_type":"Paragraph","location":{"semantic":{"path":"6.5.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":587.6129,"width":401.03226,"height":78.19354}}},"text_order":34,"token_count":173,"style":null}
```

### 3.2 Attention
```bgraph-section
{"id":"244371e4-3206-513f-bbd9-5d8d252920f9","node_type":"Section","location":{"semantic":{"path":"6.5.3","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":677.4193,"width":66.58064,"height":11.612903}}},"text_order":35,"token_count":3,"style":null}
```

An attention function can be described as mapping a query and a set of key-value pairs to an output, where the query, keys, values, and output are all vectors. The output is computed as a weighted sum
```bgraph-paragraph
{"id":"5480c6a4-65d1-5de2-b273-d1c953a4f26a","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention"]},"physical":{"page":3,"bounding_box":{"x":104.51613,"y":697.54834,"width":401.03226,"height":24.0}}},"text_order":36,"token_count":50,"style":null}
```

3
```bgraph-footer
{"id":"0288f035-c871-5421-97e3-54d035a670e6","node_type":"Footer","location":{"semantic":{"path":"6.5.3.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention"]},"physical":{"page":3,"bounding_box":{"x":299.61288,"y":737.80646,"width":10.064516,"height":12.387096}}},"text_order":37,"token_count":1,"style":null}
```

Scaled Dot-Product Attention
```bgraph-paragraph
{"id":"ed29dd75-c837-5d18-87b2-a5a64b6b2eb5","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.3","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention"]},"physical":{"page":4,"bounding_box":{"x":145.54839,"y":69.677414,"width":121.548386,"height":10.838709}}},"text_order":38,"token_count":7,"style":null}
```

Multi-Head Attention
```bgraph-paragraph
{"id":"12c84a4a-5946-5926-8b89-9ec89bcd4798","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.4","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention"]},"physical":{"page":4,"bounding_box":{"x":361.54837,"y":69.677414,"width":90.58064,"height":10.838709}}},"text_order":39,"token_count":5,"style":null}
```

Figure 2: (left) Scaled Dot-Product Attention. (right) Multi-Head Attention consists of several attention layers running in parallel.
```bgraph-paragraph
{"id":"2555c761-5a11-5f4e-bffa-0413311c8178","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.5","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":271.7419,"width":400.25806,"height":24.0}}},"text_order":40,"token_count":33,"style":null}
```

of the values, where the weight assigned to each value is computed by a compatibility function of the query with the corresponding key.
```bgraph-paragraph
{"id":"ab888ba2-2fc7-58cf-a4df-241ea41dfde8","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.6","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":315.09677,"width":400.25806,"height":24.0}}},"text_order":41,"token_count":33,"style":null}
```

### 3.2.1 Scaled Dot-Product Attention
```bgraph-section
{"id":"56058479-c10e-59d9-b869-bb3067e9d7ae","node_type":"Section","location":{"semantic":{"path":"6.5.3.7","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention"]},"physical":{"page":4,"bounding_box":{"x":105.29032,"y":349.1613,"width":160.25806,"height":11.612903}}},"text_order":42,"token_count":8,"style":null}
```

We call our particular attention "Scaled Dot-Product Attention" (Figure 2). The input consists of queries and keys of dimension $d_k$, and values of dimension $d_v$. We compute the dot products of the query with all keys, divide each by $\sqrt{d_k}$, and apply a softmax function to obtain the weights on the values.
```bgraph-paragraph
{"id":"b3917551-af1e-5132-a3b0-644ab6f0d403","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":366.96774,"width":400.25806,"height":46.45161}}},"text_order":43,"token_count":79,"style":null}
```

In practice, we compute the attention function on a set of queries simultaneously, packed together into a matrix $Q$. The keys and values are also packed together into matrices $K$ and $V$. We compute the matrix of outputs as:
```bgraph-paragraph
{"id":"80f3a924-f0bc-5823-b0cd-e4b733e2d55c","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":417.2903,"width":400.25806,"height":34.064514}}},"text_order":44,"token_count":56,"style":null}
```

$$\text{Attention}(Q, K, V) = \text{softmax}(\frac{QK^T}{\sqrt{d_k}})V \tag{1}$$
```bgraph-equation
{"id":"9406a8d4-dbc4-5e64-9ade-91884217946b","node_type":"Equation","location":{"semantic":{"path":"6.5.3.7.3","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention"]},"physical":{"page":4,"bounding_box":{"x":216.77419,"y":462.96774,"width":288.0,"height":28.64516}}},"text_order":45,"token_count":20,"style":null}
```

The two most commonly used attention functions are additive attention [2], and dot-product (multiplicative) attention. Dot-product attention is identical to our algorithm, except for the scaling factor of $\frac{1}{\sqrt{d_k}}$. Additive attention computes the compatibility function using a feed-forward network with a single hidden layer. While the two are similar in theoretical complexity, dot-product attention is much faster and more space-efficient in practice, since it can be implemented using highly optimized matrix multiplication code.
```bgraph-paragraph
{"id":"06fb6ac7-3ebd-5bff-a899-55dd3fdfaac2","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.4","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":497.80643,"width":400.25806,"height":68.90322}}},"text_order":46,"token_count":136,"style":null}
```

While for small values of $d_k$ the two mechanisms perform similarly, additive attention outperforms dot product attention without scaling for larger values of $d_k$ [3]. We suspect that for large values of $d_k$, the dot products grow large in magnitude, pushing the softmax function into regions where it has extremely small gradients $^4$. To counteract this effect, we scale the dot products by $\frac{1}{\sqrt{d_k}}$.
```bgraph-paragraph
{"id":"3a695574-3e32-57a3-ad69-dbafefd7fc97","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.5","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":570.5806,"width":400.25806,"height":49.548386}}},"text_order":47,"token_count":105,"style":null}
```

### 3.2.2 Multi-Head Attention
```bgraph-section
{"id":"7b50f31b-2946-5804-8ca4-2a77761c65fd","node_type":"Section","location":{"semantic":{"path":"6.5.3.7.6","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":4,"bounding_box":{"x":105.29032,"y":629.4193,"width":126.19354,"height":10.838709}}},"text_order":48,"token_count":6,"style":null}
```

Instead of performing a single attention function with $d_{\text{model}}$-dimensional keys, values and queries, we found it beneficial to linearly project the queries, keys and values $h$ times with different, learned linear projections to $d_k$, $d_k$ and $d_v$ dimensions, respectively. On each of these projected versions of queries, keys and values we then perform the attention function in parallel, yielding $d_v$-dimensional
```bgraph-paragraph
{"id":"0da5dc66-9af5-575d-83d4-d3dc725ac450","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":648.0,"width":400.25806,"height":45.677418}}},"text_order":49,"token_count":107,"style":null}
```

$^4$To illustrate why the dot products get large, assume that the components of $q$ and $k$ are independent random variables with mean 0 and variance 1. Then their dot product, $q \cdot k = \sum_{i=1}^{d_k} q_i k_i$, has mean 0 and variance $d_k$.
```bgraph-paragraph
{"id":"b1b18c41-5733-5798-bb27-62672263ff76","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":4,"bounding_box":{"x":104.51613,"y":699.09674,"width":400.25806,"height":25.548386}}},"text_order":50,"token_count":61,"style":null}
```

4
```bgraph-footer
{"id":"d8f964e9-775a-5953-a7a1-464c8fcec392","node_type":"Footer","location":{"semantic":{"path":"6.5.3.7.6.3","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":4,"bounding_box":{"x":301.9355,"y":741.67737,"width":7.7419353,"height":9.290322}}},"text_order":51,"token_count":1,"style":null}
```

output values. These are concatenated and once again projected, resulting in the final values, as depicted in Figure 2.
```bgraph-paragraph
{"id":"17d347d1-acc9-5133-bff7-79b8e62e25ec","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.4","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":72.77419,"width":400.25806,"height":23.225805}}},"text_order":52,"token_count":29,"style":null}
```

Multi-head attention allows the model to jointly attend to information from different representation subspaces at different positions. With a single attention head, averaging inhibits this.
```bgraph-paragraph
{"id":"0d17198c-071a-5a40-9515-ab4298ccc67d","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.5","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":99.870964,"width":400.25806,"height":24.774193}}},"text_order":53,"token_count":47,"style":null}
```

$$\begin{array}{l} \operatorname{MultiHead}(Q, K, V) = \operatorname{Concat}(\operatorname{head}_{1}, \dots, \operatorname{head}_{h}) W^{O} \\ \text { where } \operatorname{head}_{i} = \operatorname{Attention}\left(Q W_{i}^{Q}, K W_{i}^{K}, V W_{i}^{V}\right) \end{array}$$
```bgraph-equation
{"id":"929b4e55-b302-5c53-adc6-8859b9d99704","node_type":"Equation","location":{"semantic":{"path":"6.5.3.7.6.6","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":5,"bounding_box":{"x":184.25806,"y":144.0,"width":242.32257,"height":33.29032}}},"text_order":54,"token_count":68,"style":null}
```

Where the projections are parameter matrices $W_{i}^{Q} \in \mathbb{R}^{d_{\text{model}} \times d_{k}}$, $W_{i}^{K} \in \mathbb{R}^{d_{\text{model}} \times d_{k}}$, $W_{i}^{V} \in \mathbb{R}^{d_{\text{model}} \times d_{v}}$ and $W^{O} \in \mathbb{R}^{h d_{v} \times d_{\text{model}}}$.
```bgraph-paragraph
{"id":"c97ba2a9-52e8-5552-91d4-4aa4986cad15","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.7","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":202.8387,"width":400.25806,"height":24.774193}}},"text_order":55,"token_count":71,"style":null}
```

In this work we employ $h = 8$ parallel attention layers, or heads. For each of these we use $d_{k} = d_{v} = d_{\text{model}} / h = 64$. Due to the reduced dimension of each head, the total computational cost is similar to that of single-head attention with full dimensionality.
```bgraph-paragraph
{"id":"65daf3b4-df66-5b7c-be45-f34b8e29fc45","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.8","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":232.25806,"width":401.03226,"height":34.838707}}},"text_order":56,"token_count":69,"style":null}
```

### 3.2.3 Applications of Attention in our Model
```bgraph-section
{"id":"a6920f0e-1ea8-5d5e-840c-7db06d241d22","node_type":"Section","location":{"semantic":{"path":"6.5.3.7.6.9","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model"]},"physical":{"page":5,"bounding_box":{"x":105.29032,"y":278.70966,"width":199.74193,"height":12.387096}}},"text_order":57,"token_count":11,"style":null}
```

The Transformer uses multi-head attention in three different ways:
```bgraph-paragraph
{"id":"d80136cd-8c6f-5ff5-9a66-4f9e3555651b","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":298.0645,"width":269.41934,"height":12.387096}}},"text_order":58,"token_count":16,"style":null}
```

- In "encoder-decoder attention" layers, the queries come from the previous decoder layer, and the memory keys and values come from the output of the encoder. This allows every position in the decoder to attend over all positions in the input sequence. This mimics the typical encoder-decoder attention mechanisms in sequence-to-sequence models such as [38, 2, 9].
- The encoder contains self-attention layers. In a self-attention layer all of the keys, values and queries come from the same place, in this case, the output of the previous layer in the encoder. Each position in the encoder can attend to all positions in the previous layer of the encoder.
- Similarly, self-attention layers in the decoder allow each position in the decoder to attend to all positions in the decoder up to and including that position. We need to prevent leftward information flow in the decoder to preserve the auto-regressive property. We implement this inside of scaled dot-product attention by masking out (setting to $-\infty$) all values in the input of the softmax which correspond to illegal connections. See Figure 2.
```bgraph-list
{"id":"5dbeefff-d71b-5e7b-8056-90d9cda517f4","node_type":"List","location":{"semantic":{"path":"6.5.3.7.6.9.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model"]},"physical":{"page":5,"bounding_box":{"x":132.3871,"y":318.96774,"width":372.3871,"height":163.35483}}},"text_order":59,"token_count":277,"style":null}
```

### 3.3 Position-wise Feed-Forward Networks
```bgraph-section
{"id":"10c5f0d6-0083-57ae-8f56-ef0b63e51099","node_type":"Section","location":{"semantic":{"path":"6.5.3.7.6.9.3","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks"]},"physical":{"page":5,"bounding_box":{"x":105.29032,"y":494.70966,"width":189.67741,"height":11.612903}}},"text_order":60,"token_count":9,"style":null}
```

In addition to attention sub-layers, each of the layers in our encoder and decoder contains a fully connected feed-forward network, which is applied to each position separately and identically. This consists of two linear transformations with a ReLU activation in between.
```bgraph-paragraph
{"id":"cc146a2b-4c21-5851-a2f7-9a5277fd7b2e","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":516.3871,"width":400.25806,"height":33.29032}}},"text_order":61,"token_count":68,"style":null}
```

$$\operatorname{FFN}(x) = \max(0, x W_{1} + b_{1}) W_{2} + b_{2} \tag{2}$$
```bgraph-equation
{"id":"d442336b-dc85-5882-a232-9624da7f0943","node_type":"Equation","location":{"semantic":{"path":"6.5.3.7.6.9.3.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks"]},"physical":{"page":5,"bounding_box":{"x":223.74193,"y":565.9355,"width":281.03226,"height":13.16129}}},"text_order":62,"token_count":18,"style":null}
```

While the linear transformations are the same across different positions, they use different parameters from layer to layer. Another way of describing this is as two convolutions with kernel size 1. The dimensionality of input and output is $d_{\text{model}} = 512$, and the inner-layer has dimensionality $d_{ff} = 2048$.
```bgraph-paragraph
{"id":"bd5efbba-7f97-5b5e-a127-75a75d785b6a","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.3","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":588.3871,"width":401.03226,"height":45.677418}}},"text_order":63,"token_count":80,"style":null}
```

### 3.4 Embeddings and Softmax
```bgraph-section
{"id":"504a0118-0481-5877-a2f1-f94d7cfb4530","node_type":"Section","location":{"semantic":{"path":"6.5.3.7.6.9.3.4","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax"]},"physical":{"page":5,"bounding_box":{"x":105.29032,"y":646.4516,"width":137.03226,"height":11.612903}}},"text_order":64,"token_count":6,"style":null}
```

Similarly to other sequence transduction models, we use learned embeddings to convert the input tokens and output tokens to vectors of dimension $d_{\text{model}}$. We also use the usual learned linear transformation and softmax function to convert the decoder output to predicted next-token probabilities. In our model, we share the same weight matrix between the two embedding layers and the pre-softmax linear transformation, similar to [30]. In the embedding layers, we multiply those weights by $\sqrt{d_{\text{model}}}$.
```bgraph-paragraph
{"id":"2abb513b-5f5b-5d40-974e-9e2022439072","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax"]},"physical":{"page":5,"bounding_box":{"x":104.51613,"y":667.3548,"width":401.03226,"height":56.51613}}},"text_order":65,"token_count":131,"style":null}
```

5
```bgraph-footer
{"id":"ff6c4159-7017-573a-8099-c6f7cce87dbe","node_type":"Footer","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax"]},"physical":{"page":5,"bounding_box":{"x":302.70966,"y":741.67737,"width":6.193548,"height":9.290322}}},"text_order":66,"token_count":1,"style":null}
```

Table 1: Maximum path lengths, per-layer complexity and minimum number of sequential operations for different layer types. n is the sequence length, d is the representation dimension, k is the kernel size of convolutions and r the size of the neighborhood in restricted self-attention.
```bgraph-paragraph
{"id":"fdaeaa59-9ab0-5bf8-8fc2-a388c49cff61","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.3","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":69.677414,"width":400.25806,"height":33.29032}}},"text_order":67,"token_count":71,"style":null}
```

|  Layer Type | Complexity per Layer | Sequential Operations | Maximum Path Length  |
| --- | --- | --- | --- |
|  Self-Attention | \( O(n^{2} \cdot d) \) | \( O(1) \) | \( O(1) \)  |
|  Recurrent | \( O(n \cdot d^{2}) \) | \( O(n) \) | \( O(n) \)  |
|  Convolutional | \( O(k \cdot n \cdot d^{2}) \) | \( O(1) \) | \( O(log_{k}(n)) \)  |
|  Self-Attention (restricted) | \( O(r \cdot n \cdot d) \) | \( O(1) \) | \( O(n/r) \)  |
```bgraph-table
{"id":"ac61748e-675e-531d-b6e0-4cb239473051","node_type":"Table","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.4","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax"]},"physical":{"page":6,"bounding_box":{"x":116.12903,"y":110.70967,"width":379.35483,"height":78.19354}}},"text_order":68,"token_count":107,"style":null}
```

### 3.5 Positional Encoding
```bgraph-section
{"id":"dddfeb49-88ff-57aa-a963-aa505df4c351","node_type":"Section","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.5","depth":3,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax","3.5 Positional Encoding"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":211.35483,"width":111.48387,"height":13.16129}}},"text_order":69,"token_count":5,"style":null}
```

Since our model contains no recurrence and no convolution, in order for the model to make use of the order of the sequence, we must inject some information about the relative or absolute position of the tokens in the sequence. To this end, we add "positional encodings" to the input embeddings at the bottoms of the encoder and decoder stacks. The positional encodings have the same dimension  \( d_{model} \)  as the embeddings, so that the two can be summed. There are many choices of positional encodings, learned and fixed [9].
```bgraph-paragraph
{"id":"31ba6ba4-df35-5ef8-84a0-196d2f72664f","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.5.1","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax","3.5 Positional Encoding"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":231.48387,"width":400.25806,"height":67.354836}}},"text_order":70,"token_count":132,"style":null}
```

In this work, we use sine and cosine functions of different frequencies:
```bgraph-paragraph
{"id":"8093c913-9af3-5405-9f91-3123c3454ac5","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.5.2","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax","3.5 Positional Encoding"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":301.9355,"width":285.6774,"height":13.935484}}},"text_order":71,"token_count":18,"style":null}
```

\[
P E _ {(p o s, 2 i)} = \sin (p o s / 1 0 0 0 0 ^ {2 i / d _ {\mathrm{model}}})
\]
```bgraph-equation
{"id":"85441e28-ee7e-5424-8464-6d9d699f433f","node_type":"Equation","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.5.3","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax","3.5 Positional Encoding"]},"physical":{"page":6,"bounding_box":{"x":233.80644,"y":334.4516,"width":153.29031,"height":15.4838705}}},"text_order":72,"token_count":21,"style":null}
```

\[
P E _ {(p o s, 2 i + 1)} = c o s (p o s / 1 0 0 0 0 ^ {2 i / d _ {\mathrm{model}}})
\]
```bgraph-equation
{"id":"7fd3222e-9f56-5811-9fbe-44cf4f7bfc0e","node_type":"Equation","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.5.4","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax","3.5 Positional Encoding"]},"physical":{"page":6,"bounding_box":{"x":225.29031,"y":351.48386,"width":161.80644,"height":15.4838705}}},"text_order":73,"token_count":22,"style":null}
```

where \( pos \) is the position and \( i \) is the dimension. That is, each dimension of the positional encoding corresponds to a sinusoid. The wavelengths form a geometric progression from \( 2\pi \) to \( 10000 \cdot 2\pi \). We chose this function because we hypothesized it would allow the model to easily learn to attend by relative positions, since for any fixed offset \( k \), \( PE_{pos + k} \) can be represented as a linear function of \( PE_{pos} \).
```bgraph-paragraph
{"id":"fb814971-0062-5db2-bdd0-b4c78ae6b2ca","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.5.5","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax","3.5 Positional Encoding"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":374.70966,"width":400.25806,"height":57.29032}}},"text_order":74,"token_count":115,"style":null}
```

We also experimented with using learned positional embeddings [9] instead, and found that the two versions produced nearly identical results (see Table 3 row (E)). We chose the sinusoidal version because it may allow the model to extrapolate to sequence lengths longer than the ones encountered during training.
```bgraph-paragraph
{"id":"7044b7af-591c-5c48-8231-c7b09a899899","node_type":"Paragraph","location":{"semantic":{"path":"6.5.3.7.6.9.3.4.5.6","depth":4,"breadcrumbs":["3 Model Architecture","3.1 Encoder and Decoder Stacks","3.2 Attention","3.2.1 Scaled Dot-Product Attention","3.2.2 Multi-Head Attention","3.2.3 Applications of Attention in our Model","3.3 Position-wise Feed-Forward Networks","3.4 Embeddings and Softmax","3.5 Positional Encoding"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":435.09677,"width":400.25806,"height":45.677418}}},"text_order":75,"token_count":77,"style":null}
```

## 4 Why Self-Attention
```bgraph-section
{"id":"2cce3606-8fba-5ca4-b0f8-56a1e9455bee","node_type":"Section","location":{"semantic":{"path":"6.6","depth":2,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":495.48386,"width":121.548386,"height":13.935484}}},"text_order":76,"token_count":5,"style":null}
```

In this section we compare various aspects of self-attention layers to the recurrent and convolutional layers commonly used for mapping one variable-length sequence of symbol representations  \( (x_{1},\ldots,x_{n}) \)  to another sequence of equal length  \( (z_{1},\ldots,z_{n}) \) , with  \( x_{i},z_{i}\in R^{d} \) , such as a hidden layer in a typical sequence transduction encoder or decoder. Motivating our use of self-attention we consider three desiderata.
```bgraph-paragraph
{"id":"8ab02fd5-0f5e-51f4-a190-c153a37ddb1f","node_type":"Paragraph","location":{"semantic":{"path":"6.6.1","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":519.4838,"width":400.25806,"height":55.741936}}},"text_order":77,"token_count":116,"style":null}
```

One is the total computational complexity per layer. Another is the amount of computation that can be parallelized, as measured by the minimum number of sequential operations required.
```bgraph-paragraph
{"id":"3bd1892f-a016-5572-b010-e67cc0645998","node_type":"Paragraph","location":{"semantic":{"path":"6.6.2","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":579.871,"width":400.25806,"height":23.225805}}},"text_order":78,"token_count":46,"style":null}
```

The third is the path length between long-range dependencies in the network. Learning long-range dependencies is a key challenge in many sequence transduction tasks. One key factor affecting the ability to learn such dependencies is the length of the paths forward and backward signals have to traverse in the network. The shorter these paths between any combination of positions in the input and output sequences, the easier it is to learn long-range dependencies  \( [12] \) . Hence we also compare the maximum path length between any two input and output positions in networks composed of the different layer types.
```bgraph-paragraph
{"id":"082150b1-b911-5dc4-b683-a509d535050d","node_type":"Paragraph","location":{"semantic":{"path":"6.6.3","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":606.19354,"width":400.25806,"height":79.741936}}},"text_order":79,"token_count":154,"style":null}
```

As noted in Table 1, a self-attention layer connects all positions with a constant number of sequentially executed operations, whereas a recurrent layer requires \( O(n) \) sequential operations. In terms of computational complexity, self-attention layers are faster than recurrent layers when the sequence
```bgraph-paragraph
{"id":"e97ba2ea-cd7e-5abc-9118-cff5f6ba694c","node_type":"Paragraph","location":{"semantic":{"path":"6.6.4","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":6,"bounding_box":{"x":104.51613,"y":689.0322,"width":400.25806,"height":34.838707}}},"text_order":80,"token_count":76,"style":null}
```

6
```bgraph-footer
{"id":"760754a9-961f-5abf-9744-1532dcd8dd73","node_type":"Footer","location":{"semantic":{"path":"6.6.5","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":6,"bounding_box":{"x":301.9355,"y":741.67737,"width":7.7419353,"height":10.064516}}},"text_order":81,"token_count":1,"style":null}
```

length $n$ is smaller than the representation dimensionality $d$, which is most often the case with sentence representations used by state-of-the-art models in machine translations, such as word-piece [38] and byte-pair [31] representations. To improve computational performance for tasks involving very long sequences, self-attention could be restricted to considering only a neighborhood of size $r$ in the input sequence centered around the respective output position. This would increase the maximum path length to $O(n/r)$. We plan to investigate this approach further in future work.
```bgraph-paragraph
{"id":"6d5b1126-25a8-57d7-af4a-39c2207d5651","node_type":"Paragraph","location":{"semantic":{"path":"6.6.6","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":72.77419,"width":400.25806,"height":66.58064}}},"text_order":82,"token_count":147,"style":null}
```

A single convolutional layer with kernel width $k < n$ does not connect all pairs of input and output positions. Doing so requires a stack of $O(n/k)$ convolutional layers in the case of contiguous kernels, or $O(log_k(n))$ in the case of dilated convolutions [18], increasing the length of the longest paths between any two positions in the network. Convolutional layers are generally more expensive than recurrent layers, by a factor of $k$. Separable convolutions [6], however, decrease the complexity considerably, to $O(k \cdot n \cdot d + n \cdot d^2)$. Even with $k = n$, however, the complexity of a separable convolution is equal to the combination of a self-attention layer and a point-wise feed-forward layer, the approach we take in our model.
```bgraph-paragraph
{"id":"57487601-4aa4-5cba-aa27-7dc79d844d3d","node_type":"Paragraph","location":{"semantic":{"path":"6.6.7","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":143.2258,"width":401.80643,"height":88.258064}}},"text_order":83,"token_count":188,"style":null}
```

As side benefit, self-attention could yield more interpretable models. We inspect attention distributions from our models and present and discuss examples in the appendix. Not only do individual attention heads clearly learn to perform different tasks, many appear to exhibit behavior related to the syntactic and semantic structure of the sentences.
```bgraph-paragraph
{"id":"b21cde57-52ee-5e74-95bc-960926f92f2a","node_type":"Paragraph","location":{"semantic":{"path":"6.6.8","depth":3,"breadcrumbs":["3 Model Architecture","4 Why Self-Attention"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":235.35483,"width":400.25806,"height":46.45161}}},"text_order":84,"token_count":87,"style":null}
```

## 5 Training
```bgraph-section
{"id":"bc2fe380-3765-576a-96ef-e7dc0c6b8fad","node_type":"Section","location":{"semantic":{"path":"6.7","depth":2,"breadcrumbs":["3 Model Architecture","5 Training"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":298.0645,"width":67.354836,"height":13.16129}}},"text_order":85,"token_count":2,"style":null}
```

This section describes the training regime for our models.
```bgraph-paragraph
{"id":"fb61ff9f-84cd-5ed5-8794-7ddf2fefd521","node_type":"Paragraph","location":{"semantic":{"path":"6.7.1","depth":3,"breadcrumbs":["3 Model Architecture","5 Training"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":324.3871,"width":232.25806,"height":11.612903}}},"text_order":86,"token_count":14,"style":null}
```

### 5.1 Training Data and Batching
```bgraph-section
{"id":"f444a7b6-1f33-54d6-a566-077e12062cef","node_type":"Section","location":{"semantic":{"path":"6.7.2","depth":3,"breadcrumbs":["3 Model Architecture","5 Training","5.1 Training Data and Batching"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":350.70966,"width":146.32257,"height":12.387096}}},"text_order":87,"token_count":7,"style":null}
```

We trained on the standard WMT 2014 English-German dataset consisting of about 4.5 million sentence pairs. Sentences were encoded using byte-pair encoding [3], which has a shared source-target vocabulary of about 37000 tokens. For English-French, we used the significantly larger WMT 2014 English-French dataset consisting of 36M sentences and split tokens into a 32000 word-piece vocabulary [38]. Sentence pairs were batched together by approximate sequence length. Each training batch contained a set of sentence pairs containing approximately 25000 source tokens and 25000 target tokens.
```bgraph-paragraph
{"id":"82e3030f-d1eb-5969-a087-0d89425f7eca","node_type":"Paragraph","location":{"semantic":{"path":"6.7.2.1","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.1 Training Data and Batching"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":370.83868,"width":400.25806,"height":78.96774}}},"text_order":88,"token_count":147,"style":null}
```

### 5.2 Hardware and Schedule
```bgraph-section
{"id":"9d05b8f0-d20c-58ad-be53-aa4505434a6e","node_type":"Section","location":{"semantic":{"path":"6.7.3","depth":3,"breadcrumbs":["3 Model Architecture","5 Training","5.2 Hardware and Schedule"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":464.5161,"width":129.29031,"height":10.838709}}},"text_order":89,"token_count":6,"style":null}
```

We trained our models on one machine with 8 NVIDIA P100 GPUs. For our base models using the hyperparameters described throughout the paper, each training step took about 0.4 seconds. We trained the base models for a total of 100,000 steps or 12 hours. For our big models,(described on the bottom line of table 3), step time was 1.0 seconds. The big models were trained for 300,000 steps (3.5 days).
```bgraph-paragraph
{"id":"be62eb87-81b3-5d56-9743-604a37a114eb","node_type":"Paragraph","location":{"semantic":{"path":"6.7.3.1","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.2 Hardware and Schedule"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":485.41934,"width":400.25806,"height":54.96774}}},"text_order":90,"token_count":99,"style":null}
```

### 5.3 Optimizer
```bgraph-section
{"id":"4623fa3a-67d1-5751-9714-456f6a03821f","node_type":"Section","location":{"semantic":{"path":"6.7.4","depth":3,"breadcrumbs":["3 Model Architecture","5 Training","5.3 Optimizer"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":555.871,"width":70.451614,"height":11.612903}}},"text_order":91,"token_count":3,"style":null}
```

We used the Adam optimizer [20] with $\beta_1 = 0.9$, $\beta_2 = 0.98$ and $\epsilon = 10^{-9}$. We varied the learning rate over the course of training, according to the formula:
```bgraph-paragraph
{"id":"07e2b4d3-69c3-52e0-8d78-c1dee93cb97b","node_type":"Paragraph","location":{"semantic":{"path":"6.7.4.1","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.3 Optimizer"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":576.0,"width":400.25806,"height":24.0}}},"text_order":92,"token_count":44,"style":null}
```

$$lrate = d_{model}^{-0.5} \cdot \min(step\_num^{-0.5}, step\_num \cdot warmup\_steps^{-1.5}) \tag{3}$$
```bgraph-equation
{"id":"4cc54e43-a957-5143-a503-20eecb42a406","node_type":"Equation","location":{"semantic":{"path":"6.7.4.2","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.3 Optimizer"]},"physical":{"page":7,"bounding_box":{"x":158.70967,"y":616.25806,"width":346.0645,"height":15.4838705}}},"text_order":93,"token_count":25,"style":null}
```

This corresponds to increasing the learning rate linearly for the first $warmup\_steps$ training steps, and decreasing it thereafter proportionally to the inverse square root of the step number. We used $warmup\_steps = 4000$.
```bgraph-paragraph
{"id":"f14d4dc0-1b69-513f-b506-379e98829786","node_type":"Paragraph","location":{"semantic":{"path":"6.7.4.3","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.3 Optimizer"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":641.0322,"width":400.25806,"height":34.064514}}},"text_order":94,"token_count":56,"style":null}
```

### 5.4 Regularization
```bgraph-section
{"id":"3c98fe7c-9fc3-5276-97bb-7689c071a77b","node_type":"Section","location":{"semantic":{"path":"6.7.5","depth":3,"breadcrumbs":["3 Model Architecture","5 Training","5.4 Regularization"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":689.80646,"width":89.80645,"height":11.612903}}},"text_order":95,"token_count":4,"style":null}
```

We employ three types of regularization during training:
```bgraph-paragraph
{"id":"26ebcaf7-cae1-5cc5-8394-f75d57c83d73","node_type":"Paragraph","location":{"semantic":{"path":"6.7.5.1","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.4 Regularization"]},"physical":{"page":7,"bounding_box":{"x":104.51613,"y":711.4838,"width":227.6129,"height":12.387096}}},"text_order":96,"token_count":14,"style":null}
```

7
```bgraph-footer
{"id":"c0972506-b75a-5e1e-9f08-d1b95bb0de7f","node_type":"Footer","location":{"semantic":{"path":"6.7.5.2","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.4 Regularization"]},"physical":{"page":7,"bounding_box":{"x":301.9355,"y":741.67737,"width":6.967742,"height":9.290322}}},"text_order":97,"token_count":1,"style":null}
```

Table 2: The Transformer achieves better BLEU scores than previous state-of-the-art models on the English-to-German and English-to-French newstest2014 tests at a fraction of the training cost.
```bgraph-paragraph
{"id":"0c16a717-fa1f-5e3f-8abf-99eefc19386b","node_type":"Paragraph","location":{"semantic":{"path":"6.7.5.3","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.4 Regularization"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":67.354836,"width":399.48386,"height":23.225805}}},"text_order":98,"token_count":48,"style":null}
```

|  Model | BLEU |   | Training Cost (FLOPs)  |   |
| --- | --- | --- | --- | --- |
|   |  EN-DE | EN-FR | EN-DE | EN-FR  |
|  ByteNet [18] | 23.75 |  |  |   |
|  Deep-Att + PosUnk [39] |  | 39.2 |  | 1.0 · 10^{20}  |
|  GNMT + RL [38] | 24.6 | 39.92 | 2.3 · 10^{19} | 1.4 · 10^{20}  |
|  ConvS2S [9] | 25.16 | 40.46 | 9.6 · 10^{18} | 1.5 · 10^{20}  |
|  MoE [32] | 26.03 | 40.56 | 2.0 · 10^{19} | 1.2 · 10^{20}  |
|  Deep-Att + PosUnk Ensemble [39] |  | 40.4 |  | 8.0 · 10^{20}  |
|  GNMT + RL Ensemble [38] | 26.30 | 41.16 | 1.8 · 10^{20} | 1.1 · 10^{21}  |
|  ConvS2S Ensemble [9] | 26.36 | **41.29** | 7.7 · 10^{19} | 1.2 · 10^{21}  |
|  Transformer (base model) | 27.3 | 38.1 | **3.3 · 10^{18}** |   |
|  Transformer (big) | **28.4** | **41.8** | 2.3 · 10^{19} |   |
```bgraph-table
{"id":"c7f7b72a-16dd-5728-8884-8af27f3133d1","node_type":"Table","location":{"semantic":{"path":"6.7.5.4","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.4 Regularization"]},"physical":{"page":8,"bounding_box":{"x":126.967735,"y":90.58064,"width":351.48386,"height":150.19354}}},"text_order":99,"token_count":196,"style":null}
```

**Residual Dropout** We apply dropout [33] to the output of each sub-layer, before it is added to the sub-layer input and normalized. In addition, we apply dropout to the sums of the embeddings and the positional encodings in both the encoder and decoder stacks. For the base model, we use a rate of $P_{drop} = 0.1$.
```bgraph-paragraph
{"id":"e5c7def0-df42-5669-a6ca-a9da6071e8a9","node_type":"Paragraph","location":{"semantic":{"path":"6.7.5.5","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.4 Regularization"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":268.64514,"width":401.03226,"height":45.677418}}},"text_order":100,"token_count":79,"style":null}
```

**Label Smoothing** During training, we employed label smoothing of value $\epsilon_{ls} = 0.1$ [36]. This hurts perplexity, as the model learns to be more unsure, but improves accuracy and BLEU score.
```bgraph-paragraph
{"id":"2808b2f9-cdf3-5644-a159-e9565def7ae8","node_type":"Paragraph","location":{"semantic":{"path":"6.7.5.6","depth":4,"breadcrumbs":["3 Model Architecture","5 Training","5.4 Regularization"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":326.70966,"width":399.48386,"height":24.0}}},"text_order":101,"token_count":50,"style":null}
```

## 6 Results
```bgraph-section
{"id":"a3ffbad7-0f65-5e95-981e-226863b3354c","node_type":"Section","location":{"semantic":{"path":"6.8","depth":2,"breadcrumbs":["3 Model Architecture","6 Results"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":366.19354,"width":58.838707,"height":13.935484}}},"text_order":102,"token_count":2,"style":null}
```

### 6.1 Machine Translation
```bgraph-section
{"id":"3e67e6a4-1932-5610-9a2e-ac250d5a494f","node_type":"Section","location":{"semantic":{"path":"6.8.1","depth":3,"breadcrumbs":["3 Model Architecture","6 Results","6.1 Machine Translation"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":392.5161,"width":114.58064,"height":12.387096}}},"text_order":103,"token_count":5,"style":null}
```

On the WMT 2014 English-to-German translation task, the big transformer model (Transformer (big) in Table 2) outperforms the best previously reported models (including ensembles) by more than 2.0 BLEU, establishing a new state-of-the-art BLEU score of 28.4. The configuration of this model is listed in the bottom line of Table 3. Training took 3.5 days on 8 P100 GPUs. Even our base model surpasses all previously published models and ensembles, at a fraction of the training cost of any of the competitive models.
```bgraph-paragraph
{"id":"8f15ebee-b285-5b85-b578-e6a151f75018","node_type":"Paragraph","location":{"semantic":{"path":"6.8.1.1","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.1 Machine Translation"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":413.41934,"width":401.03226,"height":67.354836}}},"text_order":104,"token_count":128,"style":null}
```

On the WMT 2014 English-to-French translation task, our big model achieves a BLEU score of 41.0, outperforming all of the previously published single models, at less than 1/4 the training cost of the previous state-of-the-art model. The Transformer (big) model trained for English-to-French used dropout rate $P_{drop} = 0.1$, instead of 0.3.
```bgraph-paragraph
{"id":"91329424-c2e7-5e24-9b5a-ee5fc52137b3","node_type":"Paragraph","location":{"semantic":{"path":"6.8.1.2","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.1 Machine Translation"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":484.64514,"width":401.03226,"height":45.677418}}},"text_order":105,"token_count":85,"style":null}
```

For the base models, we used a single model obtained by averaging the last 5 checkpoints, which were written at 10-minute intervals. For the big models, we averaged the last 20 checkpoints. We used beam search with a beam size of 4 and length penalty $\alpha = 0.6$ [38]. These hyperparameters were chosen after experimentation on the development set. We set the maximum output length during inference to input length + 50, but terminate early when possible [38].
```bgraph-paragraph
{"id":"de435778-ec11-5eaf-b53e-a14f8c7e4ea4","node_type":"Paragraph","location":{"semantic":{"path":"6.8.1.3","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.1 Machine Translation"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":534.19354,"width":401.03226,"height":55.741936}}},"text_order":106,"token_count":115,"style":null}
```

Table 2 summarizes our results and compares our translation quality and training costs to other model architectures from the literature. We estimate the number of floating point operations used to train a model by multiplying the training time, the number of GPUs used, and an estimate of the sustained single-precision floating-point capacity of each GPU$^{5}$.
```bgraph-paragraph
{"id":"b0ba6908-7ee4-5f7e-bb44-5557fe6ba3c4","node_type":"Paragraph","location":{"semantic":{"path":"6.8.1.4","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.1 Machine Translation"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":593.80646,"width":401.03226,"height":45.677418}}},"text_order":107,"token_count":90,"style":null}
```

### 6.2 Model Variations
```bgraph-section
{"id":"fb630b63-6b7f-5cc1-9674-68ffa7309617","node_type":"Section","location":{"semantic":{"path":"6.8.2","depth":3,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":652.64514,"width":99.09677,"height":12.387096}}},"text_order":108,"token_count":5,"style":null}
```

To evaluate the importance of different components of the Transformer, we varied our base model in different ways, measuring the change in performance on English-to-German translation on the
```bgraph-paragraph
{"id":"25c50814-9b42-5ba8-8534-3f7a8ba8a3df","node_type":"Paragraph","location":{"semantic":{"path":"6.8.2.1","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":8,"bounding_box":{"x":104.51613,"y":673.54834,"width":401.03226,"height":24.0}}},"text_order":109,"token_count":47,"style":null}
```

$^{5}$We used values of 2.8, 3.7, 6.0 and 9.5 TFLOPS for K80, K40, M40 and P100, respectively.
```bgraph-paragraph
{"id":"3ef7acd5-7718-591c-9d52-4f173d61cf47","node_type":"Paragraph","location":{"semantic":{"path":"6.8.2.2","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":8,"bounding_box":{"x":116.12903,"y":707.61285,"width":338.32257,"height":13.16129}}},"text_order":110,"token_count":23,"style":null}
```

8
```bgraph-footer
{"id":"f2d1b33f-b934-5a9a-9bc6-be353ff6d50a","node_type":"Footer","location":{"semantic":{"path":"6.8.2.3","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":8,"bounding_box":{"x":299.61288,"y":737.80646,"width":10.064516,"height":12.387096}}},"text_order":111,"token_count":1,"style":null}
```

Table 3: Variations on the Transformer architecture. Unlisted values are identical to those of the base model. All metrics are on the English-to-German translation development set, newstest2013. Listed perplexities are per-wordpiece, according to our byte-pair encoding, and should not be compared to per-word perplexities.
```bgraph-paragraph
{"id":"af19922c-762d-58ca-8a5e-5b135d19ce5f","node_type":"Paragraph","location":{"semantic":{"path":"6.8.2.4","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":69.677414,"width":400.25806,"height":44.903225}}},"text_order":112,"token_count":80,"style":null}
```

|   | N | d_model | d_ff | h | d_k | d_v | P_drop | ε_ls | train steps | PPL (dev) | BLEU (dev) | params ×10^6  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  base | 6 | 512 | 2048 | 8 | 64 | 64 | 0.1 | 0.1 | 100K | 4.92 | 25.8 | 65  |
|  (A) |  |  |  | 1 | 512 | 512 |  |  |  | 5.29 | 24.9 |   |
|   |   |  |  | 4 | 128 | 128 |  |  |  | 5.00 | 25.5 |   |
|   |   |  |  | 16 | 32 | 32 |  |  |  | 4.91 | 25.8 |   |
|   |   |  |  | 32 | 16 | 16 |  |  |  | 5.01 | 25.4 |   |
|  (B) |  |  |  |  | 16 |  |  |  |  | 5.16 | 25.1 | 58  |
|   |   |  |  |  | 32 |  |  |  |  | 5.01 | 25.4 | 60  |
|  (C) | 2 |  |  |  |  |  |  |  |  | 6.11 | 23.7 | 36  |
|   |  4 |  |  |  |  |  |  |  |  | 5.19 | 25.3 | 50  |
|   |  8 |  |  |  |  |  |  |  |  | 4.88 | 25.5 | 80  |
|   |   | 256 |  |  | 32 | 32 |  |  |  | 5.75 | 24.5 | 28  |
|   |   | 1024 |  |  | 128 | 128 |  |  |  | 4.66 | 26.0 | 168  |
|   |   |  | 1024 |  |  |  |  |  |  | 5.12 | 25.4 | 53  |
|   |   |  | 4096 |  |  |  |  |  |  | 4.75 | 26.2 | 90  |
|  (D) |  |  |  |  |  |  | 0.0 |  |  | 5.77 | 24.6 |   |
|   |   |  |  |  |  |  | 0.2 |  |  | 4.95 | 25.5 |   |
|   |   |  |  |  |  |  |  | 0.0 |  | 4.67 | 25.3 |   |
|   |   |  |  |  |  |  |  | 0.2 |  | 5.47 | 25.7 |   |
|  (E) | positional embedding instead of sinusoids |   |   |   |   |   |   |   |   | 4.92 | 25.7 |   |
|  big | 6 | 1024 | 4096 | 16 |  |  | 0.3 |  | 300K | **4.33** | **26.4** | 213  |
```bgraph-table
{"id":"b9acbcf8-965e-5061-843f-a3eaa60cd121","node_type":"Table","location":{"semantic":{"path":"6.8.2.5","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":9,"bounding_box":{"x":106.064514,"y":127.74193,"width":404.12903,"height":257.80646}}},"text_order":113,"token_count":360,"style":null}
```

development set, newstest2013. We used beam search as described in the previous section, but no checkpoint averaging. We present these results in Table 3.
```bgraph-paragraph
{"id":"3cb7c1b3-53ca-57a5-96f0-1e4b41dcfaef","node_type":"Paragraph","location":{"semantic":{"path":"6.8.2.6","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":411.09677,"width":400.25806,"height":24.0}}},"text_order":114,"token_count":38,"style":null}
```

In Table 3 rows (A), we vary the number of attention heads and the attention key and value dimensions, keeping the amount of computation constant, as described in Section 3.2.2. While single-head attention is 0.9 BLEU worse than the best setting, quality also drops off with too many heads.
```bgraph-paragraph
{"id":"cc04cb7e-9c70-52e3-87e7-66d9934fec9a","node_type":"Paragraph","location":{"semantic":{"path":"6.8.2.7","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":438.96774,"width":400.25806,"height":34.064514}}},"text_order":115,"token_count":72,"style":null}
```

In Table 3 rows (B), we observe that reducing the attention key size d_k hurts model quality. This suggests that determining compatibility is not easy and that a more sophisticated compatibility function than dot product may be beneficial. We further observe in rows (C) and (D) that, as expected, bigger models are better, and dropout is very helpful in avoiding over-fitting. In row (E) we replace our sinusoidal positional encoding with learned positional embeddings [9], and observe nearly identical results to the base model.
```bgraph-paragraph
{"id":"21240dab-65ee-5501-b2c0-effd2b08f2da","node_type":"Paragraph","location":{"semantic":{"path":"6.8.2.8","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.2 Model Variations"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":476.9032,"width":400.25806,"height":66.58064}}},"text_order":116,"token_count":132,"style":null}
```

### 6.3 English Constituency Parsing
```bgraph-section
{"id":"b749b3c4-ba7f-5349-bcce-9a9f716b9d4c","node_type":"Section","location":{"semantic":{"path":"6.8.3","depth":3,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":9,"bounding_box":{"x":105.29032,"y":558.9677,"width":152.51613,"height":12.387096}}},"text_order":117,"token_count":8,"style":null}
```

To evaluate if the Transformer can generalize to other tasks we performed experiments on English constituency parsing. This task presents specific challenges: the output is subject to strong structural constraints and is significantly longer than the input. Furthermore, RNN sequence-to-sequence models have not been able to attain state-of-the-art results in small-data regimes [37].
```bgraph-paragraph
{"id":"bcee615b-ca59-50e9-8983-2a2e1d92e0a1","node_type":"Paragraph","location":{"semantic":{"path":"6.8.3.1","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":579.871,"width":400.25806,"height":44.903225}}},"text_order":118,"token_count":96,"style":null}
```

We trained a 4-layer transformer with d_model = 1024 on the Wall Street Journal (WSJ) portion of the Penn Treebank [25], about 40K training sentences. We also trained it in a semi-supervised setting, using the larger high-confidence and BerkleyParser corpora from with approximately 17M sentences [37]. We used a vocabulary of 16K tokens for the WSJ only setting and a vocabulary of 32K tokens for the semi-supervised setting.
```bgraph-paragraph
{"id":"27cb9e0b-7094-5b27-a1c6-0ee7ce230e11","node_type":"Paragraph","location":{"semantic":{"path":"6.8.3.2","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":628.64514,"width":400.25806,"height":57.29032}}},"text_order":119,"token_count":106,"style":null}
```

We performed only a small number of experiments to select the dropout, both attention and residual (section 5.4), learning rates and beam size on the Section 22 development set, all other parameters remained unchanged from the English-to-German base translation model. During inference, we
```bgraph-paragraph
{"id":"34eb1b1a-bfab-51f3-bc52-87ec5a560abc","node_type":"Paragraph","location":{"semantic":{"path":"6.8.3.3","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":9,"bounding_box":{"x":104.51613,"y":689.0322,"width":400.25806,"height":34.064514}}},"text_order":120,"token_count":72,"style":null}
```

9
```bgraph-footer
{"id":"0dd8aac6-3ca3-5709-b1fb-62e4d75f1bde","node_type":"Footer","location":{"semantic":{"path":"6.8.3.4","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":9,"bounding_box":{"x":301.9355,"y":741.67737,"width":6.967742,"height":9.290322}}},"text_order":121,"token_count":1,"style":null}
```

Table 4: The Transformer generalizes well to English constituency parsing (Results are on Section 23 of WSJ)
```bgraph-paragraph
{"id":"03a002aa-9fdd-54e3-95f4-3d24d4d54be7","node_type":"Paragraph","location":{"semantic":{"path":"6.8.3.5","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":68.90322,"width":400.25806,"height":22.451612}}},"text_order":122,"token_count":27,"style":null}
```

|  Parser | Training | WSJ 23 F1  |
| --- | --- | --- |
|  Vinyals & Kaiser et al. (2014) [37] | WSJ only, discriminative | 88.3  |
|  Petrov et al. (2006) [29] | WSJ only, discriminative | 90.4  |
|  Zhu et al. (2013) [40] | WSJ only, discriminative | 90.4  |
|  Dyer et al. (2016) [8] | WSJ only, discriminative | 91.7  |
|  Transformer (4 layers) | WSJ only, discriminative | 91.3  |
|  Zhu et al. (2013) [40] | semi-supervised | 91.3  |
|  Huang & Harper (2009) [14] | semi-supervised | 91.3  |
|  McClosky et al. (2006) [26] | semi-supervised | 92.1  |
|  Vinyals & Kaiser et al. (2014) [37] | semi-supervised | 92.1  |
|  Transformer (4 layers) | semi-supervised | 92.7  |
|  Luong et al. (2015) [23] | multi-task | 93.0  |
|  Dyer et al. (2016) [8] | generative | 93.3  |
```bgraph-table
{"id":"74a3adf7-31cf-5e6c-bff0-e5e787d63acb","node_type":"Table","location":{"semantic":{"path":"6.8.3.6","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":10,"bounding_box":{"x":142.45161,"y":92.12903,"width":325.9355,"height":144.77419}}},"text_order":123,"token_count":194,"style":null}
```

increased the maximum output length to input length + 300. We used a beam size of 21 and  \( \alpha = 0.3 \)  for both WSJ only and the semi-supervised setting.
```bgraph-paragraph
{"id":"7386e5a2-6ce3-5b92-81c3-0d40351cdb5a","node_type":"Paragraph","location":{"semantic":{"path":"6.8.3.7","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":259.35483,"width":400.25806,"height":24.0}}},"text_order":124,"token_count":40,"style":null}
```

Our results in Table 4 show that despite the lack of task-specific tuning our model performs surprisingly well, yielding better results than all previously reported models with the exception of the Recurrent Neural Network Grammar [8].
```bgraph-paragraph
{"id":"37d645db-d60f-52dd-8cb4-dd8228d98126","node_type":"Paragraph","location":{"semantic":{"path":"6.8.3.8","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":288.0,"width":401.80643,"height":33.29032}}},"text_order":125,"token_count":58,"style":null}
```

In contrast to RNN sequence-to-sequence models [37], the Transformer outperforms the Berkeley-Parser [29] even when training only on the WSJ training set of 40K sentences.
```bgraph-paragraph
{"id":"93334fd1-895c-5a48-8e76-03dbbd6e6500","node_type":"Paragraph","location":{"semantic":{"path":"6.8.3.9","depth":4,"breadcrumbs":["3 Model Architecture","6 Results","6.3 English Constituency Parsing"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":325.1613,"width":401.80643,"height":24.0}}},"text_order":126,"token_count":42,"style":null}
```

## 7 Conclusion
```bgraph-section
{"id":"6adce5d0-d459-5d28-99b2-7f4efb5a8f19","node_type":"Section","location":{"semantic":{"path":"6.9","depth":2,"breadcrumbs":["3 Model Architecture","7 Conclusion"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":363.09677,"width":80.51613,"height":12.387096}}},"text_order":127,"token_count":3,"style":null}
```

In this work, we presented the Transformer, the first sequence transduction model based entirely on attention, replacing the recurrent layers most commonly used in encoder-decoder architectures with multi-headed self-attention.
```bgraph-paragraph
{"id":"70d816fa-568f-5e11-b323-7a9ca257559f","node_type":"Paragraph","location":{"semantic":{"path":"6.9.1","depth":3,"breadcrumbs":["3 Model Architecture","7 Conclusion"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":387.09677,"width":400.25806,"height":34.064514}}},"text_order":128,"token_count":56,"style":null}
```

For translation tasks, the Transformer can be trained significantly faster than architectures based on recurrent or convolutional layers. On both WMT 2014 English-to-German and WMT 2014 English-to-French translation tasks, we achieve a new state of the art. In the former task our best model outperforms even all previously reported ensembles.
```bgraph-paragraph
{"id":"05ced6db-09b8-5aa0-8ab5-9fe4980a106a","node_type":"Paragraph","location":{"semantic":{"path":"6.9.2","depth":3,"breadcrumbs":["3 Model Architecture","7 Conclusion"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":425.80643,"width":400.25806,"height":44.129032}}},"text_order":129,"token_count":85,"style":null}
```

We are excited about the future of attention-based models and plan to apply them to other tasks. We plan to extend the Transformer to problems involving input and output modalities other than text and to investigate local, restricted attention mechanisms to efficiently handle large inputs and outputs such as images, audio and video. Making generation less sequential is another research goals of ours.
```bgraph-paragraph
{"id":"274c185a-0b5f-56a2-be41-96db21c2fb36","node_type":"Paragraph","location":{"semantic":{"path":"6.9.3","depth":3,"breadcrumbs":["3 Model Architecture","7 Conclusion"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":473.80643,"width":400.25806,"height":45.677418}}},"text_order":130,"token_count":100,"style":null}
```

The code we used to train and evaluate our models is available at https://github.com/tensorflow/tensor2tensor.
```bgraph-paragraph
{"id":"a48ba3a6-ddd1-5251-a833-6231e001613e","node_type":"Paragraph","location":{"semantic":{"path":"6.9.4","depth":3,"breadcrumbs":["3 Model Architecture","7 Conclusion"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":524.129,"width":400.25806,"height":22.451612}}},"text_order":131,"token_count":27,"style":null}
```

Acknowledgements We are grateful to Nal Kalchbrenner and Stephan Gouws for their fruitful comments, corrections and inspiration.
```bgraph-paragraph
{"id":"641ca444-4554-5682-aeed-e6715c63c782","node_type":"Paragraph","location":{"semantic":{"path":"6.9.5","depth":3,"breadcrumbs":["3 Model Architecture","7 Conclusion"]},"physical":{"page":10,"bounding_box":{"x":104.51613,"y":557.4193,"width":400.25806,"height":23.225805}}},"text_order":132,"token_count":32,"style":null}
```

## References
```bgraph-section
{"id":"a3d8a9cc-48ff-5908-8214-9fe5c69cce11","node_type":"Section","location":{"semantic":{"path":"6.10","depth":2,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":10,"bounding_box":{"x":106.064514,"y":594.5806,"width":58.064514,"height":12.387096}}},"text_order":133,"token_count":2,"style":null}
```

[1] Jimmy Lei Ba, Jamie Ryan Kiros, and Geoffrey E Hinton. Layer normalization. arXiv preprint arXiv:1607.06450, 2016.
```bgraph-paragraph
{"id":"0aa5902f-2d70-59da-bf97-eb9ea8112ef1","node_type":"Paragraph","location":{"semantic":{"path":"6.10.1","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":10,"bounding_box":{"x":109.93548,"y":613.16125,"width":394.83868,"height":21.677418}}},"text_order":134,"token_count":29,"style":null}
```

[2] Dzmitry Bahdanau, Kyunghyun Cho, and Yoshua Bengio. Neural machine translation by jointly learning to align and translate. CoRR, abs/1409.0473, 2014.
```bgraph-paragraph
{"id":"17b4c4aa-01f9-5aff-9776-e6b8ecf1dbab","node_type":"Paragraph","location":{"semantic":{"path":"6.10.2","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":10,"bounding_box":{"x":109.93548,"y":641.80646,"width":394.83868,"height":23.225805}}},"text_order":135,"token_count":38,"style":null}
```

[3] Denny Britz, Anna Goldie, Minh-Thang Luong, and Quoc V. Le. Massive exploration of neural machine translation architectures. CoRR, abs/1703.03906, 2017.
```bgraph-paragraph
{"id":"47038df7-f3fa-579e-b337-e108e882360b","node_type":"Paragraph","location":{"semantic":{"path":"6.10.3","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":10,"bounding_box":{"x":109.93548,"y":670.4516,"width":394.83868,"height":23.225805}}},"text_order":136,"token_count":39,"style":null}
```

[4] Jianpeng Cheng, Li Dong, and Mirella Lapata. Long short-term memory-networks for machine reading. arXiv preprint arXiv:1601.06733, 2016.
```bgraph-paragraph
{"id":"a7ea4f49-f999-5203-ae5f-7183f64aba81","node_type":"Paragraph","location":{"semantic":{"path":"6.10.4","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":10,"bounding_box":{"x":109.93548,"y":699.09674,"width":394.83868,"height":24.0}}},"text_order":137,"token_count":35,"style":null}
```

10
```bgraph-footer
{"id":"5d794046-e681-5361-b07b-5e0b5453477d","node_type":"Footer","location":{"semantic":{"path":"6.10.5","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":10,"bounding_box":{"x":300.3871,"y":741.67737,"width":10.838709,"height":9.290322}}},"text_order":138,"token_count":1,"style":null}
```

[5] Kyunghyun Cho, Bart van Merrienboer, Caglar Gulcehre, Fethi Bougares, Holger Schwenk, and Yoshua Bengio. Learning phrase representations using rnn encoder-decoder for statistical machine translation. CoRR, abs/1406.1078, 2014.
[6] Francois Chollet. Xception: Deep learning with depthwise separable convolutions. arXiv preprint arXiv:1610.02357, 2016.
[7] Junyoung Chung, Caglar Gulcehre, Kyunghyun Cho, and Yoshua Bengio. Empirical evaluation of gated recurrent neural networks on sequence modeling. CoRR, abs/1412.3555, 2014.
[8] Chris Dyer, Adhiguna Kuncoro, Miguel Ballesteros, and Noah A. Smith. Recurrent neural network grammars. In Proc. of NAACL, 2016.
[9] Jonas Gehring, Michael Auli, David Grangier, Denis Yarats, and Yann N. Dauphin. Convolutional sequence to sequence learning. arXiv preprint arXiv:1705.03122v2, 2017.
[10] Alex Graves. Generating sequences with recurrent neural networks. arXiv preprint arXiv:1308.0850, 2013.
[11] Kaiming He, Xiangyu Zhang, Shaoqing Ren, and Jian Sun. Deep residual learning for image recognition. In Proceedings of the IEEE Conference on Computer Vision and Pattern Recognition, pages 770–778, 2016.
[12] Sepp Hochreiter, Yoshua Bengio, Paolo Frasconi, and Jürgen Schmidhuber. Gradient flow in recurrent nets: the difficulty of learning long-term dependencies, 2001.
[13] Sepp Hochreiter and Jürgen Schmidhuber. Long short-term memory. Neural computation, 9(8):1735–1780, 1997.
[14] Zhongqiang Huang and Mary Harper. Self-training PCFG grammars with latent annotations across languages. In Proceedings of the 2009 Conference on Empirical Methods in Natural Language Processing, pages 832–841. ACL, August 2009.
[15] Rafal Jozefowicz, Oriol Vinyals, Mike Schuster, Noam Shazeer, and Yonghui Wu. Exploring the limits of language modeling. arXiv preprint arXiv:1602.02410, 2016.
[16] Łukasz Kaiser and Samy Bengio. Can active memory replace attention? In Advances in Neural Information Processing Systems, (NIPS), 2016.
[17] Łukasz Kaiser and Ilya Sutskever. Neural GPUs learn algorithms. In International Conference on Learning Representations (ICLR), 2016.
[18] Nal Kalchbrenner, Lasse Espeholt, Karen Simonyan, Aaron van den Oord, Alex Graves, and Koray Kavukcuoglu. Neural machine translation in linear time. arXiv preprint arXiv:1610.10099v2, 2017.
[19] Yoon Kim, Carl Denton, Luong Hoang, and Alexander M. Rush. Structured attention networks. In International Conference on Learning Representations, 2017.
[20] Diederik Kingma and Jimmy Ba. Adam: A method for stochastic optimization. In ICLR, 2015.
[21] Oleksii Kuchaiev and Boris Ginsburg. Factorization tricks for LSTM networks. arXiv preprint arXiv:1703.10722, 2017.
[22] Zhouhan Lin, Minwei Feng, Cícero Nogueira dos Santos, Mo Yu, Bing Xiang, Bowen Zhou, and Yoshua Bengio. A structured self-attentive sentence embedding. arXiv preprint arXiv:1703.03130, 2017.
[23] Minh-Thang Luong, Quoc V. Le, Ilya Sutskever, Oriol Vinyals, and Lukasz Kaiser. Multi-task sequence to sequence learning. arXiv preprint arXiv:1511.06114, 2015.
[24] Minh-Thang Luong, Hieu Pham, and Christopher D Manning. Effective approaches to attention-based neural machine translation. arXiv preprint arXiv:1508.04025, 2015.
```bgraph-paragraph
{"id":"5ea77ff4-2ea9-5a0e-9d32-fd26068251a6","node_type":"Paragraph","location":{"semantic":{"path":"6.10.6","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":11,"bounding_box":{"x":105.29032,"y":72.0,"width":400.25806,"height":651.09674}}},"text_order":139,"token_count":804,"style":null}
```

11
```bgraph-footer
{"id":"6348e249-b7b5-59cb-8db5-7de5dc7914db","node_type":"Footer","location":{"semantic":{"path":"6.10.7","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":11,"bounding_box":{"x":300.3871,"y":741.67737,"width":10.064516,"height":9.290322}}},"text_order":140,"token_count":1,"style":null}
```

[25] Mitchell P Marcus, Mary Ann Marcinkiewicz, and Beatrice Santorini. Building a large annotated corpus of english: The penn treebank. *Computational linguistics*, 19(2):313–330, 1993.[26] David McClosky, Eugene Charniak, and Mark Johnson. Effective self-training for parsing. In *Proceedings of the Human Language Technology Conference of the NAACL, Main Conference*, pages 152–159. ACL, June 2006.[27] Ankur Parikh, Oscar Täckström, Dipanjan Das, and Jakob Uszkoreit. A decomposable attention model. In *Empirical Methods in Natural Language Processing*, 2016.[28] Romain Paulus, Caiming Xiong, and Richard Socher. A deep reinforced model for abstractive summarization. *arXiv preprint arXiv:1705.04304*, 2017.[29] Slav Petrov, Leon Barrett, Romain Thibaux, and Dan Klein. Learning accurate, compact, and interpretable tree annotation. In *Proceedings of the 21st International Conference on Computational Linguistics and 44th Annual Meeting of the ACL*, pages 433–440. ACL, July 2006.[30] Ofir Press and Lior Wolf. Using the output embedding to improve language models. *arXiv preprint arXiv:1608.05859*, 2016.[31] Rico Sennrich, Barry Haddow, and Alexandra Birch. Neural machine translation of rare words with subword units. *arXiv preprint arXiv:1508.07909*, 2015.[32] Noam Shazeer, Azalia Mirhoseini, Krzysztof Maziarz, Andy Davis, Quoc Le, Geoffrey Hinton, and Jeff Dean. Outrageously large neural networks: The sparsely-gated mixture-of-experts layer. *arXiv preprint arXiv:1701.06538*, 2017.[33] Nitish Srivastava, Geoffrey E Hinton, Alex Krizhevsky, Ilya Sutskever, and Ruslan Salakhutdinov. Dropout: a simple way to prevent neural networks from overfitting. *Journal of Machine Learning Research*, 15(1):1929–1958, 2014.[34] Sainbayar Sukhbaatar, Arthur Szlam, Jason Weston, and Rob Fergus. End-to-end memory networks. In C. Cortes, N. D. Lawrence, D. D. Lee, M. Sugiyama, and R. Garnett, editors, *Advances in Neural Information Processing Systems 28*, pages 2440–2448. Curran Associates, Inc., 2015.[35] Ilya Sutskever, Oriol Vinyals, and Quoc VV Le. Sequence to sequence learning with neural networks. In *Advances in Neural Information Processing Systems*, pages 3104–3112, 2014.[36] Christian Szegedy, Vincent Vanhoucke, Sergey Ioffe, Jonathon Shlens, and Zbigniew Wojna. Rethinking the inception architecture for computer vision. *CoRR*, abs/1512.00567, 2015.[37] Vinyals & Kaiser, Koo, Petrov, Sutskever, and Hinton. Grammar as a foreign language. In *Advances in Neural Information Processing Systems*, 2015.[38] Yonghui Wu, Mike Schuster, Zhifeng Chen, Quoc V Le, Mohammad Norouzi, Wolfgang Macherey, Maxim Krikun, Yuan Cao, Qin Gao, Klaus Macherey, et al. Google's neural machine translation system: Bridging the gap between human and machine translation. *arXiv preprint arXiv:1609.08144*, 2016.[39] Jie Zhou, Ying Cao, Xuguang Wang, Peng Li, and Wei Xu. Deep recurrent models with fast-forward connections for neural machine translation. *CoRR*, abs/1606.04199, 2016.[40] Muhua Zhu, Yue Zhang, Wenliang Chen, Min Zhang, and Jingbo Zhu. Fast and accurate shift-reduce constituent parsing. In *Proceedings of the 51st Annual Meeting of the ACL (Volume 1: Long Papers)*, pages 434–443. ACL, August 2013.
```bgraph-list
{"id":"a278e809-9716-57a8-a49f-26d59e12db6f","node_type":"List","location":{"semantic":{"path":"6.10.8","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":12,"bounding_box":{"x":104.51613,"y":70.451614,"width":401.03226,"height":648.0}}},"text_order":141,"token_count":810,"style":null}
```

12
```bgraph-footer
{"id":"41a9c075-4bed-5c59-8cd0-417ba94bef43","node_type":"Footer","location":{"semantic":{"path":"6.10.9","depth":3,"breadcrumbs":["3 Model Architecture","References"]},"physical":{"page":12,"bounding_box":{"x":298.0645,"y":737.80646,"width":13.935484,"height":12.387096}}},"text_order":142,"token_count":1,"style":null}
```

## Attention Visualizations
```bgraph-section
{"id":"db514fdb-2244-5fed-93a3-61dc47b16b90","node_type":"Section","location":{"semantic":{"path":"6.11","depth":2,"breadcrumbs":["3 Model Architecture","Attention Visualizations"]},"physical":{"page":13,"bounding_box":{"x":104.51613,"y":68.90322,"width":126.967735,"height":13.16129}}},"text_order":143,"token_count":6,"style":null}
```

Figure 3: An example of the attention mechanism following long-distance dependencies in the encoder self-attention in layer 5 of 6. Many of the attention heads attend to a distant dependency of the verb 'making', completing the phrase 'making...more difficult'. Attentions here shown only for the word 'making'. Different colors represent different heads. Best viewed in color.
```bgraph-paragraph
{"id":"c2d25a91-5578-548a-a8b9-2453aca93dcb","node_type":"Paragraph","location":{"semantic":{"path":"6.11.1","depth":3,"breadcrumbs":["3 Model Architecture","Attention Visualizations"]},"physical":{"page":13,"bounding_box":{"x":104.51613,"y":308.12903,"width":401.03226,"height":45.677418}}},"text_order":144,"token_count":94,"style":null}
```

13
```bgraph-footer
{"id":"0a6e3775-1455-5e33-b504-62e3e144ad5f","node_type":"Footer","location":{"semantic":{"path":"6.11.2","depth":3,"breadcrumbs":["3 Model Architecture","Attention Visualizations"]},"physical":{"page":13,"bounding_box":{"x":297.2903,"y":738.5806,"width":14.709677,"height":12.387096}}},"text_order":145,"token_count":1,"style":null}
```

Figure 4: Two attention heads, also in layer 5 of 6, apparently involved in anaphora resolution. Top: Full attentions for head 5. Bottom: Isolated attentions from just the word 'its' for attention heads 5 and 6. Note that the attentions are very sharp for this word.
```bgraph-paragraph
{"id":"8bb73569-1f70-524c-8f0f-d3b38e0e57d2","node_type":"Paragraph","location":{"semantic":{"path":"6.11.3","depth":3,"breadcrumbs":["3 Model Architecture","Attention Visualizations"]},"physical":{"page":14,"bounding_box":{"x":103.741936,"y":609.2903,"width":402.58063,"height":35.612904}}},"text_order":146,"token_count":66,"style":null}
```

14
```bgraph-footer
{"id":"13185f18-2a49-5442-91c4-8b7202dfc097","node_type":"Footer","location":{"semantic":{"path":"6.11.4","depth":3,"breadcrumbs":["3 Model Architecture","Attention Visualizations"]},"physical":{"page":14,"bounding_box":{"x":296.5161,"y":738.5806,"width":15.4838705,"height":12.387096}}},"text_order":147,"token_count":1,"style":null}
```

Figure 5: Many of the attention heads exhibit behaviour that seems related to the structure of the sentence. We give two such examples above, from two different heads from the encoder self-attention at layer 5 of 6. The heads clearly learned to perform different tasks.
```bgraph-paragraph
{"id":"0339f18f-31d2-535e-a059-ca177ac49430","node_type":"Paragraph","location":{"semantic":{"path":"6.11.5","depth":3,"breadcrumbs":["3 Model Architecture","Attention Visualizations"]},"physical":{"page":15,"bounding_box":{"x":104.51613,"y":598.4516,"width":401.80643,"height":34.838707}}},"text_order":148,"token_count":67,"style":null}
```

15
```bgraph-footer
{"id":"a2d770e9-e52a-5e54-beb8-abd2466550af","node_type":"Footer","location":{"semantic":{"path":"6.11.6","depth":3,"breadcrumbs":["3 Model Architecture","Attention Visualizations"]},"physical":{"page":15,"bounding_box":{"x":296.5161,"y":737.80646,"width":15.4838705,"height":13.16129}}},"text_order":149,"token_count":1,"style":null}
```
