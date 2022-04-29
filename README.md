# zb: a zip blog

This is my approach at creating a self-contained website. The whole site will be embedded as a Zip archive right in the server binary file, so it is probably the most portable website you'll ever have.

One of `zb`'s unique features it that it will render HTML, Markdown and Org-Mode files, so if you prefer to write `.org` files instead of HTML, `zb` has you covered.

## Motivation

I first saw the concept of *single binary websites* in [Ted Unangst's blog](https://flak.tedunangst.com/post/the-three-line-single-binary-compiler-free-blog) where he complained that Go couldn't do this just as easily, because of its broken support for embedded Zip files. As I play with both Go and Rust every now and then, I thought "how hard could it be to do it in Rust?".

Quite surprisingly (for me), it wasn't really hard at all.

## How do I use this?

You'll need a Zip file, containing (at least) one of the following files in the root path:

* `index.htm`
* `index.html`
* `index.md`
* `index.org`

If you prefer to use a different file name, you can pass `--defaultpage name.ext` when starting the application.

Everything else in that Zip file can be any file of your choice. Relative references between them will work. (If they don't, please file a bug.)

### First steps...

* Fetch the latest code with Fossil ...
  
    fossil clone https://code.rosaelefanten.org/zb
    cd zb

* ... or Git:
  
    fossil clone https://github.com/dertuxmalwieder/zb
    cd zb

* Install Rust, then build `zb` with `cargo`:
  
    cargo build --release

### On Unix and Unix-like systems

Now, concatenate your `zb` binary with your Zip file. Assuming your Zip file is named `index.zip`:

    % cat ./target/release/zb index.zip > ./zb

### On Windows

Now, concatenate your `zb.exe` with your Zip file. Assuming your Zip file is named `index.zip`:

    # PowerShell
    cmd /c copy /b .\target\release\zb.exe+.\index.zip .\zb.exe
    
    # cmd
    copy /b .\target\release\zb.exe+.\index.zip .\zb.exe

### Once done ...

Run the concatenated `zb` binary and your website will be delivered from port 8000. (You can change that: `zb --port 8081` would make it run on port 8081 instead.) From now on, every time you want to update your website, just create a new Zip file and repeat the concatenation. (It should be easy to automatize that task.)

#### Automatic converted HTML routing

It is important to note that the file extensions for `.org`, `.md` and HTML files are optional, so the file `stuff/demofile.md` can be reached over `localhost:8000/stuff/demofile` as well.

The search order for all files is:

1. `/[path]`
2. `/[path].md`
3. `/[path].org`
4. `/[path].htm`
5. `/[path].html`

## How to contribute code

1. Read and agree to the [Code of ~~Conduct~~ Merit](CODE_OF_CONDUCT.md).
2. Implicitly agree to the [LICENSE](LICENSE). Nobody reads those. I don't either.
3. Find out if anyone has filed a GitHub Issue or even sent a Pull Request yet. Act accordingly.
4. Send me a patch, either via e-mail (`git at tuxproject dot de`) or as a GitHub Pull Request. Note that GitHub only provides a mirror, so you'd double my work if you choose the latter. :-)

If you do that well (and regularly) enough, I'll probably grant you commit access to the upstream Fossil repository.

## Donations

Writing this software and keeping it available is eating some of the time which most people would spend with their friends. Naturally, I absolutely accept financial compensation.

* PayPal: [GebtmireuerGeld](https://paypal.me/gebtmireuergeld)
* Liberapay: [Cthulhux](https://liberapay.com/Cthulhux/donate)

Thank you.

## Contact

* Mastodon: [@tux0r](https://layer8.space/@tux0r)
* Twitter: [@tux0r](https://twitter.com/tux0r)