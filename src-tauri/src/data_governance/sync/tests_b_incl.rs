            cloud_data: serde_json::json!({"content": "cloud"}),
        }];

        let result =
            SyncManager::apply_merge_strategy(MergeStrategy::UseCloud, &conflicts).unwrap();

        assert!(result.success);
        assert_eq!(result.kept_local, 0);
        assert_eq!(result.used_cloud, 1);
        assert!(result.records_to_push.is_empty());
        assert_eq!(result.records_to_pull, vec!["msg-1"]);
    }

    #[test]
    fn test_apply_merge_strategy_keep_latest() {
        let conflicts = vec![
            // 云端更新
            ConflictRecord {
                database_name: "chat_v2".to_string(),
                table_name: "messages".to_string(),
                record_id: "msg-1".to_string(),
                local_version: 3,
                cloud_version: 4,
                local_updated_at: "2024-01-01T10:00:00Z".to_string(),
                cloud_updated_at: "2024-01-01T11:00:00Z".to_string(),
                local_data: serde_json::json!({"content": "local"}),
                cloud_data: serde_json::json!({"content": "cloud"}),
            },
            // 本地更新
            ConflictRecord {
                database_name: "chat_v2".to_string(),
                table_name: "messages".to_string(),
                record_id: "msg-2".to_string(),
                local_version: 5,
                cloud_version: 3,
                local_updated_at: "2024-01-01T12:00:00Z".to_string(),
                cloud_updated_at: "2024-01-01T09:00:00Z".to_string(),
                local_data: serde_json::json!({"content": "local new"}),
                cloud_data: serde_json::json!({"content": "cloud old"}),
            },
        ];

        let result =
            SyncManager::apply_merge_strategy(MergeStrategy::KeepLatest, &conflicts).unwrap();

        assert!(result.success);
        assert_eq!(result.kept_local, 1);
        assert_eq!(result.used_cloud, 1);
        assert_eq!(result.records_to_push, vec!["msg-2"]);
        assert_eq!(result.records_to_pull, vec!["msg-1"]);
    }

    #[test]
    fn regression_m3_keep_latest_uses_device_tiebreaker() {
        let conflicts = vec![ConflictRecord {
            database_name: "chat_v2".to_string(),
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 3,
            cloud_version: 4,
            local_updated_at: "2024-01-01T10:00:00Z".to_string(),
            cloud_updated_at: "2024-01-01T10:00:00Z".to_string(),
            local_data: serde_json::json!({"content": "local", "device_id": "device-a"}),
            cloud_data: serde_json::json!({"content": "cloud", "device_id": "device-b"}),
        }];

        let result =
            SyncManager::apply_merge_strategy(MergeStrategy::KeepLatest, &conflicts).unwrap();

        assert!(result.success);
        assert_eq!(result.kept_local, 0);
        assert_eq!(result.used_cloud, 1);
        assert!(result.records_to_push.is_empty());
        assert_eq!(result.records_to_pull, vec!["msg-1"]);
    }

    #[test]
    fn test_apply_merge_strategy_manual_error() {
        let conflicts = vec![ConflictRecord {
            database_name: "chat_v2".to_string(),
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 3,
            cloud_version: 4,
            local_updated_at: "2024-01-01T10:00:00Z".to_string(),
            cloud_updated_at: "2024-01-01T11:00:00Z".to_string(),
            local_data: serde_json::json!({"content": "local"}),
            cloud_data: serde_json::json!({"content": "cloud"}),
        }];

        let result = SyncManager::apply_merge_strategy(MergeStrategy::Manual, &conflicts);

        assert!(result.is_err());
        match result {
            Err(SyncError::ManualResolutionRequired { count }) => {
                assert_eq!(count, 1);
            }
            _ => panic!("Expected ManualResolutionRequired error"),
        }
    }

    #[test]
    fn test_get_change_log_stats() {
        let conn = create_test_db();

        // 插入混合状态的变更日志
        insert_test_change_log(&conn, "messages", "msg-1", "INSERT", 0);
        insert_test_change_log(&conn, "messages", "msg-2", "UPDATE", 0);
        insert_test_change_log(&conn, "messages", "msg-3", "DELETE", 100);
        insert_test_change_log(&conn, "sessions", "sess-1", "INSERT", 200);

        let stats = SyncManager::get_change_log_stats(&conn).unwrap();

        assert_eq!(stats.total_count, 4);
        assert_eq!(stats.pending_count, 2);
        assert_eq!(stats.synced_count, 2);
    }

    #[test]
    fn test_change_operation_from_str() {
        assert_eq!(
            ChangeOperation::from_str("INSERT"),
            Some(ChangeOperation::Insert)
        );
        assert_eq!(
            ChangeOperation::from_str("insert"),
            Some(ChangeOperation::Insert)
        );
        assert_eq!(
            ChangeOperation::from_str("UPDATE"),
            Some(ChangeOperation::Update)
        );
        assert_eq!(
            ChangeOperation::from_str("DELETE"),
            Some(ChangeOperation::Delete)
        );
        assert_eq!(ChangeOperation::from_str("INVALID"), None);
    }

    #[test]
    fn test_change_operation_as_str() {
        assert_eq!(ChangeOperation::Insert.as_str(), "INSERT");
        assert_eq!(ChangeOperation::Update.as_str(), "UPDATE");
        assert_eq!(ChangeOperation::Delete.as_str(), "DELETE");
    }

    #[test]
    fn test_pending_changes_get_table_changes() {
        let entries = vec![
            ChangeLogEntry {
                id: 1,
                table_name: "messages".to_string(),
                record_id: "msg-1".to_string(),
                operation: ChangeOperation::Insert,
                changed_at: "2024-01-01T10:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
            ChangeLogEntry {
                id: 2,
                table_name: "sessions".to_string(),
                record_id: "sess-1".to_string(),
                operation: ChangeOperation::Insert,
                changed_at: "2024-01-01T11:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
            ChangeLogEntry {
                id: 3,
                table_name: "messages".to_string(),
                record_id: "msg-2".to_string(),
                operation: ChangeOperation::Update,
                changed_at: "2024-01-01T12:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
        ];

        let pending = PendingChanges::from_entries(entries);

        let message_changes = pending.get_table_changes("messages");
        assert_eq!(message_changes.len(), 2);

        let session_changes = pending.get_table_changes("sessions");
        assert_eq!(session_changes.len(), 1);

        let other_changes = pending.get_table_changes("other");
        assert!(other_changes.is_empty());
    }

    #[test]
    fn test_pending_changes_get_change_ids() {
        let entries = vec![
            ChangeLogEntry {
                id: 1,
                table_name: "messages".to_string(),
                record_id: "msg-1".to_string(),
                operation: ChangeOperation::Insert,
                changed_at: "2024-01-01T10:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
            ChangeLogEntry {
                id: 5,
                table_name: "messages".to_string(),
                record_id: "msg-2".to_string(),
                operation: ChangeOperation::Update,
                changed_at: "2024-01-01T11:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
        ];

        let pending = PendingChanges::from_entries(entries);
        let ids = pending.get_change_ids();

        assert_eq!(ids, vec![1, 5]);
    }

    #[test]
    fn test_pending_changes_time_range() {
        let entries = vec![
            ChangeLogEntry {
                id: 1,
                table_name: "messages".to_string(),
                record_id: "msg-1".to_string(),
                operation: ChangeOperation::Insert,
                changed_at: "2024-01-01T12:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
            ChangeLogEntry {
                id: 2,
                table_name: "messages".to_string(),
                record_id: "msg-2".to_string(),
                operation: ChangeOperation::Update,
                changed_at: "2024-01-01T08:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
            ChangeLogEntry {
                id: 3,
                table_name: "messages".to_string(),
                record_id: "msg-3".to_string(),
                operation: ChangeOperation::Delete,
                changed_at: "2024-01-01T15:00:00Z".to_string(),
                sync_version: 0,
                field_deltas_json: None,
            },
        ];

        let pending = PendingChanges::from_entries(entries);

        assert_eq!(
            pending.earliest_change,
            Some("2024-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            pending.latest_change,
            Some("2024-01-01T15:00:00Z".to_string())
        );
    }

    #[test]
    fn test_merge_application_result() {
        let success = MergeApplicationResult::success(3, 2);
        assert!(success.success);
        assert_eq!(success.kept_local, 3);
        assert_eq!(success.used_cloud, 2);

        let failure = MergeApplicationResult::failure(vec!["error1".to_string()]);
        assert!(!failure.success);
        assert_eq!(failure.errors, vec!["error1"]);
    }

    // ========================================================================
    // apply_downloaded_changes: data=None 跳过行为测试
    // ========================================================================

    /// 创建包含业务表的测试数据库（用于 apply 测试）
    fn create_test_db_with_business_table() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE test_records (
                id TEXT PRIMARY KEY,
                content TEXT,
                updated_at TEXT
            );
            CREATE TABLE IF NOT EXISTS __change_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                record_id TEXT NOT NULL,
                operation TEXT NOT NULL CHECK(operation IN ('INSERT', 'UPDATE', 'DELETE')),
                changed_at TEXT NOT NULL DEFAULT (datetime('now')),
                sync_version INTEGER DEFAULT 0
            );
            "#,
        )
        .unwrap();
        conn
    }

    #[test]
    fn regression_m5_apply_explicit_null_clears_column() {
        let conn = create_test_db_with_business_table();
        conn.execute(
            "INSERT INTO test_records(id, content, updated_at) VALUES(?1, ?2, ?3)",
            rusqlite::params!["rec-null", "keep-me", "2024-01-01T00:00:00Z"],
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "rec-null".to_string(),
            operation: ChangeOperation::Update,
            data: Some(json!({
                "id": "rec-null",
                "content": null,
                "updated_at": "2024-01-02T00:00:00Z",
            })),
            changed_at: "2024-01-02T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: None,
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();
        assert_eq!(result.success_count, 1);

        let content: Option<String> = conn
            .query_row(
                "SELECT content FROM test_records WHERE id='rec-null'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(content.is_none(), "显式 JSON null 必须传播为 SQL NULL");
    }

    #[test]
    fn regression_m1_single_sided_remote_tag_delete_is_not_field_merged_back() {
        let conn = create_test_db();
        conn.execute_batch(
            r#"
            CREATE TABLE notes (
                id TEXT PRIMARY KEY,
                tags TEXT,
                is_favorite INTEGER DEFAULT 0,
                updated_at TEXT
            );
            "#,
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes(id, tags, updated_at) VALUES(?1, ?2, ?3)",
            rusqlite::params!["note-1", r#"["local"]"#, "2026-02-09T00:00:00Z"],
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "notes".to_string(),
            record_id: "note-1".to_string(),
            operation: ChangeOperation::Update,
            data: Some(json!({
                "id": "note-1",
                "tags": [],
                "updated_at": "2026-02-10T00:00:00Z",
            })),
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();
        assert_eq!(result.success_count, 1);

        let tags: String = conn
            .query_row("SELECT tags FROM notes WHERE id='note-1'", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(tags, "[]", "非并发远端删 tag 不能被本地旧 tag 复活");
    }

    #[test]
    fn regression_m1_pending_local_change_allows_field_merge() {
        let conn = create_test_db();
        conn.execute_batch(
            r#"
            CREATE TABLE notes (
                id TEXT PRIMARY KEY,
                tags TEXT,
                is_favorite INTEGER DEFAULT 0,
                updated_at TEXT
            );
            "#,
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes(id, tags, updated_at) VALUES(?1, ?2, ?3)",
            rusqlite::params!["note-merge", r#"["local"]"#, "2026-02-09T00:00:00Z"],
        )
        .unwrap();
        insert_test_change_log(&conn, "notes", "note-merge", "UPDATE", 0);

        let changes = vec![SyncChangeWithData {
            table_name: "notes".to_string(),
            record_id: "note-merge".to_string(),
            operation: ChangeOperation::Update,
            data: Some(json!({
                "id": "note-merge",
                "tags": ["remote"],
                "updated_at": "2026-02-10T00:00:00Z",
            })),
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();
        assert_eq!(result.success_count, 1);

        let tags: String = conn
            .query_row("SELECT tags FROM notes WHERE id='note-merge'", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(
            tags, r#"["local","remote"]"#,
            "本地存在 pending 修改时仍允许集合字段并发合并"
        );
    }

    #[test]
    fn test_apply_insert_with_data_none_is_skipped() {
        let conn = create_test_db_with_business_table();

        let changes = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "rec-1".to_string(),
            operation: ChangeOperation::Insert,
            data: None, // 旧格式：无数据
            changed_at: "2024-01-01T10:00:00Z".to_string(),
            change_log_id: None,
            database_name: None,
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 0);
        assert_eq!(
            result.skipped_count, 1,
            "data=None INSERT should be skipped, not error"
        );
        assert_eq!(
            result.skipped_incomplete_count, 1,
            "data=None INSERT should be counted as incomplete skipped"
        );
        assert_eq!(result.failure_count, 0);

        // 验证记录不存在
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM test_records WHERE id = 'rec-1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_apply_update_with_data_none_is_skipped() {
        let conn = create_test_db_with_business_table();

        // 先插入一条记录
        conn.execute(
            "INSERT INTO test_records (id, content) VALUES ('existing', 'original')",
            [],
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "existing".to_string(),
            operation: ChangeOperation::Update,
            data: None, // 旧格式：无数据
            changed_at: "2024-01-01T10:00:00Z".to_string(),
            change_log_id: None,
            database_name: None,
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 0);
        assert_eq!(
            result.skipped_count, 1,
            "data=None UPDATE should be skipped"
        );
        assert_eq!(
            result.skipped_incomplete_count, 1,
            "data=None UPDATE should be counted as incomplete skipped"
        );
        assert_eq!(result.failure_count, 0);

        // 验证记录未被修改
        let content: String = conn
            .query_row(
                "SELECT content FROM test_records WHERE id = 'existing'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(content, "original");
    }

    #[test]
    fn test_apply_delete_without_data_succeeds() {
        let conn = create_test_db_with_business_table();

        conn.execute(
            "INSERT INTO test_records (id, content) VALUES ('to-delete', 'bye')",
            [],
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "to-delete".to_string(),
            operation: ChangeOperation::Delete,
            data: None, // DELETE 不需要数据
            changed_at: "2024-01-01T10:00:00Z".to_string(),
            change_log_id: None,
            database_name: None,
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(
            result.success_count, 1,
            "DELETE without data should succeed"
        );
        assert_eq!(result.skipped_count, 0);
        assert_eq!(result.skipped_incomplete_count, 0);

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM test_records WHERE id = 'to-delete'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_apply_mixed_data_none_and_valid() {
        let conn = create_test_db_with_business_table();

        let changes = vec![
            // 1. INSERT 无数据 → 跳过
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "no-data".to_string(),
                operation: ChangeOperation::Insert,
                data: None,
                changed_at: "2024-01-01T10:00:00Z".to_string(),
                change_log_id: None,
                database_name: None,
                suppress_change_log: None,
                source_device_id: None,
                source_seq: None,
            },
            // 2. INSERT 有数据 → 成功
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "has-data".to_string(),
                operation: ChangeOperation::Insert,
                data: Some(serde_json::json!({
                    "id": "has-data",
                    "content": "valid",
                    "updated_at": "2024-01-01"
                })),
                changed_at: "2024-01-01T10:00:01Z".to_string(),
                change_log_id: None,
                database_name: None,
                suppress_change_log: None,
                source_device_id: None,
                source_seq: None,
            },
        ];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 1, "only valid INSERT should succeed");
        assert_eq!(
            result.skipped_count, 1,
            "data=None INSERT should be skipped"
        );
        assert_eq!(
            result.skipped_incomplete_count, 1,
            "only the data=None change should be counted as incomplete"
        );
        assert_eq!(result.failure_count, 0, "no failures expected");

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM test_records WHERE id = 'has-data'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "valid record should still be applied");
    }

    #[test]
    fn test_apply_semantically_equal_skip_is_not_incomplete() {
        let conn = create_test_db_with_business_table();

        conn.execute(
            "INSERT INTO test_records (id, content, updated_at) VALUES ('same', 'value', '2024-01-01')",
            [],
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "same".to_string(),
            operation: ChangeOperation::Update,
            data: Some(serde_json::json!({
                "id": "same",
                "content": "value",
                "updated_at": "2024-01-01"
            })),
            changed_at: "2024-01-01T10:00:00Z".to_string(),
            change_log_id: None,
            database_name: None,
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 0);
        assert_eq!(result.skipped_count, 1);
        assert_eq!(
            result.skipped_incomplete_count, 0,
            "idempotent/equivalent replay should not surface as incomplete data"
        );
        assert_eq!(result.failure_count, 0);
    }

    #[test]
    fn test_get_record_data_composite_pk_with_json_record_id() {
        // 注：原测试针对 llm_usage_daily，但该表自 V20260525 起退出 RowSync
        // （派生聚合数据不再行级同步），get_record_data 会按白名单拒绝。
        // 此处改用测试白名单表验证"复合主键 + JSON record_id"解析能力。
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE test_usage_daily (
                date TEXT NOT NULL,
                caller_type TEXT NOT NULL,
                model TEXT NOT NULL,
                provider TEXT NOT NULL,
                request_count INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (date, caller_type, model, provider)
            );
            INSERT INTO test_usage_daily(date, caller_type, model, provider, request_count)
            VALUES('2026-02-10', 'chat', 'gpt-4o', 'openai', 7);
            "#,
        )
        .unwrap();

        let record_id = serde_json::json!({
            "date": "2026-02-10",
            "caller_type": "chat",
            "model": "gpt-4o",
            "provider": "openai"
        })
        .to_string();

        let data = SyncManager::get_record_data(&conn, "test_usage_daily", &record_id, "id")
            .unwrap()
            .expect("record should be found");

        assert_eq!(data["request_count"], serde_json::json!(7));
    }

    #[test]
    fn test_apply_downloaded_changes_can_suppress_change_log_echo() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE test_records (
                id TEXT PRIMARY KEY,
                content TEXT,
                updated_at TEXT
            );
            CREATE TABLE __change_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                record_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                changed_at TEXT NOT NULL DEFAULT (datetime('now')),
                sync_version INTEGER DEFAULT 0
            );
            CREATE TRIGGER trg_echo_insert
            AFTER INSERT ON test_records
            BEGIN
                INSERT INTO __change_log(table_name, record_id, operation)
                VALUES('test_records', NEW.id, 'INSERT');
            END;
            "#,
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "r1".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "r1",
                "content": "ok",
                "updated_at": "2026-02-10"
            })),
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }];

        SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        let unsynced: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM __change_log WHERE sync_version = 0",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(unsynced, 0, "echo logs should be marked as synced");
    }

    #[test]
    fn test_apply_downloaded_changes_skips_equivalent_replay_without_new_echo_log() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE test_records (
                id TEXT PRIMARY KEY,
                content TEXT,
                updated_at TEXT
            );
            CREATE TABLE __change_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                record_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                changed_at TEXT NOT NULL DEFAULT (datetime('now')),
                sync_version INTEGER DEFAULT 0
            );
            CREATE TRIGGER trg_echo_insert
            AFTER INSERT ON test_records
            BEGIN
                INSERT INTO __change_log(table_name, record_id, operation)
                VALUES('test_records', NEW.id, 'INSERT');
            END;
            CREATE TRIGGER trg_echo_update
            AFTER UPDATE ON test_records
            BEGIN
                INSERT INTO __change_log(table_name, record_id, operation)
                VALUES('test_records', NEW.id, 'UPDATE');
            END;
            "#,
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
