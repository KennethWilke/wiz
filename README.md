# Wiz

Wiz is a tool that aims to simplify the process of experimenting with digital logic design.

But right now it is a very early hot mess of a repo, so good luck!

# General usage

Wiz isn't officially distributed yet, and won't for a bit longer. If you're here anyway and interested in using it, feel free to open a GitHub issue to say hi or ask a question!

Generally if you want to use it, clone down the repo and give it a `cargo install --path .` and it'll install the `wiz` command for you. It'll also pretty much expect you have `vivado` installed and in your `$PATH`.

You'll then want to make a new folder and in there add your source files and create a `wiz.toml` and put the right stuff in there. Then cross your fingers and run `wiz build` and if your project sucessfully builds maybe `wiz program` will even flash it on to your FPGA board. After all, it works on my box!

In all seriousness, I will revisit this documentation if/when the project gets a bit more useful and stable and if folks have interest in it.
