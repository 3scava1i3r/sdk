// tslint:disable-next-line: max-line-length
// https://github.com/dfinity-lab/dfinity/blob/9bca65f8edd65701ea6bdb00e0752f9186bbc893/docs/spec/public/index.adoc#cbor-encoding-of-requests-and-responses

import borc from "borc";
import { Buffer } from "buffer";
import { Int } from "./int";

const SEMANTIC_TAG = 55799;

interface CborRecord extends Record<string, CborValue> {}

export type CborValue
  // Strings: Major type 3 (“Text string”).
  = string

  // Blobs: Major type 2 (“Byte string”)
  | Array<Int>

  // Integer numbers: Major type 0 or 1 (“Unsigned/signed integer”) if small
  // enough to fit that type, else the Bignum format is used.
  | Int

  // Nested records: Major type 5 followed by string keys.
  | CborRecord;

export const encode = (value: CborValue): Buffer => {
  return borc.encode(
    new borc.Tagged(SEMANTIC_TAG, value),
  );
};

export const decode = (input: Buffer): CborValue => {
  const decoder = new borc.Decoder({
    size: input.length,
    tags: {
      [SEMANTIC_TAG]: (value: CborValue): CborValue => value,
    },
  });
  return decoder.decodeFirst(input);
};
