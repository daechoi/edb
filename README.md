# EDB

Enterprise distributed database

## Project Goals

- **Storage:** Storage engine based on LSM for primary and secondary indexes.
- **Networking:** gRPC or Quin
- **Consensus:** Raft implementation or using dragonboat
- **Data Types:** Nulls, booleans, 64bit doubles, and short, UTF8 strings
- **Constraints:** Primary keys, unique indexes, and foreign keys
- **Transactions:** Serializable isolation with MVCC snapshot
- **Query Engine:** Heuristic-based planner and optimizer supporting expressions, functions, inner joins, outerjoins
- **Language:** Full SQL support

- **Client** Simple REPL grpc client: No authentication
