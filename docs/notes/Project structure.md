The project structure must look similar to the following:

```
project/
├── block-oracle
│   ├── Dockerfile
│   └── wrapper.sh
├── contracts.json
├── dipper
│   ├── Dockerfile
│   ├── wrapper.sh
│   └── source
├── gateway
│   ├── Dockerfile
│   └── wrapper.sh
├── graph-contracts
│   ├── Dockerfile
│   └── wrapper.sh
├── graph-node
│   ├── Dockerfile
│   └── wrapper.sh
├── indexer-agent
│   ├── Dockerfile
│   └── wrapper.sh
├── indexer-service
│   ├── Dockerfile
│   ├── wrapper.sh
│   └── source
├── overrides
│   ├── graph-node-dev
│   ├── graph-node-dev.sh
│   ├── indexer-agent-dev
│   └── README.md
├── postgres
│   └── setup.sql
├── README.md
├── setup.yaml
└── tap-escrow-manager
    ├── Dockerfile
    └── wrapper.sh
```