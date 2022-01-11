# Summary

Rust function template for OpenFaas, building on the work of Booyaa (see below), and adding:

* Tokio for async handlers
* JSON handling for arbitrary input/output objects
* A few build process changes in the Docker file

# Provenance

This is based on the work in this repo: https://github.com/booyaa/openfaas-rust-template

# Why a fork?

First of all, this fundamentally (and incompatibly) changes how the template operates, as well 
as the intention of the original project. 

The original template was for demonstration purposes for Booyaa's blog, and was relatively simple,
functionality-wise.  It accepts a string and sends it to the function, which is 
great when trying to illustrate the data flow. 

For real use, I want something a little more involved. Real use being at odds with the purpose 
of Booyaa's repo, I figured it best just to go ahead and make my own. If he thinks it makes sense to 
re-merge the projects, I'm happy to talk about that. 

## Usage

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
