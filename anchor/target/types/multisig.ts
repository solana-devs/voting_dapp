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
      "name": "proposeTransaction",
      "docs": [
        "Propose a transfer transaction"
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
          "name": "target",
          "type": "publicKey"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "isAutoApprove",
          "type": "bool"
        }
      ]
    },
    {
      "name": "proposeThresholdChange",
      "docs": [
        "Propose a threshold change"
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
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newThreshold",
          "type": "u8"
        },
        {
          "name": "nonce",
          "type": "u64"
        }
      ]
    },
    {
      "name": "approveTransaction",
      "docs": [
        "Admin or signer approves a transaction"
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
      "args": []
    },
    {
      "name": "deleteApproval",
      "docs": [
        "Admin deletes an approval"
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
          "name": "signerToRemove",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "executeTransaction",
      "docs": [
        "Execute a transaction if threshold met"
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
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
          "name": "target",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
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
            "name": "target",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
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
            "name": "Transfer"
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
      "name": "proposeTransaction",
      "docs": [
        "Propose a transfer transaction"
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
          "name": "target",
          "type": "publicKey"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "isAutoApprove",
          "type": "bool"
        }
      ]
    },
    {
      "name": "proposeThresholdChange",
      "docs": [
        "Propose a threshold change"
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
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newThreshold",
          "type": "u8"
        },
        {
          "name": "nonce",
          "type": "u64"
        }
      ]
    },
    {
      "name": "approveTransaction",
      "docs": [
        "Admin or signer approves a transaction"
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
      "args": []
    },
    {
      "name": "deleteApproval",
      "docs": [
        "Admin deletes an approval"
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
          "name": "signerToRemove",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "executeTransaction",
      "docs": [
        "Execute a transaction if threshold met"
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
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
          "name": "target",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
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
            "name": "target",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
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
            "name": "Transfer"
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
    }
  ]
};
