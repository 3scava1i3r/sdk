import { Buffer } from "buffer";
import * as cbor from "./cbor";

import {
  Int,
  makeHttpAgent,
  Request,
  requestIdOf,
} from "./index";

test("call", async () => {
  const mockFetch: jest.Mock = jest.fn((resource, init) => {
    return Promise.resolve(new Response(null, {
      status: 200,
    }));
  });

  const canisterId = Buffer.from([1]);
  const nonce = Buffer.from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

  const httpAgent = makeHttpAgent({
    canisterId,
    fetchFn: mockFetch,
    nonceFn: () => nonce,
  });

  const methodName = "greet";
  const arg = Buffer.from([]);

  const { requestId, response } = await httpAgent.call({
    methodName,
    arg,
  });

  const expectedRequest = {
    request_type: "call",
    nonce,
    canister_id: canisterId,
    method_name: methodName,
    arg,
  } as Request;

  const expectedRequestId = await requestIdOf(expectedRequest);

  const { calls, results } = mockFetch.mock;
  expect(calls.length).toBe(1);
  expect(requestId).toEqual(expectedRequestId);

  expect(calls[0][0]).toBe("http://localhost:8000/api/v1/submit");
  expect(calls[0][1]).toEqual({
    method: "POST",
    headers: {
      "Content-Type": "application/cbor",
    },
    body: cbor.encode(expectedRequest),
  });
});

test.todo("query");

test("requestStatus", async () => {
  const mockResponse = {
    status: "replied",
    reply: { arg: Buffer.from([]) },
  };

  const mockFetch: jest.Mock = jest.fn((resource, init) => {
    const body = cbor.encode(mockResponse);
    return Promise.resolve(new Response(body, {
      status: 200,
    }));
  });

  const canisterId = Buffer.from([1]);
  const nonce = Buffer.from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

  const httpAgent = makeHttpAgent({
    canisterId,
    fetchFn: mockFetch,
    nonceFn: () => nonce,
  });

  const requestId = await requestIdOf({
    request_type: "call",
    nonce,
    canister_id: canisterId,
    method_name: "greet",
    arg: Buffer.from([]),
  } as Request);

  const response = await httpAgent.requestStatus({
    requestId,
  });

  const { calls, results } = mockFetch.mock;
  expect(calls.length).toBe(1);
  expect(response).toEqual(mockResponse);

  expect(calls[0][0]).toBe("http://localhost:8000/api/v1/read");
  expect(calls[0][1]).toEqual({
    method: "POST",
    headers: {
      "Content-Type": "application/cbor",
    },
    body: cbor.encode({
      request_type: "request-status",
      nonce,
      request_id: requestId,
    }),
  });
});
