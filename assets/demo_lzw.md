---
title: "Lempel–Ziv–Welch current implementation"
sub_title: (In this project)
author: JoyousOne
theme:
 path: ./theme.yaml
---

Encoding
:
===

Lets say we have the following string: `AABABCCABC`

---

<!-- pause -->

In order to encode the string here is how we must proceed:

<!-- pause -->

<!-- column_layout: [1, 1] -->

<!-- column: 1 -->

Instanciate a dictionnary with all the different unique char in order that they can be found:

<!-- alignment: center -->

**index** | **codeword** 

 0          A

 1          B

 2          C

<!-- pause -->
<!-- column: 0 -->
<!-- alignment: left -->

Then from our string:
`AABABCCABC`

Find the longest codeword in the dictionnary:

<!-- alignment: center -->
<!-- pause -->
- <span style="color: red">A</span>ABABCCABC, index: <span style="color: yellow">0</span>
<!-- pause -->

<!-- alignment: left -->
Concatene the codeword and the next value to the dictionnary.

<!-- column: 1 -->
<!-- alignment: center -->
 3         AA

<!-- pause -->
<!-- column: 0 -->
<!-- alignment: left -->
Repeat for the remaining symbols:
<!-- alignment: center -->
<!-- pause -->

- A<span style="color: red">A</span>BABCCABC, index: <span style="color: yellow">0</span>
<!-- column: 1 -->
 4         AB
<!-- column: 0 -->
<!-- pause -->

- AA<span style="color: red">B</span>ABCCABC, index: <span style="color: yellow">1</span>
<!-- column: 1 -->
 5         BA
<!-- column: 0 -->
<!-- pause -->


- AAB<span style="color: red">AB</span>CCABC, index: <span style="color: yellow">4</span>
<!-- column: 1 -->
 6        ABC
<!-- column: 0 -->
<!-- pause -->

- AABAB<span style="color: red">C</span>CABC, index: <span style="color: yellow">2</span>
<!-- column: 1 -->
 7         CC
<!-- column: 0 -->
<!-- pause -->

- AABABC<span style="color: red">C</span>ABC, index: <span style="color: yellow">2</span>
<!-- column: 1 -->
 8         CA
<!-- column: 0 -->
<!-- pause -->

- AABABCC<span style="color: red">ABC</span>, index: <span style="color: yellow">6</span>

<!-- pause -->
<!-- alignment: left -->
 Done!
<!-- pause -->

<!-- reset_layout -->
<!-- alignment: center -->


We now have the encoded value <span style="color: yellow">0014226</span>.

```latex +render
$\text{compression ratio} = \frac{\text{uncompressed size}}{\text{compressed size}} = \frac{10}{7} \approx 1.43:1$
```

<!-- end_slide -->


Decoding
===

Let's decode `0014226` back to it original value.

---

<!-- pause -->

<!-- column_layout: [1, 1] -->

<!-- column: 1 -->

Instanciate a dictionnary from the same previous unique symbol:

> NOTE: Symbols in <span style="color: red">red</span> indicate the previous codeword to which the current symbol is added.

<!-- alignment: center -->

**index** | **codeword** 

 0          A

 1          B

 2          C

<!-- pause -->
<!-- column: 0 -->

<!-- alignment: left -->
Then from: `0014226`

Write the value in the index:
<!-- alignment: center -->
<!-- pause -->
- <span style="color: yellow">0</span>014226, codeword: <span style="color: green">A</span>
<!-- pause -->

<!-- alignment: left -->
If we have a previous codeword, we concatene it with the first symbol of the current codeword and add it to the dictionnary.


<!-- pause -->
Repeat for the remaining indexes:

<!-- alignment: center -->
<!-- pause -->

- 0<span style="color: yellow">0</span>14226, codeword: <span style="color: green">A</span>
<!-- column: 1 -->
 3         <span style="color: red">A</span>A
<!-- column: 0 -->
<!-- pause -->

- 00<span style="color: yellow">1</span>4226, codeword: <span style="color: green">B</span>
<!-- column: 1 -->
 4         <span style="color: red">A</span>B
<!-- column: 0 -->
<!-- pause -->


- 001<span style="color: yellow">4</span>226, codeword: <span style="color: green">AB</span>
<!-- column: 1 -->
 5         <span style="color: red">B</span>A
<!-- column: 0 -->
<!-- pause -->

- 0014<span style="color: yellow">2</span>26, codeword: <span style="color: green">C</span>
<!-- column: 1 -->
 6        <span style="color: red">AB</span>C
<!-- column: 0 -->
<!-- pause -->

- 00142<span style="color: yellow">2</span>6, codeword: <span style="color: green">C</span>
<!-- column: 1 -->
 7         <span style="color: red">C</span>C
<!-- column: 0 -->
<!-- pause -->

- 001422<span style="color: yellow">6</span>, codeword: <span style="color: green">ABC</span>

<!-- pause -->
<!-- alignment: left -->
 Done!
<!-- pause -->

<!-- reset_layout -->
<!-- alignment: center -->


We now have the decoded value `AABABCCABC`.


<!-- end_slide -->

Encoding with metadatas
===

<!-- skip_slide -->

<!-- end_slide -->

Decoding with metadadas
===

<!-- skip_slide -->

<!-- end_slide -->

<!-- jump_to_middle -->

The end
🎊
---

<!-- no_footer -->
