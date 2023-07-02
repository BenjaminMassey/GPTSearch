# GPTSearch

## About

This application seeks to answer your question by having ChatGPT parse your Google search results for you.

It searches your question, goes to the first number of URLs from that result, scrapes the paragraphs
from those webpages, and then asks ChatGPT your question with that web data as context information.

## Example

Here is an example of a question that is too new for ChatGPT:

![ChatGPT is too old](https://i.imgur.com/VXnCXyw.png)

And here it is being answered correctly, since GPTSearch has access to Google search results:

![GPTSearch is up to date](https://i.imgur.com/PINMEQ3.png)

It can also help ChatGPT results to be more concise.

Here, ChatGPT does not infer the necessary context:

![ChatGPT is confused](https://i.imgur.com/fPbhpEh.jpg)

Whereas GPTSearch inherently understands context such as who is most relevant because of Google:

![GPTSearch is concise](https://i.imgur.com/YEseqBK.png)

## Usage

There should not be any special setup other than standard Cargo Rust tools.

The runtime parameters are your search query and an appropriate OpenAI API key.

So run with a command similar to the following:

```
cargo run "What temperature should I cook chicken until?" Sk-abcdefghijklmnopqrstuvwxyz
```

## Contact

Feel free to contact me at benjamin.w.massey@gmail.com with any questions / inquiries.