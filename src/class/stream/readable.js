exports.createAsyncIterable = async function* (readable) {
  for await (const chunk of readable) {
    yield chunk;
  }
}
