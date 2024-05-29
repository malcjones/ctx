# ctx: Code conTeXt

ChatGPT, Claude, Perplexity, etc. are useful for programming.
*If*, you provide them the right context, they can find discrepancies in code, and suggest improvements.
I like that style of AI pair programming, where prompting the AI gives you ideas at things to look at, rather
than often poorly generated code.

This program walks a directory and prints the contents of the source files along with their filenames,
so you can prompt the AI with the code and it's location, in a format that mirrors documentation (which often is used to train LLMs).

# usage

```bash
# code in ./src, file ext. 'rs'
ctx --path ./src --ext rs
# // src/main.rs
# fn main() {
#    println!("Hello, world!");
# }
```

# couldn't you just
- use `find ./src -name "*.rs" -exec cat {} \;`?
- some convoluted tool to import your repo into a poorly documented VS Code instance running in a web browser?

This is faster, and I needed to try `walkdir`.

# credits
- [walkdir](https://github.com/BurntSushi/walkdir) - [Unlicense](https://unlicense.org/)
- [clap](https://github.com/clap-rs/clap) - [MIT](https://opensource.org/licenses/MIT)