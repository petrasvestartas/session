const e={title:"session_proto/sheet.proto",html:`<h1 id="session_protosheetproto">session_proto/sheet.proto<a class="anchor" href="#/course/kernel/sheet_proto#session_protosheetproto" aria-label="Link to this section">#</a></h1>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>syntax = &quot;proto3&quot;;

package session_proto;


// Sheet: one drawing's linework flattened to segments, published for RANGE reads.
//
// The viewer never needs the whole file to start drawing. Every big array is a packed
// FIXED-WIDTH field - double, fixed32, float - so a reader that scans the protobuf tags in
// the first kilobytes knows the byte offset and length of every array, and can fetch any
// slice of it with one HTTP Range request: segment i is bytes [48*i, 48*i+48) of \`coords\`,
// [4*i, 4*i+4) of \`colors\`, \`widths\` and \`source_ids\`. \`uint32\` (varint) would break this -
// 1 to 5 bytes each, no way to seek - so it is only used for the two scalar counts, which
// are read once from the header. Field numbers are wire order, so \`source_ids\` at 15 is the
// LAST array in the file: everything a first paint needs comes before it.
//
// Per-entity metadata (guid, name, kind, authored pen) is NOT in this message. It lives in a
// side table named by \`meta\`: magic \`SHM1\`, u32 LE count, count records of u64 LE offset and
// u64 LE length relative to the byte after the table, then UTF-8 JSON blobs. A segment's
// \`source_ids\` entry is its record index, so a pick resolves to one blob by one more Range.
message Sheet {
  string guid = 1;                                  // Unique identifier
  string name = 2;                                  // Sheet name
  repeated double coords = 3 [packed = true];       // 6 per segment: x0 y0 z0 x1 y1 z1, world mm
  repeated fixed32 colors = 4 [packed = true];      // one RGBA8 per segment, low byte red
  repeated float widths = 5 [packed = true];        // pen width in mm per segment, 0 = hairline
  uint32 segment_count = 6;                         // number of segments
  uint32 entity_count = 7;                          // records in the side table
  string meta = 8;                                  // side-table file name relative to this file, empty = none
  repeated fixed32 source_ids = 15 [packed = true]; // entity id per segment, index into the side table; the LAST field
}</code></pre></div>
`,toc:[]};export{e as default};
