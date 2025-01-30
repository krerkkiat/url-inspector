# url-inspector

Check that the URLs in the given `.pptx` file are not broken.

## Known Limitations

### Hidden XML Tags Between the URL

When a URL is showing as just a text, it may have other hidden XML tags in between, the program will be unable to detect it.

For example, 
`http://software-carpentry.org/4_0/make/` may appear as

```xml
<a:t>http://software-</a:t></a:r><a:r><a:rPr lang="en-US" sz="2800" dirty="0" err="1"/><a:t>carpentry.org</a:t></a:r><a:r><a:rPr lang="en-US" sz="2800" dirty="0"/><a:t>/4_0/make/</a:t></a:r>
```

### None `.pptx` file

Only the `.pptx` is supported.

### Site that need a valid log-in session

A URL pointing to a resource that is only accessible after the user is logged in will be reported as 404 or failed depending on the site's implementation.

Similarly, since the program does not modify user-agent, some website may also response with non-success code.
