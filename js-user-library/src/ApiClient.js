const API_VERSION = "v1";

const defaults = {
  fetch: window.fetch,
};

export class ApiClient {
  constructor(config) {
    const { canisterId, fetch, host } = {
      ...defaults,
      ...config,
    };

    this._canisterId = canisterId;

    this._fetch = (endpoint, body) => {
      return fetch(`${host}/api/${API_VERSION}/${endpoint}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/cbor",
        },
        body,
      });
    };
  }

  async _submit(requestType, fields) {
    const allFields = {
      ...fields,
      request_type: requestType,
      // expiry,
      // nonce,
      // sender,
      // sender_pubkey,
      // sender_sig,
    };
    // FIXME: convert `allFields` to `body`
    const body = new Blob([], { type: "application/cbor" });
    return this._fetch("submit", body);
  }

  async call({ methodName, arg }) {
    return this._submit("call", {
      canister_id: this._canisterId,
      method_name: methodName,
      arg,
    });
  }
}
