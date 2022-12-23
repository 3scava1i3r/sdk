import Hash "mo:base/Hash";
import Text "mo:base/Text";

import DfxVersion "DfxVersion";
import Network "Network";
import Platform "Platform";

module InvocationDetails
{
  type DfxVersion = DfxVersion.DfxVersion;
  type Hash = Hash.Hash;
  type Network = Network.Network;
  type Platform = Platform.Platform;

  public type InvocationDetails = {
    dfxVersion : DfxVersion;
    platform : Platform;
    network : Network;
  };

  public type InvocationDetailsTrieKey = {
    key : InvocationDetails;
    hash : Hash;
  };

  public func hash(v : InvocationDetails) : Hash {
    let versionHash : Nat32 = Text.hash(v.dfxVersion);

    let platformPart : Nat32 = switch (v.platform) {
      case (#linux) 0;
      case (#darwin) 1;
      case (#windows) 2;
    };
    let networkPart : Nat32 = switch (v.network) {
      case (#ic) 0;
      case (#localProject) 1;
      case (#localShared) 2;
      case (#other) 3;
    };

    let key : [Hash] = [
      versionHash & 0xff,
      (versionHash >> 8) & 0xff,
      (versionHash >> 16) & 0xff,
      (versionHash >> 24) & 0xff,
      platformPart,
      networkPart,
    ];
    Hash.hashNat8(key)
  };

  public func equal(a : InvocationDetails, b : InvocationDetails) : Bool {
    a == b
  };

  public func trieKey(v : InvocationDetails) : InvocationDetailsTrieKey {
    {
      key = v;
      hash = hash v;
    }
  };
}
