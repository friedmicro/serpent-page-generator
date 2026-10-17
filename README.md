# Serpent page generator
A simple static website generator written in rust to render markdown and dynamically inject static html pages. This is highly opinionated and it was written for my personal website. There's also support for injecting a few variables `BUILD_VERSION`, `COMMIT`, and `GEN_NAME` as well as entire pages with `%%PAGE_CONTENT()%%`.

I opted to write my own so as to not have to repurpose another static page generator with a custom template or deal with performance issues that are common on static page generators once a site reaches a certain size. I also wanted a very specific manner on how I template and link my site as well as complete control over the caching and deployment.

My website on the homepage is 13.6kb because this generator only applies a light bit of styling and css with the option to inject JS completely according to my needs.
