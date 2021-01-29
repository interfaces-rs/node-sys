const stream = require('stream');

exports.readable = function() {
  async function * generate() {
    yield 'a';
    yield 'b';
    yield 'c';
  }
  return stream.Readable.from(generate(), { objectMode: true });
};
