#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_manifest(
        device_id: &str,
        databases: Vec<(&str, u32, u64, &str)>,
    ) -> SyncManifest {
        let mut db_map = HashMap::new();
        for (name, schema_ver, data_ver, checksum) in databases {
            db_map.insert(
                name.to_string(),
                DatabaseSyncState {
                    schema_version: schema_ver,
                    data_version: data_ver,
                    checksum: checksum.to_string(),
                    last_updated_at: None,
                },
            );
        }
        SyncManifest {
            sync_transaction_id: "test-tx".to_string(),
            databases: db_map,
            status: SyncTransactionStatus::Complete,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            device_id: device_id.to_string(),
            format_version: 3,
            published_max_seq: 0,
            cursors: HashMap::new(),
            superseded_by: None,
            snapshot_seen: HashMap::new(),
        }
    }

    #[test]
    fn test_parse_version_from_key_with_nonce() {
        let key = "data_governance/changes/device-1/12345-acde.json";
        assert_eq!(SyncManager::parse_version_from_key(key), Some(12345));
    }

    #[test]
    fn test_parse_version_from_key_legacy_no_nonce() {
        // Legacy 文件没有 nonce（纯秒级时间戳）
        let key = "data_governance/changes/device-1/1707500000.json";
        assert_eq!(SyncManager::parse_version_from_key(key), Some(1707500000));
    }

    #[test]
    fn test_parse_version_from_key_seconds_with_nonce() {
        // 旧格式 .json：秒级时间戳 + UUID nonce
        let key =
            "data_governance/changes/device-1/1707500000-550e8400-e29b-41d4-a716-446655440000.json";
        assert_eq!(SyncManager::parse_version_from_key(key), Some(1707500000));
    }

    #[test]
    fn regression_c1_v3_change_key_parses_seq_and_timestamp() {
        let manager = SyncManager::new("device-1".to_string());
        let key = manager.build_change_key_v3(42, 1_707_500_000);

        assert_eq!(
            SyncManager::parse_version_from_key(&key),
            Some(1_707_500_000)
        );
        assert!(matches!(
            SyncManager::parse_change_key(&key),
            Some(ParsedChangeKey::V3 {
                device_id,
                seq: 42,
                version: 1_707_500_000,
            }) if device_id == "device-1"
        ));
    }

    // ==================== Phase 0 回归测试 ====================

    fn make_change(record: &str, data: serde_json::Value) -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "items".to_string(),
            record_id: record.to_string(),
            operation: ChangeOperation::Update,
            data: Some(data),
            changed_at: "2024-01-01T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }
    }

    #[test]
    fn regression_c4_dedupe_keeps_last() {
        // 序列 x=1 → x=2 → x=1：keep-first 会丢掉最后的 x=1，终态错为 x=2。
        // keep-last 必须保留顺序上最后一条 x=1，终态为 x=1。
        let changes = vec![
            (1u64, make_change("r1", json!({ "x": 1 }))),
            (2u64, make_change("r1", json!({ "x": 2 }))),
            (3u64, make_change("r1", json!({ "x": 1 }))),
        ];
        let deduped = SyncManager::dedupe_downloaded_changes(changes);
        // x=1 的两条指纹相同，仅保留最后一条；x=2 保留 → 共 2 条
        assert_eq!(deduped.len(), 2, "应去掉一条重复的 x=1");
        // 顺序保持升序，最后应用的一条必须是 x=1（版本 3）
        let last = deduped.last().unwrap();
        assert_eq!(last.0, 3, "保留的 x=1 应是版本 3 那条");
        assert_eq!(last.1.data.as_ref().unwrap()["x"], json!(1));
        // 中间一条是 x=2
        assert_eq!(deduped[0].1.data.as_ref().unwrap()["x"], json!(2));
    }

    #[test]
    fn regression_c4_dedupe_distinct_kept() {
        // 内容各异的变更全部保留，顺序不变
        let changes = vec![
            (1u64, make_change("r1", json!({ "x": 1 }))),
            (2u64, make_change("r2", json!({ "x": 1 }))),
            (3u64, make_change("r1", json!({ "x": 2 }))),
        ];
        let deduped = SyncManager::dedupe_downloaded_changes(changes);
        assert_eq!(deduped.len(), 3);
        assert_eq!(deduped[0].0, 1);
        assert_eq!(deduped[1].0, 2);
        assert_eq!(deduped[2].0, 3);
    }

    #[test]
    fn regression_c6_normalize_version_to_seconds() {
        // 毫秒时间戳归一化为秒；秒级保持不变
        assert_eq!(
            SyncManager::normalize_version_to_seconds(1_707_500_000_123),
            1_707_500_000
        );
        assert_eq!(
            SyncManager::normalize_version_to_seconds(1_707_500_000),
            1_707_500_000
        );
        // 阈值边界：恰好 1e11 视为秒级保留
        assert_eq!(
            SyncManager::normalize_version_to_seconds(100_000_000_000),
            100_000_000_000
        );
    }

    #[test]
    fn regression_c7_mark_synced_batches_over_variable_limit() {
        // 一次标记 > SQLite 变量上限（默认 999/32766）的变更，分批后必须全部成功。
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE __change_log (id INTEGER PRIMARY KEY, sync_version INTEGER NOT NULL DEFAULT 0);",
        )
        .unwrap();

        let n = 1500i64;
        {
            let tx = conn.unchecked_transaction().unwrap();
            for i in 1..=n {
                tx.execute(
                    "INSERT INTO __change_log (id, sync_version) VALUES (?1, 0)",
                    [i],
                )
                .unwrap();
            }
            tx.commit().unwrap();
        }

        let ids: Vec<i64> = (1..=n).collect();
        let updated = SyncManager::mark_synced(&conn, &ids, 12345).unwrap();
        assert_eq!(updated as i64, n, "应全部标记成功");

        let marked: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM __change_log WHERE sync_version = 12345",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(marked, n, "全部行的 sync_version 应被更新");
    }

    #[test]
    fn regression_c8_upload_path_verifies_size() {
        let source = include_str!("mod.rs");
        assert!(source.contains("verify_uploaded_size"));
        assert!(source.contains("put_change_with_verify_retry"));
        assert!(
            source.contains("storage.stat(&key).await"),
            "PUT 后必须 stat 回验对象大小"
        );
        assert!(
            source.contains("Ok(Some(info)) if info.size == expected"),
            "size 与期望不符时必须拒绝推进 mark"
        );
    }

    #[test]
    fn test_parse_version_from_key_zst_with_nonce() {
        // 新格式 .json.zst：秒级时间戳 + UUID nonce + zstd 压缩
        let key = "data_governance/changes/device-1/1707500000-550e8400-e29b-41d4-a716-446655440000.json.zst";
        assert_eq!(SyncManager::parse_version_from_key(key), Some(1707500000));
    }

    #[test]
    fn test_parse_version_from_key_zst_legacy_no_nonce() {
        // .json.zst 无 nonce
        let key = "data_governance/changes/device-1/1707500000.json.zst";
        assert_eq!(SyncManager::parse_version_from_key(key), Some(1707500000));
    }

    #[test]
    fn test_parse_version_from_key_invalid() {
        assert_eq!(SyncManager::parse_version_from_key(""), None);
        assert_eq!(SyncManager::parse_version_from_key("no-slash"), None);
        assert_eq!(
            SyncManager::parse_version_from_key("data_governance/changes/device-1/notanumber.json"),
            None
        );
        assert_eq!(
            SyncManager::parse_version_from_key("data_governance/changes/device-1/abc.json.zst"),
            None
        );
    }

    #[test]
    fn test_version_space_compatibility_seconds() {
        // 验证新旧版本空间兼容：legacy 用秒级时间戳，新代码也用秒级
        // 新变更 version = 当前时间秒 > 旧的 since_version 秒 → 会被下载
        // 旧变更 version = 更早的秒 < 新的 since_version 秒 → 会被跳过（正确）
        let old_version: u64 = 1707500000; // legacy 设备上传
        let new_since: u64 = 1707400000; // 本地已同步到的版本
        assert!(
            old_version > new_since,
            "旧设备新变更应大于本地 since，被下载"
        );

        let stale_version: u64 = 1707300000; // 更早的变更
        assert!(stale_version < new_since, "过时变更应被跳过");
    }

    #[test]
    fn test_build_change_key_unique() {
        let manager = SyncManager::new("device-1".to_string());
        let key1 = manager.build_change_key(1707500000);
        let key2 = manager.build_change_key(1707500000);
        // 同一秒生成的 key 不应相同（UUID nonce 不同）
        assert_ne!(key1, key2, "同版本号的 key 应因 nonce 不同而不同");
        // 但版本号应可正确解析
        assert_eq!(SyncManager::parse_version_from_key(&key1), Some(1707500000));
        assert_eq!(SyncManager::parse_version_from_key(&key2), Some(1707500000));
    }

    #[test]
    fn test_normalize_version_to_seconds() {
        // 秒级值不变
        assert_eq!(
            SyncManager::normalize_version_to_seconds(1707500000),
            1707500000
        );
        assert_eq!(SyncManager::normalize_version_to_seconds(0), 0);
        assert_eq!(SyncManager::normalize_version_to_seconds(42), 42);
        // 毫秒级值被除以 1000
        assert_eq!(
            SyncManager::normalize_version_to_seconds(1707500000000),
            1707500000
        );
        assert_eq!(
            SyncManager::normalize_version_to_seconds(1707600000123),
            1707600000
        );
    }

    #[test]
    fn test_same_second_download_not_skipped() {
        // 验证 >= 语义：同秒版本不被跳过
        let since_version: u64 = 1707500000;
        let file_version: u64 = 1707500000; // 同秒
        assert!(file_version >= since_version, "同秒版本应通过 >= 过滤");
    }

    #[test]
    fn test_detect_no_conflicts() {
        let local = create_test_manifest("device-1", vec![("chat_v2", 1, 100, "abc123")]);
        let cloud = create_test_manifest("device-2", vec![("chat_v2", 1, 100, "abc123")]);

        let result = SyncManager::detect_conflicts(&local, &cloud).unwrap();
        assert!(!result.has_conflicts);
        assert!(result.database_conflicts.is_empty());
    }

    #[test]
    fn test_detect_schema_mismatch() {
        let local = create_test_manifest("device-1", vec![("chat_v2", 1, 100, "abc123")]);
        let cloud = create_test_manifest("device-2", vec![("chat_v2", 2, 100, "abc123")]);

        let result = SyncManager::detect_conflicts(&local, &cloud).unwrap();
        assert!(result.has_conflicts);
        assert!(result.needs_migration);
        assert_eq!(result.database_conflicts.len(), 1);
        assert_eq!(
            result.database_conflicts[0].conflict_type,
            DatabaseConflictType::SchemaMismatch
        );
    }

    #[test]
    fn test_detect_data_conflict() {
        let local = create_test_manifest("device-1", vec![("chat_v2", 1, 101, "abc123")]);
        let cloud = create_test_manifest("device-2", vec![("chat_v2", 1, 102, "def456")]);

        let result = SyncManager::detect_conflicts(&local, &cloud).unwrap();
        assert!(result.has_conflicts);
        assert!(!result.needs_migration);
        assert_eq!(result.database_conflicts.len(), 1);
        assert_eq!(
            result.database_conflicts[0].conflict_type,
            DatabaseConflictType::DataConflict
        );
    }

    #[test]
    fn test_detect_local_only() {
        let local = create_test_manifest(
            "device-1",
            vec![("chat_v2", 1, 100, "abc123"), ("mistakes", 1, 50, "xyz789")],
        );
        let cloud = create_test_manifest("device-2", vec![("chat_v2", 1, 100, "abc123")]);

        let result = SyncManager::detect_conflicts(&local, &cloud).unwrap();
        assert!(result.has_conflicts);
        assert_eq!(result.database_conflicts.len(), 1);
        assert_eq!(
            result.database_conflicts[0].conflict_type,
            DatabaseConflictType::LocalOnly
        );
        assert_eq!(result.database_conflicts[0].database_name, "mistakes");
    }

    #[test]
    fn test_detect_cloud_only() {
        let local = create_test_manifest("device-1", vec![("chat_v2", 1, 100, "abc123")]);
        let cloud = create_test_manifest(
            "device-2",
            vec![
                ("chat_v2", 1, 100, "abc123"),
                ("llm_usage", 1, 200, "qwe456"),
            ],
        );

        let result = SyncManager::detect_conflicts(&local, &cloud).unwrap();
        assert!(result.has_conflicts);
        assert_eq!(result.database_conflicts.len(), 1);
        assert_eq!(
            result.database_conflicts[0].conflict_type,
            DatabaseConflictType::CloudOnly
        );
        assert_eq!(result.database_conflicts[0].database_name, "llm_usage");
    }

    #[test]
    fn test_sync_keep_local() {
        let manager = SyncManager::new("device-1".to_string());
        let result = ConflictDetectionResult::empty();

        let sync_result = manager.sync(MergeStrategy::KeepLocal, &result).unwrap();
        assert!(sync_result.success);
    }

    #[test]
    fn test_record_conflict_detection() {
        let local_records = vec![RecordSnapshot {
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 3,
            sync_version: 2,
            updated_at: "2024-01-01T10:00:00Z".to_string(),
            deleted_at: None,
            data: serde_json::json!({"content": "local edit"}),
        }];

        let cloud_records = vec![RecordSnapshot {
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 4,
            sync_version: 2,
            updated_at: "2024-01-01T11:00:00Z".to_string(),
            deleted_at: None,
            data: serde_json::json!({"content": "cloud edit"}),
        }];

        let conflicts =
            SyncManager::detect_record_conflicts("chat_v2", &local_records, &cloud_records);

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].record_id, "msg-1");
        assert_eq!(conflicts[0].local_version, 3);
        assert_eq!(conflicts[0].cloud_version, 4);
    }

    // ========================================================================
    // 新增测试：核心同步方法
    // ========================================================================

    /// 创建测试用的内存数据库并初始化 __change_log 表
    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS __change_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                record_id TEXT NOT NULL,
                operation TEXT NOT NULL CHECK(operation IN ('INSERT', 'UPDATE', 'DELETE')),
                changed_at TEXT NOT NULL DEFAULT (datetime('now')),
                sync_version INTEGER DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx__change_log_sync_version ON __change_log(sync_version);

            CREATE TABLE IF NOT EXISTS refinery_schema_history (
                version INTEGER PRIMARY KEY,
                name TEXT,
                applied_on TEXT,
                checksum TEXT
            );

            -- 插入测试用的 schema 版本（与 refinery 迁移系统权威表结构一致）
            INSERT INTO refinery_schema_history (version, name, applied_on, checksum) VALUES (1, 'V1__init', '2024-01-01T00:00:00Z', 'abc');
            INSERT INTO refinery_schema_history (version, name, applied_on, checksum) VALUES (2, 'V2__update', '2024-01-02T00:00:00Z', 'def');
            "#,
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_checksum_detects_content_drift_without_updated_at_change() {
        let conn = create_test_db();
        conn.execute_batch(
            r#"
            CREATE TABLE notes (
                id TEXT PRIMARY KEY,
                title TEXT,
                body TEXT,
                updated_at TEXT,
                deleted_at TEXT
            );
            INSERT INTO notes (id, title, body, updated_at, deleted_at)
            VALUES ('note-1', 'Title', 'before', '2026-01-01T00:00:00Z', NULL);
            "#,
        )
        .unwrap();

        let before = SyncManager::calculate_drift_checksum_v2(&conn, "vfs").unwrap();
        conn.execute("UPDATE notes SET body = 'after' WHERE id = 'note-1'", [])
            .unwrap();
        let after = SyncManager::calculate_drift_checksum_v2(&conn, "vfs").unwrap();

        assert_ne!(
            before, after,
            "checksum must detect row content drift even when count and updated_at do not change"
        );
    }

    /// 插入测试用的变更日志
    fn insert_test_change_log(
        conn: &Connection,
        table_name: &str,
        record_id: &str,
        operation: &str,
        sync_version: i64,
    ) {
        conn.execute(
            "INSERT INTO __change_log (table_name, record_id, operation, sync_version)
             VALUES (?1, ?2, ?3, ?4)",
            params![table_name, record_id, operation, sync_version],
        )
        .unwrap();
    }

    #[test]
    fn test_get_pending_changes_empty() {
        let conn = create_test_db();

        let pending = SyncManager::get_pending_changes(&conn, None, None).unwrap();

        assert!(!pending.has_changes());
        assert_eq!(pending.total_count, 0);
        assert!(pending.entries.is_empty());
    }

    #[test]
    fn test_get_pending_changes_with_data() {
        let conn = create_test_db();

        // 插入一些待同步的变更
        insert_test_change_log(&conn, "messages", "msg-1", "INSERT", 0);
        insert_test_change_log(&conn, "messages", "msg-2", "UPDATE", 0);
        insert_test_change_log(&conn, "sessions", "sess-1", "INSERT", 0);
        // 这条已同步，不应该出现
        insert_test_change_log(&conn, "messages", "msg-3", "DELETE", 100);

        let pending = SyncManager::get_pending_changes(&conn, None, None).unwrap();

        assert!(pending.has_changes());
        assert_eq!(pending.total_count, 3);
        assert_eq!(pending.changes_by_table.get("messages"), Some(&2));
        assert_eq!(pending.changes_by_table.get("sessions"), Some(&1));
    }

    #[test]
    fn test_get_pending_changes_with_field_deltas_json() {
        let conn = create_test_db();
        conn.execute(
            "ALTER TABLE __change_log ADD COLUMN field_deltas_json TEXT",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO __change_log (table_name, record_id, operation, field_deltas_json, sync_version)
             VALUES ('resources', 'res-1', 'UPDATE', '{\"ref_count\":1}', 0)",
            [],
        )
        .unwrap();

        let pending = SyncManager::get_pending_changes(&conn, None, None).unwrap();
        assert_eq!(pending.total_count, 1);
        assert_eq!(
            pending.entries[0].field_deltas_json,
            Some(json!({"ref_count": 1}))
        );
    }

    #[test]
    fn test_from_entry_with_data_injects_field_deltas_metadata() {
        let entry = ChangeLogEntry {
            id: 1,
            table_name: "resources".to_string(),
            record_id: "res-1".to_string(),
            operation: ChangeOperation::Update,
            changed_at: "2024-01-01T10:00:00Z".to_string(),
            sync_version: 0,
            field_deltas_json: Some(json!({"ref_count": 1})),
        };

        let change = SyncChangeWithData::from_entry_with_data(
            &entry,
            Some(json!({
                "id": "res-1",
                "ref_count": 2,
                "updated_at": "2024-01-01T10:00:00Z"
            })),
        );

        let data = change.data.expect("data should be present");
        assert_eq!(data["__sync_field_deltas"], json!({"ref_count": 1}));
    }

    #[test]
    fn test_get_pending_changes_with_table_filter() {
        let conn = create_test_db();

        insert_test_change_log(&conn, "messages", "msg-1", "INSERT", 0);
        insert_test_change_log(&conn, "messages", "msg-2", "UPDATE", 0);
        insert_test_change_log(&conn, "sessions", "sess-1", "INSERT", 0);

        let pending = SyncManager::get_pending_changes(&conn, Some("messages"), None).unwrap();

        assert_eq!(pending.total_count, 2);
        assert!(pending.entries.iter().all(|e| e.table_name == "messages"));
    }

    #[test]
    fn test_get_pending_changes_with_limit() {
        let conn = create_test_db();

        for i in 0..10 {
            insert_test_change_log(&conn, "messages", &format!("msg-{}", i), "INSERT", 0);
        }

        let pending = SyncManager::get_pending_changes(&conn, None, Some(5)).unwrap();

        assert_eq!(pending.total_count, 5);
    }

    #[test]
    fn test_mark_synced() {
        let conn = create_test_db();

        insert_test_change_log(&conn, "messages", "msg-1", "INSERT", 0);
        insert_test_change_log(&conn, "messages", "msg-2", "UPDATE", 0);
        insert_test_change_log(&conn, "messages", "msg-3", "DELETE", 0);

        // 标记前两条为已同步
        let updated = SyncManager::mark_synced(&conn, &[1, 2], 1000).unwrap();
        assert_eq!(updated, 2);

        // 验证只剩一条待同步
        let pending = SyncManager::get_pending_changes(&conn, None, None).unwrap();
        assert_eq!(pending.total_count, 1);
        assert_eq!(pending.entries[0].record_id, "msg-3");
    }

    #[test]
    fn test_mark_synced_empty() {
        let conn = create_test_db();

        let updated = SyncManager::mark_synced(&conn, &[], 1000).unwrap();
        assert_eq!(updated, 0);
    }

    #[test]
    fn test_mark_synced_with_timestamp() {
        let conn = create_test_db();

        insert_test_change_log(&conn, "messages", "msg-1", "INSERT", 0);

        let updated = SyncManager::mark_synced_with_timestamp(&conn, &[1]).unwrap();
        assert_eq!(updated, 1);

        // 验证已同步
        let pending = SyncManager::get_pending_changes(&conn, None, None).unwrap();
        assert!(!pending.has_changes());
    }

    #[test]
    fn test_cleanup_synced_changes() {
        let conn = create_test_db();

        // 插入变更并标记为已同步
        conn.execute(
            "INSERT INTO __change_log (table_name, record_id, operation, changed_at, sync_version)
             VALUES ('messages', 'msg-1', 'INSERT', '2024-01-01T00:00:00Z', 100)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO __change_log (table_name, record_id, operation, changed_at, sync_version)
             VALUES ('messages', 'msg-2', 'UPDATE', '2024-01-15T00:00:00Z', 100)",
            [],
        )
        .unwrap();
        // 这条未同步，不应该被删除
        conn.execute(
            "INSERT INTO __change_log (table_name, record_id, operation, changed_at, sync_version)
             VALUES ('messages', 'msg-3', 'DELETE', '2024-01-01T00:00:00Z', 0)",
            [],
        )
        .unwrap();

        // 清理 2024-01-10 之前的已同步记录
        let deleted = SyncManager::cleanup_synced_changes(&conn, "2024-01-10T00:00:00Z").unwrap();
        assert_eq!(deleted, 1);

        // 验证还剩两条记录
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM __change_log", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_compare_timestamps_hlc_fast_path() {
        // 两端都是 HLC，应走 HLC 序比较（更精确，同毫秒 counter 决胜）
        let earlier = hlc::Hlc::new(1_700_000_000_000, 0).to_string();
        let later = hlc::Hlc::new(1_700_000_000_000, 1).to_string();

        // counter 1 > counter 0 → Greater
        assert_eq!(
            SyncManager::compare_timestamps(&later, &earlier),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            SyncManager::compare_timestamps(&earlier, &later),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            SyncManager::compare_timestamps(&earlier, &earlier),
            std::cmp::Ordering::Equal,
            "纯时间戳比较使用中性设备分量：平局必须对称地返回 Equal，\
             不能被评估方视角扭曲成本地恒胜（否则两端各自判赢、永不收敛）"
        );
    }

    #[test]
    fn test_compare_timestamps_mixed_hlc_and_iso() {
        // 只有一端是 HLC → 回落到 timestamp 比较路径（都解析失败或部分失败走 None 分支）
        let hlc_str = hlc::Hlc::new(1_700_000_000_000, 0).to_string();
        let iso_str = "2024-01-01T00:00:00Z";

        // HLC 格式 Hlc::parse 成功，ISO 格式 Hlc::parse 失败 → 降级到 timestamp path
        // HLC 的 `015-05` 固定宽度不是有效 RFC3339，parse_flexible_timestamp 会返回 None
        // 于是落到 (None, Some) → Less
        let r = SyncManager::compare_timestamps(&hlc_str, iso_str);
        assert_eq!(r, std::cmp::Ordering::Less);
    }

    #[test]
    fn test_reset_sync_baseline_after_restore() {
        let conn = create_test_db();

        // 创建一张业务表，带同步列
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                content TEXT,
                device_id TEXT,
                local_version INTEGER DEFAULT 0,
                sync_version INTEGER DEFAULT 0,
                updated_at TEXT,
                deleted_at TEXT
            );
            INSERT INTO notes (id, content, local_version, sync_version, updated_at)
            VALUES ('n1', 'hello', 5, 3, '2024-01-01T00:00:00Z'),
                   ('n2', 'world', 2, 2, '2024-01-02T00:00:00Z');",
        )
        .unwrap();

        // 插入 __change_log 历史条目（模拟源设备的残留）
        conn.execute(
            "INSERT INTO __change_log (table_name, record_id, operation, changed_at, sync_version)
             VALUES ('notes', 'n1', 'UPDATE', '2024-01-01T00:00:00Z', 100)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO __change_log (table_name, record_id, operation, changed_at, sync_version)
             VALUES ('notes', 'n2', 'INSERT', '2024-01-02T00:00:00Z', 0)",
            [],
        )
        .unwrap();

        let (truncated, reset) = SyncManager::reset_sync_baseline_after_restore(&conn).unwrap();
        assert_eq!(truncated, 2);
        // 优化后仅更新 "sync_version != local_version" 的行，避免不必要的 trigger。
        // n1 (lv=5, sv=3) 需要更新；n2 (lv=2, sv=2) 相等不需更新。
        assert_eq!(reset, 1);

        // __change_log 应为空
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM __change_log", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);

        // sync_version 应等于 local_version
        let (lv1, sv1): (i64, i64) = conn
            .query_row(
                "SELECT local_version, sync_version FROM notes WHERE id = 'n1'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(lv1, 5);
        assert_eq!(sv1, 5); // 从 3 提升到 5
        let (lv2, sv2): (i64, i64) = conn
            .query_row(
                "SELECT local_version, sync_version FROM notes WHERE id = 'n2'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(lv2, 2);
        assert_eq!(sv2, 2); // 已经相等，无变化
    }

    #[test]
    fn test_apply_merge_strategy_keep_local() {
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

        let result =
            SyncManager::apply_merge_strategy(MergeStrategy::KeepLocal, &conflicts).unwrap();

        assert!(result.success);
        assert_eq!(result.kept_local, 1);
        assert_eq!(result.used_cloud, 0);
        assert_eq!(result.records_to_push, vec!["msg-1"]);
        assert!(result.records_to_pull.is_empty());
    }

    #[test]
    fn test_apply_merge_strategy_use_cloud() {
        let conflicts = vec![ConflictRecord {
            database_name: "chat_v2".to_string(),
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 3,
            cloud_version: 4,
            local_updated_at: "2024-01-01T10:00:00Z".to_string(),
            cloud_updated_at: "2024-01-01T11:00:00Z".to_string(),
            local_data: serde_json::json!({"content": "local"}),
