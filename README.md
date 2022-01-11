# Summary

Rust function template for OpenFaas, building on the work of Booyaa (see below), and adding:

* Tokio for async handlers
* JSON handling for arbitrary request/response objects
* A few build process changes in the Docker file
* Catch panic unwinds and return a formatted JSON message

# Provenance

This is based on the work in this repo: https://github.com/booyaa/openfaas-rust-template

# Roadmap

* I'd like to migrate this to `of-watchdog`. If I can do that without changing the handler API, 
I'll probably go ahead and change this template. If you don't have an appetite for that sort 
of thing, feel free to fork this repo. It's probably coming. 

* I think it'd be useful to corral HTTP headers and the like into an event context to pass into
the function. That might be a breaking api change, though should require only a change to the
handler function signature.


# Usage

This is not an "official" template, so you have to pull it by URL.

```shell
faas-cli template pull https://github.com/dspadea/openfaas-rust
```

If you already have a template called "rust", you can add `--overwrite`, but be warned that it will
clobber the rust template you currently have in your environment. (Truth in advertising, the flag is
`--overwrite`.)

```shell
faas-cli template pull --overwrite https://github.com/dspadea/openfaas-rust
```

The `template pull` command doesn't seem to support pulling with a new name, which is unfortunate.
You could also pull the template to a different location, move it into your `templates/` directory with
a different name, and refer to the new name in your `template.yml` file.


# Questions

## Why a fork?

First of all, this fundamentally (and incompatibly) changes how the template operates, as well 
as the intention of the original project. 

The original template was for demonstration purposes for Booyaa's blog, and was relatively simple,
functionality-wise.  It accepts a string and sends it to the function, which is 
great when trying to illustrate the data flow. 

For real use, I want something a little more involved. Real use being at odds with the purpose 
of Booyaa's repo, I figured it best just to go ahead and make my own. If he thinks it makes sense to 
re-merge the projects, I'm happy to talk about that. 

## Ok, Why Async Handlers?

Functions only execute one at a time, so why are handlers async?

1. Because there's a large part of the Rust ecosystem which is async-first in their API design, and 
it's very annoying to use when the function is not in an async context. If you don't need async, 
there's not much of a penalty, but if you do, this makes it much easier. 

2. Because sometimes you want to execute calls in the function concurrently. Spawning tokio tasks
to collect data from backend REST services, for example, can make your function execute significantly
faster than running your backend requests serially. 

On the balance, I think this makes it more flexible and less annoying to use. 

## License

MIT License
