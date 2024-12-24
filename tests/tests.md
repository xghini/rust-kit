tests/ 目录


用于集成测试（integration tests）
测试代码从外部视角使用你的库
每个文件都被视为独立的 crate
只能测试公开的 API

Copymy_project/
└── tests/
    ├── api_tests.rs
    └── integration_tests.rs