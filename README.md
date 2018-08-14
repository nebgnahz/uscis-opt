USCIS OPT Crawler, Analyzer, and Visualizer
---

### Live Webpages

- [query](http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/)
- [freshness](http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/freshness)
- [per-day](http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/per-day)

### Tech Stack:

- [Crawler in NodeJS](crawler/index.js)
- [Scheduler in Rust](src/main.rs)
- [Backend/Storage with AWS](Makefile)
- [Visualization](frontend)

### Deployment:

- Crawler on Heroku (dynamic IP)
- Scheduler on a low-end box




