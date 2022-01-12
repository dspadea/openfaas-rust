
# Summary

Rust function template for OpenFaas, building on the work of Booyaa (see below), and adding:

* Tokio for async handlers
* JSON handling for arbitrary request/response objects
* A few build process changes in the Docker file

I think this is a good interim solution to easily get Rust functions with async and serde built in, 
but I think this could be a lot better. There's a fair chance I'll wind up re-writing this (see roadmap), 
so if this becomes important to you in any way, make sure you're forking it on Github or checking it into
your project repo. 


# Provenance

This is based on the work in this repo: https://github.com/booyaa/openfaas-rust-template

# Roadmap

* I'd like to migrate this to `of-watchdog`. If I can do that without changing the handler API, 
I'll probably go ahead and change this template. If you don't have an appetite for that sort 
of thing, feel free to fork this repo. It's probably coming. 

* I think it'd be useful to corral HTTP headers and the like into an event context to pass into
the function. That might be a breaking api change, though should require only a change to the
handler function signature.

* Ultimately, I think a re-think of all this will be in order. I'm not a huge fan of how this builds.
It leads to weird compile errors when something doesn't match up, it's not very flexible, and it has to 
compile twice - once for the lib, once for the main bin. I'm also not crazy about the enormous
docker image size. 


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


## License

MIT License
