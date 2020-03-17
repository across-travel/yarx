# Yarx

**Yarx** : The fastest way for building static website !
>  All projects's website in Kayrx written in **Yarx**.

## 直接使用

```rust
git clone https://github.com/kayrx/yarx.git
cd yarx

cargo build --bin watch
cargo build --bin run
```

```shell
watch run docs target/dist --ext .html
```

## Watch

**watch** is a tool for recursively watching the current working directory and running a command when its contents change.

It's integrated with Git, so it won't rerun the command if an ignored file changes.

**Usage**

```shell
watch <command> [<arg>]...
```

## Run

**run** is a tool for processing a directory of text files. It allows you to define files that include other files, and substitute variables.

The primary use-case is to make building pure HTML sites a little bit easier.

**Features**

* Process a directory of files
* Include files in other files
* Set variables to arbitrary values
* Include environment and scoped variable values in files

**Usage**

```bash
run <input-directory> <output-directory> [--ext <extension>]...
```

**Example**

```bash
run src target/dist --ext .html --ext .css
```

Start will walk the input directory and process all files.

* If a file begins with `_` it will be skipped.
* If a file ends in `.html` or `.css`, it will be processed and written to the output directory.
* If a file doesn't, it will be copied verbatim to the output directory.

Note that the directory structure is perserved.

### Directives

Start's features are provided via *directives* which are simple statements in your files.  Directives are enclosed in `[]` -- e.g. `[include raw my-file.html]`. A directive is a space separated list of arguments.

> To simplify its behavior, Start does not trim white-space. If a directive cannot be parsed, the program exits with a failure.

#### Variable substitution

You can use the `var` or `opt` directives to substitute environment variables into the file. `var` indicates that the variable is required, while `opt` indicates it is optional.

If a variable isn't defined, and `opt` is used, the directive will evaluate to the empty string.

```
var <format> <variable>
```

```
opt <format> <variable>
```

Format may be `html` or `raw`. If `html`, it will be escaped for use in an HTML document. If `raw`, it will be substituted directly.

*When using `raw`, be sure that you're not subjecting yourself to XSS attacks.*

```html
<p>Hello [var html MY_VAR]</p>
```

#### Set

You can set a variable for use in the current file or included ones.

```
set <name> <value>
```

```html
[set name John]
[include raw _template.html]
```

### Stash

Stash will take all of the current evaluated content and place it into the specified variable. Content after the stash directive is excluded. This is useful for defining some content in a file, and then evaluating it in the context of a template that renders variables. Also known as the decorator pattern.

```
stash <variable>
```

```html
<p>This is my content</p>

[stash content][require _layout.html]
```

### Includes

You can include files in other files. If a file includes a file that includes itself, that include will be ignored to break the cycle.

File paths are relative to the file that is being processed.

If the file doesn't exist, the directive will evaluate to the empty string.

```
include <format> [path]
```

Format may be `html` or `raw`. If `html`, it will be escaped for use in an HTML document. If `raw`, it will be substituted directly.

*When using `raw`, be sure that you're not subjecting yourself to XSS attacks.*

```html
<pre>[include raw /etc/passwd]</pre>
```

### Requires

You can require files in other files. If a file includes a file that includes itself, the program will exit with a failure.

File paths are relative to the file that is being processed.

If the file doesn't exist, the program will exit with a failure.

```
require <format> [path]
```

Format may be `html` or `raw`. If `html`, it will be escaped for use in an HTML document. If `raw`, it will be substituted directly.

*When using `raw`, be sure that you're not subjecting yourself to XSS attacks.*

```html
<pre>[require raw /etc/passwd]</pre>
```