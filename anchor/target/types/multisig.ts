/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/multisig.json`.
 */
export type Multisig = {
  "address": "Fg6PaFpoGXkYsidMpWxqSWFEXvUfsicV7opJ2zG9JWxD",
  "metadata": {
    "name": "multisig",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "approveTransaction",
      "docs": [
        "Approve a transaction"
      ],
      "discriminator": [
        224,
        39,
        88,
        181,
        36,
        59,
        155,
        122
      ],
      "accounts": [
        {
          "name": "transaction",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "executeTransaction",
      "docs": [
        "Execute transaction if threshold is met"
      ],
      "discriminator": [
        231,
        173,
        49,
        91,
        235,
        24,
        68,
        19
      ],
      "accounts": [
        {
          "name": "transaction",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "docs": [
        "Initialize a multisig account with signers and threshold"
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true,
          "signer": true
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "signers",
          "type": {
            "vec": "pubkey"
          }
        },
        {
          "name": "threshold",
          "type": "u8"
        }
      ]
    },
    {
      "name": "proposeTransaction",
      "docs": [
        "Propose a new transaction"
      ],
      "discriminator": [
        35,
        204,
        169,
        240,
        74,
        70,
        31,
        236
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "transaction",
          "writable": true,
          "signer": true
        },
        {
          "name": "proposer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "target",
          "type": "pubkey"
        },
        {
          "name": "data",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "multisig",
      "discriminator": [
        224,
        116,
        121,
        186,
        68,
        161,
        79,
        236
      ]
    },
    {
      "name": "transaction",
      "discriminator": [
        11,
        24,
        174,
        129,
        203,
        117,
        242,
        23
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidThreshold",
      "msg": "Invalid threshold value"
    },
    {
      "code": 6001,
      "name": "unauthorized",
      "msg": "Signer not authorized"
    },
    {
      "code": 6002,
      "name": "alreadyApproved",
      "msg": "Transaction already approved by this signer"
    },
    {
      "code": 6003,
      "name": "notEnoughApprovals",
      "msg": "Not enough approvals to execute"
    }
  ],
  "types": [
    {
      "name": "multisig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "signers",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "owner",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "transaction",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "multisig",
            "type": "pubkey"
          },
          {
            "name": "target",
            "type": "pubkey"
          },
          {
            "name": "data",
            "type": "bytes"
          },
          {
            "name": "approvals",
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    }
  ]
};
