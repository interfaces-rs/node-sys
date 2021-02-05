const buffer = require('buffer');
const stream = require('stream');

exports.readable_buffers = function() {
  async function* generate() {
    yield buffer.Buffer.from([1]);
    yield buffer.Buffer.from([2, 3]);
    yield buffer.Buffer.from([4, 5, 6]);
  }
  return stream.Readable.from(generate());
};

exports.readable_strings = function() {
  async function* generate() {
    yield 'foo';
    yield 'bar';
    yield 'baz';
  }
  return stream.Readable.from(generate());
};
