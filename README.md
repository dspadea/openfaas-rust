
# Summary

Rust function template for OpenFaas, building on the work of Booyaa (see below), and adding:

* Tokio for async handlers
* JSON handling for arbitrary request/response objects
* A few build process changes in the Docker file
* Upgraded Rust compiler, Alpine, and Watchdog

I think this template is a good interim solution to easily get Rust functions with async and serde built in, 
but I think it could be a lot better. There's a fair chance I'll wind up re-writing it (see roadmap), 
so if it becomes important to you in any way, make sure you're forking it on Github or checking it into
your project repo. 

# What's in a name?

`rust-json-classic` refers to the language, the request/response format it expects, and the watchdog type. 
I believe those are the most likely differentiators between what would be various versions of this template,
so calling them out in the name to help tell the difference seems the thing to do. 

# Provenance

This is based on the work in this repo: https://github.com/booyaa/openfaas-rust-template

# Roadmap

* I'd like to migrate this to `of-watchdog`, or implementing the equivalent watchdog functionality 
directly in rust. If I can do that without changing the handler API, I'll probably go ahead 
and change this template. 

* I think it'd be useful to corral HTTP headers and the like into an event context to pass into
the function. That might be a breaking api change, though should require only a change to the
handler function signature.

* Ultimately, I think a re-think of all this will be in order. I'm not a huge fan of how this builds.
It leads to weird compile errors when something doesn't match up, it's not very flexible, and it has to 
compile twice - once for the lib, once for the main bin. I'm also not crazy about the enormous
docker image size. 


# Usage

This is not an "official" template in the template store, so you have to pull it by URL.

```shell
faas-cli template pull https://github.com/dspadea/openfaas-rust-json-classic
```


## License

MIT License
