# Matching Balanced Braces with Nom

I'm trying to figure out how to use Nom to identify strings containing balanced sets of curly braces.

The use-case here may be a bit strange, but basically I'm trying to identify substrings that look "JSON-like" (meaning they start with a brace and end with a balanced amount of closing braces). However I don't really want to implement a full JSON parser with Nom, because I already have [GeoJSON Library](https://github.com/georust/geojson) that I'm going to use for the actual parsing. However I may also have other input types in my stream (lat/lon, wkt, etc), so I need to be able to recognize the pieces that could plausibly be JSON, and then send them to the geojson parser for actual handling.

I thought I might be able to concoct something pretty simple using the `recognize` and `delimited` features of Nom, but so far I haven't figured out the proper way to put them together, or whether this is even a reasonable thing to do with Nom.

Example with test case [here](https://github.com/worace/nom_example/blob/master/src/example.rs)

#### Current Result

```
running 1 test
test example::tests::test_brace_matching ... FAILED

failures:

---- example::tests::test_brace_matching stdout ----
thread 'example::tests::test_brace_matching' panicked at 'assertion failed: `(left == right)`
  left: `Ok(("", "{a{b}a}"))`,
 right: `Err(Error(Code("{b}a}", Tag)))`', src/example.rs:20:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    example::tests::test_brace_matching

```
