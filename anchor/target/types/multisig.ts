export type Multisig = {
  "version": "0.1.0",
  "name": "multisig",
  "instructions": [
    {
      "name": "initialize",
      "docs": [
        "Initialize multisig and escrow with admin and signers"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "approvalList",
          "type": {
            "vec": "publicKey"
          }
        },
        {
          "name": "threshold",
          "type": "u8"
        },
        {
          "name": "initialBalance",
          "type": "u64"
        }
      ]
    },
    {
      "name": "propose",
      "docs": [
        "Propose a transfer transaction or threshold change"
      ],
      "accounts": [
        {
          "name": "proposer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "txType",
          "type": {
            "defined": "TransactionType"
          }
        },
        {
          "name": "isAutoApprove",
          "type": "bool"
        }
      ]
    },
    {
      "name": "approve",
      "docs": [
        "Admin or signer approves a transaction or threshold change"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        }
      ]
    },
    {
      "name": "deleteApproval",
      "docs": [
        "Admin deletes approval"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "signerToRemove",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "execute",
      "docs": [
        "Execute a transaction if threshold met"
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "balance",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "multisig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "approvals",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "nonce",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
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
            "type": "publicKey"
          },
          {
            "name": "approvals",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "executed",
            "type": "bool"
          },
          {
            "name": "nonce",
            "type": "u64"
          },
          {
            "name": "transactionType",
            "type": {
              "defined": "TransactionType"
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "TransactionType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Transfer",
            "fields": [
              {
                "name": "target",
                "type": "publicKey"
              },
              {
                "name": "amount",
                "type": "u64"
              }
            ]
          },
          {
            "name": "ThresholdChange",
            "fields": [
              "u8"
            ]
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "TransactionEvent",
      "fields": [
        {
          "name": "txKey",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "action",
          "type": "string",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "Unauthorized",
      "msg": "Unauthorized signer"
    },
    {
      "code": 6001,
      "name": "InvalidThreshold",
      "msg": "Invalid threshold value"
    },
    {
      "code": 6002,
      "name": "AlreadyExecuted",
      "msg": "Transaction already executed"
    },
    {
      "code": 6003,
      "name": "NotEnoughApprovals",
      "msg": "Not enough approvals"
    },
    {
      "code": 6004,
      "name": "InvalidNonce",
      "msg": "Invalid nonce"
    },
    {
      "code": 6005,
      "name": "AlreadyApproved",
      "msg": "Approval already exists"
    },
    {
      "code": 6006,
      "name": "ApprovalNotFound",
      "msg": "Approval not found"
    },
    {
      "code": 6007,
      "name": "InvalidAmount",
      "msg": "Amount should be greater than 0"
    },
    {
      "code": 6008,
      "name": "InvalidTarget",
      "msg": "Target must be set"
    },
    {
      "code": 6009,
      "name": "MissingAccounts",
      "msg": "Missing accounts"
    }
  ]
};

export const IDL: Multisig = {
  "version": "0.1.0",
  "name": "multisig",
  "instructions": [
    {
      "name": "initialize",
      "docs": [
        "Initialize multisig and escrow with admin and signers"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "approvalList",
          "type": {
            "vec": "publicKey"
          }
        },
        {
          "name": "threshold",
          "type": "u8"
        },
        {
          "name": "initialBalance",
          "type": "u64"
        }
      ]
    },
    {
      "name": "propose",
      "docs": [
        "Propose a transfer transaction or threshold change"
      ],
      "accounts": [
        {
          "name": "proposer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "txType",
          "type": {
            "defined": "TransactionType"
          }
        },
        {
          "name": "isAutoApprove",
          "type": "bool"
        }
      ]
    },
    {
      "name": "approve",
      "docs": [
        "Admin or signer approves a transaction or threshold change"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        }
      ]
    },
    {
      "name": "deleteApproval",
      "docs": [
        "Admin deletes approval"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "signerToRemove",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "execute",
      "docs": [
        "Execute a transaction if threshold met"
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "transaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "balance",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "multisig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "approvals",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "nonce",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
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
            "type": "publicKey"
          },
          {
            "name": "approvals",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "executed",
            "type": "bool"
          },
          {
            "name": "nonce",
            "type": "u64"
          },
          {
            "name": "transactionType",
            "type": {
              "defined": "TransactionType"
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "TransactionType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Transfer",
            "fields": [
              {
                "name": "target",
                "type": "publicKey"
              },
              {
                "name": "amount",
                "type": "u64"
              }
            ]
          },
          {
            "name": "ThresholdChange",
            "fields": [
              "u8"
            ]
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "TransactionEvent",
      "fields": [
        {
          "name": "txKey",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "action",
          "type": "string",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "Unauthorized",
      "msg": "Unauthorized signer"
    },
    {
      "code": 6001,
      "name": "InvalidThreshold",
      "msg": "Invalid threshold value"
    },
    {
      "code": 6002,
      "name": "AlreadyExecuted",
      "msg": "Transaction already executed"
    },
    {
      "code": 6003,
      "name": "NotEnoughApprovals",
      "msg": "Not enough approvals"
    },
    {
      "code": 6004,
      "name": "InvalidNonce",
      "msg": "Invalid nonce"
    },
    {
      "code": 6005,
      "name": "AlreadyApproved",
      "msg": "Approval already exists"
    },
    {
      "code": 6006,
      "name": "ApprovalNotFound",
      "msg": "Approval not found"
    },
    {
      "code": 6007,
      "name": "InvalidAmount",
      "msg": "Amount should be greater than 0"
    },
    {
      "code": 6008,
      "name": "InvalidTarget",
      "msg": "Target must be set"
    },
    {
      "code": 6009,
      "name": "MissingAccounts",
      "msg": "Missing accounts"
    }
  ]
};
