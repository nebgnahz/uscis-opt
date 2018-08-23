USCIS OPT Crawler, Analyzer, and Visualizer
---

[Live Webpages](http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/)

### Tech Stack:

- [Crawler in NodeJS](crawler/index.js)
- [Scheduler in Rust](src/main.rs)
- [Backend/Storage with AWS](Makefile)
- [Visualization](frontend)

### Deployment:

- Crawler on Heroku (dynamic IP)
- Scheduler on a low-end box

### Gallery:

USCIS OPT Progress: Crawler and Tracker

<a href="http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/query.html">
<p>What is the current status for a range?</p>
<img src="frontend/query.png" width="400" />
</a>

<a href="http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/freshness.html">
<p>How fresh is this result?</p>
<img src="frontend/freshness.png" width="400" />
</a>
<br/>
<a href="http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/per-day.html">
<p>How many cases has USCIS approved today?</p>
<img src="frontend/per-day.png" width="400" />
</a>

<a href="http://uscis-opt.s3-website-us-west-1.amazonaws.com/vis/trend.html">
<p>I want to track a particular range!</p>
<img src="frontend/trend.png" width="400" />
</a>

<a href="http://uscis-opt.s3-website-us-west-1.amazonaws.com/raw-data/raw.html">
Can I get your raw data? Yes!
</a>
