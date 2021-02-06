exports.endPromise = (writable) => {
  return new Promise((resolve) => {
    writable.end(resolve);
  });
};

exports.endWithDataPromise = (writable, buffer) => {
  return new Promise((resolve) => {
    writable.end(buffer, resolve);
  });
};

exports.endWithStringPromise = (writable, string, encoding) => {
  return new Promise((resolve) => {
    writable.end(string, encoding, resolve);
  });
};

exports.writeWithDataPromise = (writable, buffer) => {
  return new Promise((resolve) => {
    if (!writable.write(buffer)) {
      writable.once('drain', resolve);
    } else {
      process.nextTick(resolve);
    }
  });
};

exports.writeWithStringPromise = (writable, string, encoding) => {
  return new Promise((resolve) => {
    if (!writable.write(string, encoding)) {
      writable.once('drain', resolve);
    } else {
      process.nextTick(resolve);
    }
  });
};
