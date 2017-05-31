text = "
---
layout: post
title: Blogging Like a Hacker
---

### About this blog post

lorem ipsum...
"

p text.match(/(?:---)\s+(.*)\s+(?:---)/m)[1]

